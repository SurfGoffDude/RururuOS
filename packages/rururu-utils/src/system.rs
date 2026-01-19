use serde::{Deserialize, Serialize};
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Failed to get system info: {0}")]
    InfoError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub core_count: usize,
    pub frequency_mhz: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSummary {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime_seconds: u64,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
}

pub struct SystemInfo {
    sys: System,
}

impl SystemInfo {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn refresh_cpu(&mut self) {
        self.sys.refresh_cpu_all();
    }

    pub fn refresh_memory(&mut self) {
        self.sys.refresh_memory();
    }

    pub fn hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| "unknown".to_string())
    }

    pub fn os_name(&self) -> String {
        System::name().unwrap_or_else(|| "unknown".to_string())
    }

    pub fn os_version(&self) -> String {
        System::os_version().unwrap_or_else(|| "unknown".to_string())
    }

    pub fn kernel_version(&self) -> String {
        System::kernel_version().unwrap_or_else(|| "unknown".to_string())
    }

    pub fn uptime(&self) -> Duration {
        Duration::from_secs(System::uptime())
    }

    pub fn cpu_info(&self) -> CpuInfo {
        let cpus = self.sys.cpus();
        let first_cpu = cpus.first();

        let total_usage: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();
        let avg_usage = if cpus.is_empty() {
            0.0
        } else {
            total_usage / cpus.len() as f32
        };

        CpuInfo {
            name: first_cpu
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            vendor: first_cpu
                .map(|c| c.vendor_id().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            core_count: cpus.len(),
            frequency_mhz: first_cpu.map(|c| c.frequency()).unwrap_or(0),
            usage_percent: avg_usage,
        }
    }

    pub fn memory_info(&self) -> MemoryInfo {
        MemoryInfo {
            total_bytes: self.sys.total_memory(),
            used_bytes: self.sys.used_memory(),
            free_bytes: self.sys.free_memory(),
            available_bytes: self.sys.available_memory(),
            swap_total_bytes: self.sys.total_swap(),
            swap_used_bytes: self.sys.used_swap(),
        }
    }

    pub fn disk_info(&self) -> Vec<DiskInfo> {
        use sysinfo::Disks;
        let disks = Disks::new_with_refreshed_list();
        
        disks
            .iter()
            .map(|d| DiskInfo {
                name: d.name().to_string_lossy().to_string(),
                mount_point: d.mount_point().to_string_lossy().to_string(),
                file_system: d.file_system().to_string_lossy().to_string(),
                total_bytes: d.total_space(),
                available_bytes: d.available_space(),
                is_removable: d.is_removable(),
            })
            .collect()
    }

    pub fn process_list(&self) -> Vec<ProcessInfo> {
        self.sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcessInfo {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
                status: format!("{:?}", proc.status()),
            })
            .collect()
    }

    pub fn top_processes_by_cpu(&self, count: usize) -> Vec<ProcessInfo> {
        let mut procs = self.process_list();
        procs.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        procs.truncate(count);
        procs
    }

    pub fn top_processes_by_memory(&self, count: usize) -> Vec<ProcessInfo> {
        let mut procs = self.process_list();
        procs.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
        procs.truncate(count);
        procs
    }

    pub fn summary(&self) -> SystemSummary {
        SystemSummary {
            hostname: self.hostname(),
            os_name: self.os_name(),
            os_version: self.os_version(),
            kernel_version: self.kernel_version(),
            uptime_seconds: System::uptime(),
            cpu: self.cpu_info(),
            memory: self.memory_info(),
            disks: self.disk_info(),
        }
    }

    pub fn is_low_memory(&self) -> bool {
        let mem = self.memory_info();
        let usage = (mem.used_bytes as f64) / (mem.total_bytes as f64);
        usage > 0.9
    }

    pub fn is_high_cpu_usage(&self) -> bool {
        let cpu = self.cpu_info();
        cpu.usage_percent > 90.0
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_creation() {
        let info = SystemInfo::new();
        assert!(!info.hostname().is_empty());
    }

    #[test]
    fn test_memory_info() {
        let info = SystemInfo::new();
        let mem = info.memory_info();
        assert!(mem.total_bytes > 0);
    }

    #[test]
    fn test_cpu_info() {
        let info = SystemInfo::new();
        let cpu = info.cpu_info();
        assert!(cpu.core_count > 0);
    }
}
