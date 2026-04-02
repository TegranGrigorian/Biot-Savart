use std::env::args;

use crate::{app::screen::run_viewer, test};

pub fn get_args() -> Vec<String>{
    let args: Vec<String> = args().skip(1).collect(); // collect all strings skipping the first one (the command name)   
    return args;
}

pub fn get_arg_state(arg: &Vec<String>) {
    for _arg in arg {
        if _arg == "--help" {
            println!("Biot-Savart CLI for DEVELOPMENT");
            println!("   --help: opens help output, this output you are reading bozo");
            println!("   -t, --test: runs the test on the engine to ensure proper output");
        }
        else if _arg == "-t" || _arg == "--test"{
            println!("ENTERING TEST");
            test::test_biot_savart();
        }
    }
    if arg.is_empty() {
        println!("ATTEMPTING TO ENTERING VIEWER");
        run_viewer();
    }
}