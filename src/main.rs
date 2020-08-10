use std::env;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let dirname = args.next().unwrap_or("".to_string());
    let dir = env::current_dir().unwrap();
    println!("{:?}", subfolders(&dir));
}

fn subfolders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter_map(|r| r.ok().map(|x| x.path()))
        .filter(|r| r.is_dir())
        .collect())
}
