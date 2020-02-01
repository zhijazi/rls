use std::{ fs };
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

use chrono::offset::Local;
use chrono::DateTime;

use users::{get_user_by_uid};

use crate::perms;

pub struct Flags {
    pub detailed: bool
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            detailed: false
        }
    }
}

pub fn print_details(flags: Flags, files: Vec<fs::DirEntry>) {
    if flags.detailed {
        print_detailed(files);
    } else {
        print_simple(files);
    }
}

fn print_simple(files: Vec<fs::DirEntry>) {
    for file in files {
        println!("{}", file.file_name().to_string_lossy());
    }
}

fn print_detailed(files: Vec<fs::DirEntry>) {
    for file in files {
        let file_name = file.file_name();

        let metadata = file.metadata().expect("Metadata cannot be acquired");

        let user = get_user_by_uid(metadata.st_uid()).expect("Cannot get user");        
        let group = get_user_by_uid(metadata.st_gid()).expect("Cannot get group");
        let last_modified: DateTime<Local> = metadata.modified().expect("Cannot get last modified").into();
        
        let mode = metadata.permissions().mode();
        let perm_str = perms::perms_to_str(mode); // TODO Redo how perms are converted

        let file_type: char = if metadata.is_dir() { 'd' } else { '-' };

        println!("{}{} {} {} {} {} {} {}", file_type, perm_str,
            metadata.st_nlink(), user.name().to_string_lossy(),
            group.name().to_string_lossy(), metadata.len(),
            last_modified.format("%b %d %H:%M"), file_name.to_string_lossy());
    }
}
