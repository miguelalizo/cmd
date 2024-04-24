use cmd::command_handler::CommandHandler;
use cmd::cmd::Cmd;
use cmd::handlers::Quit;

/// CommandHandler that provides a user greeting command
#[derive(Debug, Default)]
pub struct Greeting { }

impl CommandHandler for Greeting {
    fn execute(&self, _args: String) {
        if _args.len() == 0 {
            println!("Hello there, stranger!");
        } else {
            println!("Hello there, {}", _args);
        }
    }
}

/// CommandHandler that prints out help message
#[derive(Debug, Default)]
pub struct Help {}

impl CommandHandler for Help {
    fn execute(&self, _args: String) {
        println!("Help message");
    }
}

/// CommandHandler that emulates the basic bash touch command to create a new file
#[derive(Debug, Default)]
pub struct Touch { }

impl CommandHandler for Touch {
    fn execute(&self, _args: String) {
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
    }

}

fn main() -> Result<(), std::io::Error>{
    let mut cmd = Cmd::new();

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
