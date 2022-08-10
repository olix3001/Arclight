
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
                    println!("Ignoring error: {}", err.to_short_string());
                }
            }
        )+

        Err(crate::error!(crate::utils::error::ErrorKind::ParserError, "Could not parse any"))
    })()}
}

#[macro_export]
macro_rules! test_token {
    ($token: expr) => {
        crate::lexer::lexer::Token { token_type: $token, line: 1, column: 1 }
    }
}
