pub struct Command
{
    cmd_type: String,
    args: String,
    //used for troubleshooting
    line_number: usize,
    file_path: String,
}

impl Command {
    pub fn return_type(&self) -> String
    {
        return self.cmd_type.clone();
    }

    pub fn return_args(&self) -> String
    {
        return self.args.clone()
    }

    #[allow(dead_code)]
    pub fn error(&self) -> String
    {
        format!("Error in file: {} on line: {}", self.file_path, self.line_number)
    }

    pub fn new() -> Command
    {
        Command {
            cmd_type: String::new(),
            args: String::new(),
            line_number: 0,
            file_path: String::new(),
        }
    }

    pub fn set_type(&mut self, cmd_type: &str)
    {
        self.cmd_type = cmd_type.to_string();
    }

    pub fn set_args(&mut self, args: &str)
    {
        self.args = args.to_string();
    }

}