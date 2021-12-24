use amzn_smt_ir::{logic::ALL, Command, Constant, IConst, ISort, ISymbol, Script, Term};
use num::{self, bigint::ToBigUint};

pub fn constant<T: ToBigUint>(x: T) -> Term<ALL> {
    return Term::Constant(IConst::from(Constant::Numeral(x.to_biguint().unwrap())));
}

// TODO make generic
fn script(variables: &[Term<ALL>], assertions: &[Term<ALL>]) -> Script<Term<ALL>> {
    let mut s = Script::new();

    // Variable declarations
    s.extend(variables.into_iter().map(|t| match t {
        Term::Variable(x) => Command::<Term>::DeclareConst {
            symbol: x.to_string().into(),
            sort: ISort::int(),
        },
        _ => todo!(),
    }));

    // Assertions
    s.add_asserts(assertions.into_iter().map(|a| a.clone()));

    // Check satisfiable and get solution
    s.extend(vec![
        Command::CheckSat,
        Command::GetValue {
            terms: variables.to_vec(),
        },
        Command::Exit,
    ]);

    return s;
}

#[derive(Debug)]
pub struct Solution {
    bindings: Vec<(ISymbol, Term)>,
}

#[derive(Debug, Clone)]
pub struct UnsatError;

impl TryFrom<Script<Term<ALL>>> for Solution {
    type Error = UnsatError;

    fn try_from(s: Script<Term<ALL>>) -> Result<Self, Self::Error> {
        return Ok(Solution {
            bindings: solve(s)?,
        });
    }
}

impl Solution {
    pub fn try_new(variables: &[Term<ALL>], assertions: &[Term<ALL>]) -> Result<Self, UnsatError> {
        let s = script(variables, assertions);
        return Self::try_from(s);
    }

    pub fn get(&self, t: &Term) -> Option<&Term> {
        match t {
            Term::Variable(x) => {
                let sym: ISymbol = x.to_string().into();

                // TODO -- why does it say b is a double reference
                return self.bindings.iter().find(|b| b.0 == sym).map(|it| &it.1);
            }

            _ => {
                panic!("{} {:?}", "Can only look up variables in solutions", "t")
            }
        }
    }
}

fn solve(s: Script<Term<ALL>>) -> Result<Vec<(ISymbol, Term)>, UnsatError> {
    let filename = "/tmp/out.smtlib";

    std::fs::write(filename, s.to_string()).unwrap();

    let out = std::process::Command::new("z3")
        .arg(filename)
        .output()
        .unwrap();

    let xout = String::from_utf8(out.stdout).unwrap();
    let error_msg = format!("{} {}", "Unsat could not parse sat solver output", xout);

    let mut lines = xout.lines();
    let first_line = lines.next().ok_or(&error_msg).unwrap();

    if out.status.success() && first_line == "sat" {
        let rest = lines.collect::<Vec<&str>>().join("\n");
        return Ok(parse_bindings(parse_term(&rest)));
    } else if !out.status.success() && first_line == "unsat" {
        return Err(UnsatError);
    } else {
        panic!("{}", error_msg);
    }
}

fn parse_bindings(term: Term) -> Vec<(ISymbol, Term)> {
    match term {
        Term::Let(l) => {
            return l.bindings.clone();
        }

        _ => {
            panic!("{} {:?}", "Can only parse bindings out of let term", term)
        }
    }
}

fn parse_term(smt: &str) -> Term {
    let smt = format!("(assert (let {} 1))", smt);

    let s = Script::<Term>::parse(smt.as_bytes()).unwrap();

    return s.into_asserted_terms().next().unwrap();
}
