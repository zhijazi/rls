extern crate getopts;

use std::{ env, fs };
use std::os::unix::fs::PermissionsExt;
use getopts::Options;

fn main() -> std::io::Result<()>{
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
        output_dir_detailed(&path[..]);
    } else {
        output_dir_contents(&path[..]);
    }

    Ok(())
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
        println!("{:?} {} {} {} {} {:?} {:?}", metadata.permissions().mode(), 1, "zein", "zein", metadata.len(), metadata.modified(), file.file_name());
    }
    Ok(())
}
