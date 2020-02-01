extern crate getopts;
extern crate chrono;
extern crate users;
extern crate regex;

pub mod perms;
pub mod file_parser;
pub mod file_info;

use std::{ env };
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("a", "all", "do not ignore entries starting with .");
    opts.optflag("A", "almost-all", "do not list implied . and ..");
    opts.optflag("B", "ignore-backups", "do not list implied entries ending with ~");
    opts.optopt("", "hide", "do not list implied entries matching shell PATTERN (overriden by -a or -A)", "PATTERN");
    opts.optopt("I", "ignore", "do not list implied entries matching shell PATTERN", "PATTERN");
    opts.optflag("l", "", "Lists detailed information about the file");

    let arg_matches = match opts.parse(&args[1..]) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let mut file_filter_flags = file_parser::Flags::new();
    let mut output_filter_flags = file_info::Flags::new();

    let path = if !arg_matches.free.is_empty() {
        arg_matches.free[0].clone()
    } else {
        String::from(".")
    };

    file_filter_flags.almost_all = arg_matches.opt_present("A");
    file_filter_flags.all = arg_matches.opt_present("a");
    file_filter_flags.hide = arg_matches.opt_str("hide");
    file_filter_flags.ignore = arg_matches.opt_str("I");
    file_filter_flags.ignore_backups = arg_matches.opt_present("B");

    output_filter_flags.detailed = arg_matches.opt_present("l");

    let files = file_parser::get_files(file_filter_flags, &path);
    file_info::print_details(output_filter_flags, files);
}

