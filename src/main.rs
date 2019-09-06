use std::env;

mod config;
mod process;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let arg_num: usize = args.len() - 1;

    let child = process::Process::spawn("echo jared rocks");

    let cfg = config::get_tom_cfg();

    println!("{:?}", cfg);

    if arg_num > 0 {
        match String::as_str(&args[1]) {
            "cfg" => println!("mode is cfg"),
            "prof" => println!("mode is prof"),
            "-p" => println!("process will be specified next"),
            "-r" => println!("profile will be specified next"),
            _ => util::log::usage(&args[0], Some(&args)),
        }
    } else {
        util::log::error("no arguments");
        util::log::usage(&args[0], None);
    }
}
