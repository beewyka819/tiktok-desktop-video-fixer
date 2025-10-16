use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("TikTok Video Encoding Fixer");
    println!("========================");
    
    let input_file = get_input("Enter input file name (e.g., input.mp4): ");
    
    if !Path::new(&input_file).exists() {
        eprintln!("Error: Input file '{}' does not exist in the current directory.", input_file);
        std::process::exit(1);
    }
    
    let output_file = get_input("Enter output file name (e.g., output.mp4): ");
    
    if Path::new(&output_file).exists() {
        eprintln!("Error: Output file '{}' already exists. Please choose a different name.", output_file);
        std::process::exit(1);
    }
    
    let ffmpeg_path = get_ffmpeg();
    
    let result = Command::new(&ffmpeg_path)
        .arg("-i")
        .arg(&input_file)
        .arg("-vf")
        .arg("setpts=0.5*PTS")
        .arg("-af")
        .arg("atempo=1.0")
        .arg(&output_file)
        .status();
    
    match result {
        Ok(exit_status) => {
            if !exit_status.success() {
                eprintln!("FFmpeg failed with exit code: {:?}", exit_status.code());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error executing FFmpeg: {}", e);
            std::process::exit(1);
        }
    }

    println!("\nVideo fixed successfully!");
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn get_ffmpeg() -> PathBuf {
    // Get the directory where the executable is located
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let ffmpeg_file_name = if cfg!(target_os = "windows") {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    };
    
    // Vendor path relative to executable
    let vendor_dir = exe_dir.join("vendor").join("ffmpeg");
    let ffmpeg_path = vendor_dir.join(ffmpeg_file_name);
    
    // If ffmpeg already exists locally, then return the path immediately.
    if ffmpeg_path.exists() {
        return ffmpeg_path;
    }
    
    println!("Local FFmpeg distribution not found. Downloading...");
    
    // Create vendor directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&vendor_dir) {
        eprintln!("Error: Failed to create vendor directory: {}", e);
        std::process::exit(1);
    }
    
    // Download and extract FFmpeg
    if let Err(e) = download_ffmpeg(&vendor_dir) {
        eprintln!("Error: Failed to download FFmpeg: {}", e);
        std::process::exit(1);
    }
    
    if !ffmpeg_path.exists() {
        eprintln!("Error: FFmpeg executable not found after download.");
        std::process::exit(1);
    }
    
    println!("FFmpeg downloaded successfully.");
    ffmpeg_path
}

fn download_ffmpeg(vendor_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (download_url, is_zip) = if cfg!(target_os = "windows") {
        ("https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip", true)
    } else if cfg!(target_os = "linux") {
        ("https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz", false)
    } else {
        return Err("Unsupported operating system".into());
    };
    
    println!("Downloading from: {}", download_url);
    println!("This may take a few minutes...");
    
    let response = reqwest::blocking::get(download_url)?;
    let bytes = response.bytes()?;
    
    // Save to temporary file
    let temp_file = vendor_dir.join("ffmpeg_download.tmp");
    fs::write(&temp_file, &bytes)?;
    
    if is_zip {
        extract_zip(&temp_file, vendor_dir)?;
    } else {
        extract_tar_xz(&temp_file, vendor_dir)?;
    }
    
    // Clean up temp file
    let _ = fs::remove_file(temp_file);
    
    Ok(())
}

fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    // Find and extract ffmpeg.exe
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = file.name();
        
        if file_path.ends_with("ffmpeg.exe") {
            let output_path = dest_dir.join("ffmpeg.exe");
            let mut output_file = fs::File::create(output_path)?;
            io::copy(&mut file, &mut output_file)?;
            break;
        }
    }
    
    Ok(())
}

fn extract_tar_xz(tar_path: &Path, dest_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Extract using tar command
    let status = Command::new("tar")
        .arg("-xJf")
        .arg(tar_path)
        .arg("-C")
        .arg(dest_dir)
        .arg("--strip-components=1")
        .arg("--wildcards")
        .arg("*/ffmpeg")
        .status()?;
    
    if !status.success() {
        return Err("Failed to extract tar.xz archive".into());
    }
    
    // Make FFmpeg executable
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::PermissionsExt;
        let ffmpeg_path = dest_dir.join("ffmpeg");
        let mut perms = fs::metadata(&ffmpeg_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&ffmpeg_path, perms)?;
    }
    
    Ok(())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    input.trim().to_string()
}
