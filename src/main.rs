use std::env;
use std::fs;

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
                    // make directories
                    // move files
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    });
}
