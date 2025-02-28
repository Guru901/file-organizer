use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pathname = args
        .get(1)
        .expect("Please provide a directory path as an argument.");

    let files: Vec<_> = fs::read_dir(pathname)
        .expect("Failed to read directory")
        .collect();

    files.iter().for_each(|f| match f {
        Ok(entry) => {
            let path = entry.path();
            if !path.is_dir() {
                let extensions = path.extension();

                if let Some(extension) = extensions.clone() {
                    make_dirs(extension.to_str().unwrap());
                    move_files(path.display().to_string(), extension.to_str().unwrap());
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    });
}

fn make_dirs(name: &str) {
    Command::new("mkdir")
        .arg(name.to_string() + "s")
        .spawn()
        .expect("coudlnt make directory");
}

fn move_files(name: String, path: &str) {
    Command::new("mv").arg(name).arg(path).spawn().unwrap();
}
