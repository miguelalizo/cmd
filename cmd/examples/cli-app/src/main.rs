use std::io;

use cmd::command_handler::CommandHandler;
use cmd::cmd::Cmd;
use cmd::handlers::Quit;

/// CommandHandler that provides a user greeting command
#[derive(Debug, Default)]
pub struct Greeting { }

impl<W: io::Write> CommandHandler<W> for Greeting {
    fn execute(&self, _stdout: &mut W, _args: String) -> usize {
        if _args.len() == 0 {
            println!("Hello there, stranger!");
        } else {
            println!("Hello there, {}", _args);
        }
        1
    }
}

/// CommandHandler that prints out help message
#[derive(Debug, Default)]
pub struct Help {}

impl<W: io::Write> CommandHandler<W> for Help {
    fn execute(&self, _stdout: &mut W, _args: String) -> usize {
        println!("Help message");
        1
    }
}

/// CommandHandler that emulates the basic bash touch command to create a new file
#[derive(Debug, Default)]
pub struct Touch { }

impl<W: io::Write> CommandHandler<W> for Touch {
    fn execute(&self, _stdout: &mut W, _args: String) -> usize {
        let filename = _args.split_whitespace().next().unwrap_or_default();

        if filename.len() == 0 {
            println!("Need to specify a filename");
        } else {
            let fs_result = std::fs::File::create(filename);
            match fs_result {
                Ok(file) => println!("Created file: {:?}", file),
                Err(_) => println!("Could not create file: {}", filename)
            }
        }
        1
    }

}

fn main() -> Result<(), std::io::Error>{
    let stdout = io::stdout();
    let stdin = io::BufReader::new(io::stdin());
    let mut cmd: Cmd<io::BufReader<io::Stdin>, io::Stdout> = Cmd::new(stdin, stdout);

    let help = Help::default();
    let hello = Touch::default();
    let quit = Quit::default();
    let greet = Greeting::default();

    cmd.add_cmd(String::from("help"), Box::new(help));
    cmd.add_cmd(String::from("touch"), Box::new(hello));
    cmd.add_cmd(String::from("quit"), Box::new(quit));
    cmd.add_cmd(String::from("greet"), Box::new(greet));

    cmd.run()?;

    Ok(())

}
