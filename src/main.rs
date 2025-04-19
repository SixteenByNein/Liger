use std::env::{current_dir, set_current_dir};

mod initiate;
mod populate;
mod list_files;
mod text_gen;
mod clear_folder;


fn main() {

    //let _ = set_current_dir("/home/gunnar/Desktop/LigerTest");


    if true
    {

    let _populated = populate::populate(&(current_dir().as_ref().unwrap().to_str().unwrap().to_string()));

    };


    if false
    {

    initialize(current_dir().unwrap().to_str().unwrap());

    };


}

fn initialize(file_path: &str)
{

    initiate::init(file_path);

}


