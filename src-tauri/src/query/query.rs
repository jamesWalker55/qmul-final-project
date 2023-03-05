// TODO: Make this module be able to handle complicated queries like in src/repo.rs:478

use std::collections::VecDeque;
use crate::helpers::sql::{escape_fts5_string, escape_like_pattern};

#[derive(Debug, Eq, PartialEq)]
enum Symbol {
    /// A tag. E.g. `kick`
    Tag(String),
    /// A key-value pair. E.g. `inpath:res/audio/`
    InPath(String),
}

enum QueryConversionError {
    NoAssociatedFTSString,
    NoAssociatedWhereClause,
}

impl Symbol {
    fn to_fts_string(&self) -> Result<String, QueryConversionError> {
        match self {
            Symbol::Tag(name) => Ok(format!(r#"tags:"{}""#, escape_fts5_string(name.as_str()))),
            Symbol::InPath(_) => Err(QueryConversionError::NoAssociatedFTSString),
        }
    }

    fn to_where_clause(&self) -> Result<String, QueryConversionError> {
        match self {
            Symbol::Tag(_) => Err(QueryConversionError::NoAssociatedWhereClause),
            Symbol::InPath(path) => Ok(format!(
                r#"path LIKE '{}' ESCAPE '\'"#,
                escape_like_pattern(path, '\\')
            )),
        }
    }
}

#[derive(Debug)]
enum Expr {
    /// Represents AND between 2 terms: `a & b`
    And(Box<Expr>, Box<Expr>),
    /// Represents OR between 2 terms: `a | b`
    Or(Box<Expr>, Box<Expr>),
    /// Represents a negation of an expression: `~a`
    Not(Box<Expr>),
    /// Represents a single term
    Term(Symbol),
}

// impl Expr {
//   fn to_where_clause(&self) -> String {
//     match self {
//       And(a, b) => {},
//       Or(a, b) => {},
//       Not(a) => {},
//       Term(sym) => {},
//     }
//   }
// }

/// Depth-first search iterator for an expression
struct ExprDFSIterator<'a> {
    remaining_nodes: VecDeque<&'a Expr>,
}

impl<'a> ExprDFSIterator<'a> {
    fn new(expr: &'a Expr) -> Self {
        Self {
            remaining_nodes: VecDeque::from([expr]),
        }
    }
}

impl<'a> Iterator for ExprDFSIterator<'a> {
    type Item = &'a Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let next_node = self.remaining_nodes.pop_front();
        match next_node {
            Some(Expr::And(a, b)) | Some(Expr::Or(a, b)) => {
                self.remaining_nodes.push_back(&**a);
                self.remaining_nodes.push_back(&**b);
            }
            // Some(expr) => {}
            Some(Expr::Not(a)) => {
                self.remaining_nodes.push_back(&**a);
            }
            Some(Expr::Term(_)) => (),
            None => (),
        }
        next_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn and(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::And(a, b))
    }

    fn or(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Or(a, b))
    }

    fn not(a: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Not(a))
    }

    fn tag(name: &str) -> Box<Expr> {
        Box::new(Expr::Term(Symbol::Tag(String::from(name))))
    }

    fn inpath(path: &str) -> Box<Expr> {
        Box::new(Expr::Term(Symbol::InPath(String::from(path))))
    }

    #[test]
    /// The query:
    ///
    ///     a b -e inpath:1 | d e inpath:0
    ///
    fn my_test() {
        let expr = or(
            and(and(tag("a"), tag("b")), and(not(tag("e")), inpath("1"))),
            and(tag("d"), and(tag("e"), inpath("0"))),
        );
        // dbg!(expr);
        for x in ExprDFSIterator::new(&expr) {
            println!("{:?}", x);
        }
    }
}
