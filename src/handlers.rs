use crate::command_handler::{CommandHandler, CommandResult};
use std::io;

/// Ready-to-use command to quit the cmd loop
///
/// Return code 0 instructs the Cmd.run() loop to break
#[derive(Debug, Default)]
pub struct Quit {}

impl<W: io::Write> CommandHandler<W> for Quit {
    fn execute(&self, _cmd: &mut W, _args: &[&str]) -> CommandResult {
        CommandResult::Stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit() {
        let q = Quit::default();
        assert!(matches!(
            q.execute(&mut io::stdout(), &[]),
            CommandResult::Stop
        ))
    }
}
