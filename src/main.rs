use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    file_organizer::run(&args);
}
