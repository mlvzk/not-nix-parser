use std::{borrow::Cow, collections::HashMap};

use crate::ast;

pub(crate) struct Interpreter<'input> {
    variables: HashMap<&'input str, i64>,
}

impl<'input> Interpreter<'input> {
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub(crate) fn interpret(&mut self, expr: &'input ast::Expression) -> i64 {
        match expr {
            ast::Expression::Number(n) => *n,
            ast::Expression::Ident(ident) => self.variables[ident.0],
            ast::Expression::Add(a, b) => self.interpret(a) + self.interpret(b),
            ast::Expression::Multiply(a, b) => self.interpret(a) * self.interpret(b),
            ast::Expression::Let {
                name,
                value,
                in_expr,
            } => {
                let value = self.interpret(value);
                self.variables.insert(name.0, value);
                self.interpret(in_expr)
            }
            ast::Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                if self.interpret(condition) != 0 {
                    self.interpret(then_expr)
                } else {
                    self.interpret(else_expr)
                }
            }
            ast::Expression::LessThan(_, _) => todo!(),
            ast::Expression::And(_, _) => todo!(),
        }
    }
}
