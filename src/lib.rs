use regex::Regex;
use rlua::{Lua, Value, Error};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

mod prelude;
mod lexer;
use prelude::{get_prelude};
use lexer::{Token, lex};

/// Transpile from Koda code to valid Lua code
pub fn transpile(code: &str) -> String {
    lex(&code)
        .iter()
        .map(|token| convert_token(token))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn get_lua_state(env: Option<Vec<String>>)-> Result<Lua, Error>{
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        // Add env constants to the global scope
        if let Some(arguments) = env {
            // add "_FILNAMN" to globals
            globals.set("_FILNAMN", arguments[0].clone())?;

            // add env arguments to argument_table,
            // skipping the first one (filename).
            let arguments_table = lua_ctx.create_table()?;
            for (i, arg) in arguments.iter().skip(1).enumerate() {
                arguments_table.set(i + 1, arg.clone())?;
            }

            // add the arguments_table to the global scope of the lua_ctx.
            // Note: the name should perhaps be changed.
            globals.set("_PARAMETRAR", arguments_table)?;
        }
        
        // Add the current Koda package version to the global scope
        if let Some(version) = option_env!("CARGO_PKG_VERSION") {
            globals.set("_KODA_VERSION", format!("Koda {}",version))?;
        }

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

        // function that prints what will be lexed
        let tokens = lua_ctx.create_function(|_, s: String| {
            for token in lex(&s) {
                println!("{:?}", token);
            }
            Ok(())
        })?;
        globals.set("_TOKENS", tokens)?;

        // Load the prelude
        lua_ctx
            .load(get_prelude())
            .set_name("Koda prelude")?
            .exec()?;
        Ok(())
    })?;
    Ok (lua)
}

/// Run the transpiled lua code
pub fn run_lua_code(code: &str, arguments: &[String]) -> Result<(), Error>{
    let lua = get_lua_state(Some(arguments.to_owned()))?;
    lua.context(|lua_ctx| {
        lua_ctx.load(&code).exec()?;
        Ok(())
    })?;
    Ok(())
}

/// Convert Token to Lua code (stored in a String).
fn convert_token(token: &Token) -> String{
    match token {
        Token::And                  => "and".to_string(),
        Token::Break                => "break".to_string(),
        Token::Do                   => "do".to_string(),
        Token::Else                 => "else".to_string(),
        Token::Elseif               => "elseif".to_string(),
        Token::End                  => "end".to_string(),
        Token::False                => "false".to_string(),
        Token::For                  => "for".to_string(),
        Token::Function             => "function".to_string(),
        Token::If                   => "if".to_string(),
        Token::In                   => "in".to_string(),
        Token::Local                => "local".to_string(),
        Token::Not                  => "not".to_string(),
        Token::Or                   => "or".to_string(),
        Token::Repeat               => "repeat".to_string(),
        Token::Return               => "return".to_string(),
        Token::True                 => "true".to_string(),
        Token::Until                => "until".to_string(),
        Token::While                => "while".to_string(),
        Token::Then                 => "then".to_string(),
        Token::Nil                  => "nil".to_string(),
        Token::VarArgs              => "...".to_string(),
        Token::LeftParenthesis      => "(".to_string(),
        Token::RightParenthesis     => ")".to_string(),
        Token::LeftCurly            => "{".to_string(),
        Token::RightCurly           => "}".to_string(),
        Token::LeftBracket          => "[".to_string(),
        Token::RightBracket         => "]".to_string(),
        Token::Period               => ".".to_string(),
        Token::Comma                => ",".to_string(),
        Token::Colon                => ":".to_string(),
        Token::SemiColon            => ";".to_string(),
        Token::AssignmentOperator   => "=".to_string(),
        Token::Concat               => "..".to_string(),
        Token::LengthOperator       => "#".to_string(),
        Token::Equal                => "==".to_string(),
        Token::NotEqual             =>"~=".to_string(),
        Token::GreaterThan          => ">".to_string(),
        Token::LessThan             => "<".to_string(),
        Token::GreaterOrEqual       => ">=".to_string(),
        Token::LessOrEqual          => "<=".to_string(),
        Token::Multiply             => "*".to_string(),
        Token::Divide               => "/".to_string(),
        Token::Modulus              => "%".to_string(),
        Token::Add                  => "+".to_string(),
        Token::Subtract             => "-".to_string(),
        Token::Exponent             => "^".to_string(),
        Token::Str(value)           => value.clone(),
        Token::Ident(value)         => escape_keywords(&replace_swe_chars(&value)),
        Token::Number(value)        => value.clone(),
        _                           => String::from("")
    }
}

/// Convert the swedish characters "åäöÅÄÖ" 
/// to a representation that only uses English characters
fn replace_swe_chars(code: &str) -> String {
    let mut replaced_code = String::from(code);
    let idents : HashMap<&str, Regex> = [
        ("__ao__", Regex::new(r"å").unwrap()),
        ("__ae__", Regex::new(r"ä").unwrap()),
        ("__oe__", Regex::new(r"ö").unwrap()),
        ("__AO__", Regex::new(r"Å").unwrap()),
        ("__AE__", Regex::new(r"Ä").unwrap()),
        ("__OE__", Regex::new(r"Ö").unwrap()),
    ].iter().cloned().collect();

    for (eng_translation, re) in idents {
        replaced_code = re.replace_all(&replaced_code, eng_translation).into();
    }

    replaced_code
}

/// Escape reserved lua keywords
fn escape_keywords(token: &str) -> String {
    let keywords = ["and", "break", "do", "else",
                    "elseif", "end", "false", "for",
                    "function", "if", "in", "local", "nil",
                    "not", "or", "repeat", "return", "then",
                    "true", "until", "while"];

    if keywords.contains(&token) {
       String::from(format!("__escaped_lua_keyword__{}", token)) 
    } else {
        String::from(token)
    }
}

/// Convert from the escaped strings
/// back to what the user actually wrote.
pub fn show_escaped_idents(msg: &str) -> String {
    let mut replaced_msg = String::from(msg);

    let idents : HashMap<&str, Regex> = [
        ("å", Regex::new(r"__ao__").unwrap()),
        ("ä", Regex::new(r"__ae__").unwrap()),
        ("ö", Regex::new(r"__oe__").unwrap()),
        ("Å", Regex::new(r"__AO__").unwrap()),
        ("Ä", Regex::new(r"__AE__").unwrap()),
        ("Ö", Regex::new(r"__OE__").unwrap()),
        ("", Regex::new(r"__escaped_lua_keyword__").unwrap()),
    ].iter().cloned().collect();

    for (ident_sve, re) in idents {
        replaced_msg = re.replace_all(&replaced_msg, ident_sve).into()
    }

    replaced_msg
}

/// represent a rlua::Value in Swedish
/// This function could be improved a lot
pub fn show_swedish_values(value: &Value) -> String{
    match value {
        Value::Nil              => String::from("ingenting"),
        Value::Boolean(false)   => String::from("falskt"),
        Value::Boolean(true)    => String::from("sant"),
        Value::LightUserData(_) => String::from("pekare (Light user data)"),
        Value::Integer(x)       => x.to_string(),
        Value::Number(x)        => x.to_string(),
        Value::String(lua_str)  => format!("\"{}\"", lua_str.to_str().unwrap()),
        Value::Table(_)         => String::from("tabell"),
        Value::Function(_)      => String::from("funktion"),
        Value::Thread(_)        => String::from("tråd"),
        Value::UserData(_)      => String::from("userData"),
        Value::Error(error)     => show_escaped_idents(&error.to_string()),
    }
}

/// A string representation for rlua::Error
pub fn error_repr(e: Error) -> String{
    match e {
        Error::SyntaxError { message, .. } => {
            format!("Fel: Det är ett syntax-fel som har uppstått.\n\
            De brukar bero på att man stavat fel på en variabel eller glömt något tecken.\n\
            Här är ett meddelande på engelska som berättar om felet: \n{}", 
            show_escaped_idents(&message))

        },
        Error::RuntimeError(message) => {
            format!("Fel: Det är ett runtime-fel som har uppstått.\n\
            Här är ett meddelande på engelska som berättar om felet: \n{}",
            show_escaped_idents(&message))
        },
        e => {
            format!("Fel: Här är en text på engelska där felet förklaras: \n{}", 
            show_escaped_idents(&e.to_string())
            )
        }
    }
}