//! A crate for creating custom line-oriented command interpreters.
//!
//! # A tour of cmd
//!
//! cmd consists of three crates:
//! - cmd: Used for creating the cmd::Cmd struct that contains the CommandHandler implementations as in a HashMap
//! - command_handler: Contains the CommandHandler trait
//! - handlers: Contains ready-to-use Quit CommandHandler struct
//!
//! ## Example
//! ```rust
//! use std::io;
//! use std::io::Write;
//!
//! use rusty_cmd::cmd::Cmd;
//! use rusty_cmd::command_handler::{CommandHandler, CommandResult};
//! use rusty_cmd::handlers::Quit;
//!
//! /// CommandHandler that prints out help message
//! #[derive(Default)]
//! pub struct Help;
//!
//! impl<W> CommandHandler<W> for Help
//! where
//!     W: std::io::Write,
//! {
//!     fn execute(&self, output: &mut W, _args: &[&str]) -> CommandResult {
//!         writeln!(output, "Help message").expect("Should be able to write to output");
//!         CommandResult::Continue
//!     }
//! }
//!
//! /// CommandHandler that emulates the basic bash touch command to create a new file
//! #[derive(Default)]
//! pub struct Touch;
//!
//! impl<W> CommandHandler<W> for Touch
//! where
//!     W: std::io::Write,
//! {
//!     fn execute(&self, output: &mut W, _args: &[&str]) -> CommandResult {
//!         let option_filename = _args.first();
//!
//!         match option_filename {
//!             Some(filename) => {
//!                 let fs_result = std::fs::File::create(filename);
//!                 match fs_result {
//!                     Ok(file) => writeln!(output, "Created file: {:?}", file)
//!                         .expect("Should be able to write to output"),
//!                     Err(_) => writeln!(output, "Could not create file: {}", filename)
//!                         .expect("Should be able to write to output"),
//!                 }
//!             }
//!             None => println!("Need to specify a filename"),
//!         }
//!         CommandResult::Continue
//!     }
//! }
//!
//! fn main() -> Result<(), std::io::Error> {
//!     let mut cmd = Cmd::new(io::BufReader::new(io::stdin()), io::stdout());
//!
//!     let help = Help;
//!     let hello = Touch;
//!     let quit = Quit::default();
//!
//!     cmd.add_cmd(String::from("help"), help)?;
//!     cmd.add_cmd(String::from("touch"), hello)?;
//!     cmd.add_cmd_fn(String::from("greet"), |output, _args| {
//!         writeln!(output, "hello!").expect("Should be able to write to output");
//!         CommandResult::Continue
//!     })?;
//!     cmd.add_cmd(String::from("quit"), quit)?;
//!
//!     // cmd.run()?; // uncomment to run
//!
//!     Ok(())
//! }
//! ```

/// Used for creating the cmd::Cmd struct that contains the CommandHandler implementations as in a HashMap.
pub mod cmd;

/// Contains the CommandHandler trait.
pub mod command_handler;

/// Contains common ready-to-use handlers
pub mod handlers;
