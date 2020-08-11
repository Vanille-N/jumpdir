use std::env;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;

fn main() {
    let mut args = env::args().skip(2);
    // args.for_each(|p| eprintln!("{}", p));
    let arg = String::from(
        match args.next().as_deref() {
            Some("cd") => "",
            Some(s) => s,
            None => "",
        });
    let cwd = env::current_dir().expect("Failed to detect environment");
    match subfolders(&cwd) {
        Ok(sub) => {
            let valid = sub.into_iter()
                .filter(|p| is_completion(p, &cwd, &arg))
                .map(|p| shorten(&p, &cwd))
                .collect::<Vec<_>>();
            if valid.len() == 1 {
                println!("{}/", valid[0]);
            } else {
                valid.into_iter()
                    .for_each(|s| println!("{} ", s));
            }
        }
        Err(_) => println!("{}", &arg),
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
    // eprintln!("<{:?}> <{:?}> <{:?}>", p, cwd, arg);
    p.canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with(&(
            cwd.canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned() + "/" + arg
            ))
}

fn shorten(p: &Path, cwd: &Path) -> String {
    let p = p.canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let cwd = cwd.canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    String::from(&p[cwd.len()+1..])
}
