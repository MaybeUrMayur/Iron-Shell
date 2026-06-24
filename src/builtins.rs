use std::env;
use std::fs;
use std::io;
use std::path::Path;

pub fn cd(args: &[String]) -> (bool, String) {
    let new_dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].clone()
    };

    let path = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(path) {
        return (true, format!("iron_shell: cd: {}: {}", new_dir, e));
    }
    (true, String::new())
}

pub fn exit(_args: &[String]) -> (bool, String) {
    (false, String::new())
}

pub fn create(args: &[String]) -> (bool, String) {
    if args.len() < 2 {
        return (true, "iron_shell: usage: create <folder|file> <name>".to_string());
    }
    let target = &args[0];
    let name = &args[1];
    
    match target.as_str() {
        "folder" => {
            if let Err(e) = fs::create_dir(name) {
                return (true, format!("iron_shell: failed to create folder {}: {}", name, e));
            }
        }
        "file" => {
            if let Err(e) = fs::File::create(name) {
                return (true, format!("iron_shell: failed to create file {}: {}", name, e));
            }
        }
        _ => return (true, format!("iron_shell: unknown target '{}', use 'folder' or 'file'", target)),
    }
    (true, String::new())
}

pub fn delete(args: &[String]) -> (bool, String) {
    if args.len() < 2 {
        return (true, "iron_shell: usage: delete <folder|file> <name>".to_string());
    }
    let target = &args[0];
    let name = &args[1];
    
    match target.as_str() {
        "folder" => {
            if let Err(e) = fs::remove_dir_all(name) {
                return (true, format!("iron_shell: failed to delete folder {}: {}", name, e));
            }
        }
        "file" => {
            if let Err(e) = fs::remove_file(name) {
                return (true, format!("iron_shell: failed to delete file {}: {}", name, e));
            }
        }
        _ => return (true, format!("iron_shell: unknown target '{}', use 'folder' or 'file'", target)),
    }
    (true, String::new())
}

pub fn list(args: &[String]) -> (bool, String) {
    if args.is_empty() || args[0] != "folder" {
        return (true, "iron_shell: usage: list folder [path]".to_string());
    }
    
    let path = if args.len() > 1 { &args[1] } else { "." };
    let mut out = String::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(e) = entry {
                    let file_name = e.file_name();
                    out.push_str(&file_name.to_string_lossy());
                    out.push('\n');
                }
            }
        }
        Err(e) => return (true, format!("iron_shell: failed to list folder {}: {}", path, e)),
    }
    (true, out.trim_end().to_string())
}

pub fn read(args: &[String]) -> (bool, String) {
    if args.len() < 2 || args[0] != "file" {
        return (true, "iron_shell: usage: read file <name>".to_string());
    }
    let name = &args[1];
    
    match fs::read_to_string(name) {
        Ok(contents) => (true, contents),
        Err(e) => (true, format!("iron_shell: failed to read file {}: {}", name, e)),
    }
}

pub fn copy(args: &[String]) -> (bool, String) {
    if args.len() < 3 {
        return (true, "iron_shell: usage: copy <file|folder> <src> <dest>".to_string());
    }
    let target = &args[0];
    let src = &args[1];
    let dest = &args[2];
    
    match target.as_str() {
        "file" => {
            if let Err(e) = fs::copy(src, dest) {
                return (true, format!("iron_shell: failed to copy file from {} to {}: {}", src, dest, e));
            }
        }
        "folder" => {
            if let Err(e) = copy_dir_all(src, dest) {
                return (true, format!("iron_shell: failed to copy folder from {} to {}: {}", src, dest, e));
            }
        }
        _ => return (true, format!("iron_shell: unknown target '{}', use 'folder' or 'file'", target)),
    }
    (true, String::new())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn move_cmd(args: &[String]) -> (bool, String) {
    if args.len() < 3 {
        return (true, "iron_shell: usage: move <file|folder> <src> <dest>".to_string());
    }
    let target = &args[0];
    let src = &args[1];
    let dest = &args[2];
    
    match target.as_str() {
        "file" | "folder" => {
            if let Err(e) = fs::rename(src, dest) {
                return (true, format!("iron_shell: failed to move {} from {} to {}: {}", target, src, dest, e));
            }
        }
        _ => return (true, format!("iron_shell: unknown target '{}', use 'folder' or 'file'", target)),
    }
    (true, String::new())
}

pub fn rename(args: &[String]) -> (bool, String) {
    move_cmd(args)
}

pub fn hi(_args: &[String]) -> (bool, String) {
    let username = env::var("USERNAME")
        .or_else(|_| env::var("USER"))
        .unwrap_or_else(|_| String::from("there"));
    (true, format!("hii {}", username))
}
