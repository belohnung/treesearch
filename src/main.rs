use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "treesearch", about = "allows you to search in a file tree")]
struct Opt {
    ///Changes mode to exact
    #[structopt(short, long)]
    exact: bool,

    ///Silent
    #[structopt(short, long)]
    silent: bool,

    /// Start folder
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Search string
    #[structopt()]
    search_for: String,
}

fn main() {
    let opt = Opt::from_args();
    let x = fs::read_dir(opt.input).unwrap();
    for entry in x {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            print_dir(entry, &opt.search_for, opt.exact,opt.silent);
        } else {
            check_match(entry, &opt.search_for, opt.exact,opt.silent)
        }
    }
}
fn print_dir(entry: DirEntry, search_for: &str, exact: bool, silent: bool) {
    let x = fs::read_dir(entry.path()).unwrap();
    for entry in x {
        let entry = entry.unwrap();

        if entry.file_type().unwrap().is_dir() {
            print_dir(entry, &search_for, exact,silent)
        } else {
            check_match(entry, &search_for, exact,silent)
        }
    }
}
fn check_match(entry: DirEntry, search_for: &str, exact: bool,silent: bool) {
    let filename = entry.file_name().into_string().unwrap();
    let is_match = if exact { filename.as_str() == search_for } else { filename.contains(search_for) };
    let format_string = if silent { "" } else { "Match found: " };
    if is_match {
        println!("{}{}",format_string ,entry.path().display());
    }
}
