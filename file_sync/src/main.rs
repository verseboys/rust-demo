use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// 复制文件的函数
fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::copy(src, dest)?;
    Ok(())
}

// 同步目录的函数
fn sync_dir(src_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    for entry in WalkDir::new(src_dir) {
        let entry = entry?;
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(src_dir).unwrap();
        let dest_path = dest_dir.join(relative_path);

        if src_path.is_file() {
            copy_file(src_path, &dest_path)?;
        } else if src_path.is_dir() {
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否有正确的参数传递
    if args.len() != 3 {
        println!("Usage: file_sync <source_dir> <destination_dir>");
        return Ok(());
    }

    // 获取源文件夹和目标文件夹路径
    let src_dir = PathBuf::from(&args[1]);
    let dest_dir = PathBuf::from(&args[2]);

    println!("同步文件夹: {:?} -> {:?}", src_dir, dest_dir);

    // 执行同步操作
    match sync_dir(&src_dir, &dest_dir) {
        Ok(_) => println!("同步成功!"),
        Err(e) => writeln!(io::stderr(), "同步失败: {}", e).unwrap(),
    }

    Ok(())
}
