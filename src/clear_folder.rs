use std::fs::{remove_dir_all, remove_file};

use crate::list_files;

pub fn clear_folder(dir_path: String)
{

    let dirs = list_files::list_folders(&dir_path);
    let files = list_files::list_files(dir_path.clone());

    for item in dirs
    {

        let _ = remove_dir_all(item.path());

    }

    for item in files
    {

        let _ = remove_file(item.path());

    }


}