extern crate getopts;
extern crate chrono;
extern crate users;

use std::{ env, fs };
use std::os::linux::fs::MetadataExt;

use getopts::Options;

use chrono::offset::Local;
use chrono::DateTime;

use users::{get_user_by_uid};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("l", "", "Lists detailed information about the file");

    let arg_matches = match opts.parse(&args[1..]) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let path = if !arg_matches.free.is_empty() {
        arg_matches.free[0].clone()
    } else {
        String::from(".")
    };

    if arg_matches.opt_present("l") {
        match output_dir_detailed(&path[..]) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string())
        };
    } else {
        match output_dir_contents(&path[..]) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string())
        };
    }
}

fn output_dir_contents(path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        println!("{:?}", file.file_name());
    }
    Ok(())
}

fn output_dir_detailed(path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
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

        println!("<perms> {} {} {} {} {} {:?}",metadata.st_nlink(), user.name().to_string_lossy(), group.name().to_string_lossy(), metadata.len(), last_modified.format("%b %d %H:%M"), file.file_name());
    }
    Ok(())
}

