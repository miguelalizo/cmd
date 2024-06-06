use std::collections::HashMap;
use std::io;

use crate::command_handler::{CommandHandler, CommandResult};

/// Command interpreter implemented as struct that contains
/// boxed CommandHandlers in a hashmap with Strings as the keys
pub struct Cmd<R, W>
where
    W: io::Write + 'static,
    R: io::BufRead + 'static,
{
    handles: HashMap<String, Box<dyn CommandHandler<W>>>,
    stdin: R,
    stdout: W,
}

impl<R, W> Cmd<R, W>
where
    W: io::Write + 'static,
    R: io::BufRead + 'static,
{
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
                let (command, args) = parse_cmd(inputs);
                let args = split_args(args);

                // attempt to execute command
                if let Some(handler) = self.handles.get(command) {
                    if matches!(
                        handler.execute(&mut self.stdout, &args),
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

    /// Insert new handler into the Cmd handles HashMap defined by a function or closure
    ///
    /// ## Note: Will not overwrite existing handler names
    pub fn add_cmd_fn(
        &mut self,
        name: String,
        handler: impl Fn(&mut W, &[&str]) -> CommandResult + 'static,
    ) -> Result<(), io::Error> {
        self.add_cmd(name, handler)
    }

    /// Insert new handler into the Cmd handles HashMap
    ///
    /// ## Note: Will not overwrite existing handler names
    pub fn add_cmd(
        &mut self,
        name: String,
        handler: impl CommandHandler<W> + 'static,
    ) -> Result<(), io::Error> {
        match self.handles.get(&name) {
            Some(_) => write!(
                self.stdout,
                "Warning: Command with handle {} already exists.",
                name
            )?,
            None => {
                self.handles.insert(name, Box::new(handler));
            }
        }

        Ok(())
    }

    #[cfg(test)]
    fn get_cmd(&self, key: String) -> Option<&Box<dyn CommandHandler<W>>> {
        self.handles.get(&key)
    }
}

// Parse command string into command, and args Strings
fn parse_cmd(line: &str) -> (&str, &str) {
    let line = line.trim();
    let first_space = line.find(' ').unwrap_or(line.len());
    let command = &line[..first_space];

    let args = line[command.len()..].trim();
    (command, args)
}

fn split_args(args: &str) -> Vec<&str> {
    args.split_whitespace().map(|arg| arg.trim()).collect()
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;
    use std::io::{self, BufReader, Write};

    use super::*;
    use crate::command_handler::CommandResult;
    use crate::handlers::Quit;

    #[derive(Default)]
    pub struct Greeting {}

    impl<W: io::Write> CommandHandler<W> for Greeting {
        fn execute(&self, stdout: &mut W, _args: &[&str]) -> CommandResult {
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
        app.add_cmd(String::from("greet"), greet_handler).unwrap();
        app.add_cmd(String::from("quit"), Quit::default()).unwrap();
        app
    }

    #[test]
    fn test_add_cmd() {
        let app = setup();
        let mut stdout = vec![];

        // Verify that the key-value pair exists in the HashMap
        let h = app.get_cmd(String::from("greet"));
        assert!(h.is_some());

        // Verify right handler was added to hashmap
        h.unwrap().execute(&mut stdout, &[]);
        assert_eq!(String::from_utf8(stdout).unwrap(), "Hello there!");
    }

    #[test]
    fn test_add_existing_cmd() {
        let mut app = setup();

        // Verify message is printed out when a handle with existing name is added
        app.add_cmd("greet".to_string(), Greeting {}).unwrap();

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
        let _ok = app.add_cmd("greet".to_string(), Greeting {}).unwrap();
        let e = app.add_cmd("greet".to_string(), Greeting {}).unwrap_err();

        assert_eq!(e.to_string(), "failed on write");
        assert_eq!(e.kind(), io::ErrorKind::Other);
    }

    #[test]
    fn test_parse_cmd() {
        let line = "command arg1 arg2";
        assert_eq!(parse_cmd(line), ("command", "arg1 arg2"))
    }
    #[test]

    fn test_parse_cmd_empty_line() {
        assert_eq!(parse_cmd(""), ("", ""));
        assert_eq!(parse_cmd("    "), ("", ""));
    }

    #[test]
    fn test_parse_cmd_remove_extra_spaces() {
        let line = "     command arg1 arg2";
        assert_eq!(parse_cmd(line), ("command", "arg1 arg2"))
    }

    #[test]
    fn test_parse_cmd_empty_args() {
        let line = "command";
        assert_eq!(parse_cmd(line), ("command", ""));

        let line = "     command";
        assert_eq!(parse_cmd(line), ("command", ""));
    }

    #[test]
    fn test_run() {
        let mut app = setup();

        app.run().unwrap();

        // let std_out_lines = ;
        let line1 = String::from_utf8(app.stdout).unwrap();

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

    #[test]
    fn test_split_args() {
        let args = "arg1 arg2 arg3";
        let expected = vec!["arg1", "arg2", "arg3"];
        assert_eq!(split_args(args), expected);
    }

    #[test]
    fn split_empty_args() {
        let args = "";
        let expected: Vec<&str> = vec![];
        assert_eq!(split_args(args), expected);
    }
}
