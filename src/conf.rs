use std::{fs};
use std::path::PathBuf;
use serde_derive::Serialize;
use serde_derive::Deserialize;

//#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    // `Option<>` makes it optional, and incorporates `Result<>`-style features
    pub(crate) delay: Option<u64>,
    pub(crate) rsync_bin: Option<String>,
}

// Design pattern stolen from: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b3f6fc761a52083ff13337a5a386ed74
impl Config {
    pub(crate) fn merge(self, other: Config) -> Self {
        Self {
            // .or() is shorthand for an Option<> object to replace itself with another if it is available
            // If it is not Some, of course, it must be None
            delay: other.delay.or(self.delay),
            rsync_bin: other.rsync_bin.or(self.rsync_bin)
        }
    }
}

// PathBuf is like String but for paths,
// Just as Path is like str but for paths
pub fn load_one_of(paths: Vec<PathBuf>) -> String {
    // TODO: add Result<> object to output
    for path in paths {
        let result = fs::read_to_string(path);
        if result.is_ok() {
            return result.unwrap();
        }
    }
    return String::new();
}
