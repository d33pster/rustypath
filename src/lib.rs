use std::{env, path::PathBuf};

use dirs;

pub struct RPath{
    path:PathBuf,
}

impl RPath{
    pub fn join(&self, p:String) -> RPath{
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
            None => PathBuf::new(),
        };

        pbuf
    }

    pub fn pwd() -> Self {
        let pwd: PathBuf = match env::current_dir() {
            Ok(value) => value,
            Err(_err) => PathBuf::new(),
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
        let path = self.path.as_path().canonicalize().unwrap();
        RPath{path}
    }
}