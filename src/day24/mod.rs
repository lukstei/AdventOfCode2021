use crate::day24::Instr::Inp;
use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use regex::internal::Inst;
use regex::Regex;
use std::fmt::{format, Display, Formatter};

type HashMap<T, V> = rustc_hash::FxHashMap<T, V>;
type HashSet<T> = rustc_hash::FxHashSet<T>;

type Word = i64;

#[derive(Debug)]
struct State<'a> {
    x: Word,
    y: Word,
    z: Word,
    w: Word,

    input: &'a Vec<Word>,
    input_i: usize,

    err: Option<Err>,
}

#[derive(Debug)]
enum Err {
    Div,
    Mod,
}

impl<'a> State<'a> {
    fn assign(&mut self, tgt: Reg, val: Word) {
        match tgt {
            Reg::X => self.x = val,
            Reg::Y => self.y = val,
            Reg::Z => self.z = val,
            Reg::W => self.w = val,
        }
    }
    fn read(&self, tgt: Reg) -> Word {
        match tgt {
            Reg::X => self.x,
            Reg::Y => self.y,
            Reg::Z => self.z,
            Reg::W => self.w,
        }
    }

    fn exec_until_end(&mut self, instr: &Vec<Instr>) -> Option<Word> {
        for x in instr.iter() {
            x.clone().exec(self);
            if let Some(e) = &self.err {
                return None;
            }
        }
        Some(self.z)
    }
    pub fn new(input: &'a Vec<Word>) -> Self {
        State {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            input,
            input_i: 0,
            err: None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Reg {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, Copy)]
enum Val {
    Static(Word),
    Reg(Reg),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Inp(Val),
    Add(Val, Val),
    Mul(Val, Val),
    Div(Val, Val),
    Mod(Val, Val),
    Eql(Val, Val),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Inp(usize),
    Val(Word),
    Dep(Reg),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Eql(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Inp(x) => write!(f, "inp_{}", x),
            Expr::Val(x) => write!(f, "{}", x),
            Expr::Add(x, y) => write!(f, "({}+{})", x, y),
            Expr::Mul(x, y) => write!(f, "({}*{})", x, y),
            Expr::Div(x, y) => write!(f, "({}/{})", x, y),
            Expr::Mod(x, y) => write!(f, "({}%{})", x, y),
            Expr::Eql(x, y) => write!(f, "({}=={})", x, y),
            Expr::Dep(r) => write!(f, "{}", BackAnalyzer::reg_str(*r)),
        }
    }
}

impl Expr {
    fn simplify(&self) -> Expr {
        match self {
            Expr::Add(x, y) => {
                let x = x.simplify();
                let y = y.simplify();

                if let Expr::Val(0) = x {
                    return y;
                } else if let Expr::Val(0) = y {
                    return x;
                }

                if let Expr::Val(x) = x {
                    if let Expr::Val(y) = y {
                        return Expr::Val(x + y);
                    }
                }
            }
            Expr::Mul(x, y) => {
                let x = x.simplify();
                let y = y.simplify();

                if let Expr::Val(0) = x {
                    return Expr::Val(0);
                } else if let Expr::Val(0) = y {
                    return Expr::Val(0);
                }

                if let Expr::Val(1) = x {
                    return y;
                } else if let Expr::Val(1) = y {
                    return x;
                }

                if let Expr::Val(x) = x {
                    if let Expr::Val(y) = y {
                        return Expr::Val(x * y);
                    }
                }
            }
            Expr::Div(x, y) => {
                let x = x.simplify();
                let y = y.simplify();

                if let Expr::Val(0) = x {
                    return Expr::Val(0);
                }

                if let Expr::Val(1) = y {
                    return x;
                }

                if let Expr::Val(x) = x {
                    if let Expr::Val(y) = y {
                        return Expr::Val(x / y);
                    }
                }
            }
            Expr::Mod(x, y) => {
                if let Expr::Val(x) = x.simplify() {
                    if let Expr::Val(y) = y.simplify() {
                        return Expr::Val(x % y);
                    }
                }
            }
            Expr::Eql(x, y) => {
                let x = x.simplify();
                let y = y.simplify();

                match (&x, &y) {
                    (Expr::Val(x), Expr::Inp(_)) | (Expr::Inp(_), Expr::Val(x)) => {
                        if *x <= 0 || *x > 9 {
                            return Expr::Val(0);
                        }
                    }
                    _ => {}
                }

                if let Expr::Val(x) = x {
                    if let Expr::Val(y) = y {
                        return Expr::Val(if x == y { 1 } else { 0 });
                    }
                }
            }
            _ => (),
        }
        self.clone()
    }
}

impl Instr {
    fn read_val(state: &State, val: Val) -> Word {
        match val {
            Val::Static(x) => x,
            Val::Reg(x) => state.read(x),
        }
    }

    fn exec(self, state: &mut State) {
        match self {
            Instr::Inp(Val::Reg(x)) => {
                state.assign(x, state.input[state.input_i]);
                state.input_i += 1;
            }
            Instr::Add(Val::Reg(x), y) => {
                state.assign(x, state.read(x) + Instr::read_val(state, y))
            }
            Instr::Mul(Val::Reg(x), y) => {
                state.assign(x, state.read(x) * Instr::read_val(state, y))
            }
            Instr::Div(Val::Reg(x), y) => {
                let x2 = Instr::read_val(state, y);
                if x2 == 0 {
                    state.err = Some(Err::Div)
                } else {
                    state.assign(x, state.read(x) / x2)
                }
            }
            Instr::Mod(Val::Reg(x), y) => {
                let x1 = state.read(x);
                let x2 = Instr::read_val(state, y);

                if x1 < 0 || x2 <= 0 {
                    state.err = Some(Err::Mod)
                } else {
                    state.assign(x, x1 % x2)
                }
            }
            Instr::Eql(Val::Reg(x), y) => state.assign(
                x,
                if state.read(x) == Instr::read_val(state, y) {
                    1
                } else {
                    0
                },
            ),
            _ => {
                panic!("Invalid instruction {:?}: State: {:?}", self, state)
            }
        };
    }
}

pub struct BackAnalyzer {
    pub x: Expr,
    pub y: Expr,
    pub z: Expr,
    pub w: Expr,
    pub inp_i: usize,
}

impl BackAnalyzer {
    pub fn new() -> BackAnalyzer {
        BackAnalyzer {
            x: Expr::Val(0),
            y: Expr::Val(0),
            z: Expr::Val(0),
            w: Expr::Val(0),
            inp_i: 0,
        }
    }
    pub fn new_with_deps() -> BackAnalyzer {
        BackAnalyzer {
            x: Expr::Dep(Reg::X),
            y: Expr::Dep(Reg::Y),
            z: Expr::Dep(Reg::Z),
            w: Expr::Dep(Reg::W),
            inp_i: 0,
        }
    }

    fn exec(&mut self, instr: &Vec<Instr>) {
        for x in instr.iter() {
            self.add(x.clone());
        }
    }
    fn replace(&mut self, reg: Reg, expr: Expr) {
        let expr = expr.simplify();

        match reg {
            Reg::X => self.x = expr,
            Reg::Y => self.y = expr,
            Reg::Z => self.z = expr,
            Reg::W => self.w = expr,
        }
    }

    fn value(&self, v: Val) -> Expr {
        match v {
            Val::Static(x) => Expr::Val(x),
            Val::Reg(x) => self.value_reg(x),
        }
    }

    fn value_reg(&self, x: Reg) -> Expr {
        match x {
            Reg::X => self.x.clone(),
            Reg::Y => self.y.clone(),
            Reg::Z => self.z.clone(),
            Reg::W => self.w.clone(),
        }
    }

    fn reg_str(reg: Reg) -> &'static str {
        match reg {
            Reg::X => "x",
            Reg::Y => "y",
            Reg::Z => "z",
            Reg::W => "w",
        }
    }

    fn add(&mut self, instr: Instr) {
        match instr {
            Instr::Inp(Val::Reg(x)) => {
                self.replace(x, Expr::Inp(self.inp_i));
                self.inp_i += 1;
            }
            Instr::Add(Val::Reg(x), y) => self.replace(
                x,
                Expr::Add(Box::new(self.value_reg(x)), Box::new(self.value(y))),
            ),
            Instr::Mul(Val::Reg(x), y) => self.replace(
                x,
                Expr::Mul(Box::new(self.value_reg(x)), Box::new(self.value(y))),
            ),
            Instr::Div(Val::Reg(x), y) => self.replace(
                x,
                Expr::Div(Box::new(self.value_reg(x)), Box::new(self.value(y))),
            ),
            Instr::Mod(Val::Reg(x), y) => self.replace(
                x,
                Expr::Mod(Box::new(self.value_reg(x)), Box::new(self.value(y))),
            ),
            Instr::Eql(Val::Reg(x), y) => self.replace(
                x,
                Expr::Eql(Box::new(self.value_reg(x)), Box::new(self.value(y))),
            ),
            _ => panic!("unexpected expr"),
        }
    }
}

fn parse(s: &str) -> Vec<Instr> {
    s.trim()
        .lines()
        .map(|l| {
            fn parse_val(s: &str) -> Val {
                match s {
                    "w" => Val::Reg(Reg::W),
                    "x" => Val::Reg(Reg::X),
                    "y" => Val::Reg(Reg::Y),
                    "z" => Val::Reg(Reg::Z),
                    _ => Val::Static(s.parse().unwrap()),
                }
            }

            let s = l.trim().split(' ').collect_vec();
            match s[0] {
                "inp" => Instr::Inp(parse_val(s[1])),
                "add" => Instr::Add(parse_val(s[1]), parse_val(s[2])),
                "mul" => Instr::Mul(parse_val(s[1]), parse_val(s[2])),
                "div" => Instr::Div(parse_val(s[1]), parse_val(s[2])),
                "mod" => Instr::Mod(parse_val(s[1]), parse_val(s[2])),
                "eql" => Instr::Eql(parse_val(s[1]), parse_val(s[2])),
                _ => panic!("Unexepected instruction: {}", l),
            }
        })
        .collect_vec()
}

fn solution1(input: &str) -> Result<String> {
    let sub_programs = vec![
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 0
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
",
        "inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y",
    ];

    let mut target_z_states: HashSet<Word> = Default::default();
    target_z_states.insert(0);
    //(11..=19).for_each(|x|{target_z_states.insert(x);});
    //let mut valid_states: Vec<HashSet<(Word, Word, Word)>> = (0..14).map(|x| Default::default()).collect_vec();

    for (i, x) in sub_programs.iter().enumerate() {
        let mut valid_z_states: HashSet<Word> = Default::default();

        for input in 1..=9 {
            for z in target_z_states.iter() {
                let inp = vec![input];
                let mut state = State::new(&inp);
                state.z = *z;
                match state.exec_until_end(&parse(*x)) {
                    None => {}
                    Some(z_out) => {
                        valid_z_states.insert(z_out);
                        //valid_states[i].insert((z_out, *z, input));
                    }
                };
            }
        }

        target_z_states = valid_z_states;
    }

    println!("States len:{}", target_z_states.len());

    Ok(format!("{}", "??"))
}

fn find_highest_reachable_number(
    x: &Vec<HashMap<Word, HashMap<Word, Word>>>,
    i: usize,
    z: Word,
) -> Option<String> {
    if i == x.len() {
        //assert_eq!(z, 0);
        return Some("".into());
    }

    (0..=9)
        .rev()
        .filter_map(|num| match x[i].get(&num) {
            None => None,
            Some(zs) => match zs.get(&z) {
                None => None,
                Some(next_z) => find_highest_reachable_number(x, i + 1, *next_z)
                    .map(|num2| num.to_string() + &num2),
            },
        })
        .next()
}

fn solution2(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}

mod tests {
    use crate::day24::{parse, solution1, solution2, BackAnalyzer, Expr, State};
    use crate::run_solution;
    use indoc::indoc;

    const INPUT: &'static str = "day24.txt";

    // PART 1

    #[test]
    fn test_ins() {
        let v = vec![3];
        assert_eq!(
            Some(-3),
            State::new(&vec![3]).exec_until_end(&parse(
                "inp z
        mul z -1"
            ))
        );

        let v = vec![3, 9];
        let mut state = State::new(&v);
        let s = state.exec_until_end(&parse(
            "inp z
inp x
mul z 3
eql z x",
        ));
        assert!(s.is_some());
        assert_eq!(1, state.z);

        let v = vec![3, 10];
        let mut state = State::new(&v);
        let s = state.exec_until_end(&parse(
            "inp z
inp x
mul z 3
eql z x",
        ));
        assert!(s.is_some());
        assert_eq!(0, state.z);
    }

    #[test]
    fn test_back() {
        let instr = parse(
            "inp z
inp x
mul z 3
eql z x",
        );
        //z*3 == x
        // z = ((inp_1)*3) == (inp_0)
        let mut ba = BackAnalyzer::new();
        for x in instr.iter() {
            ba.add(x.clone());
            println!("z={}", ba.z);
        }

        assert_eq!("((inp_0*3)==inp_1)", format!("{}", ba.z));
    }

    #[test]
    fn test_back2() {
        let instr = parse(
            "inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y",
        );

        let mut ba = BackAnalyzer::new_with_deps();
        ba.exec(&instr);
        println!("x={}\n\ny={}\n\nz={}\n\nw={}", ba.x, ba.y, ba.z, ba.w);

        assert_eq!("((inp_0*3)==inp_1)", format!("{}", ba.z));
    }

    #[test]
    fn test_simplify() {
        assert_eq!(
            "0",
            format!(
                "{}",
                Expr::Mul(Expr::Val(0).into(), Expr::Inp(2).into()).simplify()
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!("??", solution1(indoc!("")).unwrap());
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("??", solution2(indoc!("")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

pub fn day24() {
    let instr = parse(
        "inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 0
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y",
    );

    let mut ba = BackAnalyzer::new();
    let mut i = 0;
    for x in instr.iter() {
        ba.add(x.clone());
        println!("i={}", i);
        i += 1;
    }

    assert_eq!("((inp_0*3)==inp_1)", format!("{}", ba.z));
}
