use std::process;
use std::env;
use ddc_hi::Ddc;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: <brightness>");
        eprintln!("1-100");
        process::exit(1);
    }
    
    let brightness = match args[1].parse::<u16>() {
        Ok(val) => {
            if val < 1 || val > 100 {
                eprintln!("Must be between 1 and 100");
                process::exit(1);
            }
            val
        },
        Err(_) => {
            eprintln!("Must be between 1 and 100");
            process::exit(1);
        }
    };
    
    let mut displays = ddc_hi::Display::enumerate();
    let mut success = false;
        
    for (_index, display) in displays.iter_mut().enumerate() {
        if display.handle.set_vcp_feature(0x10, brightness).is_ok() {
            success = true;
        }
    }
    
    // Built-in laptop screens usually don't support DDC
    match fallback(brightness) {
        Ok(_) => {},
        Err(e) => {
            if !success {
                eprintln!("Could not set brightness: {}", e);
                process::exit(1);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn fallback(brightness: u16) -> Result<(), String> {
    let command = format!(
        "(Get-WmiObject -Namespace root/wmi -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1, {})",
        brightness
    );
    
    let status = match process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &command])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status() {
        Ok(status) => status,
        Err(e) => return Err(e.to_string()),
    };
    
    if !status.success() {
        return Err("Could not find any displays".to_string());
    }
    
    Ok(())
}

#[cfg(target_os = "linux")]
fn fallback(_brightness: u16) -> Result<(), String> {
    // TODO
    Err("Could not find any displays".to_string())
}

#[cfg(target_os = "macos")]
fn fallback(_brightness: u16) -> Result<(), String> {
    // TODO
    Err("Could not find any displays".to_string())
}
