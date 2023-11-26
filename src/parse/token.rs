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

    #[regex(r#""([^"]*)""#, as_string_value)]
    #[regex(r#"'([^']*)'"#, as_string_value)]
    Str(String),

    #[regex("[a-zA-Z_]+", as_identifier)]
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
    LessThan,

    #[token("flex on")]
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
    VarDec,

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

fn as_string_value(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let string_value = &slice[1..slice.len() - 1];
    Some(string_value.to_owned())
}

fn as_identifier(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_owned())
}

#[cfg(test)]
mod test {
    use super::Token;
    use logos::Logos;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_idents() {
        let input = r"
            snake_case
            camelCase
            PascalCase
        ";

        let expected = vec![
            Token::Ident("snake_case".into()),
            Token::Ident("camelCase".into()),
            Token::Ident("PascalCase".into()),
        ];

        let lex = Token::lexer(input);

        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }

    #[test]
    fn test_strings() {
        let input = r#"
            "fr" 'cap' "'lowkey'" '"flex on"'
        "#;

        let expected = vec![
            Token::Str("fr".into()),
            Token::Str("cap".into()),
            Token::Str("'lowkey'".into()),
            Token::Str(r#""flex on""#.into()),
        ];

        let lex = Token::lexer(input);

        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }

    #[test]
    fn test_ints() {
        let input = r"
            5
            10
            2345987
        ";

        let expected = vec![Token::Int(5), Token::Int(10), Token::Int(2345987)];

        let lex = Token::lexer(input);

        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }

    #[test]
    fn test_bools() {
        let input = r"
            cap
            nocap
        ";

        let expected = vec![Token::Bool(false), Token::Bool(true)];

        let lex = Token::lexer(input);

        lex.zip(expected)
            .map(|(t, ex)| (t.unwrap(), ex))
            .for_each(|(token, expected_token)| assert_eq!(token, expected_token));
    }
}
