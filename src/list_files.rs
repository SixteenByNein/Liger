//generates an array of file paths for files in the layout folder

use std::{fs::{read_dir, DirEntry}, path::Path};




pub fn list_files(current_dir: String) -> Vec<DirEntry>
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

pub fn list_folders(current_dir: &String) -> Vec<DirEntry>
{

    let mut paths: Vec<DirEntry> = vec![];

    for item in read_dir(current_dir).unwrap(){

        let dir = item.unwrap();

        if dir.file_type().unwrap().is_dir(){
        

            paths.push(dir);

        }

    }

    return paths;

}

pub fn get_tree(current_dir: &String) -> Vec<String>
{


    //Gets root length to be subtracted later





    let mut tree:Vec<String> = vec![];

    let mut gen_buffer:Vec<String> = vec![];



    gen_buffer.push("".to_string());



    let mut gen_tail:Vec<String>;

    let mut live:bool = true;


    while live {

        for item in gen_buffer.clone()
        {

            tree.push(item);

        }

        gen_tail = gen_buffer.clone();

        gen_buffer.clear();

        for item in gen_tail {

            let selected = &(current_dir.to_owned() + &item);
            
            for jitem in list_folders (selected)
            {

                gen_buffer.push(truncate(jitem, current_dir));

            }


        }
        
        live = !gen_buffer.is_empty();

    }
    

    return tree;

}







pub fn truncate(dir: DirEntry, current_dir: &String) -> String
{

    let root_components = Path::new(&current_dir).components();

    let mut root_length= 0;


    for _item in root_components
    {

        root_length += 1;

    }


    let mut loop_count = 0;

    let mut truncated: String = "".to_string();

    for comp in dir.path().components()
    {

        loop_count += 1;

        if loop_count > root_length
        {


            truncated = truncated + "/" + comp.as_os_str().to_str().unwrap();

        }

    }

    return truncated;

}






pub fn enstring(dirs: Vec<DirEntry>, current_dir: &String) -> Vec<String>
{

    let mut enstringed: Vec<String> = [].to_vec();

    for item in dirs{

        enstringed.push(truncate(item, current_dir));

    }

    return enstringed;

}