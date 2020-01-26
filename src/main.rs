extern crate getopts;
extern crate chrono;
extern crate users;
extern crate regex;

pub mod perms;
pub mod file_parser;

use std::{ env, fs };
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

use getopts::Options;

use chrono::offset::Local;
use chrono::DateTime;

use users::{get_user_by_uid};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("l", "", "Lists detailed information about the file");
    opts.optflag("a", "", "Lists hidden files");

    let arg_matches = match opts.parse(&args[1..]) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let path = if !arg_matches.free.is_empty() {
        arg_matches.free[0].clone()
    } else {
        String::from(".")
    };

    let show_hidden = match arg_matches.opt_present("a") {
        true => true,
        false => false
    };

    let mut flags = file_parser::Flags::new();
    flags.almost_all = show_hidden;

    let files = file_parser::get_files(flags, &path);
    println!("{:?}", files);
}

fn output_dir_contents(path: &str, show_hidden: bool) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;

        if let Some(first_char) = file.file_name().to_string_lossy().chars().next() {
            if first_char == '.' && !show_hidden {
                continue;
            }
        }
        println!("{:?}", file.file_name());
    }
    Ok(())
}

fn output_dir_detailed(path: &str, show_hidden: bool) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let file_name = file.file_name();

        if let Some(first_char) = file_name.to_string_lossy().chars().next() {
            if first_char == '.' && !show_hidden {
                continue;
            }
        }

        let metadata = file.metadata()?;

        let user = match get_user_by_uid(metadata.st_uid()) {
            Some(x) => x,
            None => panic!("Could not find user")
        };

        let group = match get_user_by_uid(metadata.st_gid()) {
            Some(x) => x,
            None => panic!("Could not find group")
        };

        let last_modified = metadata.modified()?;
        let last_modified: DateTime<Local> = last_modified.into();

        let mode = metadata.permissions().mode();
        let perm_str = perms::perms_to_str(mode);

        let file_type: char = {
            if metadata.is_dir() {
                'd'
            } else {
                '-'
            }
        };

        println!("{}{} {} {} {} {} {} {:?}", file_type, perm_str, metadata.st_nlink(), user.name().to_string_lossy(), group.name().to_string_lossy(), metadata.len(), last_modified.format("%b %d %H:%M"), file.file_name());
    }
    Ok(())
}

