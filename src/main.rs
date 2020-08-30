use rlua::{Error};
use dont_disappear::enter_to_continue;
use koda::{run_lua_code, transpile, show_swedish};
use std::path::PathBuf;
use::std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Du måste ange en fil!");
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
    
    
    let raw_code = fs::read_to_string(filename).expect("Misslyckades med att läsa filen");
    let lua_code = transpile(&raw_code);
    match run_lua_code(&lua_code, &args) {
        Ok(_) => (),
        Err(error) => {
            println!("Hoppsan! Det finns ett problem i din kod! \n");
            match error {
                Error::SyntaxError { message, .. } => {
                    println!("Det är ett syntax-fel som har uppstått.");
                    println!("De brukar bero på att man stavat fel på en variabel eller glömt något tecken.");
                    println!(
                        "Här är ett meddelande på engelska som berättar om felet: {}",
                        show_swedish(&message)
                    );
                }
                Error::RuntimeError(message) => {
                    println!("Det är ett runtime-fel som har uppstått.");
                    println!(
                        "Här är ett meddelande på engelska som berättar om felet: {}",
                        show_swedish(&message)
                    );
                }
                e => {
                    println!("Här är en text på engelska där felet förklaras: {}", 
                             show_swedish(&(e.to_string()))
                    );
                }
            }
        }
    }

    if cfg!(windows) {
        enter_to_continue::custom_msg("Tryck på ENTER-tangenten för att avsluta programmet.");
    }
}
