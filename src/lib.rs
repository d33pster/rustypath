use std::{env, path::{Path, PathBuf}};

use dirs;

#[derive(Clone)]
pub struct RPath{
    path:PathBuf,
}

impl RPath{
    #[cfg(feature = "stable_feature")]
    pub fn join(&self, p:&str) -> RPath{
        let newpath = self.path.clone().join(p);
        RPath { path: newpath }
    }

    #[cfg(feature = "stable_feature")]
    pub fn convert_to_pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
    
    #[cfg(feature = "stable_feature")]
    pub fn convert_to_string(&self) -> String {
        let convstr = self.path.clone().into_os_string().into_string().unwrap_or_else(|p| p.to_string_lossy().into_owned());
        convstr
    }

    #[cfg(feature = "conversion")]
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

    #[cfg(feature = "stable_feature")]
    pub fn basename(&self) -> &str {
        let basename = match self.path.file_name() {
            Some(filename) => match filename.to_str() {
                Some(a) => a,
                None => {
                    eprintln!("Failed to convert basename from OsStr to str.");
                    std::process::exit(1);
                },
            },
            None => {
                eprintln!("Failed to get basename.");
                std::process::exit(1);
            },
        };

        basename
    }

    #[cfg(feature = "stable_feature")]
    pub fn dirname(&self) -> Self {
        let dirpath = match self.path.parent() {
            Some(a) => a.to_path_buf(),
            None => {
                eprintln!("Failed to get dirname.");
                std::process::exit(1);
            },
        };

        Self {
            path: dirpath,
        }
    }
    
    #[cfg(feature = "stable_feature")]
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

    #[cfg(feature = "stable_feature")]
    pub fn gethomedir() -> Self {
        RPath {path:RPath::convert_optionpathbuf_to_pathbuf(dirs::home_dir())}
    }

    #[cfg(feature = "stable_feature")]
    pub fn new_from(p: &str) -> RPath {
        RPath{path:PathBuf::from(p)}
    }

    #[cfg(feature = "stable_feature")]
    pub fn clone(&self) -> RPath {
        RPath{
            path: self.path.clone()
        }
    }

    #[cfg(feature = "stable_feature")]
    pub fn expand(&self) -> RPath {
        let path = self.path.canonicalize().unwrap_or(self.convert_to_pathbuf());
        RPath{path}
    }

    #[cfg(feature = "stable_feature")]
    pub fn new_from_path(p: &Path) -> RPath {
        let path = p.to_str().expect("Failed to conver to string");
        RPath::new_from(path)
    }

    #[cfg(feature = "stable_feature")]
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