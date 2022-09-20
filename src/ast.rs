#[derive(Debug)]
pub struct Ident<'input>(pub &'input str);

#[derive(Debug)]
pub enum Expression<'input> {
    Number(i64),
    Ident(Ident<'input>),
    Add(Box<Expression<'input>>, Box<Expression<'input>>),
    Multiply(Box<Expression<'input>>, Box<Expression<'input>>),
    LessThan(Box<Expression<'input>>, Box<Expression<'input>>),
    And(Box<Expression<'input>>, Box<Expression<'input>>),
    Let {
        name: Ident<'input>,
        value: Box<Expression<'input>>,
        in_expr: Box<Expression<'input>>,
    },
    If {
        condition: Box<Expression<'input>>,
        then_expr: Box<Expression<'input>>,
        else_expr: Box<Expression<'input>>,
    },
}
