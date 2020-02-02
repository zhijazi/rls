use std::{ fs };
use std::path::{Path, PathBuf};
use regex::Regex;

pub struct Flags {
    pub all: bool,
    pub almost_all: bool,
    pub ignore_backups: bool,
    pub directory: bool,
    pub follow_sym_link: bool,
    pub hide: Option<String>,
    pub ignore: Option<String>,
    pub recursive: bool
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            all: false,
            almost_all: false,
            ignore_backups: false,
            directory: false,
            follow_sym_link: false,
            hide: None,
            ignore: None,
            recursive: false
        }
    }
}

pub fn get_files(flags: Flags, path: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    if flags.directory {
        files.push(Path::new(".").to_path_buf());
        return files
    }

    if flags.all {
        files.push(Path::new(".").to_path_buf());
        files.push(Path::new("..").to_path_buf());
    }
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                let file_name = if let Some(name) = path.file_name() {
                    name.to_string_lossy()
                } else {
                    path.to_string_lossy()
                };
                
                if !flags.almost_all && !flags.all {
                    if file_name.starts_with(".") {
                        continue;
                    }

                    if let Some(ref pattern) = flags.hide {
                        let re = Regex::new(&pattern).expect("Invalid regular found");
                        if re.is_match(&file_name) {
                            continue;
                        }
                    }
                }

                if flags.ignore_backups {
                    if file_name.ends_with("~") {
                        continue;
                    }
                }

                if let Some(ref pattern) = flags.ignore {
                    let re = Regex::new(&pattern).expect("Invalid regular expression");
                    if re.is_match(&file_name) {
                        continue;
                    }
                }

                files.push(path);
            }
        }
    } 

    files
}
