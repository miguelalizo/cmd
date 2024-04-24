use std::{any::Any, fmt};

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
/// # Examples
///
/// ```rust
///    #[derive(Debug, Default)]
///    pub struct Greeting { name: Option<String> }
///
///    impl command_handler::CommandHandler for Greeting {
///        fn execute(&self) {
///            match &self.name {
///                Some(n) => println!("Welcome {}, a cli command interpreter", n),
///                None => println!("Welcome! This is a cli command interpreter"),
///            }
///        }
///
///        fn add_attr(&mut self, attr: &str) {
///            self.name = Some(String::from(attr));
///        }
///    }
///
///    /// CommandHandler that prints out help message
///    #[derive(Debug, Default)]
///    pub struct Help {}
///
///    impl command_handler::CommandHandler for Help {
///        fn execute(&self) {
///            println!("Help message");
///        }
///
///        fn add_attr(&mut self, _attr: &str) { }
///
///    }
///
///    /// CommandHandler that emulates the basic bash touch command to create a new file
///    #[derive(Debug, Default)]
///    pub struct Touch { filename: String }
///
///    impl command_handler::CommandHandler for Touch {
///        fn execute(&self) {
///            match self.filename.as_str() {
///                "" => println!("A filename arg needs to be provided!"),
///                _ => {
///                    let fs_result = fs::File::create(&self.filename);
///                    match fs_result {
///                        Ok(file) => println!("Created file: {:?}", file),
///                        Err(_) => println!("Could not create file: {}", self.filename)
///                    }
///                }
///            }
///        }
///
///        fn add_attr(&mut self, attr: &str) {
///            self.filename = attr
///                .split(" ")
///                .next()
///                .unwrap_or_default()
///                .to_string();
///        }
///    }
/// ```
pub trait CommandHandler: fmt::Debug + AToAny {
    /// Required method to execute a command
    fn execute(&self, _args: String);
}