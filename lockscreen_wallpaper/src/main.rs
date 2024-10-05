use std::path::PathBuf;
use windows::core::Result;
use windows::Win32::System::Settings::SetLockScreenWallpaper;
use windows::core::HSTRING;

fn set_lock_screen_wallpaper(path: PathBuf) -> Result<()> {
    // 获取图片的绝对路径
    let abs_path = path.canonicalize().expect("Could not get the canonical form of the path.");

    // 转换路径为 HSTRING
    let hstring_path = HSTRING::from(abs_path.to_str().unwrap());

    // 调用 Windows API 来设置锁屏壁纸
    unsafe {
        SetLockScreenWallpaper(&hstring_path)?;
    }

    println!("Successfully set lock screen wallpaper to {:?}", abs_path);

    Ok(())
}

fn main() -> Result<()> {
    // 设置壁纸的路径
    let wallpaper_path = PathBuf::from("C:\\path\\to\\your\\wallpaper.jpg");

    // 调用设置锁屏壁纸的函数
    set_lock_screen_wallpaper(wallpaper_path)?;

    Ok(())
}
