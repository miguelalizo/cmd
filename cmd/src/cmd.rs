use std::io;
use std::collections::HashMap;
use std::io::Write;

use crate::command_handler;

/// Command interpreter implemented as struct that contains
/// a handles HashMap of command strings and Boxed CommandHandlers
#[derive(Debug, Default)]
pub struct Cmd { 
    handles: HashMap<String, Box<dyn command_handler::CommandHandler>>
 }

impl Cmd {
    /// Create new Cmd instance
    pub fn new() -> Cmd {
        let handles: HashMap<String, Box<dyn command_handler::CommandHandler>> = HashMap::new();

        Cmd { handles }

    }

    /// Start the command interpreter
    ///
    /// ## Defult commands
    ///
    /// - exit: terminates the process running the command interpreter.
    pub fn run(&mut self) -> Result<(), io::Error>{
        loop {
            // print promt at every iteration and flush stdout to ensure user
            // can type on same line as promt
            print!("(cmd) ");
            io::stdout().flush()?;

            // get user input from stdin
            let mut inputs = String::new();
            io::stdin().read_line(&mut inputs)?;

            // separate user input into a command and optional args
            let mut inputs = inputs.trim().split_whitespace();
            let command = inputs.next().unwrap_or("");
            let args = inputs.collect::<Vec<&str>>().join(" ");

            // attempt to execute command
            match command {
                "exit" => {
                    return Ok(());
                },
                "" => { 
                    println!("Need to provide a command") 
                },
                _ => {
                    // execute command and add optional args if any
                    match self.handles.get_mut(command) {
                        Some(handler) => {
                            // set Handler optional args if any
                            match &args.len() {
                                0 => { },
                                _ => handler.add_attr(&args)
                            }
                            handler.execute();
                        },
                        None => println!("No command {command}")    
                    }
                }
            }
        }
    }

    /// Insert new command into the Cmd handles HashMap
    ///
    /// ## Note: Will not overwrite existing handles.
    pub fn add_cmd(&mut self, name: String, handler: Box<dyn command_handler::CommandHandler>) {
        if let Some(_) = self.handles.get(&name) {
            println!("Warning: Command with handle {name} already exists.");
            return
        }
        self.handles.insert(name, handler);
    }
}

