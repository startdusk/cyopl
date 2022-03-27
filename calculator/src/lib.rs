pub mod compiler;

pub mod ast;
pub mod parser;

pub use crate::ast::{Node, Operator};

pub type Result<T> = anyhow::Result<T>;

// ANCHOR: compile_trait
pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compiling the source: {}", source);
        let ast: Vec<Node> = parser::parse(source).unwrap();
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}
// ANCHOR_END: compile_trait
