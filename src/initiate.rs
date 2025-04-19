use std::fs;

pub fn init(_file_path: &str) -> bool
{

    let _layout = fs::create_dir(_file_path.to_string() + "/layout");
    let _final_site = fs::create_dir(_file_path.to_string() + "/final");
    let _parts = fs::create_dir(_file_path.to_string() + "/parts");

    return true;

}