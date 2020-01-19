extern crate getopts;
extern crate chrono;
extern crate users;

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

    if arg_matches.opt_present("l") {
        match output_dir_detailed(&path[..], show_hidden) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string())
        };
    } else {
        match output_dir_contents(&path[..], show_hidden) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string())
        };
    }
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
        let perm_str = perms_to_str(mode);

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

fn perms_to_str(mut perm_oct: u32) -> String {
    let mut perm_str = String::from("");
    let others = perm_oct%8;
    perm_oct/=8;
    let group = perm_oct%8;
    perm_oct/=8;
    let owner = perm_oct%8;

    perm_str.push_str(&octal_to_str(owner));
    perm_str.push_str(&octal_to_str(group));
    perm_str.push_str(&octal_to_str(others));

    perm_str
}

fn octal_to_str(oct: u32) -> String {
    let mut perm = String::from("");
    if (oct >> 2) & 1 == 1 {
        perm.push('r');
    } else {
        perm.push('-');
    }

    if (oct >> 1) & 1 == 1 {
        perm.push('w');
    } else {
        perm.push('-');
    }

    if oct & 1 == 1 {
        perm.push('x');
    } else {
        perm.push('-');
    }

    perm
}
