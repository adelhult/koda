use std::fs;
use std::env;
use std::path::PathBuf;
use std::io::{stdin,stdout,Write};
use std::collections::HashMap;
use rlua::{Lua, Error};
use dont_disappear::enter_to_continue;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    start_with_args(args);
    enter_to_continue::custom_msg("Tryck på ENTER-tangenten för att avsluta programmet.");
}

/// Take cli args, check if a valid code file exists,
/// then execute code and handle potential errors.
fn start_with_args(args: Vec<String>) {
    if args.len() < 1 {
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
            if !(extension == "lua" ||extension == "kod") {
                println!("Du måste ange en fil som slutar med .kod eller .lua");
                return;
            }
        }
    }

    let raw_code = fs::read_to_string(&filename)
        .expect("Failed to read the given file.");

    // parse raw_code. First, replace keywords and then convert swe characters.
    let code = convert_swe_characters(&replace_keywords(&raw_code));

    // run the code and handle potential errors
    match run_code(code, &args) {
        Ok(_) => (),
        Err(error) => {
            println!("Hoppsan! Det finns ett problem i din kod! \n");
            match error {
                Error::SyntaxError{message, incomplete_input: _} => {
                    println!("Det är ett syntax-fel som har uppstått.");
                    println!("De brukar bero på att man stavat fel på en variabel eller glömt något tecken.");
                    println!("Här är ett meddelande på engelska som berättar om felet: {}", message);
                    return;
                },
                Error::RuntimeError(message) => {
                    println!("Det är ett runtime-fel som har uppstått.");
                    println!("Här är ett meddelande på engelska som berättar om felet: {}", message);
                    return;
                }
                e => {
                    println!("Här är en text på engelska där felet förklaras: {:?}", e);
                    return;
                }
            }
        }
    }
}

fn run_code(code: String, arguments: &Vec<String>) -> Result<(), Error> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        // add "_filnanmn" to globals
        let globals = lua_ctx.globals();
        globals.set("_filnamn", arguments[0].clone())?;

        // add env arguments to argument_table, 
        // skipping the first one (filename).
        let arguments_table = lua_ctx.create_table()?;
        for (i, arg) in arguments.iter().skip(1).enumerate() {
            arguments_table.set(i + 1, arg.clone())?;
        }
        // add the arguments_table to the global scope of the lua_ctx.
        // Note: the name should perhaps be changed.
        globals.set("_parametrar", arguments_table)?;

        // a replacement for print with support for special characters.
        let skriv = lua_ctx.create_function(|_, msg: String| {
            println!("{}", msg);
            Ok(())
        })?;
        globals.set("skriv", skriv)?;

        // a function to handle user input
        let ask = lua_ctx.create_function(|_, msg:String| {
            let mut response = String::new();
            println!("{}", msg);
            let _ = stdout().flush();
            stdin()
                .read_line(&mut response)
                .expect("Did not enter a correct string");
            if let Some('\n') = response.chars().next_back() {
                response.pop();
            }
            if let Some('\r') = response.chars().next_back() {
                response.pop();
            }

            Ok(response)
        })?;
        globals.set("frae1ga", ask)?;

        // open function to open websites and files
        let open_fn = lua_ctx.create_function(|_, s: String| {
            open::that(&s).unwrap();
            Ok(())
        })?;
        globals.set("oeppna", open_fn)?;

        // pairs function translated to swe
        lua_ctx.load(r#"function par(t) return pairs(t) end"#)
            .set_name("par() function")?
            .exec()?;
        
        // ipairs function translated to swe
        lua_ctx.load(r#"function ipar(t) return ipairs(t) end"#)
            .set_name("ipar() function")?
            .exec()?;
        
        // lower function
        lua_ctx.load(r#"function gemener(s) return string.lower(s) end"#)
            .set_name("gemener() function")?
            .exec()?;
        
        // upper function
        lua_ctx.load(r#"function versaler(s) return string.upper(s) end"#)
            .set_name("versaler() function")?
            .exec()?; 

        // random function
        lua_ctx.load(r#"function slumpa(a, b) return math.random(a, b) end"#)
            .set_name("slumpa() function")?
            .exec()?;     
        
        // random dice function
        lua_ctx.load(r#"function tae2rning() return math.random(6) end"#)
            .set_name("tärning() function")?
            .exec()?;
        
        // tostring in swedish
        lua_ctx.load(r#"function tillstrae2ng(n) return tostring(n) end"#)
            .set_name("tilsträng() function")?
            .exec()?;

        // tonumber in swedish 
        lua_ctx.load(r#"function tillnummer(s) return tonumber(s) end"#)
            .set_name("tillnummer() function")?
            .exec()?;
        
        
        // Finally, execute the users code.
        lua_ctx.load(&code).exec()?;

        Ok(())
    })?;

    Ok(())
}

/// Convert åäöÅÄÖ to english letters (except in strings),
/// this allowes variable names with swedish characters to be used
/// This is hacky solution though, and it makes debugging even worse.
fn convert_swe_characters(code: &str) -> String {
    enum LookingFor {
        SingleQuote,
        DoubleQuote,
        Both
    }

    let code_as_string = String::from(code);
    let characters = code_as_string.chars();
    let mut parsed_code: Vec<String> = vec![];
    let mut inside_string = false;
    let mut looking_for: LookingFor = LookingFor::Both;

    for c in characters {
        match (&looking_for, c) {
            // found first double qoute = now inside of string.
            (LookingFor::Both, '\"') => {
                inside_string = true;
                looking_for = LookingFor::DoubleQuote;
            },
            // a second double quote was found = end of string.
            (LookingFor::DoubleQuote, '\"') => {
                inside_string = false;
                looking_for = LookingFor::Both;
            },
            // found first single qoute = now inside of string.
            (LookingFor::Both, '\'') => {
                inside_string = true;
                looking_for = LookingFor::SingleQuote;
            },
            // found second single qoute = end of string.
            (LookingFor::SingleQuote, '\'') => {
                inside_string = false;
                looking_for = LookingFor::Both;
            },
            _ => ()
        }

        match (inside_string, c) {
            (false, 'å')    => parsed_code.push("ae1".to_string()),
            (false, 'ä')    => parsed_code.push("ae2".to_string()),
            (false, 'ö')    => parsed_code.push("oe".to_string()),
            (false ,'Å')    => parsed_code.push("AE1".to_string()),
            (false, 'Ä')    => parsed_code.push("AE2".to_string()),
            (false, 'Ö')    => parsed_code.push("OE".to_string()),
            (_, letter)     => parsed_code.push(letter.to_string())
        }
    }
    parsed_code.join("")
}

/// Convert keywords in swedish to real lua keywords
fn replace_keywords(code: &str) -> String{
    let mut parsed_code = String::from(code);

    let keywords: HashMap<&str, Regex> = [
        ("and",         Regex::new(r"\boch\b").unwrap()),
        ("break",       Regex::new(r"\bbryt\b").unwrap()),
        ("do",          Regex::new(r"\bgör\b").unwrap()),
        ("else",        Regex::new(r"\bannars\b").unwrap()),
        ("elseif",      Regex::new(r"\bannarsom\b").unwrap()),
        ("end",         Regex::new(r"\bslut\b").unwrap()),
        ("false",       Regex::new(r"\bfalskt\b").unwrap()),
        ("for",         Regex::new(r"\bför\b").unwrap()),
        ("function",    Regex::new(r"\bfunktion\b").unwrap()),
        ("if",          Regex::new(r"\bom\b").unwrap()),
        ("in",          Regex::new(r"\bi\b").unwrap()),
        ("local",       Regex::new(r"\blokal\b").unwrap()),
        //("nil",         Regex::new(r"\bingenting\b").unwrap()),
        ("not",         Regex::new(r"\binte\b").unwrap()),
        ("or",          Regex::new(r"\beller\b").unwrap()),
        ("repeat",      Regex::new(r"\bupprepa\b").unwrap()),
        ("return",      Regex::new(r"\bge\b").unwrap()),
        ("then",        Regex::new(r"\butför\b").unwrap()),
        ("true",        Regex::new(r"\bsant\b").unwrap()),
        ("until",       Regex::new(r"\btills\b").unwrap()),
        ("while",       Regex::new(r"\bmedans\b").unwrap()),
    ].iter().cloned().collect();
    
    for (keyword_eng, re) in keywords {
        parsed_code = re.replace_all(&parsed_code, keyword_eng).into();
    }

    parsed_code
}