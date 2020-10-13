use rlua::{Error, MultiValue};
use koda::{run_lua_code, 
           transpile,
           get_lua_state,
           show_swedish_values,
           error_repr};
use std::path::PathBuf;
use::std::env;
use std::fs;
use rustyline::Editor;

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
                let code = transpile(&line);

                match ctx.load(&code).eval::<MultiValue>() {
                    Ok(values) => {
                        editor.add_history_entry(line);
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|value| show_swedish_values(&value))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
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
                        eprintln!("{}", error_repr(e));
                        break; 
                    }
                }
            }
        }
    });
}