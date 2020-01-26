use std::{ fs };
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

pub fn get_files(flags: Flags, path: &str) -> Vec<fs::DirEntry> {
    let mut files: Vec<fs::DirEntry> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();

                if !flags.almost_all && !flags.all {
                    if let Some('.') = file_name.to_string_lossy().clone().chars().next() {
                        continue;
                    }

                    if let Some(ref pattern) = flags.hide {
                        let re = Regex::new(&pattern).expect("Invalid regular found");
                        if re.is_match(&file_name.to_string_lossy()) {
                            continue;
                        }
                    }
                }

                if flags.ignore_backups {
                    if let Some('~') = file_name.to_string_lossy().clone().chars().last() {
                        continue;
                    }
                }

                if let Some(ref pattern) = flags.ignore {
                    let re = Regex::new(&pattern).expect("Invalid regular expression");
                    if re.is_match(&file_name.to_string_lossy()) {
                        continue;
                    }
                }

                files.push(entry);
            }
        }
    } 

    files
}
