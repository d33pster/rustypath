use std::{env, path::{Path, PathBuf}};

use dirs;

pub struct RPath{
    path:PathBuf,
}

impl RPath{
    pub fn join(&self, p:&str) -> RPath{
        let newpath = self.path.clone().join(p);
        RPath { path: newpath }
    }

    pub fn convert_to_pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
    
    pub fn convert_to_string(&self) -> String {
        let convstr = self.path.clone().into_os_string().into_string().unwrap_or_else(|p| p.to_string_lossy().into_owned());
        convstr
    }

    pub fn convert_optionpathbuf_to_pathbuf(p: Option<PathBuf>) -> PathBuf {
        let pbuf = match p{
            Some(pb) => pb,
            None => {
                eprintln!("Failed to convert option<pathbuf> to pathbuf");
                std::process::exit(1);
            },
        };

        pbuf
    }

    pub fn pwd() -> Self {
        let pwd: PathBuf = match env::current_dir() {
            Ok(value) => value,
            Err(_err) => {
                eprintln!("Failed to get current dir.");
                std::process::exit(1);
            },
        };

        RPath {
            path: pwd,
        }
    }

    pub fn gethomedir() -> Self {
        RPath {path:RPath::convert_optionpathbuf_to_pathbuf(dirs::home_dir())}
    }

    pub fn new_from(p: &str) -> RPath {
        RPath{path:PathBuf::from(p)}
    }

    pub fn clone(&self) -> RPath {
        RPath{
            path: self.path.clone()
        }
    }

    pub fn expand(&self) -> RPath {
        let path = self.path.as_path().canonicalize().unwrap_or_else(|error| {
            eprintln!("Failed to expand path for {}: {}", &self.convert_to_string(), error);
            std::process::exit(1);
        });
        RPath{path}
    }

    pub fn new_from_path(p: &Path) -> RPath {
        let path = p.to_str().expect("Failed to conver to string");
        RPath::new_from(path)
    }

    pub fn new_from_pbuf(p: &PathBuf) -> RPath {
        let path = match p.to_str() {
            Some(p) => p.to_string(),
            None => {
                eprintln!("Failed to convert path to string.");
                std::process::exit(1);
            },
        };

        
        RPath::new_from(&path)
    }


}