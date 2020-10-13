// REPL based on the rlua repl example code
use rlua::{Error, Lua, MultiValue};
use rustyline::Editor;
use koda::{get_lua_state, transpile_code};

pub fn repl() {
    lua = get_lua_state(None);
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
                let code = transpile_code(&line);

                match ctx.load(&code).eval::<MultiValue>() {
                    Ok(values) => {
                        editor.add_history_entry(line);
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|value| format!("{:?}", value))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                        break;
                    }
                    Err(Error::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        // continue reading input and append it to `line`
                        line.push_str("\n"); // separate input lines
                        prompt = ">> ";
                    }
                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        }
    });
}