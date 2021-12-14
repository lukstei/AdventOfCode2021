use std::collections::{HashMap, HashSet};
use std::detect::__is_feature_detected::cmpxchg16b;
use anyhow::{Result};
use itertools::{Itertools, izip, max};
use crate::util::{parse_lines, parse_lines_regex};


fn from_pair(p: (char, char), map: &str) -> Vec<u64> {
    let mut v = (0..map.len()).map(|x| 0).collect_vec();
    v[map.find(p.0).unwrap()] = 1;
    v
}

fn cnt_rec(map: &str, cache: &mut HashMap<(char, char, usize), Vec<u64>>, step: usize, max_step: usize, pair: (char, char), rules: &HashMap<(char, char), char>) -> Vec<u64> {
    if step == max_step {
        return from_pair(pair, map);
    }

    match cache.get(&(pair.0, pair.1, step)) {
        None => {
            let x1 = match rules.get(&pair) {
                None => (0..map.len()).map(|x| 0).collect_vec(),
                Some(s) => cnt_rec(map, cache, step + 1, max_step, (pair.0, *s), &rules).iter()
                    .zip(cnt_rec(map, cache, step + 1, max_step, (*s, pair.1), &rules)).map(|(x1, x2)| x1 + x2)
                    .collect_vec()
            };
            cache.insert((pair.0, pair.1, step), x1.clone());
            x1
        }
        Some(r) => r.clone()
    }
}

fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(&input.lines().skip(1).join("\n").trim(), "^(.+?) -> (.+?)$")?;

    let mut x = input.lines().next().unwrap().chars().collect_vec();
    let mut all_chars:HashSet<char> = HashSet::new();

    let ys = xs.iter()
        .map(|l| {
            let (v1, v2) = (
                l[1].as_str(),
                l[2].as_str().chars().next().unwrap()
            );

            for x in v1.chars() {
                all_chars.insert(x);
            }

            ((v1.chars().nth(0).unwrap(), v1.chars().nth(1).unwrap()), v2)
        })
        .collect::<Vec<((char, char), char)>>();


    let mut map: HashMap<(char, char), char> = HashMap::new();
    for x in ys {
        map.insert(x.0, x.1);
    }

    let char_map = all_chars.iter().join("");

    let res = (0..x.len() - 1).map(|i| cnt_rec(&char_map, &mut Default::default(), 0, 40, (x[i], x[i + 1]), &map))
        .fold((0..char_map.len()).map(|x| 0).collect_vec(), |mut x, s| {
            for i in 0..x.len() {
                x[i]+=s[i];
            }
            x
        });

    Ok(format!("{:?}", res))
}

fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(&input.lines().skip(1).join("\n").trim(), "^(.+?) -> (.+?)$")?;

    let mut x = input.lines().next().unwrap().chars().collect_vec();
    let ys = xs.iter()
        .map(|l| {
            let (v1, v2) = (
                l[1].as_str(),
                l[2].as_str().chars().next().unwrap()
            );

            (v1, v2)
        })
        .collect::<Vec<(&str, char)>>();


    for _step in 0..40 {
        let mut i = 0;
        loop {
            if i >= x.len() - 1 {
                break;
            }

            let mut has_inserted = false;

            let c1 = x[i];
            let c2 = x[i + 1];
            for ins in ys.iter() {
                if (ins.0.chars().nth(0).unwrap() == c1 && ins.0.chars().nth(1).unwrap() == c2) {
                    i += 1;
                    //println!("Insert {} at pos {}", ins.1, i);
                    x.insert(i, ins.1);
                    has_inserted = true;
                }
            }
            //NBCCNBBBCBHCB
            //NBCCNBBBC HCB

            i += 1;
        }

        println!("step {} len {}", _step, x.len());
        // println!("{}", x.iter().join(""));
    }

    x.sort();

    let mut least_key = ('-', 100000000);
    let mut most_key = ('-', 0);

    let mut group = *x.iter().next().unwrap();
    let mut cnt = 0;
    for c in x {
        if group != c {
            if cnt < least_key.1 {
                least_key = (c, cnt);
            }
            if cnt > most_key.1 {
                most_key = (c, cnt);
            }

            cnt = 0;
            group = c;
        }
        cnt += 1;
    }

    Ok(format!("{}", most_key.1 - least_key.1))
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day14::{solution1, solution2};

    const INPUT: &'static str = "day14.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("1588",
                   solution1(indoc!("NNCB
CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C")).unwrap());
    }


    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("1588",
                   solution2(indoc!("NNCB
CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

