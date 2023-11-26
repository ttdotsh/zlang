#![allow(dead_code)]

pub mod ast;
pub mod stream;
mod token;

use std::iter::Peekable;

use ast::Node;
use stream::TokenStream;
use token::Token;

use self::ast::{Expr, Stmt};

struct Parser<'p> {
    tokens: Peekable<TokenStream<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new<'src: 'p>(source_code: &'src str) -> Parser<'p> {
        let tokens = TokenStream::new(source_code).peekable();
        Parser { tokens }
    }
}

impl Parser<'_> {
    fn next_node(&mut self) -> Option<Node> {
        use Token::*;

        let inner = match self.tokens.peek() {
            Some(VarDec) => self.parse_ngl_stmt(),
            Some(token) => todo!("found token {:?} in next_node", token),
            None => todo!("none in next_node"),
        };

        match inner {
            Ok(into_node) => Some(into_node.into()),
            Err(_) => None,
        }
    }

    fn parse_ngl_stmt(&mut self) -> Result<impl Into<Node>, &'static str> {
        use Token::*;

        self.tokens.next();
        let ident = match self.tokens.next() {
            Some(Ident(str)) => Ok(str),
            _ => Err("not an identifier"),
        }?;

        match self.tokens.peek() {
            Some(Assign) => self.tokens.next(),
            _ => return Err("missing assignment operator"),
        };

        let val = self.parse_expr()?;

        if let Some(StmtEnd) = self.tokens.peek() {
            self.tokens.next();
        }

        Ok(Stmt::Ngl(ident, val))
    }

    fn parse_expr(&mut self) -> Result<Expr, &'static str> {
        use Token::*;

        match self.tokens.next() {
            Some(Bool(b)) => Ok(Expr::Bool(b)),
            Some(Int(i)) => Ok(Expr::Int(i)),
            Some(Str(s)) => Ok(Expr::Str(s)),
            None => Err("no tokens left to parse expr from"),
            _ => todo!("in parse expr"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ast::{Expr::*, Node::*, Stmt::*};
    use super::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_ngl_stmt() {
        let input = r#"
            ngl foo be 5 rn
            ngl bar be "hello" rn
            ngl baz be cap rn
        "#;

        let mut parser = Parser::new(input);

        assert_eq!(parser.next_node().unwrap(), Stmt(Ngl("foo".into(), Int(5))));
        assert_eq!(
            parser.next_node().unwrap(),
            Stmt(Ngl("bar".into(), Str("hello".into())))
        );
        assert_eq!(
            parser.next_node().unwrap(),
            Stmt(Ngl("baz".into(), Bool(false)))
        );
    }
}
