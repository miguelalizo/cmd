use std::io;
use crate::command_handler::CommandHandler;

/// Ready-to-use command to quit the cmd loop
#[derive(Debug, Default)]
pub struct Quit {}

impl<W: io::Write> CommandHandler<W> for Quit {
    fn execute(&self, _cmd: &mut W, _args: String) -> usize {
        0
    }
}

