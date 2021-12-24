use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::os::macos::raw::stat;
use regex::internal::Inst;
use crate::day24::Instr::Inp;

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

    fn exec_until_end(&mut self, instr: &Vec<Instr>)->Option<Word>{
        for x in instr.iter() {
            x.clone().exec(self);
            if let Some(e) = &self.err{
                return None
            }
        }
        Some(self.z)
    }
    pub fn new(input: &'a Vec<Word>) -> Self {
        State{
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            input,
            input_i: 0,
            err: None
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Reg {
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

impl Instr {
    fn read_val(state: &State, val: Val) -> Word {
        match val {
            Val::Static(x) => x,
            Val::Reg(x) => state.read(x)
        }
    }

    fn exec(self, state: &mut State) {
        match self {
            Instr::Inp(Val::Reg(x)) => {
                state.assign(x, state.input[state.input_i]);
                state.input_i += 1;
            }
            Instr::Add(Val::Reg(x), y) => state.assign(x, state.read(x) + Instr::read_val(state, y)),
            Instr::Mul(Val::Reg(x), y) => state.assign(x, state.read(x) * Instr::read_val(state, y)),
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
            Instr::Eql(Val::Reg(x), y) => state.assign(x, if state.read(x) == Instr::read_val(state, y) { 1 } else { 0 }),
            _ => {
                panic!("Invalid instruction {:?}: State: {:?}", self, state)
            }
        };
    }
}

struct BackAnalyzer{
    x: String,
    y: String,
    z: String,
    w: String,
    inp_i: usize
}

impl BackAnalyzer {
    pub fn new() -> BackAnalyzer {
        BackAnalyzer{
            x: "0".to_string(),
            y: "0".to_string(),
            z: "0".to_string(),
            w: "0".to_string(),
            inp_i: 0
        }
    }
}

impl BackAnalyzer{
    fn simplify(expr:&str){

    }

    fn replace(&mut self, reg: Reg, mut expr: &str){
        fn r(s: &str, from: &str, e: &str)->String{
            s.replace(from, e)
        }
        
        match reg{
            Reg::X => self.x = expr.into(),
            Reg::Y => self.y = expr.into(),
            Reg::Z => self.z = expr.into(),
            Reg::W => self.w = expr.into(),
        }
    }

    fn value(&self, v: Val) -> String {
        match v{
            Val::Static(x) => x.to_string(),
            Val::Reg(x) => self.value_reg(x)
        }
    }
    fn value_reg(&self, x: Reg) -> String {
        match x{
            Reg::X => self.x.clone(),
            Reg::Y => self.y.clone(),
            Reg::Z => self.z.clone(),
            Reg::W => self.w.clone(),
        }
    }

    fn reg_str(reg: Reg) -> &'static str {
        match reg{
            Reg::X => "x",
            Reg::Y => "y",
            Reg::Z => "z",
            Reg::W => "w",
        }
    }

    fn add(&mut self, instr: Instr){

        match instr{
            Instr::Inp(Val::Reg(x)) => {
                self.replace(x, &format!("inp_{}", self.inp_i));
                self.inp_i+=1;
            }
            Instr::Add(Val::Reg(x), y) => self.replace(x, &format!("({}+{})", self.value_reg(x), self.value(y))),
            Instr::Mul(Val::Reg(x), y) => {
                if let Val::Static(0) = y{
                    self.replace(x, "0")
                }else{
                    self.replace(x, &format!("({}*{})", self.value_reg(x), self.value(y)))
                }
            },
            Instr::Div(Val::Reg(x), y) => self.replace(x, &format!("({}/{})", self.value_reg(x), self.value(y))),
            Instr::Mod(Val::Reg(x), y) => self.replace(x, &format!("({}%{})", self.value_reg(x), self.value(y))),
            Instr::Eql(Val::Reg(x), y) => self.replace(x, &format!("({}=={})", self.value_reg(x), self.value(y))),
            _ => panic!("unexpected expr")
        }
    }
}

fn parse(s: &str) -> Vec<Instr> {
    s.trim().lines().map(|l| {
        fn parse_val(s: &str) -> Val {
            match s {
                "w" => Val::Reg(Reg::W),
                "x" => Val::Reg(Reg::X),
                "y" => Val::Reg(Reg::Y),
                "z" => Val::Reg(Reg::Z),
                _ => Val::Static(s.parse().unwrap())
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
            _ => panic!("Unexepected instruction: {}", l)
        }
    }).collect_vec()
}


fn solution1(input: &str) -> Result<String> {
    let instrs = parse(input);

    let mut num = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    assert_eq!(num.len(), 14);

    let mut i = num.len()-1;

    loop{
        if let Some(x) = State::new(&num).exec_until_end(&instrs){
            if x==0{
                return Ok(num.iter().map(|x|x.to_string()).join(""))
            }
        }

        if num[i] == 1{
            i-=1;
        }
        num[i]-=1;
    }

    Ok(format!("{}", "?"))
}

fn solution2(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}

mod tests {
    use crate::run_solution;
    use crate::day24::{BackAnalyzer, parse, solution1, solution2, State};
    use indoc::indoc;

    const INPUT: &'static str = "day24.txt";

    // PART 1

    #[test]
    fn test_ins() {
        let v = vec![3];
        assert_eq!(Some(-3), State::new(&vec![3]).exec_until_end(&parse("inp z
        mul z -1")));


        let v = vec![3, 9];
        let mut state = State::new(&v);
        let s = state.exec_until_end(&parse("inp z
inp x
mul z 3
eql z x"));
        assert!(s.is_some());
        assert_eq!(1, state.z);

        let v = vec![3, 10];
        let mut state = State::new(&v);
        let s = state.exec_until_end(&parse("inp z
inp x
mul z 3
eql z x"));
        assert!(s.is_some());
        assert_eq!(0, state.z);
    }

    #[test]
    fn test_back() {
        let instr = parse("inp z
inp x
mul z 3
eql z x");
        //z*3 == x
// z = ((inp_1)*3) == (inp_0)
        let mut ba = BackAnalyzer::new();
        for x in instr.iter() {
            ba.add(x.clone());
            println!("z={}", ba.z);
        }

        assert_eq!("((inp_0*3)==inp_1)", ba.z);
    }
    #[test]
    fn test_back2() {
        let instr = parse("inp w
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
add z y");

        let mut ba = BackAnalyzer::new();
        let i = 0;
        for x in instr.iter() {
            ba.add(x.clone());
            //println!("z={}", ba.z);
        }

        assert_eq!("??", ba.z);
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
