use std::{any::Any, fmt, io};

// TODO: Look into why this works!
pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Interface for creating new commands
///
/// Defines io::Stdout as the default generic type
///
/// # Examples
///
/// ```rust
/// // CommandHandler that prints out help message
/// use std::io;
/// use std::io::Write;
/// use rusty_cmd::command_handler::CommandHandler;
///
/// #[derive(Debug, Default)]
/// pub struct Help;
///
/// impl CommandHandler for Help {
///     fn execute(&self, _stdout: &mut io::Stdout, _args: String) -> usize {
///         writeln!(_stdout, "Help message").unwrap();
///         1
///     }
/// }
///
/// /// CommandHandler that prints out a greeting
/// #[derive(Debug, Default)]
/// pub struct Greet;
///
/// impl<W: io::Write> CommandHandler<W> for Greet {
///     fn execute(&self, _stdout: &mut W, _args: String) -> usize {
///         match _args.len() {
///             0 => _stdout.write(format!("Hello, {}!", _args).as_bytes()).unwrap(),
///             _ => _stdout.write(b"Hello!").unwrap(),
///         };
///         1
///     }
/// }
/// ```
pub trait CommandHandler<W = io::Stdout>: fmt::Debug + AToAny {
    /// Required method to execute a command
    fn execute(&self, _stdout: &mut W, _args: &str) -> usize;
}
