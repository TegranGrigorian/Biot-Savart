use std::env::args;

use crate::{app::screen::run_viewer, test};

pub fn get_args() -> Vec<String>{
    let args: Vec<String> = args().skip(1).collect(); // collect all strings skipping the first one (the command name)   
    return args;
}

pub fn get_arg_state(arg: &Vec<String>) {
    if arg.is_empty() {
        println!("ATTEMPTING TO ENTERING VIEWER");
        run_viewer();
    }
    for _arg in arg {
        if _arg == "--help" {
            println!("Biot-Savart CLI for DEVELOPMENT");
            println!("   --help: opens help output, this output you are reading bozo");
            println!("   -t, --test: runs the test on the engine to ensure proper output");
            println!("No flag will bring you right to the viewer. So `./Biot-Savart`")
        }
        else if _arg == "-t" || _arg == "--test"{
            println!("ENTERING TEST");
            // test::test_equation_strip();
        } else {
            println!("Blud what flag did u enter? do --help");
        }
    }
    
}