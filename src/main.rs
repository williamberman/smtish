use amzn_smt_ir::{
    logic::{ArithOp, BvOp},
    CoreOp, ISort, Term,
};
use num::bigint::ToBigUint;
use smallvec::smallvec;
use solver::Solution;

use crate::solver::{bv_constant, int_constant};

mod solver;

fn main() {
    println!("************");
    prob1();
    println!("************");
    unsat_prob();
    println!("************");
    more_sorts();
    println!("************");
    bv_bigger();
}

pub fn prob1() {
    let x = Term::Variable("x".into());
    let y = Term::Variable("y".into());

    let c1 = ArithOp::Gt(smallvec![x.clone(), y.clone()]);
    let c2 = ArithOp::Lt(smallvec![y.clone(), int_constant(2)]);

    let c3 = CoreOp::Eq(smallvec![
        ArithOp::Plus(smallvec![
            x.clone(),
            ArithOp::Mul(smallvec![int_constant(2), y.clone()]).into()
        ])
        .into(),
        int_constant(7)
    ]);

    let s = solver::Solution::try_new(
        &vec![(x.clone(), ISort::int()), (y.clone(), ISort::int())],
        &vec![c1.into(), c2.into(), c3.into()],
    )
    .unwrap();

    println!("{}", s.get(&x).unwrap());
    println!("{}", s.get(&y).unwrap());
}

pub fn unsat_prob() {
    let x = Term::Variable("x".into());
    let y = Term::Variable("y".into());

    let c1 = ArithOp::Gt(smallvec![x.clone(), y.clone()]);
    let c2 = ArithOp::Lt(smallvec![x.clone(), y.clone()]);

    let s = solver::Solution::try_new(
        &vec![(x, ISort::int()), (y, ISort::int())],
        &vec![c1.into(), c2.into()],
    )
    .unwrap_err();

    println!("{:?}", s);
}

pub fn more_sorts() {
    let x = Term::Variable("x".into());
    let y = Term::Variable("y".into());

    let c1 = BvOp::BvUgt(x.clone(), y.clone());
    let c2 = BvOp::BvUlt(y.clone(), bv_constant(vec![2]));

    let c3 = CoreOp::Eq(smallvec![
        BvOp::BvAdd(smallvec![
            x.clone(),
            BvOp::BvMul(smallvec![bv_constant(vec![2]), y.clone()]).into()
        ])
        .into(),
        bv_constant(vec![7])
    ]);

    let s = solver::Solution::try_new(
        &vec![
            (x.clone(), ISort::bitvec(4_i32.to_biguint().unwrap())),
            (y.clone(), ISort::bitvec(4_i32.to_biguint().unwrap())),
        ],
        &vec![c1.into(), c2.into(), c3.into()],
    )
    .unwrap();

    println!("{}", s.get(&x).unwrap());
    println!("{}", s.get(&y).unwrap());
}

pub fn bv_bigger() {
    let x = Term::Variable("x".into());
    let c1 = CoreOp::Eq(smallvec![x.clone(), bv_constant(vec![1, 255])]);

    let s = Solution::try_new(
        &vec![(x.clone(), ISort::bitvec(12_i32.to_biguint().unwrap()))],
        &vec![c1.into()],
    )
    .unwrap();

    println!("{}", s.get(&x).unwrap())
}
