use crate::day18::Node::Num;
use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};
use std::fmt::{write, Display, Formatter};

#[derive(Clone)]
enum Node {
    Num(u32),
    Leaf(Box<Tree>),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Num(x) => write!(f, "{}", x),
            Node::Leaf(x) => x.fmt(f),
        }
    }
}

impl Node {
    fn magnitude(&self) -> u32 {
        match self {
            Num(x) => *x,
            Node::Leaf(s) => s.l.magnitude() * 3 + s.r.magnitude() * 2,
        }
    }

    fn reduce(&mut self) {
        loop {
            if !(self.explode(0).is_some() || self.split()) {
                return;
            }
        }
    }

    // incraeses the left most value
    fn incr(&mut self, left: bool, num: u32) -> bool {
        match self {
            Node::Num(c) => {
                *c += num;
                return true;
            }
            Node::Leaf(c) => {
                if left {
                    c.l.incr(left, num) || c.r.incr(left, num)
                } else {
                    c.r.incr(left, num) || c.l.incr(left, num)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Num(x) => false,
            Node::Leaf(s) => {
                if let Node::Num(l) = s.l {
                    if l >= 10 {
                        s.l = Node::Leaf(Box::new(Tree {
                            l: Node::Num(l / 2),
                            r: Node::Num((l + 1) / 2),
                        }));
                        return true;
                    }
                }
                if s.l.split() {
                    return true;
                }

                if let Node::Num(r) = s.r {
                    if r >= 10 {
                        s.r = Node::Leaf(Box::new(Tree {
                            l: Node::Num(r / 2),
                            r: Node::Num((r + 1) / 2),
                        }));
                        return true;
                    }
                }
                if s.r.split() {
                    return true;
                }
                false
            }
        }
    }

    fn add(l: Node, r: Node) -> Node {
        let mut node = Node::Leaf(Box::new(Tree { l, r }));
        node.reduce();
        node
    }

    fn explode(&mut self, d: u32) -> Option<(Option<u32>, Option<u32>)> {
        if let Node::Leaf(c) = self {
            if d == 4 {
                if let (Num(l), Num(r)) = (&c.l, &c.r) {
                    Some((Some(*l), Some(*r)))
                } else {
                    panic!("sd")
                }
            } else {
                if let Some((l, r)) = c.l.explode(d + 1) {
                    if d == 3 {
                        c.l = Node::Num(0);
                    }

                    if let Some(rr) = r {
                        if c.r.incr(true, rr) {
                            return Some((l, None));
                        }
                    }
                    Some((l, r))
                } else if let Some((l, r)) = c.r.explode(d + 1) {
                    if d == 3 {
                        c.r = Node::Num(0);
                    }

                    if let Some(ll) = l {
                        if c.l.incr(false, ll) {
                            return Some((None, r));
                        }
                    }
                    Some((l, r))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn add_all(nodes: Vec<Node>) -> Node {
        nodes.into_iter().reduce(|s, x| Node::add(s, x)).unwrap()
    }
}

#[derive(Clone)]
struct Tree {
    l: Node,
    r: Node,
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        self.l.fmt(f)?;
        write!(f, ",")?;
        self.r.fmt(f)?;
        write!(f, "]")?;
        Ok(())
    }
}

impl Tree {
    fn parse_tree(s: &str) -> (&str, Tree) {
        assert_eq!('[', s.chars().next().unwrap());
        let (s, l) = Self::parse_node(s.split_at(1).1);
        assert_eq!(',', s.chars().next().unwrap());
        let (s, r) = Self::parse_node(s.split_at(1).1);
        assert_eq!(']', s.chars().next().unwrap());
        (s.split_at(1).1, Tree { l, r })
    }
    fn parse_node(s: &str) -> (&str, Node) {
        match s.chars().next().unwrap() {
            '[' => {
                let (s, tree) = Self::parse_tree(s);
                (s, Node::Leaf(Box::new(tree)))
            }
            _ => {
                let numidx = s.chars().take_while(|x| x.to_digit(10).is_some()).count();
                let (num, rest) = s.split_at(numidx);
                (rest, Node::Num(num.parse().unwrap()))
            }
        }
    }
}

fn solution1(input: &str) -> Result<String> {
    let trees = input
        .lines()
        .map(|x| Node::Leaf(Box::new(Tree::parse_tree(x.trim()).1)))
        .collect_vec();

    let node = Node::add_all(trees);
    Ok(format!("{}", node.magnitude()))
}

fn solution2(input: &str) -> Result<String> {
    let trees = input
        .lines()
        .map(|x| Node::Leaf(Box::new(Tree::parse_tree(x.trim()).1)))
        .collect_vec();

    let max = trees
        .iter()
        .tuple_combinations()
        .map(|(x1, x2)| {
            Node::add(x1.clone(), x2.clone())
                .magnitude()
                .max(Node::add(x2.clone(), x1.clone()).magnitude())
        })
        .max()
        .unwrap();

    Ok(format!("{}", max))
}

mod tests {
    use crate::day18::{solution1, solution2, Node, Tree};
    use crate::run_solution;
    use indoc::indoc;
    use itertools::Itertools;

    const INPUT: &'static str = "day18.txt";

    // PART 1

    #[test]
    fn test_add_all() {
        let trees = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"
            .lines()
            .map(|x| Node::Leaf(Box::new(Tree::parse_tree(x.trim()).1)))
            .collect_vec();

        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            format!("{}", Node::add_all(trees))
        );
    }

    #[test]
    fn test_add_all2() {
        let trees = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            .lines()
            .map(|x| Node::Leaf(Box::new(Tree::parse_tree(x.trim()).1)))
            .collect_vec();

        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            format!("{}", Node::add_all(trees))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!("[1,2]", solution1(indoc!("[1,2]")).unwrap());
        assert_eq!("[[1,2],3]", solution1(indoc!("[[1,2],3]")).unwrap());
        assert_eq!(
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
            solution1(indoc!(
                "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
            ))
            .unwrap()
        );
    }

    #[test]
    fn test_magnitude() {
        let mut tree = Node::Leaf(Box::new(
            Tree::parse_tree("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").1,
        ));

        assert_eq!(3488, tree.magnitude());
    }

    #[test]
    fn test_reduce() {
        let mut tree = Node::Leaf(Box::new(
            Tree::parse_tree("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").1,
        ));
        tree.reduce();
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format!("{}", tree));
    }

    #[test]
    fn test_split() {
        let mut tree = Node::Leaf(Box::new(
            Tree::parse_tree("[[[[0,7],4],[15,[0,13]]],[1,1]]").1,
        ));
        tree.split();
        assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", format!("{}", tree));
    }

    #[test]
    fn test_explode() {
        let mut tree = Node::Leaf(Box::new(Tree::parse_tree("[[[[[9,8],1],2],3],4]").1));
        tree.explode(0);
        assert_eq!("[[[[0,9],2],3],4]", format!("{}", tree));
    }

    #[test]
    fn test_explode2() {
        let mut tree = Node::Leaf(Box::new(Tree::parse_tree("[7,[6,[5,[4,[3,2]]]]]").1));
        tree.explode(0);
        assert_eq!("[7,[6,[5,[7,0]]]]", format!("{}", tree));
    }

    #[test]
    fn test_explode3() {
        let mut tree = Node::Leaf(Box::new(Tree::parse_tree("[[6,[5,[4,[3,2]]]],1]").1));
        tree.explode(0);
        assert_eq!("[[6,[5,[7,0]]],3]", format!("{}", tree));
    }

    #[test]
    fn test_explode4() {
        let mut tree = Node::Leaf(Box::new(
            Tree::parse_tree("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").1,
        ));
        tree.explode(0);
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", format!("{}", tree));
    }

    #[test]
    fn test_explode5() {
        let mut tree = Node::Leaf(Box::new(
            Tree::parse_tree("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").1,
        ));
        tree.explode(0);
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", format!("{}", tree));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            "4140",
            solution1(indoc!(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ))
            .unwrap()
        );
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
