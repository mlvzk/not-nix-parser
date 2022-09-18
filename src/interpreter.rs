use std::{borrow::Cow, collections::HashMap};

use crate::ast;

#[derive(Debug, Clone)]
enum Variables<'a> {
    None,
    Some(HashMap<&'a str, (Cow<'a, Variables<'a>>, &'a ast::Expression<'a>)>),
}

pub(crate) struct Interpreter<'input> {
    variables: Vec<(&'input str, &'input ast::Expression<'input>)>,
}

impl<'input> Interpreter<'input> {
    pub(crate) fn new() -> Self {
        Self {
            variables: Variables::Some(HashMap::new()),
        }
    }

    pub(crate) fn interpret(&mut self, expr: &'input ast::Expression) -> f64 {
        match expr {
            ast::Expression::Number(n) => *n,
            ast::Expression::Ident(ident) => self.interpret(self.variables[ident.0].1),
            ast::Expression::Add(a, b) => self.interpret(a) + self.interpret(b),
            ast::Expression::Multiply(a, b) => self.interpret(a) * self.interpret(b),
            ast::Expression::Let {
                name,
                value,
                in_expr,
            } => {
                self.variables
                    .insert(name.0, (self.variables.clone(), value));
                self.interpret(in_expr)
            }
            ast::Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                if self.interpret(condition) != 0.0 {
                    self.interpret(then_expr)
                } else {
                    self.interpret(else_expr)
                }
            }
        }
    }
}
