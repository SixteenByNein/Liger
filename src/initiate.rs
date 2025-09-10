use std::fs::{self, File};

pub fn init(_file_path: &str) -> bool
{

    let _layout = fs::create_dir(_file_path.to_string() + "/layout");
    let _final_site = fs::create_dir(_file_path.to_string() + "/final");
    let _parts = fs::create_dir(_file_path.to_string() + "/parts");

    let _index = File::create(_file_path.to_string() + "/index.html");
    let _template = File::create(_file_path.to_string() + "/template.html");

    return true;

}