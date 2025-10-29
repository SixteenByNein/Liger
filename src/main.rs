use std::env::{current_dir};
use std::env;

use std::path::Path;

use crate::clock::main as clock;

mod text_effects;
mod clock;
mod initiate;
mod populate;
mod list_files;
mod text_gen;
mod command;
mod parse_lgr;


fn main() {


   let _start = clock();

   print!("{}", text_effects::frame(&clock(), 2, 0, 0));


   // For testing purposes only:
   let test_dir = env::set_current_dir(Path::new("/home/gunnar/Desktop/ligerTest"));
   match test_dir {
       Ok(_) => println!("Changed directory successfully."),
       Err(e) => println!("Failed to change directory: {}", e)};
   // End testing purposes only:



   
   println!("Compiling {}", current_dir().unwrap().to_str().unwrap());

    let _populated = populate::populate(current_dir().unwrap());
    







    print!("{}", text_effects::frame(&"COMPILATION COMPLETE".to_string(), 1, 2, 0));

}




