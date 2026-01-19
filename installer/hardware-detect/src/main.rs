use rururu_hardware_detect::{detect_all, generate_report};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let format = args.get(1).map(|s| s.as_str()).unwrap_or("text");
    
    let info = detect_all();
    
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&info).unwrap());
        }
        "markdown" | "md" => {
            println!("{}", generate_report(&info));
        }
        _ => {
            print_text(&info);
        }
    }
}

fn print_text(info: &rururu_hardware_detect::HardwareInfo) {
    println!("RururuOS Hardware Detection");
    println!("===========================\n");
    
    println!("CPU: {} ({:?})", info.cpu.model, info.cpu.arch);
    println!("     {} cores, {} threads", info.cpu.cores, info.cpu.threads);
    println!();
    
    println!("GPU:");
    for gpu in &info.gpu {
        print!("  - {} ({:?})", gpu.name, gpu.vendor);
        if let Some(vram) = gpu.vram_mb {
            print!(", {} MB VRAM", vram);
        }
        println!();
    }
    println!();
    
    println!("Memory: {} GB {:?}", info.memory.total_gb, info.memory.memory_type);
    println!();
    
    println!("Storage:");
    for disk in &info.storage {
        println!("  - {} ({:?}): {} GB", disk.name, disk.storage_type, disk.size_gb);
    }
    println!();
    
    println!("Displays:");
    for display in &info.displays {
        print!("  - {} ({})", display.name, display.connector);
        if let Some((w, h)) = display.resolution {
            print!(", {}x{}", w, h);
        }
        if display.hdr_capable {
            print!(", HDR");
        }
        println!();
    }
    println!();
    
    println!("Audio: {:?}", info.audio.server);
    println!("  Devices: {}", info.audio.devices.len());
    println!("  Low-latency capable: {}", info.audio.latency_capable);
    println!();
    
    if !info.recommendations.is_empty() {
        println!("Recommendations:");
        for rec in &info.recommendations {
            let priority = match rec.priority {
                rururu_hardware_detect::Priority::Critical => "[CRITICAL]",
                rururu_hardware_detect::Priority::High => "[HIGH]",
                rururu_hardware_detect::Priority::Medium => "[MEDIUM]",
                rururu_hardware_detect::Priority::Low => "[LOW]",
            };
            println!("  {} {}: {}", priority, rec.title, rec.description);
            if let Some(action) = &rec.action {
                println!("      Run: {}", action);
            }
        }
    }
}
