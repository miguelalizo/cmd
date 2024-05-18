use std::collections::HashMap;
use std::io;

use crate::command_handler::{CommandHandler, CommandResult};

/// Command interpreter implemented as struct that contains
/// boxed CommandHandlers in a hashmap with Strings as the keys
#[derive(Debug)]
pub struct Cmd<R: io::BufRead, W: io::Write> {
    handles: HashMap<String, Box<dyn CommandHandler<W>>>,
    stdin: R,
    stdout: W,
}

impl<R: io::BufRead + 'static, W: io::Write + 'static> Cmd<R, W> {
    /// Create new Cmd instance
    pub fn new(reader: R, writer: W) -> Cmd<R, W>
    where
        W: io::Write,
        R: io::Read,
    {
        Cmd {
            handles: HashMap::new(),
            stdin: reader,
            stdout: writer,
        }
    }

    /// Start the command interpreter
    ///
    /// Handlers with return code 0 will break the loop
    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            // print promt at every iteration and flush stdout to ensure user
            // can type on same line as promt
            write!(self.stdout, "(cmd) ")?;
            self.stdout.flush()?;

            // get user input from stdin
            let mut inputs = String::new();
            self.stdin.read_line(&mut inputs)?;
            let inputs = inputs.trim();

            // separate user input into a command and optional args
            if !inputs.is_empty() {
                let (command, args) = self.parse_cmd(inputs);

                // attempt to execute command
                if let Some(handler) = self.handles.get(command) {
                    if matches!(
                        handler.execute(&mut self.stdout, args),
                        CommandResult::Break
                    ) {
                        break;
                    }
                } else {
                    writeln!(self.stdout, "No command {}", command)?;
                }
            }
        }
        Ok(())
    }

    /// Insert new command into the Cmd handles HashMap
    ///
    /// ## Note: Will not overwrite existing handler names
    pub fn add_cmd(
        &mut self,
        name: String,
        handler: Box<dyn CommandHandler<W>>,
    ) -> Result<(), io::Error> {
        match self.handles.get(&name) {
            Some(_) => write!(
                self.stdout,
                "Warning: Command with handle {} already exists.",
                name
            )?,
            None => {
                self.handles.insert(name, handler);
            }
        }

        Ok(())
    }

    // Parse command string into command, and args Strings
    fn parse_cmd<'a>(&self, line: &'a str) -> (&'a str, &'a str) {
        let line = line.trim();
        let first_space = line.find(' ').unwrap_or(line.len());
        let command = &line[..first_space];

        let args = line[command.len()..].trim();
        (command, args)
    }

    #[cfg(test)]
    fn get_cmd(&self, key: String) -> Option<&Box<dyn CommandHandler<W>>> {
        self.handles.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufReader, Write};
    use std::{any::Any, io::BufRead};

    use super::*;
    use crate::command_handler::CommandResult;
    use crate::handlers::Quit;

    #[derive(Debug, Default)]
    pub struct Greeting {}

    impl<W: io::Write> CommandHandler<W> for Greeting {
        fn execute(&self, stdout: &mut W, _args: &str) -> CommandResult {
            write!(stdout, "Hello there!").unwrap();
            CommandResult::Continue
        }
    }

    // Mock object for stdin that always errs on stdin.read()
    struct StdinAlwaysErr;

    impl io::Read for StdinAlwaysErr {
        fn read(&mut self, _: &mut [u8]) -> Result<usize, std::io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "failed on read"))
        }
    }

    // Mock object for stdout that always errs on stdout.write()
    struct StdoutWriteErr;

    impl io::Write for StdoutWriteErr {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "failed on write"))
        }
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
    // Mock object for stdout that always errs on stdout.flush()
    struct StdoutFlushErr;

    impl io::Write for StdoutFlushErr {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error> {
            Ok(1)
        }
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "failed on flush"))
        }
    }

    fn setup() -> Cmd<io::BufReader<std::fs::File>, Vec<u8>> {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);

        let stdout: Vec<u8> = Vec::new();
        let mut app: Cmd<io::BufReader<std::fs::File>, Vec<u8>> = Cmd::new(stdin, stdout);
        let greet_handler = Greeting::default();

        // Add the trait object to the HashMap
        app.add_cmd(String::from("greet"), Box::new(greet_handler))
            .unwrap();
        app.add_cmd(String::from("quit"), Box::new(Quit::default()))
            .unwrap();
        app
    }

    #[test]
    fn test_add_cmd() {
        let mut app = setup();

        let h = app.get_cmd(String::from("greet"));

        // Verify that the key-value pair exists in the HashMap
        assert!(h.is_some());

        // Verify the value can cast down to Greeting
        let it: &dyn Any = h.unwrap().as_any();
        assert!(!it.downcast_ref::<Greeting>().is_none());

        // Verify message is printed out when a handle with existing name is added
        app.add_cmd("greet".to_string(), Box::new(Greeting {}))
            .unwrap();

        let mut std_out_lines = app.stdout.lines();
        let line1 = std_out_lines.next().unwrap().unwrap();

        assert_eq!(line1, "Warning: Command with handle greet already exists.");
    }

    #[test]
    fn test_add_cmd_always_error() {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);
        let stdout = StdoutWriteErr;
        let mut app = Cmd::new(stdin, stdout);

        // add same command twice, which will cause the self.stdout.write() path to output error
        let _ok = app
            .add_cmd("greet".to_string(), Box::new(Greeting {}))
            .unwrap();
        let e = app
            .add_cmd("greet".to_string(), Box::new(Greeting {}))
            .unwrap_err();

        assert_eq!(e.to_string(), "failed on write");
        assert_eq!(e.kind(), io::ErrorKind::Other);
    }

    #[test]
    fn test_parse_cmd() {
        let app = setup();
        let line = "command arg1 arg2";
        assert_eq!(app.parse_cmd(line), ("command", "arg1 arg2"))
    }
    #[test]

    fn test_parse_cmd_empty_line() {
        let app = setup();
        assert_eq!(app.parse_cmd(""), ("", ""));

        assert_eq!(app.parse_cmd("    "), ("", ""));
    }

    #[test]
    fn test_parse_cmd_remove_extra_spaces() {
        let app = setup();
        let line = "     command arg1 arg2";
        assert_eq!(app.parse_cmd(line), ("command", "arg1 arg2"))
    }

    #[test]
    fn test_parse_cmd_empty_args() {
        let app = setup();
        let line = "command";
        assert_eq!(app.parse_cmd(line), ("command", ""));

        let line = "     command";
        assert_eq!(app.parse_cmd(line), ("command", ""));
    }

    #[test]
    fn test_run() {
        let mut app = setup();

        app.run().unwrap();

        let std_out_lines = app.stdout;
        let line1 = String::from_utf8(std_out_lines).unwrap();

        assert_eq!(
            line1,
            "(cmd) Hello there!(cmd) (cmd) No command non\n(cmd) "
        );
    }

    #[test]
    fn test_run_stdout_write_err() {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);
        let stdout = StdoutWriteErr;
        let mut app = Cmd::new(stdin, stdout);

        app.stdout.flush().unwrap(); // this line is here to ensure all statements are run during testing

        let e = app.run().unwrap_err();

        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "failed on write");
    }

    #[test]
    fn test_run_stdout_flush_err() {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);
        let stdout = StdoutFlushErr;
        let mut app = Cmd::new(stdin, stdout);

        let e = app.run().unwrap_err();

        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "failed on flush");
    }

    #[test]
    fn test_run_stdin_read_err() {
        let stdin = BufReader::new(StdinAlwaysErr);
        let stdout = io::stdout();
        let mut app = Cmd::new(stdin, stdout);

        let e = app.run().unwrap_err();

        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "failed on read");
    }
}
