# cmd

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.60%2B-blue.svg)](https://www.rust-lang.org/)

A crate for creating custom line-oriented command interpreters in Rust.

I wrote this as a Rust learning project and was inspired by Python's [cmd](https://docs.python.org/3/library/cmd.html) and the [dev.to article: A small library for writing line oriented-command interpreters in rust](https://dev.to/raminfp/a-small-library-for-writing-line-oriented-command-interpreters-in-the-rust-4phl).

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
use std::io;

use cmd::command_handler::CommandHandler;
use cmd::cmd::Cmd;
use cmd::handlers::Quit;


/// CommandHandler that prints out help message
#[derive(Debug, Default)]
pub struct Help;

impl<W: io::Write> CommandHandler<W> for Help {
    fn execute(&self, _stdout: &mut W, _args: String) -> usize {
        println!("Help message");
        1
    }
}

/// CommandHandler that emulates the basic bash touch command to create a new file
#[derive(Debug, Default)]
pub struct Touch;

impl<W: io::Write> CommandHandler<W> for Touch {
    fn execute(&self, _stdout: &mut W, _args: String) -> usize {
        let filename = _args.split_whitespace().next().unwrap_or_default();

        if filename.len() == 0 {
            println!("Need to specify a filename");
        } else {
            let fs_result = std::fs::File::create(filename);
            match fs_result {
                Ok(file) => println!("Created file: {:?}", file),
                Err(_) => println!("Could not create file: {}", filename)
            }
        }
        1
    }
}


fn main() -> Result<(), std::io::Error>{
    let mut cmd = Cmd::<io::BufReader<io::Stdin>, io::Stdout>::default();

    let help = Help::default();
    let hello = Touch::default();
    let quit = Quit::default();

    cmd.add_cmd(String::from("help"), Box::new(help));
    cmd.add_cmd(String::from("touch"), Box::new(hello));
    cmd.add_cmd(String::from("quit"), Box::new(quit));

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