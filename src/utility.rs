use std::path::PathBuf;

fn pathbuf_to_string(path: PathBuf) -> String {
    path.into_os_string().into_string().unwrap()
}