# rusty-cmd

A crate for creating custom line-oriented command interpreters in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.60%2B-blue.svg)](https://www.rust-lang.org/)
[![Documentation](https://img.shields.io/badge/Documentation-Link-blue.svg)](https://docs.rs/rusty-cmd/latest/rusty_cmd/)

I wrote this as a Rust learning project and was inspired by Python's [cmd](https://docs.python.org/3/library/cmd.html) and the [dev.to article: A small library for writing line oriented-command interpreters in rust](https://dev.to/raminfp/a-small-library-for-writing-line-oriented-command-interpreters-in-the-rust-4phl).

## Features

- Create custom command interpreters.
- Easily define and execute commands.
- Implement optional command arguments.
- Redirect output to types that implement io::Write.
- Create teardown function by returing 0 from a command handler.

## Overview

rusty-cmd provides two crates:
- `cmd`: Used for creating the `Cmd` struct that contains the `CommandHandler` implementations in a `HashMap`.
- `command_handler`: Contains the `CommandHandler` trait.

## Example

```rust
use std::io;
use std::io::Write;

use rusty_cmd::command_handler::{CommandHandler, CommandResult};
use rusty_cmd::cmd::Cmd;
use rusty_cmd::handlers::Quit;


/// CommandHandler that prints out help message
#[derive(Debug, Default)]
pub struct Help;

impl CommandHandler for Help {
    fn execute(&self, _stdout: &mut io::Stdout, _args: &[&str]) -> CommandResult {
        writeln!(_stdout, "Help message").unwrap();
        CommandResult::Continue
    }
}

/// CommandHandler that emulates the basic bash touch command to create a new file
#[derive(Debug, Default)]
pub struct Touch;

impl CommandHandler for Touch {
    fn execute(&self, _stdout: &mut io::Stdout, _args: &[&str]) -> CommandResult {
        let option_filename = _args.first();
    
        match option_filename {
            Some(filename) => {
                let fs_result = std::fs::File::create(filename);
                match fs_result {
                    Ok(file) => println!("Created file: {:?}", file),
                    Err(_) => println!("Could not create file: {}", filename),
                }
            }
            None => println!("Need to specify a filename"),
        }
        CommandResult::Continue
    }
}


fn main() -> Result<(), std::io::Error>{
    let mut cmd = Cmd::new(
        io::BufReader::new(io::stdin()),
        io::stdout()
    );


    let help = Help::default();
    let hello = Touch::default();
    let quit = Quit::default();

    cmd.add_cmd(String::from("help"), Box::new(help))?;
    cmd.add_cmd(String::from("touch"), Box::new(hello))?;
    cmd.add_cmd(String::from("quit"), Box::new(quit))?;

    cmd.run()?;

    Ok(())

}
```

## Usage

To use rusty-cmd in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty-cmd = "1.0.0"
```

Then import the crate in your Rust code:

```rust
use rusty_cmd::command_handler::{CommandHandler, CommandResult};
use rusty_cmd::cmd::Cmd;
```

## Contributing
We welcome contributions! See our [CONTIBUTING guide](CONTRIBUTING.md)

## Contributors ✨

<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/miguelalizo"><img src="https://avatars.githubusercontent.com/u/108839050?v=4" width="100px;" alt="Miguel Alizo"/><br /><sub><b>Miguel Alizo</b></sub></a><br/> <a>🦀✅</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/aughey"><img src="https://avatars.githubusercontent.com/u/12618?v=4" width="100px;" alt="John Aughey"/><br /><sub><b>John Aughey</b></sub></a><br/> <a>🦀✅</a></td>
</table>


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
