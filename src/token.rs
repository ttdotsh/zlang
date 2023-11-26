use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'src> {
    Int(i32),
    Bool(bool),
    Str(&'src str),
    Ident(&'src str),
    Assign,
    Var,
    Fn,
    ScopeStart,
    ScopeEnd,
    StmtEnd,
    Comma,
    If,
    Else,
}

fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<Token<'src>>> {
    let int = text::int(10).from_str().unwrapped().map(Token::Int);

    let bool = choice((just("cap").to(false), just("nocap").to(true))).map(Token::Bool);

    let str = choice((
        none_of('"').repeated().to_slice().padded_by(just('"')),
        none_of('\'').repeated().to_slice().padded_by(just('\'')),
    ))
    .map(Token::Str);

    let keyword = choice((
        text::unicode::keyword("ngl").to(Token::Var),
        text::unicode::keyword("finna").to(Token::Fn),
        text::unicode::keyword("be").to(Token::Assign),
        text::unicode::keyword("rn").to(Token::StmtEnd),
        text::unicode::keyword("lowkey").to(Token::ScopeStart),
        text::unicode::keyword("bet").to(Token::ScopeEnd),
        text::unicode::keyword("fr").to(Token::If),
        text::unicode::keyword("cope").to(Token::Else),
    ));

    let ident = text::unicode::ident().map(Token::Ident);

    let token = choice((int, bool, str, keyword, ident));

    token.padded().repeated().collect()
}

#[cfg(test)]
mod test {
    use super::{lexer, Token};
    use chumsky::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_int_token() {
        use Token::*;

        let input = r"
            69
            420
        ";

        let expected = vec![Int(69), Int(420)];

        let lexer = lexer();

        let output = lexer.parse(input).into_result().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_bool_token() {
        use Token::*;

        let input = r"
            cap
            nocap
        ";

        let expected = vec![Bool(false), Bool(true)];

        let lexer = lexer();

        let output = lexer.parse(input).into_result().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_str_token() {
        use Token::*;

        let input = r#"
            "hello"
            'hello'
        "#;

        let expected = vec![Str("hello"), Str("hello")];

        let lexer = lexer();

        let output = lexer.parse(input).into_result().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_valid_ident_token() {
        use Token::*;

        let valid_idents = r#"
            foo
            snake_case
            camelCase
            PascalCase
            snake_case2
        "#;

        let expected = vec![
            Ident("foo"),
            Ident("snake_case"),
            Ident("camelCase"),
            Ident("PascalCase"),
            Ident("snake_case2"),
        ];

        let lexer = lexer();

        let output = lexer.parse(valid_idents).into_result().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_invalid_ident_token() {
        // this won't always return an error when some operators are implemented
        // TODO: find a better assertion to make

        let lexer = lexer();
        let invalid_idents = vec![
            "foo!", "foo@", "foo#", "foo$", "foo%", "foo^", "foo&", "foo*", "foo(", "foo)", "foo[",
            "foo]", "foo{", "foo}", "foo|", "foo=", "foo+", "foo-", "foo?", "foo/", "foo~", "foo`",
            "foo<", "foo>", "foo,", "foo.", "foo;", "foo:", "foo\\",
        ];

        invalid_idents
            .into_iter()
            .for_each(|i| assert!(lexer.parse(i).has_errors()));
    }

    #[test]
    fn test_keyword_tokens() {
        use Token::*;

        let input = r"
            ngl
            be
            rn
            finna
            fr
            cope
            lowkey
            bet
        ";

        let expected = vec![Var, Assign, StmtEnd, Fn, If, Else, ScopeStart, ScopeEnd];

        let lexer = lexer();

        let output = lexer.parse(input).into_result().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_syntax() {
        use Token::*;

        let input = r#"
            ngl foo be 5 rn
            ngl bar be "cap" rn
            ngl baz be cap rn
        "#;

        let expected = vec![
            Var,
            Ident("foo"),
            Assign,
            Int(5),
            StmtEnd,
            Var,
            Ident("bar"),
            Assign,
            Str("cap"),
            StmtEnd,
            Var,
            Ident("baz"),
            Assign,
            Bool(false),
            StmtEnd,
        ];

        let lexer = lexer();

        let output = lexer.parse(input).into_result().unwrap();
        assert_eq!(output, expected);
    }
}
