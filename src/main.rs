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
    
    if displays.is_empty() {
        eprintln!("Could not find any displays");
        process::exit(1);
    }
        
    for (_index, display) in displays.iter_mut().enumerate() {
        let _ = display.handle.set_vcp_feature(0x10, brightness);
    }
}