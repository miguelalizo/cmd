# cmd

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.60%2B-blue.svg)](https://www.rust-lang.org/)
[![GitHub stars](https://img.shields.io/github/stars/miguelalizo/cmd-rs.svg?style=social)](https://github.com/miguelalizo/cmd)

A crate for creating custom line-oriented command interpreters in Rust.

## Features

- Create custom command interpreters.
- Easily define and execute commands.
- Implement optional command arguments.

## Overview

cmd provides two crates:
- `cmd`: Used for creating the `Cmd` struct that contains the `CommandHandler` implementations in a `HashMap`.
- `command_handler`: Contains the `CommandHandler` trait.

## Example

```rust
use std::fs;
use cmd::command_handler;
use cmd::cmd::Cmd;

// Define a command handler for greeting
#[derive(Debug, Default)]
pub struct Greeting { name: Option<String> }

impl command_handler::CommandHandler for Greeting {
    fn execute(&self) {
        match &self.name {
            Some(n) => println!("Welcome {}, a CLI command interpreter", n),
            None => println!("Welcome! This is a CLI command interpreter"),
        }
    }

    fn add_attr(&mut self, attr: &str) {
        self.name = Some(String::from(attr));
    }
}

// Define a command handler for printing help message
#[derive(Debug, Default)]
pub struct Help {}

impl command_handler::CommandHandler for Help {
    fn execute(&self) {
        println!("Help message");
    }

    fn add_attr(&mut self, _attr: &str) { }
}

// Define a command handler for creating a new file
#[derive(Debug, Default)]
pub struct Touch { filename: String }

impl command_handler::CommandHandler for Touch {
    fn execute(&self) {
        match self.filename.as_str() {
            "" => println!("A filename arg needs to be provided!"),
            _ => {
                let fs_result = fs::File::create(&self.filename);
                match fs_result {
                    Ok(file) => println!("Created file: {:?}", file),
                    Err(_) => println!("Could not create file: {}", self.filename)
                }
            }
        }
    }

    fn add_attr(&mut self, attr: &str) {
        self.filename = attr
            .split(" ")
            .next()
            .unwrap_or_default()
            .to_string();
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut cmd = Cmd::new();

    let help = Help::default();
    let hello = Touch::default();
    let greet = Greeting::default();

    cmd.add_cmd(String::from("help"), Box::new(help));
    cmd.add_cmd(String::from("touch"), Box::new(hello));
    cmd.add_cmd(String::from("greet"), Box::new(greet));

    cmd.run()?;

    Ok(())
}
```

## Usage

To use cmd-rs in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cmd_interpreter = "0.1.0"
```

Then import the crate in your Rust code:

```rust
use cmd::command_handler;
use cmd::cmd::Cmd;
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```

Make sure to replace `miguelalizo` with your GitHub username in the shields' URLs to make them work correctly. This `README.md` provides an overview of your project, showcases its features, includes an example, explains how to use it, and mentions the license.