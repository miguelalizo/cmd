use std::io;
use std::collections::HashMap;

use crate::command_handler::CommandHandler;

/// Command interpreter implemented as struct that contains
/// a handles HashMap of command strings and Boxed CommandHandlers
#[derive(Debug, Default)]
pub struct Cmd<R: io::BufRead, W: io::Write>{
    handles: HashMap<String, Box<dyn CommandHandler<W>>>,
    stdin: R,
    stdout: W
}

impl<R: io::BufRead + 'static, W: io::Write + 'static> Cmd<R, W>{
    /// Create new Cmd instance
    pub fn new(reader: R, writer: W) -> Cmd<R, W>
    where
        W: io::Write,
        R: io::Read
    {
        Cmd {
            handles: HashMap::new(),
            stdin: reader,
            stdout: writer
        }
    }

    pub fn default() -> Cmd<io::BufReader<io::Stdin>, io::Stdout> {
        let reader = io::BufReader::new(io::stdin());
        let writer = io::stdout();

        Cmd {
            handles: HashMap::new(),
            stdin: reader,
            stdout: writer
        }
    }

    /// Start the command interpreter
    ///
    pub fn run(&mut self) -> Result<(), io::Error>{
        loop {
            // print promt at every iteration and flush stdout to ensure user
            // can type on same line as promt
            self.stdout.write(b"(cmd) ")?;
            self.stdout.flush()?;

            // get user input from stdin
            let mut inputs = String::new();
            self.stdin.read_line(&mut inputs)?;
            let inputs = inputs.trim();

            // separate user input into a command and optional args
            if !inputs.is_empty() {
                let (command, args) = self.parse_cmd(inputs);

                // attempt to execute command
                if let Some(handler) = self.handles.get(&command) {
                    if let 0 = handler.execute(&mut self.stdout, args) { break; }
                } else {
                    self.stdout.write(format!("No command {command}\n").as_bytes())?;
                }
            }
        }
        Ok(())
    }


    /// Insert new command into the Cmd handles HashMap
    ///
    /// ## Note: Will not overwrite existing handles.
    pub fn add_cmd(&mut self, name: String, handler: Box<dyn CommandHandler<W>>) -> Result<(), io::Error> {
        if let Some(_) = self.handles.get(&name) {
            self.stdout.write(format!("Warning: Command with handle {name} already exists.").as_bytes())?;
        } else {
        self.handles.insert(name, handler);
        }
        Ok(())
    }

    fn parse_cmd(&self, line: &str) -> (String, String) {
        let mut words = line.split_whitespace();
        let command = words.next().unwrap_or_default().to_string();
        let args: String = words.collect::<Vec<_>>().join(" ");
        (command, args)
    }

    #[cfg(test)]
    fn get_cmd(&self, key: String) -> Option<&Box<dyn CommandHandler<W>>> {
        self.handles.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, io::BufRead};
    use std::io::{self, BufReader, Write};

    use super::*;
    use crate::handlers::Quit;

    #[derive(Debug, Default)]
    pub struct Greeting { }

    impl<W: io::Write> CommandHandler<W> for Greeting {
        fn execute(&self, stdout: &mut W, _args: String) -> usize {
            write!(stdout, "Hello there!").unwrap();
            1
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
            Ok(0)
        }
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "failed on flush"))
        }
    }

    fn setup() -> Cmd<io::BufReader<std::fs::File>, Vec<u8>> {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);

        let stdout: Vec<u8> = Vec::new();
        let mut app: Cmd<io::BufReader<std::fs::File>, Vec<u8>> = Cmd::new( stdin, stdout );
        let greet_handler = Greeting::default();

        // Add the trait object to the HashMap
        app.add_cmd(String::from("greet"), Box::new(greet_handler)).unwrap();
        app.add_cmd(String::from("quit"), Box::new(Quit::default())).unwrap();
        app

    }

    #[test]
    fn test_add_cmd() -> Result<(), io::Error> {
        let mut app = setup();

        let h = app.get_cmd(String::from("greet"));

        // Verify that the key-value pair exists in the HashMap
        assert!(h.is_some());

        // Verify the value can cast down to Greeting
        let it: &dyn Any = h.unwrap().as_any();
        assert!(!it.downcast_ref::<Greeting>().is_none());

        // Verify message is printed out when a handle with existing name is added
        app.add_cmd("greet".to_string(), Box::new(Greeting {} ))?;

        let mut std_out_lines = app.stdout.lines();
        let line1 = std_out_lines.next().unwrap().unwrap();

        assert_eq!(line1, "Warning: Command with handle greet already exists.");
        Ok(())

    }

    #[test]
    fn test_add_cmd_always_error() {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);
        let stdout = StdoutWriteErr;
        let mut app = Cmd::new( stdin, stdout );

        // add same command twice, which will cause the self.stdout.write() path to output error
        let _ok = app.add_cmd("greet".to_string(), Box::new(Greeting {} )).unwrap();
        let e = app.add_cmd("greet".to_string(), Box::new(Greeting {} )).unwrap_err();

        assert_eq!(e.to_string(), "failed on write");
        assert_eq!(e.kind(), io::ErrorKind::Other);
    }

    #[test]
    fn test_parse_cmd(){
        let app = setup();
        let line = "command arg1 arg2";
        assert_eq!(app.parse_cmd(line), ("command".to_string(), "arg1 arg2".to_string()))
    }

    #[test]
    fn test_run() -> Result<(), io::Error> {
        let mut app = setup();

        app.run()?;

        let std_out_lines = app.stdout;
        let line1 = String::from_utf8(std_out_lines).unwrap();

        assert_eq!(line1, "(cmd) Hello there!(cmd) (cmd) No command non\n(cmd) ");
        Ok(())
    }

    #[test]
    fn test_run_stdout_write_err() {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);
        let stdout = StdoutWriteErr;
        let mut app = Cmd::new( stdin, stdout );
        app.stdout.flush().unwrap();

        let e = app.run().unwrap_err();

        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "failed on write");
    }

    #[test]
    fn test_run_stdout_flush_err() {
        let f = std::fs::File::open("test_files/test_in.txt").unwrap();
        let stdin = io::BufReader::new(f);
        let stdout = StdoutFlushErr;
        let mut app = Cmd::new( stdin, stdout );
        app.stdout.write(b"hi").unwrap();

        let e = app.run().unwrap_err();

        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "failed on flush");
    }

    #[test]
    fn test_run_stdin_read_err() {
        let stdin = BufReader::new(StdinAlwaysErr);
        let stdout = io::stdout();
        let mut app = Cmd::new( stdin, stdout );

        let e = app.run().unwrap_err();

        assert_eq!(e.kind(), io::ErrorKind::Other);
        assert_eq!(e.to_string(), "failed on read");
    }

    #[test]
    fn test_default() {
        let app = Cmd::<io::BufReader<io::Stdin>, io::Stdout>::default();
        assert!(app.handles.is_empty())
    }
}

