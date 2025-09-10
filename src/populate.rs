use std::io::prelude::*;

use std::fs::{copy, create_dir, remove_file, File};

use crate::clear_folder::clear_folder;
use crate::list_files::{enstring, get_tree, list_files, list_folders};

use crate::text_gen::text_gen;






pub fn populate(current_dir: &str)
{


    clear_folder(&(current_dir.to_string() + "/final"));


    let _ = remove_file(&(current_dir.to_string() + "/index.html"));

    let mut targeted_index = File::create(current_dir.to_owned() + "/index.html").unwrap();
            

    let _ = targeted_index.write(text_gen(&(current_dir.to_owned() + "/index.html"), current_dir.to_owned() + "/template.html").as_bytes());


    let tree = get_tree(&(current_dir.to_string() + "/layout"));





    for item in tree
    {

        populate_dir(item, current_dir.to_string());

    }


    println!("Compilation complete!");

    
}


fn populate_dir(dir: String, root: String)
{

    let root_borrowed = &root.clone();

    let folders = enstring(list_folders(&(root.clone() + "/layout" + &dir)), &(root + "/layout"));


    for item in folders
    {

        let _ = create_dir(root_borrowed.to_owned() + "/final" + &item);

    }


    let files = enstring(list_files(&(root_borrowed.to_string() + "/layout" + &dir)), &(root_borrowed.to_string() + "/layout"));
    
    for item in files
    {

        let origin = root_borrowed.to_string() + "/layout" + &item;


        if origin.ends_with("html")
        {

            let mut target_file = File::create(root_borrowed.to_string() + "/final" + &item).unwrap();
            

            let _ = target_file.write(text_gen(&origin, root_borrowed.to_string()).as_bytes());


        } else {

            let _ = copy(origin, root_borrowed.to_string() + "/final" + &item);

        }

    }


}
