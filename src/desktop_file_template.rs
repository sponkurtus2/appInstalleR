use clap::Parser;
use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::fs;
use dirs;

/// Command line arguments for the AppImage desktop entry creator
#[derive(Debug, Parser)]
struct Args {
    /// Path to the AppImage file
    #[arg(short, long)]
    app_file: PathBuf,
}

/// Validates and returns the path to the AppImage file
pub fn take_app_image() -> PathBuf {
    let args = Args::parse();

    // Check if file exists
    if !args.app_file.exists() {
        eprintln!("Error, file -> {} does not exist", args.app_file.display());
        std::process::exit(1);
    }

    // Check if the file is an .AppImage
    let file_extension = Path::new(&args.app_file)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    if file_extension != "AppImage" {
        eprintln!(
            "Error, file -> {}, is not an AppImage",
            args.app_file.display()
        );
        std::process::exit(1);
    }

    args.app_file
}

/// Gets the full path to the AppImage file
pub fn get_app_path(file_name: PathBuf) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    file_path
}

/// Creates a .desktop file for the AppImage
pub fn create_desktop_file() -> io::Result<()> {
    // Get the AppImage path
    let app_image = take_app_image();

    // Get the app name without the .AppImage extension
    let app_name = app_image
        .file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid filename"))?;

    // Create the desktop file path in the user's applications directory
    let apps_dir = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?
        .join(".local/share/applications");

    // Create applications directory if it doesn't exist
    fs::create_dir_all(&apps_dir)?;

    let desktop_path = apps_dir.join(format!("{}.desktop", app_name));

    // Get the absolute path for the Exec field
    let file_path = get_app_path(app_image.clone());

    // Create the desktop entry content
    let content = format!(
        r#"[Desktop Entry]
Name={}
Comment=Misc application
Exec={}
Terminal=false
Type=Application
"#,
        app_name,
        file_path.display(),
    );

    // Create and write to the file
    fs::write(&desktop_path, content)?;

    // Make the file executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&desktop_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&desktop_path, perms)?;
    }

    println!("Created desktop entry: {}", desktop_path.display());
    Ok(())
}