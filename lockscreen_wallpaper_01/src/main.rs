use std::path::PathBuf;
use std::env;
use winreg::enums::*;
use winreg::RegKey;

fn set_lock_screen_wallpaper(path: &PathBuf) {
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let personalization_csp = hkcu
        .open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PersonalizationCSP",
            KEY_WRITE,
        )
        .expect("Failed to open registry key");

    personalization_csp
        .set_value("LockScreenImagePath", &path.to_str().unwrap())
        .expect("Failed to set lock screen wallpaper");
  // 写入十六进制值 0 // 将 0 转换为 u32 类型

  personalization_csp
        .set_value("LockScreenImageStatus", &0u32)
        .expect("Failed to set lock screen wallpaper");


    personalization_csp
        .set_value("LockScreenImageUrl", &path.to_str().unwrap())
        .expect("Failed to set lock screen wallpaper");

    println!("Successfully set lock screen wallpaper in registry.");
}

fn remove_lock_screen_wallpaper() {
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let personalization_csp = hkcu
        .open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PersonalizationCSP",
            KEY_WRITE,
        )
        .expect("Failed to open registry key");

    personalization_csp
        .delete_value("LockScreenImagePath")
        .expect("Failed to delete lock screen wallpaper path");

    personalization_csp
        .delete_value("LockScreenImageStatus")
        .expect("Failed to delete lock screen image status");

    personalization_csp
        .delete_value("LockScreenImageUrl")
        .expect("Failed to delete lock screen image URL");

    println!("Successfully removed lock screen wallpaper from registry.");
}

fn main() {
    // let wallpaper_path = PathBuf::from("C:\\Users\\project\\wallpaper.jpg");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: program_name [set|remove] [path]");
        return;
    }

    match args[1].as_str() {
        "set" => {
            if args.len() != 3 {
                eprintln!("Usage: program_name set path");
                return;
            }
            let wallpaper_path = PathBuf::from(&args[2]);
            set_lock_screen_wallpaper(&wallpaper_path);
        }
        "remove" => {
            remove_lock_screen_wallpaper();
        }
        _ => {
            eprintln!("Invalid command. Usage: program_name [set|remove] [path]");
        }
    }
}
