use plonky2::{
    field::{
        goldilocks_field::GoldilocksField,
        types::PrimeField64,
        types::{Field, PrimeField},
    },
    hash::hash_types::RichField,
    iop::{
        target::{BoolTarget, Target},
        witness::Witness,
    },
    plonk::{
        circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
        config::PoseidonGoldilocksConfig,
    },
};
use std::collections::HashMap;

use crate::ast;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

#[derive(Debug)]
struct ComparisonGenerator {
    x: Target,
    y: Target,
    lt: BoolTarget,
    gt: BoolTarget,
}

impl<F: RichField> plonky2::iop::generator::SimpleGenerator<F> for ComparisonGenerator {
    fn dependencies(&self) -> Vec<Target> {
        vec![self.x, self.y]
    }

    fn run_once(
        &self,
        witness: &plonky2::iop::witness::PartitionWitness<F>,
        out_buffer: &mut plonky2::iop::generator::GeneratedValues<F>,
    ) {
        let x = witness.get_target(self.x);
        let y = witness.get_target(self.y);

        out_buffer.set_bool_target(self.lt, x.to_canonical_u64() < y.to_canonical_u64());
        out_buffer.set_bool_target(self.gt, x.to_canonical_u64() > y.to_canonical_u64());
    }
}

pub(crate) struct Builder<'input> {
    builder: CircuitBuilder<F, 2>,
    targets: HashMap<&'input str, Target>,
}

impl<'input> Builder<'input> {
    pub(crate) fn new() -> Self {
        Self {
            builder: CircuitBuilder::new(CircuitConfig::standard_recursion_config()),
            targets: HashMap::new(),
        }
    }

    pub fn lt(&mut self, x: Target, y: Target) -> BoolTarget {
        let lt = self.builder.add_virtual_bool_target();
        let gt = self.builder.add_virtual_bool_target();

        self.builder
            .add_simple_generator(ComparisonGenerator { x, y, lt, gt });

        lt
    }

    pub(crate) fn build(&mut self, expr: &'input ast::Expression) -> Target {
        match expr {
            ast::Expression::Number(n) => self.builder.constant(GoldilocksField(*n as u64)),
            ast::Expression::Ident(ident) => self.targets[ident.0],
            ast::Expression::Add(a, b) => {
                let a = self.build(a);
                let b = self.build(b);
                self.builder.add(a, b)
            }
            ast::Expression::Multiply(a, b) => {
                let a = self.build(a);
                let b = self.build(b);
                self.builder.mul(a, b)
            }
            ast::Expression::Let {
                name,
                value,
                in_expr,
            } => {
                let value = self.build(value);
                self.targets.insert(name.0, value);
                self.build(in_expr)
            }
            ast::Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.build(condition);
                let then_expr = self.build(then_expr);
                let else_expr = self.build(else_expr);

                let condition = BoolTarget::new_unsafe(condition);

                self.builder._if(condition, then_expr, else_expr)
            }
            ast::Expression::LessThan(a, b) => {
                let a = self.build(a);
                let b = self.build(b);

                self.lt(a, b).target
            }
            ast::Expression::And(a, b) => {
                let a = self.build(a);
                let b = self.build(b);

                let a = BoolTarget::new_unsafe(a);
                let b = BoolTarget::new_unsafe(b);

                self.builder.and(a, b).target
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use plonky2::iop::witness::PartialWitness;

    use super::*;
    use crate::nix;

    #[test]
    fn test() {
        let expr = nix::ExpressionParser::new()
            .parse(
                r#"
            let a = 1 in
            let b = 2 in
            if a < b && a then
                1
            else 0
        "#,
            )
            .unwrap();

        let mut builder = Builder::new();
        let target = builder.build(&expr);
        let one = builder.builder.constant(GoldilocksField(1));
        builder.builder.connect(target, one);

        let data = builder.builder.build::<C>();
        let pw = PartialWitness::<F>::new();
        let proof = data.prove(pw).unwrap();
        match data.verify(proof) {
            Ok(()) => println!("Verified"),
            Err(x) => {
                dbg!(x);
            }
        }
    }
}
