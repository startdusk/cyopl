use pest::iterators::Pair;
use pest::{self, Parser};

use crate::ast::*;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn build_ast_from_expr(pair: Pair<Rule>) -> Node {
    todo!()
}
