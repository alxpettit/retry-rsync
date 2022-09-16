// Copyleft Alexandria P., 2022 GNU GPLv3
// Note: I am a Rust baby, and have no idea what I'm doing...
// so, I'm going to include a lot of obvious comments documenting the nature and behavior of basic features.
// So... leave me alone >.<

// TODO: try to remember wtf configurability I was planning to add, I legit just forgot
// TODO: figure out how to separate into multiple binaries referencing the same code, either static or dynamically linked

use std::env;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus};
use std::thread::sleep;
use std::time::Duration;
use directories::BaseDirs;
use conf::Config;

mod conf;
mod shell;
mod utility;

fn main() {
    let base_dirs = BaseDirs::new().unwrap();

    let paths: Vec<PathBuf> = vec!(
        Path::new("/etc").join("retry-rsync.toml").to_path_buf(),
        Path::new("/etc").join("retry-rsync").join("retry-rsync.toml").to_path_buf(),
        base_dirs.config_dir().join("retry-rsync.toml"),
        base_dirs.config_dir().join("retry-rsync").join("retry-rsync.toml"),
    );

    let contents = conf::load_one_of(paths);

    let mut config = Config {
        // Our default config.
        // We hardcode these options, then overwrite them to create layered config system
        delay: Some(1000),
        rsync_bin: Some("rsync".to_string()),
            ..Config::default()
    };

    if ! contents.is_empty() {
        config = config.merge(toml::from_str(contents.as_str())
            .expect("Syntax error in TOML file"));
    }

    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let mut status: ExitStatus;

    loop {
        status = shell::shell(config.rsync_bin
            .as_ref()
            .unwrap()
            .as_str(), &args)
            .expect("Couldn't start rsync :(");
        match status.code() {

            Some(1) => { // Syntax or usage error
                println!("Status code 1. Syntax or usage error.");
                break;
            }

            Some(20) => { // Ctrl+C
                println!("Status code 20. Probably SIGUSR1 or SIGINT. Terminating loop.");
                break;
            }

            Some(0) => { // Success
                println!("Status code 0. ");
                break;
            }

            _ => { // Catch-all
                sleep(Duration::from_millis(config.delay.unwrap()));
            }
        }
    }
}
