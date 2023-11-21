use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(skip "[ \t\r\n]+")]
pub enum Token {
    /* Type Literals and Identifiers */
    #[regex("[0-9]+", as_int)]
    Int(i32),

    #[token("cap", |_| false)]
    #[token("nocap", |_| true)]
    Bool(bool),

    #[regex(r#""([^"]*)""#, as_string)]
    #[regex(r#"'([^']*)'"#, as_string)]
    Str(String),

    #[regex("[a-zA-Z_]+", as_ident)]
    Ident(String),

    /* Operators */
    #[token("be")]
    Assign,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("felloff")]
    #[token("<")]
    LessThan,

    #[token("flexin")]
    #[token(">")]
    GreaterThan,

    #[token("do be")]
    Equal,

    #[token("do not be")]
    NotEqual,

    #[token("not")]
    Not,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    /* Delimiters */
    #[token(",")]
    Comma,

    #[token("rn")]
    StmtEnd,

    #[token("lowkey")]
    ScopeStart,

    #[token("bet")]
    ScopeEnd,

    /* Keywords */
    #[token("ngl")]
    VarDef,

    #[token("finna")]
    FnDef,

    /* Control Flow */
    #[token("fr")]
    If,

    #[token("cope")]
    Else,
}

fn as_int(lex: &mut Lexer<Token>) -> Option<i32> {
    let slice = lex.slice();
    let int: i32 = slice[..].parse().ok()?;
    Some(int)
}

fn as_string(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let string_value = &slice[1..slice.len() - 1];
    Some(string_value.to_owned())
}

fn as_ident(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_owned())
}

#[cfg(test)]
mod test {
    use super::Token;
    use logos::Logos;

    #[test]
    fn test_syntax() {
        let input = r#"
            ngl foo be 5 rn
            ngl bar be "hello" rn
            
            finna add x, y be lowkey
                x + y
            bet

            ngl baz be add foo, 5 rn

            fr baz flexin or do be 10 lowkey
                go_off bar rn
            cope
                go_off "sheesh" rn
            bet
        "#;

        let expected = vec![
            Token::VarDef,
            Token::Ident("foo".into()),
            Token::Assign,
            Token::Int(5),
            Token::StmtEnd,
            Token::VarDef,
            Token::Ident("bar".into()),
            Token::Assign,
            Token::Str("hello".into()),
            Token::StmtEnd,
            Token::FnDef,
            Token::Ident("add".into()),
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Assign,
            Token::ScopeStart,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::ScopeEnd,
            Token::VarDef,
            Token::Ident("baz".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Ident("foo".into()),
            Token::Comma,
            Token::Int(5),
            Token::StmtEnd,
            Token::If,
            Token::Ident("baz".into()),
            Token::GreaterThan,
            Token::Or,
            Token::Equal,
            Token::Int(10),
            Token::ScopeStart,
            Token::Ident("go_off".into()),
            Token::Ident("bar".into()),
            Token::StmtEnd,
            Token::Else,
            Token::Ident("go_off".into()),
            Token::Str("sheesh".into()),
            Token::StmtEnd,
            Token::ScopeEnd,
        ];

        let lex = Token::lexer(input);

        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }

    #[test]
    fn test_strings() {
        let input = r#"
            "hello" 'hello' "'hello'" '"hello"'
        "#;

        let expected = vec![
            Token::Str("hello".into()),
            Token::Str("hello".into()),
            Token::Str("'hello'".into()),
            Token::Str(r#""hello""#.into()),
        ];

        let lex = Token::lexer(input);

        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }

    #[test]
    fn test_bools() {
        let input = "cap nocap";

        let expected = vec![Token::Bool(false), Token::Bool(true)];

        let lex = Token::lexer(input);
        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }
}
