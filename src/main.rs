use amzn_smt_ir::{
    logic::{ArithOp, ALL},
    CoreOp, Term,
};
use smallvec::smallvec;

use crate::solver::constant;

mod solver;

fn main() {
    prob1();
    unsat_prob();
}

pub fn prob1() {
    let x = Term::<ALL>::Variable("x".into());
    let y = Term::Variable("y".into());

    let c1 = ArithOp::Gt(smallvec![x.clone(), y.clone()]);
    let c2 = ArithOp::Lt(smallvec![y.clone(), constant(2)]);

    let c3 = CoreOp::Eq(smallvec![
        ArithOp::Plus(smallvec![
            x.clone(),
            ArithOp::Mul(smallvec![constant(2), y.clone()]).into()
        ])
        .into(),
        constant(7)
    ]);

    let s = solver::Solution::try_new(
        &vec![x.clone(), y.clone()],
        &vec![c1.into(), c2.into(), c3.into()],
    )
    .unwrap();

    println!("{:?}", s.get(&x));
    println!("{:?}", s.get(&y));
}

pub fn unsat_prob() {
    let x = Term::<ALL>::Variable("x".into());
    let y = Term::Variable("y".into());

    let c1 = ArithOp::Gt(smallvec![x.clone(), y.clone()]);
    let c2 = ArithOp::Lt(smallvec![x.clone(), y.clone()]);

    let s = solver::Solution::try_new(&vec![x, y], &vec![c1.into(), c2.into()]).unwrap_err();

    println!("{:?}", s);
}
