use std::{fs::{self, File}, path::PathBuf, env::current_dir};

use crate::command::Command;




  
  pub fn parse(path: PathBuf)
  {

    let file_text = fs::read_to_string(&path).expect("Failed to read file");
    let commands = get_commands(&file_text);
    run_commands(commands, path.parent().unwrap().to_path_buf());

    println!("Local root for this file: {}", path.parent().unwrap().to_str().unwrap());
    

  }






  fn run_commands(commands: Vec<Command>, local_root: PathBuf)
  {

    let mut target: PathBuf = PathBuf::new();

    let mut contents = String::new();

    for i in commands
    {

      match i.return_type().as_str()
      {
        "to" => {


            let absolute_path = absolute_path(&i.return_args(), &local_root);


            println!("Generating HTML in: {}", absolute_path.to_string());

            let _file = File::create(&absolute_path);

            target = PathBuf::from(absolute_path);


        }

        "external" => {

          let abs_path: String = absolute_path(&i.return_args(), &local_root);

          let external_text= fs::read_to_string(abs_path).expect("Failed to read external file");

          println!("Fetching external text from: {}", i.return_args().trim());

          contents.push_str(&external_text);
          

        }

        "html" => {

          contents.push_str(&i.return_args());

        }

        _ => {

          println!("{} is not a valid Liger tag", i.return_type());

        }

      }

    }


    fs::write(target, contents).expect("Failed to write to file");

  }






  pub fn get_commands(file_text: &str) -> Vec<Command>
  // Parses the file text and returns a vector of commands
  {

    let mut text_buffer = String::new();

    let mut inside = false;

    let mut commands : Vec<Command> = vec![];

    let mut command_buffer = Command::new();

   for i in file_text.chars()
    {

      if i != '[' && i != ']' {

        if inside{

        text_buffer.push(i);

        }
        else{

          if !i.is_whitespace()
          {

            text_buffer.push(i);

          }

        }
      
      }

      else if i == '[' 
      {
        
        command_buffer.set_type(&text_buffer);

        text_buffer.clear();
        inside = true;
        

      }
      else if i == ']' 
      {

        command_buffer.set_args(&text_buffer);

        text_buffer.clear();
        inside = false;

        //println!("Type: {}, Args: {}", command_buffer.return_type(), command_buffer.return_args());

        commands.push(command_buffer);
        command_buffer = Command::new();
          
      }
  
    } 

    return commands;

  }

  

  fn absolute_path(path: &str, local_root: &PathBuf) -> String
    {

      let trimmed = path.trim();
  
      let mut absolute_path = PathBuf::new();
  
      if trimmed.starts_with("/")
      {
  
        absolute_path.push(current_dir().unwrap());
        absolute_path.push(&trimmed[1..]);

        println!("{} is an absolute path.", trimmed);
  
      }
      else{
  
        absolute_path.push(local_root);
        absolute_path.push(trimmed);

        println!("{} is not an absolute path.", trimmed);
  
      }
  
      return absolute_path.to_str().unwrap().to_string();
  
    }