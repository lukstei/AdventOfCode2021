use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::day16::Package::{Lit, Op};
use crate::util::{parse_lines, parse_lines_regex};

enum Package{
    Lit(u32, u64),
    Op(u32, u32, Vec<Package>)
}

fn take_bits(deque: &mut VecDeque<char>, n: usize)->u32{
    let mut x = String::new();
    for i in 0..n {
        x += &String::from(deque.pop_front().unwrap());
    }
    //println!("Took {} bits got {}", n, x);

    return u32::from_str_radix(&x, 2).unwrap();
}

fn take_binary_str(deque: &mut VecDeque<char>, n: usize)->String{
    let mut x = String::new();
    for i in 0..n {
        x += &String::from(deque.pop_front().unwrap());
    }
    //println!("Took {} bits got {}", n, x);
    return x;
}

fn parse_lit_package(v: u32, s: &mut VecDeque<char>) -> Package{
    let mut x = String::new();
    loop{
        let cont = take_bits(s, 1)==1;
        x += &take_binary_str(s, 4);
        if !cont{
            break;
        }
    }
    let i = u64::from_str_radix(&x, 2).unwrap();

    println!("Lit package: got {}", i);
    return Lit(v, i);
}

fn parse_op_package(v: u32, typ: u32, s: &mut VecDeque<char>) -> Package{
    let l = take_bits(s, 1);

    let vec = match l{
        0 =>{
            let bits = take_bits(s, 15) as usize;
            println!("Operator Package: bits = {}", bits);
            let mut r = Vec::new();

            let len_start = s.len();
            loop{
                r.push(parse_package(s));
                println!("   Parsed subpacket consumed already {} bits, must consume {} bits", len_start-s.len(), bits);
                if len_start-s.len() == bits{
                    break;
                }
            }
            r
        },
        1 =>{
            let len = take_bits(s, 11);
            println!("Operator Package: len = {}", len);
            (0..len).map(|_s|parse_package(s)).collect_vec()
        }
        _ => panic!("asdjkhl")
    };

    Op(v, typ, vec)
}

fn parse_package(s: &mut VecDeque<char>) -> Package{
    let version = take_bits(s, 3);
    let typ = take_bits(s, 3);
    println!("Parsing pkg v={}, type={}", version, typ);

    match typ{
        4 => parse_lit_package(version,s),
        _ => parse_op_package(version, typ, s)
    }
}

fn solution1(input: &str) -> Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let mut zero_vec = line.chars().map(|x| format!("{:04b}", x.to_digit(16).unwrap())).join("");
        let mut deque = VecDeque::new();
        for c in zero_vec.chars() {
            deque.push_back(c);
        }

        dbg!(zero_vec);

        fn extract_res(p: &Package)->u64{
            match p {
                Lit(_v, val) => *val,
                Op(_v, typ, p) => match typ{
                    0 => p.iter().map(|x|extract_res(x)).sum::<u64>(),
                    1 => p.iter().map(|x|extract_res(x)).product::<u64>(),
                    2 => p.iter().map(|x|extract_res(x)).min().unwrap(),
                    3 => p.iter().map(|x|extract_res(x)).max().unwrap(),
                    5 => {
                       let vv = p.iter().map(|x|extract_res(x)).collect_vec();
                        if vv[0]>vv[1]{1}else{0}
                    },
                    6 => {
                        let vv = p.iter().map(|x|extract_res(x)).collect_vec();
                        if vv[0]<vv[1]{1}else{0}
                    },
                    7 => {
                        let vv = p.iter().map(|x|extract_res(x)).collect_vec();
                        if vv[0]==vv[1]{1}else{0}
                    },
                    _ => panic!("unexpected typ")
                }
            }
        }

        let package = parse_package(&mut deque);
        sum+= extract_res(&package);
    }
    Ok(format!("{}", sum))
}

fn solution2(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day16::{solution1, solution2};
    const INPUT: &'static str = "day16.txt";

    // PART 1

    #[test]
    fn test_lit() {
        assert_eq!("1",
                   solution1(indoc!("9C0141080250320F1802104A08")).unwrap());
    }
    #[test]
    fn test_rr() {
        assert_eq!("16",
                   solution1(indoc!("EE00D40C823060")).unwrap());
    }


    #[test]
    fn test_part1() {
        assert_eq!("28",
                   solution1(indoc!("8A004A801A8002F478
                   620080001611562C8802118E34")).unwrap());
    }


    #[test]
    fn run_solution1(){
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("??", solution2(indoc!("")).unwrap());
    }

    #[test]
    fn run_solution2(){
        run_solution(INPUT, solution2).unwrap()
    }

}

