use crate::command_handler::CommandHandler;

/// Ready-to-use command to quit the cmd loop
#[derive(Debug, Default)]
pub struct Quit {}

impl CommandHandler for Quit {
    fn execute(&self, _args: String) {
        std::process::exit(0);
    }
}

