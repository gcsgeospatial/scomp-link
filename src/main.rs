use clap::{Arg, Command};
use std::fs;
use std::process;
use scomp_link::*;

/// Generate target images with specified parameters and save them as PNG files.
fn generate_targets(
    bits: u32,
    output_dir: &str,
    radius_inner_dot: u32,
    radius_inner_black: u32,
    radius_outer_white: u32,
    radius_outer_black: u32,
    width: u32,
    height: u32,
    transitions: Option<u32>,
    max_codes: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate bits parameter first (matching Python behavior)
    if bits <= 0 || bits % 2 != 0 {
        eprintln!("Error: Number of bits must be positive and even.");
        return Ok(());
    }
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;
    
    // Check if ImageMagick is available
    if process::Command::new("magick").arg("--version").output().is_err() {
        eprintln!("Error: ImageMagick is not installed or not in the system's PATH.");
        return Ok(());
    }
    
    let codes = generate_codes(bits, transitions, max_codes);
    println!("{:?}", codes);
    
    for code in codes {
        let center = (width as f64 / 2.0, height as f64 / 2.0);
        let radius_outer = radius_outer_black + 2; // Example outer radius, adjust based on needs
        
        let arc_commands = generate_arc_commands(code, bits, center, radius_outer as f64);
        
        let filename = format!("{}/{}.png", output_dir, code);
        
        // Construct the ImageMagick command using the provided radii and other parameters
        let outer_black_circle = format!(
            "-fill black -draw \"circle {}, {} {}, {}\"",
            center.0 as i32, center.1 as i32,
            center.0 as i32, center.1 as i32 + radius_outer_black as i32
        );
        let outer_white_circle = format!(
            "-fill white -draw \"circle {}, {} {}, {}\"",
            center.0 as i32, center.1 as i32,
            center.0 as i32, center.1 as i32 + radius_outer_white as i32
        );
        let inner_black_circle = format!(
            "-fill black -draw \"circle {}, {} {}, {}\"",
            center.0 as i32, center.1 as i32,
            center.0 as i32, center.1 as i32 + radius_inner_black as i32
        );
        let inner_white_circle = format!(
            "-fill white -draw \"circle {}, {} {}, {}\"",
            center.0 as i32, center.1 as i32,
            center.0 as i32, center.1 as i32 + radius_inner_dot as i32
        );
        
        let command = format!(
            "magick -size {}x{} xc:white {} {} {} {} {} {}",
            width, height,
            outer_black_circle,
            arc_commands,
            outer_white_circle,
            inner_black_circle,
            inner_white_circle,
            filename
        );
        
        // Execute the command cross-platform
        let output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .args(["/C", &command])
                .output()
        } else {
            process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
        };
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Generated {}", filename);
                } else {
                    eprintln!("Error generating {}: {}", filename, String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
                eprintln!("Command: {}", command);
                if cfg!(target_os = "windows") {
                    eprintln!("Make sure ImageMagick is installed and 'magick' is in your PATH");
                } else {
                    eprintln!("Make sure ImageMagick is installed and accessible");
                }
                return Err(e.into());
            }
        }
    }
    
    Ok(())
}

fn main() {
    let matches = Command::new("scomp-link")
        .about("Photogrammetry Target Generator - generates circular photogrammetry targets with encoded bit patterns")
        .version("1.0.0")
        .arg(Arg::new("bits")
            .long("bits")
            .value_name("INTEGER")
            .help("Number of bits to encode")
            .default_value("12"))
        .arg(Arg::new("output-dir")
            .long("output-dir")
            .value_name("TEXT")
            .help("Directory where PNG files will be written")
            .default_value("."))
        .arg(Arg::new("radius-inner-dot")
            .long("radius-inner-dot")
            .value_name("INTEGER")
            .help("Radius of the inner dot")
            .default_value("24"))
        .arg(Arg::new("radius-inner-black")
            .long("radius-inner-black")
            .value_name("INTEGER")
            .help("Radius of the inner black circle")
            .default_value("288"))
        .arg(Arg::new("radius-outer-white")
            .long("radius-outer-white")
            .value_name("INTEGER")
            .help("Radius of the outer white circle")
            .default_value("660"))
        .arg(Arg::new("radius-outer-black")
            .long("radius-outer-black")
            .value_name("INTEGER")
            .help("Radius of the outer black circle")
            .default_value("1032"))
        .arg(Arg::new("width")
            .long("width")
            .value_name("INTEGER")
            .help("Width of the PNG")
            .default_value("3000"))
        .arg(Arg::new("height")
            .long("height")
            .value_name("INTEGER")
            .help("Height of the PNG")
            .default_value("3000"))
        .arg(Arg::new("transitions")
            .long("transitions")
            .value_name("INTEGER")
            .help("Optional number of bit transitions"))
        .arg(Arg::new("max-codes")
            .long("max-codes")
            .value_name("INTEGER")
            .help("Maximum number of codes to generate"))
        .get_matches();
    
    let bits: u32 = matches.get_one::<String>("bits").unwrap().parse().unwrap_or(12);
    let output_dir = matches.get_one::<String>("output-dir").unwrap();
    let radius_inner_dot: u32 = matches.get_one::<String>("radius-inner-dot").unwrap().parse().unwrap_or(24);
    let radius_inner_black: u32 = matches.get_one::<String>("radius-inner-black").unwrap().parse().unwrap_or(288);
    let radius_outer_white: u32 = matches.get_one::<String>("radius-outer-white").unwrap().parse().unwrap_or(660);
    let radius_outer_black: u32 = matches.get_one::<String>("radius-outer-black").unwrap().parse().unwrap_or(1032);
    let width: u32 = matches.get_one::<String>("width").unwrap().parse().unwrap_or(3000);
    let height: u32 = matches.get_one::<String>("height").unwrap().parse().unwrap_or(3000);
    
    let transitions = matches.get_one::<String>("transitions").and_then(|s| s.parse().ok());
    let max_codes = matches.get_one::<String>("max-codes").and_then(|s| s.parse().ok());
    
    if let Err(e) = generate_targets(
        bits,
        output_dir,
        radius_inner_dot,
        radius_inner_black,
        radius_outer_white,
        radius_outer_black,
        width,
        height,
        transitions,
        max_codes,
    ) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
