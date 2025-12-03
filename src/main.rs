use std::env::{current_dir};
use std::env;

use std::path::Path;

use crate::clock::main as clock;

mod text_effects;
mod clock;
mod initiate;
mod populate;
mod list_files;
mod command;
mod parse_lgr;

use std::io;


fn main() {


    let _start = clock();

    print!("{}", text_effects::frame(&clock(), 2, 0, 0));

    let args: Vec<String> = env::args().collect(); 

    if args.len() > 1{

    match args[1].as_str() {

    "remote" =>{

        println!("What directory should I compile for you? /n");

        let mut remote_dir = String::new();

        let stdin = io::stdin();

        stdin.read_line(&mut remote_dir);

        let test_dir = env::set_current_dir(Path::new(&remote_dir));
        match test_dir {
        Ok(_) => println!("Changed directory successfully."),
        Err(e) => println!("Failed to change directory: {}", e)};
    }

    _ => println!("Argument '{}' not recognized.",args[1])
       
   }

    }





   
   println!("Compiling {}", current_dir().unwrap().to_str().unwrap());

    let _populated = populate::populate(current_dir().unwrap());
    







    print!("{}", text_effects::frame(&"COMPILATION COMPLETE".to_string(), 1, 2, 0));

}




