//! A crate for creating custom line-oriented command interpreters.
//!
//! # A tour of cmd
//!
//! cmd consists of two crates:
//! - cmd: Used for creating the cmd::Cmd struct that contains the CommandHandler implementations as in a HashMap
//! - command_handler: Contains the CommandHandler trait
//!
//! ## Example
//! ```rust
//!     use std::fs;
//!     use cmd::command_handler;
//!     use cmd::cmd::Cmd;
//!
//!     /// CommandHandler that provides a user greeting command
//!     #[derive(Debug, Default)]
//!     pub struct Greeting { name: Option<String> }
//!
//!     impl command_handler::CommandHandler for Greeting {
//!         fn execute(&self) {
//!             match &self.name {
//!                 Some(n) => println!("Welcome {}, a cli command interpreter", n),
//!                 None => println!("Welcome! This is a cli command interpreter"),
//!             }
//!         }
//!
//!         fn add_attr(&mut self, attr: &str) {
//!             self.name = Some(String::from(attr));
//!         }
//!     }
//!
//!     /// CommandHandler that prints out help message
//!     #[derive(Debug, Default)]
//!     pub struct Help {}
//!
//!     impl command_handler::CommandHandler for Help {
//!         fn execute(&self) {
//!             println!("Help message");
//!         }
//!
//!         fn add_attr(&mut self, _attr: &str) { }
//!
//!     }
//!
//!     /// CommandHandler that emulates the basic bash touch command to create a new file
//!     #[derive(Debug, Default)]
//!     pub struct Touch { filename: String }
//!
//!     impl command_handler::CommandHandler for Touch {
//!         fn execute(&self) {
//!             match self.filename.as_str() {
//!                 "" => println!("A filename arg needs to be provided!"),
//!                 _ => {
//!                     let fs_result = fs::File::create(&self.filename);
//!                     match fs_result {
//!                         Ok(file) => println!("Created file: {:?}", file),
//!                         Err(_) => println!("Could not create file: {}", self.filename)
//!                     }
//!                 }
//!             }
//!         }
//!
//!         fn add_attr(&mut self, attr: &str) {
//!             self.filename = attr
//!                 .split(" ")
//!                 .next()
//!                 .unwrap_or_default()
//!                 .to_string();
//!         }
//!     }
//!
//!     fn main() -> Result<(), std::io::Error>{
//!         let mut cmd = Cmd::new();
//!
//!         let help = Help::default();
//!         let hello = Touch::default();
//!         let greet = Greeting::default();
//!
//!         cmd.add_cmd(String::from("help"), Box::new(help));
//!         cmd.add_cmd(String::from("touch"), Box::new(hello));
//!         cmd.add_cmd(String::from("greet"), Box::new(greet));
//!
//!         cmd.run()?;
//!
//!         Ok(())
//!
//!     }
//! ```

/// Used for creating the cmd::Cmd struct that contains the CommandHandler implementations as in a HashMap.
pub mod cmd;

/// Contains the CommandHandler trait.
pub mod command_handler;

/// Contains common ready-to-use handlers
pub mod handlers;