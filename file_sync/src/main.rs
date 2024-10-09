use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// 复制文件的函数
fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    // 检查目标路径是否是文件夹，如果不存在则创建
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // 拷贝文件
    fs::copy(src, dest)?;
    Ok(())
}

// 同步目录的函数
fn sync_dir(src_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    for entry in WalkDir::new(src_dir) {
        let entry = entry?;
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(src_dir).unwrap(); // 获取相对路径
        let dest_path = dest_dir.join(relative_path); // 目标路径

        if src_path.is_file() {
            copy_file(src_path, &dest_path)?;
        } else if src_path.is_dir() {
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?; // 如果目标文件夹不存在，创建它
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // 指定源文件夹和目标文件夹路径
    let src_dir = PathBuf::from("path/to/source/folder"); // 源文件夹路径
    let dest_dir = PathBuf::from("path/to/destination/folder"); // 目标文件夹路径

    println!("同步文件夹: {:?} -> {:?}", src_dir, dest_dir);

    // 执行同步操作
    match sync_dir(&src_dir, &dest_dir) {
        Ok(_) => println!("同步成功!"),
        Err(e) => writeln!(io::stderr(), "同步失败: {}", e).unwrap(),
    }

    Ok(())
}
