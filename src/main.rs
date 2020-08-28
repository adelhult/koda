use dont_disappear::enter_to_continue;
use regex::Regex;
use rlua::{Error, Lua};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    start_with_args(args);
    enter_to_continue::custom_msg("Tryck på ENTER-tangenten för att avsluta programmet.");
}

/// Take cli args, check if a valid code file exists,
/// then execute code and handle potential errors.
fn start_with_args(args: Vec<String>) {
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

    let raw_code = fs::read_to_string(&filename).expect("Failed to read the given file.");

    // parse raw_code. First, replace keywords and then convert swe characters.
    let code = convert_swe_characters(&replace_keywords(&raw_code));

    // run the code and handle potential errors
    match run_code(code, &args) {
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
                             //FIXME needs to call show_swedish but has wrong type
                             e
                    );
                }
            }
        }
    }
}

fn run_code(code: String, arguments: &[String]) -> Result<(), Error> {
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
        let ask = lua_ctx.create_function(|_, msg: String| {
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
        globals.set("fr__ao__ga", ask)?;

        // open function to open websites and files
        let open_fn = lua_ctx.create_function(|_, s: String| {
            open::that(&s).unwrap();
            Ok(())
        })?;
        globals.set("__oe__ppna", open_fn)?;

        // pairs function translated to swe
        lua_ctx
            .load(r#"function par(t) return pairs(t) end"#)
            .set_name("par() function")?
            .exec()?;

        // ipairs function translated to swe
        lua_ctx
            .load(r#"function ipar(t) return ipairs(t) end"#)
            .set_name("ipar() function")?
            .exec()?;

        // lower function
        lua_ctx
            .load(r#"function gemener(s) return string.lower(s) end"#)
            .set_name("gemener() function")?
            .exec()?;

        // upper function
        lua_ctx
            .load(r#"function versaler(s) return string.upper(s) end"#)
            .set_name("versaler() function")?
            .exec()?;

        // random function
        lua_ctx
            .load(r#"function slumpa(a, b) return math.random(a, b) end"#)
            .set_name("slumpa() function")?
            .exec()?;

        // random dice function
        lua_ctx
            .load(r#"function t__ae__rning() return math.random(6) end"#)
            .set_name("tärning() function")?
            .exec()?;

        // tostring in swedish
        lua_ctx
            .load(r#"function tillstr__ae__ng(n) return tostring(n) end"#)
            .set_name("tilsträng() function")?
            .exec()?;

        // tonumber in swedish
        lua_ctx
            .load(r#"function tillnummer(s) return tonumber(s) end"#)
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
        Both,
    }

    let code_as_string = String::from(code);
    let characters = code_as_string.chars();
    let mut parsed_code: Vec<String> = vec![];
    let mut inside_string = false;
    let mut looking_for: LookingFor = LookingFor::Both;
    let mut next_escaped : bool = false;

    for c in characters {
        if !next_escaped {
            match (&looking_for, c) {
                // found first double qoute = now inside of string.
                (LookingFor::Both, '\"') => {
                    inside_string = true;
                    looking_for = LookingFor::DoubleQuote;
                }
                // a second double quote was found = end of string.
                (LookingFor::DoubleQuote, '\"') => {
                    inside_string = false;
                    looking_for = LookingFor::Both;
                }
                // found first single qoute = now inside of string.
                (LookingFor::Both, '\'') => {
                    inside_string = true;
                    looking_for = LookingFor::SingleQuote;
                }
                // found second single qoute = end of string.
                (LookingFor::SingleQuote, '\'') => {
                    inside_string = false;
                    looking_for = LookingFor::Both;
                }
                (_, '\\') => {
                    next_escaped = true;
                }
                _ => (),
            }
        } else {
            next_escaped = false;
        }

        match (inside_string, c) {
            // names like this are not convension, and such
            // are unlikely to collide with user definitions
            (false, 'å') => parsed_code.push("__ao__".to_string()),
            (false, 'ä') => parsed_code.push("__ae__".to_string()),
            (false, 'ö') => parsed_code.push("__oe__".to_string()),
            (false, 'Å') => parsed_code.push("__AO__".to_string()),
            (false, 'Ä') => parsed_code.push("__AE__".to_string()),
            (false, 'Ö') => parsed_code.push("__OE__".to_string()),
            (_,  letter) => parsed_code.push(letter.to_string()),
        }
    }
    parsed_code.join("")
}

/// Convert compiler defined åäö replacements
/// for showing in error messages
fn show_swedish(msg: &str) -> String {
    let mut replaced_msg = String::from(msg);

    let idents : HashMap<&str, Regex> = [
        ("å", Regex::new(r"__ao__").unwrap()),
        ("ä", Regex::new(r"__ae__").unwrap()),
        ("ö", Regex::new(r"__oe__").unwrap()),
        ("Å", Regex::new(r"__AO__").unwrap()),
        ("Ä", Regex::new(r"__AE__").unwrap()),
        ("Ö", Regex::new(r"__OE__").unwrap()),
    ].iter().cloned().collect();

    for (ident_sve, re) in idents {
        replaced_msg = re.replace_all(&replaced_msg, ident_sve).into()
    }

    replaced_msg
}

/// Convert keywords in swedish to real lua keywords
fn replace_keywords(code: &str) -> String {
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
        //("nil",       Regex::new(r"\bingenting\b").unwrap()),
        ("not",         Regex::new(r"\binte\b").unwrap()),
        ("or",          Regex::new(r"\beller\b").unwrap()),
        ("repeat",      Regex::new(r"\bupprepa\b").unwrap()),
        ("return",      Regex::new(r"\bge\b").unwrap()),
        ("then",        Regex::new(r"\butför\b").unwrap()),
        ("true",        Regex::new(r"\bsant\b").unwrap()),
        ("until",       Regex::new(r"\btills\b").unwrap()),
        ("while",       Regex::new(r"\bmedan\b").unwrap()),
    ]
    .iter()
    .cloned()
    .collect();

    for (keyword_eng, re) in keywords {
        parsed_code = re.replace_all(&parsed_code, keyword_eng).into();
    }

    parsed_code
}
