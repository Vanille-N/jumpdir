use std::env;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let arg = args.next().unwrap();
    let dir = Path::new(&arg);
    match subfolders(&dir) {
        Ok(sub) => {
            if sub.len() == 1 {
                println!("{}", sub[0].display());
            } else {
                println!("{}", &dir.display());
            }
        }
        Err(_) => println!("{}", &dir.display()),
    }
}

fn subfolders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter_map(|r| r.ok().map(|x| x.path()))
        .filter(|r| r.is_dir())
        .collect())
}

fn is_completion(p: &Path, cwd: &Path, arg: &str) -> bool {
    println!("<{:?}> <{:?}> <{:?}>", p, cwd, arg);
    p.to_str().unwrap().starts_with(&(cwd.to_str().unwrap().to_owned() + "/" + arg))
}
