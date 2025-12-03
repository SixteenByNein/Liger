//use std::io::prelude::*;

//use std::fs::{copy, create_dir, remove_file, File};
use std::path::PathBuf;

use crate::list_files::{/*enstring, get_tree,*/ list_files, list_folders};

use crate::parse_lgr::parse;






pub fn populate(current_dir: PathBuf)
{

    
    let mut folders = list_folders(current_dir.to_str().unwrap());

    folders.insert(0, current_dir);

    let mut templates: Vec<PathBuf> = vec![];

    println!("{} folders found.", folders.len());

    println!("");

    println!("Searching in the following directories:");

    for i in &folders {

        println!("{}", i.to_str().unwrap());

    }

    for i in folders {

        let files = list_files(i.to_str().unwrap());

        for j in files {

            if j.path().extension().and_then(|s| s.to_str()) == Some("lgr") {

                templates.push(j.path());

            }

        }
    }

    if templates.len() == 1 {

        println!("\n Compiling from the following template:");
    } else {

    println!("\nCompiling from the following {} templates:", templates.len());

    }

    for i in &templates {

        println!("{}", i.to_str().unwrap());

        parse(i.to_path_buf());

    }

}
