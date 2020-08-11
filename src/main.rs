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
    let target = Path::new(&arg);
    let cwd = env::current_dir()
        .expect("Failed to detect environment");
    let partial = cwd
        .join(&target.strip_filename());
    // println!("{:?} {:?}", cwd, target);
    match subfolders(&partial) {
        Ok(sub) => {
            let valid = sub.into_iter()
                .filter(|p| is_completion(p, &partial, &target.get_filename()))
                .map(|p| shorten(&p, &cwd))
                .collect::<Vec<_>>();
            if valid.len() == 1 {
                println!("{}/", valid[0]);
            } else {
                valid.into_iter()
                    .for_each(|s| println!("{} ", s));
            }
        }
        Err(_) => println!("{}", target.display()),
    }
}

fn subfolders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter_map(|r| r.ok().map(|x| x.path()))
        .filter(|r| r.is_dir())
        .collect())
}

fn is_completion(p: &Path, cwd: &Path, arg: &Path) -> bool {
    // eprintln!("<{:?}> <{:?}> <{:?}>", p, cwd, arg);
    p.canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with(
            &cwd.canonicalize()
                .unwrap()
                .join(arg)
                .to_str()
                .unwrap()
            )
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

trait StripFilename {
    type OwnedSelf;
    fn strip_filename(&self) -> Self::OwnedSelf;
    fn get_filename(&self) -> Self::OwnedSelf;
}

impl StripFilename for Path {
    type OwnedSelf = PathBuf;
    fn strip_filename(&self) -> Self::OwnedSelf {
        match self.to_str().unwrap().chars().last() {
            Some('/') => PathBuf::from(self),
            _ => PathBuf::from(self.parent().unwrap_or(Path::new(""))),
        }
    }

    fn get_filename(&self) -> Self::OwnedSelf {
        match self.to_str().unwrap().chars().last() {
            Some('/') => PathBuf::from(""),
            _ => PathBuf::from(self.file_name().unwrap_or(std::ffi::OsStr::new(""))),
        }
    }
}
