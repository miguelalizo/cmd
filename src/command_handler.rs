/// Trait for creating new commands
///
/// Returning CommandResult::Break instructs the Cmd.run() loop to break.
///
/// # Examples
///
/// ```rust
/// // CommandHandler that prints out help message
/// use std::io;
/// use std::io::Write;
/// use rusty_cmd::command_handler::{CommandHandler, CommandResult};
///
/// #[derive(Default)]
/// pub struct Help;
///
/// impl<W> CommandHandler<W> for Help
///     where W: std::io::Write {
///     fn execute(&self, output: &mut W, args: &[&str]) -> CommandResult {
///         writeln!(output, "Help message").unwrap();
///         CommandResult::Continue
///     }
/// }
///
/// /// CommandHandler that prints out a greeting
/// #[derive(Default)]
/// pub struct Greet;
///
/// impl<W> CommandHandler<W> for Greet
///     where W: std::io::Write {
///     fn execute(&self, output: &mut W, args: &[&str]) -> CommandResult {
///         let joined_args = args.join(", ");
///         match args.len() {
///             0 => output.write(format!("Hello, {}!", joined_args).as_bytes()).unwrap(),
///             _ => output.write(b"Hello!").unwrap(),
///         };
///         CommandResult::Continue
///     }
/// }
/// ```
pub trait CommandHandler<W>
where
    W: std::io::Write,
{
    /// Required method to execute a command
    fn execute(&self, output: &mut W, args: &[&str]) -> CommandResult;
}

/// Enum to determine whether to continue or break the Cmd.run() loop
pub enum CommandResult {
    Continue,
    Break,
}

/// Blanket CommandHandler implementation for Fn(&mut W, &[&str]) -> CommandResult
/// allows CommandHandlers to be registered as closures
impl<F, W> CommandHandler<W> for F
where
    W: std::io::Write,
    F: Fn(&mut W, &[&str]) -> CommandResult,
{
    fn execute(&self, output: &mut W, args: &[&str]) -> CommandResult {
        self(output, args)
    }
}
