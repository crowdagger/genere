use genere::{Generator, Result, Gender};

use std::env;
use std::process::exit;
use std::io::{self, Read};

// Display help and exit
fn help() -> ! {
    println!(r#"
Genere, version {version}

USAGE:
genere <symbol>
    instantiate the `symbol` in your JSON content.
    Content is read from standard input. If you want to read from a file:
    genere some_symbol < file.json

OTHER USAGES:
genere --help
    will display this help message instead of parsing content.

genere --regender m
    will parse the content as a string instead of a JSON structure and will gender it
    according to the specified gender (can be 'm', 'f', or 'n').

"#,
    version = env!("CARGO_PKG_VERSION"));
    exit(0);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
    } else {
        match args[1].as_str() {
            "--help" => help(),
            "--regender" => {
                if args.len() < 3 {
                    println!("Error: --regender takes a gender (m/f/n) as additional argument");
                    exit(0);
                } else {
                    let gender = match args[2].as_str() {
                        "m" => Gender::Male,
                        "n" => Gender::Neutral,
                        "f" => Gender::Female,
                        _ => {
                            println!("Gender must either be 'm', 'f', or 'n'");
                            exit(0);
                        },
                    };
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer)?;
                    let mut generator = Generator::new();
                    generator.add_move(String::from("main[gender]"),
                                                    vec![buffer])?;
                    generator.set_gender("gender", gender);
                    println!("{}", generator.instantiate("main")?);
                    Ok(())
                }
            },
            symbol => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                let mut generator = Generator::new();
                generator.add_json(&buffer)?;
                println!("{}", generator.instantiate(symbol)?);
                Ok(())
            }
        }
    }
}
