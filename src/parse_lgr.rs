use std::{fs, path::PathBuf};

use crate::command::Command;




  
  pub fn parse(path: PathBuf)
  {

    let file_text = fs::read_to_string(path).expect("Failed to read file");
    let commands = get_commands(&file_text);
    run_commands(commands);
    

  }






  fn run_commands(commands: Vec<Command>)
  {

    let target: PathBuf = PathBuf::new();

    for i in commands
    {

      match i.return_type().as_str()
      {
        "to" => {

          println!("Generating HTML in: {}", i.return_args());

        }

        "external" => {

          // Placeholder for external command handling

        }

        "html" => {

          // Placeholder for HTML generation command handling

        }

        _ => {

          println!("{} is not a valid Liger tag", i.return_type());

        }

      }

    }

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