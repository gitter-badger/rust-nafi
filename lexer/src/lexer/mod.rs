use tokens::Token;

mod literals;
mod unicode;
mod whitespace;

use lexer::literals::{integer_literal, string_literal};
use lexer::whitespace::whitespace;

/// Vec<Token>
named! {
    pub tokens<&str, Vec<Token>>,
    many0!(token)
}

/// Token
named! {
    token<&str, Token>,
    alt_complete!(
        integer_literal |
        string_literal |
        whitespace |
        _unknown
    )
}

/// Token::_Unknown
named! {
    _unknown<&str, Token>,
    do_parse!(
        take!(1) >>
        (Token::_Unknown)
    )
}
