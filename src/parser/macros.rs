#[macro_export]
macro_rules! parser_error {
    ($msg: expr) => {
        panic!("Unspecified error: {}", $msg)
    };

    ($token: expr, $msg: expr) => {{
            println!("Error: {}", $msg);
            println!("Position: {}:{}", $token.line, $token.column);
            panic!("Program exited with an error.")
        }
    };
}

#[macro_export]
macro_rules! try_parse {
    ( $tokens: expr, $pos: expr, $( $expr: ty ) + ) => {(|| {
        $(
            let temp_pos = $pos;
            let expr = <$expr as Parseable>::parse(&$tokens, &mut $pos);
            match expr {
                Ok(expr) => return Ok(expr),
                Err(err) => {
                    $pos = temp_pos;
                    println!("Ignoring error: {}", err);
                }
            }
        )+

        Err(format!("Could not parse any: {}", stringify!($( $expr )+)))
    })()}
}