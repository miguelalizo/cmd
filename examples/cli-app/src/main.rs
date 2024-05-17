use rusty_cmd::cmd::Cmd;
use rusty_cmd::command_handler::{CommandHandler, CommandResult};
use rusty_cmd::handlers::Quit;
use std::io;
use std::io::Write;

/// CommandHandler that prints out help message
#[derive(Debug, Default)]
pub struct Help;

impl CommandHandler for Help {
    fn execute(&self, _stdout: &mut io::Stdout, _args: &str) -> CommandResult {
        writeln!(_stdout, "Help message").unwrap();
        CommandResult::Continue
    }
}

/// CommandHandler that emulates the basic bash touch command to create a new file
#[derive(Debug, Default)]
pub struct Touch;

impl CommandHandler for Touch {
    fn execute(&self, _stdout: &mut io::Stdout, _args: &str) -> CommandResult {
        let filename = _args.split_whitespace().next().unwrap_or_default();

        if filename.len() == 0 {
            println!("Need to specify a filename");
        } else {
            let fs_result = std::fs::File::create(filename);
            match fs_result {
                Ok(file) => println!("Created file: {:?}", file),
                Err(_) => println!("Could not create file: {}", filename),
            }
        }
        CommandResult::Continue
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut cmd = Cmd::new(io::BufReader::new(io::stdin()), io::stdout());

    let help = Help::default();
    let hello = Touch::default();
    let quit = Quit::default();

    cmd.add_cmd(String::from("help"), Box::new(help))?;
    cmd.add_cmd(String::from("touch"), Box::new(hello))?;
    cmd.add_cmd(String::from("quit"), Box::new(quit))?;

    cmd.run()?;

    Ok(())
}
