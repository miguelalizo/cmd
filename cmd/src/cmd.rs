use std::io;
use std::collections::HashMap;
use std::io::Write;

use crate::command_handler::CommandHandler;

/// Command interpreter implemented as struct that contains
/// a handles HashMap of command strings and Boxed CommandHandlers
#[derive(Debug, Default)]
pub struct Cmd {
    handles: HashMap<String, Box<dyn CommandHandler>>
 }

impl Cmd {
    /// Create new Cmd instance
    pub fn new() -> Cmd {
        Cmd {
            handles: HashMap::new()
        }
    }

    /// Start the command interpreter
    ///
    pub fn run(&mut self) -> Result<(), io::Error>{
        loop {
            // print promt at every iteration and flush stdout to ensure user
            // can type on same line as promt
            print!("(cmd) ");
            io::stdout().flush()?;

            // get user input from stdin
            let mut inputs = String::new();
            io::stdin().read_line(&mut inputs)?;

            if inputs.is_empty() {
                continue;
            }

            // separate user input into a command and optional args
            let inputs = inputs.trim(); //.split_whitespace();
            let (command, args) = self.parse_cmd(inputs);

            // attempt to execute command
            if let Some(handler) = self.handles.get(&command) {
                handler.execute(args)
            } else {
                println!("No command {command}");
            }
        }
    }


    /// Insert new command into the Cmd handles HashMap
    ///
    /// ## Note: Will not overwrite existing handles.
    pub fn add_cmd(&mut self, name: String, handler: Box<dyn CommandHandler>) {
        if let Some(_) = self.handles.get(&name) {
            println!("Warning: Command with handle {name} already exists.");
            return
        }
        self.handles.insert(name, handler);
    }

    fn parse_cmd(&self, line: &str) -> (String, String) {
        let mut words = line.split_whitespace();
        let command = words.next().unwrap_or_default().to_string();
        let args: String = words.collect::<Vec<_>>().join(" ");
        (command, args)
    }

    #[cfg(test)]
    fn get_cmd(&self, key: String) -> Option<&Box<dyn CommandHandler>> {
        self.handles.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::any::Any;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Greeting { }

    impl CommandHandler for Greeting {
        fn execute(&self, _args: String) {
            println!("Help message");
        }
    }

    #[test]
    fn test_add_cmd() {
        // create Cmd app and Greeting
        let mut app = Cmd::new();
        let greet_handler = Greeting::default();

        // Add the trait object to the HashMap
        app.add_cmd(String::from("greet"), Box::new(greet_handler));


        // Verify that the key-value pair exists in the HashMap
        match app.get_cmd(String::from("greet")) {
            Some(handler) => {
                let it: &dyn Any = handler.as_any();

                match it.downcast_ref::<Greeting>() {
                    Some(_t) => (),
                    None => panic!("Not expected type!"),
                }
            },
            None => panic!("key-value paior does not exist in the HashMap")
        }
    }

    #[test]
    fn test_parse_cmd(){
        let app = Cmd::new();
        let line = "command arg1 arg2";
        assert_eq!(app.parse_cmd(line), ("command".to_string(), "arg1 arg2".to_string()))
    }
}

