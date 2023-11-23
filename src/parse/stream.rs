use super::token::Token;
use logos::{Lexer, Logos};

pub struct TokenStream<'source> {
    tokens: Lexer<'source, Token>,
}

impl<'source> TokenStream<'source> {
    pub fn new(src: &'source str) -> TokenStream<'source> {
        let lex = Token::lexer(src);
        TokenStream { tokens: lex }
    }
}

impl Iterator for TokenStream<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()?.ok()
    }
}

#[cfg(test)]
mod test {
    use super::{Token, TokenStream};

    #[test]
    fn test_stream() {
        let input = r#"
            ngl foo be 5 rn
            ngl bar be "hello" rn
            
            finna add x, y be lowkey
                x + y
            bet

            ngl baz be add foo, 5 rn

            fr baz flex on or do be 10 lowkey
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

        let stream = TokenStream::new(input);

        stream
            .zip(expected)
            .for_each(|(token, expected)| assert_eq!(token, expected));
    }
}
