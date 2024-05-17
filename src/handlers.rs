use crate::command_handler::CommandHandler;
use std::io;

/// Ready-to-use command to quit the cmd loop
///
/// Return code 0 instructs the Cmd.run() loop to break
#[derive(Debug, Default)]
pub struct Quit {}

impl<W: io::Write> CommandHandler<W> for Quit {
    fn execute(&self, _cmd: &mut W, _args: &str) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit() {
        let q = Quit::default();
        assert_eq!(q.execute(&mut io::stdout(), ""), 0)
    }
}
