use regex::Regex;
use rlua::{Lua};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
mod prelude;
use prelude::{get_prelude};

/// Run the transpiled lua code
pub fn run_lua_code(code: &str, arguments: &[String]) -> Result<(), rlua::Error>{
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

        // Load the prelude
        lua_ctx
            .load(get_prelude())
            .set_name("Koda prelude")?
            .exec()?;
        
        // Finally, execute the users code.
        lua_ctx.load(&code).exec()?;

        Ok(())
    })?;

    Ok(())
}


/// Transpile from Koda code to valid Lua code
pub fn transpile(code: &str) -> String {
    let segments = to_segments(&code);
    let code = segments
                .iter()
                .map(|segment| convert_segment(segment))
                .collect::<Vec<String>>()
                .join(" ");
    
    code
}


#[derive(Debug)]
enum Segment {
    Code(String),
    Str(String),
}


/// Convert code to a vector of Segments
fn to_segments(code: &str) -> Vec<Segment>{
    let cleaned_code = without_comments(&code);
    let chars: Vec<char> = cleaned_code.chars().collect();
    let mut segments: Vec<Segment> = vec![];
    
    let mut next_escaped = false;
    let mut searching_for: Option<Divider> = None;
    let mut last = 0;
    
    for (index, c) in chars.iter().enumerate() {
        if next_escaped {
            next_escaped = false;
            continue;
        }
        match (is_divider(&c), &searching_for) {
            // Found a space outside of a string
            // add a code segment
            (Some(Divider::Space), None) => {
                if let Some(segment) = new_code_segment(last, index, &chars) {
                    segments.push(segment);
                }
                last = index + 1;
            },
            // Found first double qoute,
            // add the code segment and look for end of string
            (Some(Divider::DoubleQoute), None) => {
                searching_for = Some(Divider::DoubleQoute);
                if let Some(segment) = new_code_segment(last, index, &chars) {
                    segments.push(segment);
                }
                last = index;
            },
            // Found second double qoute, add the string segment
            (Some(Divider::DoubleQoute), Some(Divider::DoubleQoute)) => {
                searching_for = None;
                if let Some(segment) = new_str_segment(last, index, &chars) {
                    segments.push(segment);
                }
                last = index + 1;
            },
            // Found first single qoute,
            // add the code segment and look for end of string
            (Some(Divider::SingleQoute), None) => {
                searching_for = Some(Divider::SingleQoute);
                if let Some(segment) = new_code_segment(last, index, &chars) {
                    segments.push(segment);
                }
                last = index;
            },
            // Found second single qoute, add the string segment
            (Some(Divider::SingleQoute), Some(Divider::SingleQoute)) => {
                if let Some(segment) = new_str_segment(last, index, &chars) {
                    segments.push(segment);
                }
                last = index + 1;
            },
            _ => {
                // Check for escape characters
                if *c == '\\' {
                   next_escaped = true;
                }
            }
        }
    }
    // Finally, add the end of the file as a code segment:
    if last < chars.len() {
        if let Some(segment) = new_code_segment(last, chars.len(), &chars) {
            segments.push(segment);
        }
    }

    segments
}


/// Returns a copy of the code, but without any commentss
fn without_comments(code: &str) -> String{
    // Note, the Koda spec does not support multi-line comments
    let re = Regex::new(r"--.*").unwrap(); 
    re.replace_all(&code, "").into()
}


#[derive(Debug)]
enum Divider {
    SingleQoute,
    DoubleQoute,
    Space,
}


/// Determine if a char is a divider
fn is_divider(c: &char) -> Option<Divider> {
    match c {
        '\"'    => Some(Divider::DoubleQoute),
        '\''    => Some(Divider::SingleQoute),
        '\t'    => Some(Divider::Space),
        ' '     => Some(Divider::Space),
        '\n'    => Some(Divider::Space),
        _       => None
    }
}


/// Helper function that creates a code segment
fn new_code_segment(last: usize, index: usize, chars: &Vec<char>) -> Option<Segment>{
    if last != index {
        let mut content = chars[last..index]
                            .iter()
                            .cloned()
                            .collect::<String>();
        
        content.retain(|c| !c.is_whitespace());
        
        if content != "" {
            return Some(Segment::Code(content));
        }
    }
   None
}


/// Helper function that creates a str segment
fn new_str_segment(last: usize, index: usize, chars: &Vec<char>) -> Option<Segment>{
    if last != index {
        let content = chars[last..(index + 1)]
                        .iter()
                        .cloned()
                        .collect::<String>();
        return Some(Segment::Str(content));
    }
   None
}


/// Convert a Segment to valid code
fn convert_segment(segment: &Segment) -> String{
    match segment {
        Segment::Str(content) => content.to_owned(),
        Segment::Code(content) => {
            let mut updated_code = replace_keywords(&content);
            updated_code = replace_swe_chars(&updated_code);
            updated_code
        }
    }
}


/// Replace one of Koda's keywords with one of Lua's
fn replace_keywords(code: &str) -> String {
    match code {
        "och"       => String::from("and"),
        "bryt"      => String::from("break"),
        "gör"       => String::from("do"),
        "annars"    => String::from("else"),
        "annarsom"  => String::from("elseif"),
        "slut"      => String::from("end"),
        "falskt"    => String::from("false"),
        "för"       => String::from("for"),
        "funktion"  => String::from("function"),
        "om"        => String::from("if"),
        "i"         => String::from("in"),
        "lokal"     => String::from("local"),
        "inte"      => String::from("not"),
        "eller"     => String::from("or"),
        "upprepa"   => String::from("repeat"),
        "ge"        => String::from("return"),
        "sant"      => String::from("true"),
        "tills"     => String::from("until"),
        "medan"     => String::from("while"),
        "utför"     => String::from("then"),
        non_keyword => String::from(non_keyword)
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


/// Convert from the special strings to normal
/// Swedish letters again
pub fn show_swedish(msg: &str) -> String {
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