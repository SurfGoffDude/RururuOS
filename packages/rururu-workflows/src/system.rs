use crate::{Result, WorkflowError};
use crate::profiles::{CpuGovernor, IoScheduler, SwapUsage, SystemSettings};
use std::fs;
use std::path::Path;

pub fn apply_system_settings(settings: &SystemSettings) -> Result<()> {
    set_cpu_governor(settings.cpu_governor)?;
    set_swap_usage(settings.swap_usage)?;
    
    if settings.realtime_audio {
        configure_realtime_audio()?;
    }
    
    for process in &settings.high_priority_processes {
        set_process_priority(process, -10)?;
    }
    
    Ok(())
}

pub fn set_cpu_governor(governor: CpuGovernor) -> Result<()> {
    let governor_str = match governor {
        CpuGovernor::Performance => "performance",
        CpuGovernor::Powersave => "powersave",
        CpuGovernor::Schedutil => "schedutil",
        CpuGovernor::Ondemand => "ondemand",
    };
    
    // Find all CPU cores
    let cpufreq_path = Path::new("/sys/devices/system/cpu/cpufreq");
    if !cpufreq_path.exists() {
        return Ok(()); // No cpufreq support
    }
    
    if let Ok(entries) = fs::read_dir(cpufreq_path) {
        for entry in entries.flatten() {
            let governor_path = entry.path().join("scaling_governor");
            if governor_path.exists() {
                // This requires root privileges
                let _ = fs::write(&governor_path, governor_str);
            }
        }
    }
    
    // Alternative: use cpupower
    let _ = std::process::Command::new("sudo")
        .args(["cpupower", "frequency-set", "-g", governor_str])
        .status();
    
    Ok(())
}

pub fn set_swap_usage(usage: SwapUsage) -> Result<()> {
    let swappiness = match usage {
        SwapUsage::Minimal => 10,
        SwapUsage::Balanced => 60,
        SwapUsage::Aggressive => 100,
    };
    
    // Try sysctl
    let _ = std::process::Command::new("sudo")
        .args(["sysctl", &format!("vm.swappiness={}", swappiness)])
        .status();
    
    Ok(())
}

pub fn set_io_scheduler(scheduler: IoScheduler, device: &str) -> Result<()> {
    let scheduler_str = match scheduler {
        IoScheduler::Bfq => "bfq",
        IoScheduler::Mq_Deadline => "mq-deadline",
        IoScheduler::Kyber => "kyber",
        IoScheduler::None => "none",
    };
    
    let scheduler_path = format!("/sys/block/{}/queue/scheduler", device);
    let _ = fs::write(&scheduler_path, scheduler_str);
    
    Ok(())
}

pub fn configure_realtime_audio() -> Result<()> {
    // Set PipeWire for low latency
    if let Some(config_dir) = dirs::config_dir() {
        let pipewire_conf = config_dir.join("pipewire/pipewire.conf.d/10-realtime.conf");
        
        if let Some(parent) = pipewire_conf.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        let config = r#"
context.properties = {
    default.clock.rate = 48000
    default.clock.quantum = 64
    default.clock.min-quantum = 32
    default.clock.max-quantum = 1024
}
"#;
        
        let _ = fs::write(pipewire_conf, config);
    }
    
    // Add user to audio group if not already
    let _ = std::process::Command::new("sudo")
        .args(["usermod", "-aG", "audio", &whoami()])
        .status();
    
    // Set rtkit limits
    let limits_conf = "/etc/security/limits.d/99-realtime.conf";
    let limits = format!(
        "@audio - rtprio 99\n@audio - memlock unlimited\n{} - rtprio 99\n{} - memlock unlimited\n",
        whoami(),
        whoami()
    );
    
    let _ = std::process::Command::new("sudo")
        .args(["tee", limits_conf])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(limits.as_bytes());
            }
            child.wait()
        });
    
    Ok(())
}

pub fn set_process_priority(process_name: &str, priority: i32) -> Result<()> {
    // Find process ID
    let output = std::process::Command::new("pgrep")
        .arg(process_name)
        .output()?;
    
    if output.status.success() {
        let pids = String::from_utf8_lossy(&output.stdout);
        for pid in pids.lines() {
            let _ = std::process::Command::new("renice")
                .args([&priority.to_string(), "-p", pid])
                .status();
        }
    }
    
    Ok(())
}

pub fn set_gpu_performance_mode(enabled: bool) -> Result<()> {
    // NVIDIA
    if Path::new("/usr/bin/nvidia-smi").exists() {
        let mode = if enabled { "1" } else { "0" };
        let _ = std::process::Command::new("nvidia-smi")
            .args(["-pm", mode])
            .status();
        
        if enabled {
            let _ = std::process::Command::new("nvidia-smi")
                .args(["-lgc", "0,9999"])
                .status();
        }
    }
    
    // AMD
    let amd_perf = "/sys/class/drm/card0/device/power_dpm_force_performance_level";
    if Path::new(amd_perf).exists() {
        let mode = if enabled { "high" } else { "auto" };
        let _ = fs::write(amd_perf, mode);
    }
    
    Ok(())
}

fn whoami() -> String {
    std::env::var("USER").unwrap_or_else(|_| "user".to_string())
}

pub fn get_system_info() -> SystemInfo {
    let cpu_count = num_cpus();
    let memory_total = total_memory();
    let gpu_info = detect_gpu();
    
    SystemInfo {
        cpu_count,
        memory_total_gb: memory_total / 1024 / 1024 / 1024,
        gpu: gpu_info,
        has_nvidia: Path::new("/usr/bin/nvidia-smi").exists(),
        has_amd: Path::new("/sys/class/drm/card0/device/vendor").exists(),
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu_count: usize,
    pub memory_total_gb: u64,
    pub gpu: String,
    pub has_nvidia: bool,
    pub has_amd: bool,
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1)
}

fn total_memory() -> u64 {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(kb) = line.split_whitespace().nth(1) {
                    return kb.parse::<u64>().unwrap_or(0) * 1024;
                }
            }
        }
    }
    0
}

fn detect_gpu() -> String {
    if let Ok(output) = std::process::Command::new("lspci")
        .args(["-nnk"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("VGA") || line.contains("3D") {
                return line.to_string();
            }
        }
    }
    "Unknown".to_string()
}
