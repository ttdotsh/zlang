use super::token::Token;
use logos::{Lexer, Logos};

pub struct TokenStream<'source> {
    tokens: Lexer<'source, Token>,
}

impl<'src> TokenStream<'src> {
    pub fn new(src: &'src str) -> TokenStream<'src> {
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
    use super::{Token::*, TokenStream};

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
            VarDec,
            Ident("foo".into()),
            Assign,
            Int(5),
            StmtEnd,
            VarDec,
            Ident("bar".into()),
            Assign,
            Str("hello".into()),
            StmtEnd,
            FnDef,
            Ident("add".into()),
            Ident("x".into()),
            Comma,
            Ident("y".into()),
            Assign,
            ScopeStart,
            Ident("x".into()),
            Plus,
            Ident("y".into()),
            ScopeEnd,
            VarDec,
            Ident("baz".into()),
            Assign,
            Ident("add".into()),
            Ident("foo".into()),
            Comma,
            Int(5),
            StmtEnd,
            If,
            Ident("baz".into()),
            GreaterThan,
            Or,
            Equal,
            Int(10),
            ScopeStart,
            Ident("go_off".into()),
            Ident("bar".into()),
            StmtEnd,
            Else,
            Ident("go_off".into()),
            Str("sheesh".into()),
            StmtEnd,
            ScopeEnd,
        ];

        let stream = TokenStream::new(input);

        stream
            .zip(expected)
            .for_each(|(token, expected)| assert_eq!(token, expected));
    }
}
