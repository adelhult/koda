use rlua::{Error, MultiValue};
use colored::*;
use rustyline::Editor;
use std::path::PathBuf;
use::std::env;
use std::fs;
use koda::{
    run_lua_code, 
    transpile,
    get_lua_state,
    show_swedish_values,
    error_repr}
;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Start the repl if no file was specified
    if args.is_empty() {
        repl();
        return;
    }

    let filename = PathBuf::from(args[0].clone());
    match filename.extension() {
        None => {
            println!("Du måste ange en fil som slutar med .kod eller .lua");
            return;
        }
        Some(extension) => {
            if !(extension == "lua" || extension == "kod") {
                println!("Du måste ange en fil som slutar med .kod eller .lua");
                return;
            }
        }
    }
      
    let raw_code = fs::read_to_string(filename)
        .expect("Misslyckades med att läsa filen");
        
    let lua_code = transpile(&raw_code);

    match run_lua_code(&lua_code, &args) {
        Ok(_) => (),
        Err(error) => {
            println!("Hoppsan! Det finns ett problem i din kod! \n");
            println!("{}", error_repr(error));
        }
    }
}

// REPL based on rlua example code 
pub fn repl() {
    let lua = get_lua_state(None).unwrap();
    lua.context(|ctx| {
        let mut editor = Editor::<()>::new();
        loop {
            let mut prompt = "> ";
            let mut line = String::new();
        
            loop {
                match editor.readline(prompt) {
                    Ok(input) => line.push_str(&input),
                    Err(_) => return,
                }

                // did the user input a repl command?
                if let Some(':') = line.chars().next() {
                    match run_command(&line) {
                        0 => return,
                        _ => break,
                    };
                }

                let code = transpile(&line);

                match ctx.load(&code).eval::<MultiValue>() {
                    Ok(values) => {
                        editor.add_history_entry(line);
                        let output = values
                                    .iter()
                                    .map(|value| show_swedish_values(&value))
                                    .collect::<Vec<_>>()
                                    .join("\t");
                        if cfg!(windows) {
                            println!("{}", output);
                        } else {
                            println!("{}", output.green());
                        }
                        break;
                    }
                    Err(Error::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        line.push_str("\n");
                        prompt = ">> ";
                    }
                    Err(e) => {
                        if cfg!(windows) {
                            eprintln!("{}", error_repr(e));
                        } else {
                            eprintln!("{}", error_repr(e).red());
                        }
                        break; 
                    }
                }
            }
        }
    });
}

/// Returns true if you should quit the repl
fn run_command(c: &str) -> u8 {
    let help = "För att lära dig mer om Koda, besök https://github.com/adelhult/koda";
    match c {
        ":q" => 0,
        ":quit" => 0,
        ":a" => 0,
        ":avsluta" => 0,
        ":h" => {
            println!("{}", help);
            1
        }
        ":hjälp" => {
            println!("{}", help);
            1
        }
        _ => {
            println!("Okänt kommando. Prova ':avsluta' eller ':hjälp'");
            1
        },
    }
}