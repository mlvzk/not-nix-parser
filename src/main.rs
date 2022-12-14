mod ast;
mod builder;
mod interpreter;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub nix);

fn main() {
    let expr = nix::ExpressionParser::new()
        .parse(
            r#"
            let a = 1 in
            let b = 2 in
            if a then
                a + a * b
            else 0
        "#,
        )
        .unwrap();

    let mut interpreter = interpreter::Interpreter::new();

    dbg!(&expr);
    dbg!(interpreter.interpret(&expr));
}
