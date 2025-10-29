//generates an array of file paths for files in the layout folder

use std::{fs::{DirEntry, read_dir}, path::PathBuf};




pub fn list_files(current_dir: &str) -> Vec<DirEntry>
{


    let mut paths: Vec<DirEntry> = vec![];

    for item in read_dir(current_dir).unwrap(){

        let dir = item.unwrap();

        if dir.file_type().unwrap().is_file(){
        

            paths.push(dir);

        }

    }

    return paths;

}

pub fn list_folders(current_dir: &str) -> Vec<PathBuf>
{

    let mut paths: Vec<PathBuf> = vec![];

    for item in read_dir(current_dir).unwrap(){

        let dir = item.unwrap();

        if dir.file_type().unwrap().is_dir(){
        

            paths.push(dir.path());

        }

    }

    return paths;

}