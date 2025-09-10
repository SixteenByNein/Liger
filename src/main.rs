use std::env::current_dir;
use std::env;

mod initiate;
mod populate;
mod list_files;
mod text_gen;
mod clear_folder;


fn main() {

    //let _ = set_current_dir("/home/gunnar/Desktop/LigerTest");


    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {

    let _populated = populate::populate(&(current_dir().as_ref().unwrap().to_str().unwrap().to_string()));

    }

    else {
        


    match args[1].as_str()
    {

    "init" => initialize(current_dir().unwrap().to_str().unwrap()),

    _ => println!("Unrecognized argument"),

    }
    }
    ;


}

fn initialize(file_path: &str)
{

    initiate::init(file_path);

}


