use std::env;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let dirname = args.next().unwrap_or("".to_string());
    let dir = env::current_dir().unwrap();
    println!("{:?}", folders(&dir));
}

fn folders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(dir)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}
