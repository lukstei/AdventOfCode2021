use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use bit_set::BitSet;
use enumset::{EnumSet, EnumSetType};
use itertools::{izip, Itertools};
use std::fmt::{Display, Formatter};
use std::mem::{swap, transmute};
use Amphipod::*;
use Location::*;

type HashMap<T, V> = rustc_hash::FxHashMap<T, V>;
type HashSet<T> = rustc_hash::FxHashSet<T>;

#[derive(Debug, Hash, EnumSetType)]
enum Location {
    AL2,
    AL1,
    AB,
    BC,
    CD,
    DR1,
    DR2,

    RA1,
    RA2,
    RB1,
    RB2,
    RC1,
    RC2,
    RD1,
    RD2,

    RA3,
    RA4,
    RB3,
    RB4,
    RC3,
    RC4,
    RD3,
    RD4,
}

impl Location {
    fn is_room(&self) -> bool {
        match self {
            RA1 | RA2 | RB1 | RB2 | RC1 | RC2 | RD1 | RD2 | RA3 | RA4 | RB3 | RB4 | RC3 | RC4
            | RD3 | RD4 => true,
            _ => false,
        }
    }
    fn is_hallway(&self) -> bool {
        !self.is_room()
    }

    fn get_rooms_in_order_for_room(&self) -> Vec<Location> {
        assert!(self.is_room());

        match self {
            RA1 | RA2 | RA3 | RA4 => vec![RA1, RA2, RA3, RA4],
            RB1 | RB2 | RB3 | RB4 => vec![RB1, RB2, RB3, RB4],
            RC1 | RC2 | RC3 | RC4 => vec![RC1, RC2, RC3, RC4],
            RD1 | RD2 | RD3 | RD4 => vec![RD1, RD2, RD3, RD4],
            _ => panic!(""),
        }
    }

    fn get_first_room_if_is_second(&self) -> Option<Location> {
        match self {
            RA2 => Some(RA1),
            RB2 => Some(RB1),
            RC2 => Some(RC1),
            RD2 => Some(RD1),
            _ => None,
        }
    }

    fn get_second_room_if_is_first(&self) -> Option<Location> {
        match self {
            RA1 => Some(RA2),
            RB1 => Some(RB2),
            RC1 => Some(RC2),
            RD1 => Some(RD2),
            _ => None,
        }
    }

    fn is_first_room(&self) -> bool {
        self.get_second_room_if_is_first().is_some()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn target_room2(&self) -> Location {
        match self {
            A => RA2,
            B => RB2,
            C => RC2,
            D => RD2,
        }
    }
}

enum Room {
    A,
    B,
    C,
    D,
}

type LocationSet = EnumSet<Location>;

#[derive(Clone)]
struct State {
    locations: HashMap<Location, Amphipod>,
    score: u32,
}

impl State {
    fn new(ls: Vec<(Location, Amphipod)>) -> State {
        let mut locations: HashMap<Location, Amphipod> = Default::default();
        for (l, a) in ls {
            locations.insert(l, a);
        }
        State {
            locations,
            score: 0,
        }
    }

    fn get(&self, l: Location) -> Option<Amphipod> {
        self.locations.get(&l).cloned()
    }
    fn is_occupied(&self, l: Location) -> bool {
        self.locations.contains_key(&l)
    }

    fn all_moves_from_room(&self) -> LocationSet {
        AL2 | AL1 | AB | BC | CD | DR1 | DR2
    }

    fn can_go_on_hallway(&self, from: Location, to: Location) -> bool {
        let order = vec![AL2, AL1, AB, BC, CD, DR1, DR2];

        let get_index_from_room_too_hallway_right = |mut t: Location| {
            t = match t {
                RA1 | RA2 | RA3 | RA4 => AB,
                RB1 | RB2 | RB3 | RB4 => BC,
                RC1 | RC2 | RC3 | RC4 => CD,
                RD1 | RD2 | RD3 | RD4 => DR1,
                _ => panic!("room"),
            };
            order.iter().position(|x| *x == t).unwrap()
        };

        let mut i1 = if from.is_room() {
            let mut i1 = get_index_from_room_too_hallway_right(from);
            if i1 > order.iter().position(|x| *x == to).unwrap() {
                // we go to the next left position
                i1 -= 1
            }
            i1
        } else {
            order.iter().position(|x| *x == from).unwrap()
        };

        let mut i2 = if to.is_room() {
            let mut i2 = get_index_from_room_too_hallway_right(to);
            if order.iter().position(|x| *x == from).unwrap() < i2 {
                // we go to the next left position
                i2 -= 1
            }
            i2
        } else {
            order.iter().position(|x| *x == to).unwrap()
        };

        if i1 == i2 {
            // this is the special case when we come/go to a room
            return to.is_room() || !self.is_occupied(order[i1]);
        }

        let omit_check_for = if !from.is_room() { Some(i1) } else { None };

        if i1 > i2 {
            swap(&mut i1, &mut i2);
        }

        // we don't check the position we're currently in
        (i1..=i2).all(|i| {
            (omit_check_for.is_some() && omit_check_for.unwrap() == i)
                || !self.is_occupied(order[i])
        })
    }

    fn finished(&self) -> bool {
        self.locations.iter().all(|(l, a)| self.is_finished(*l, *a))
    }

    fn calc_moves(from: Location, to: Location) -> u32 {
        let order = vec![AL2, AL1, AB, BC, CD, DR1, DR2];

        let get_index_from_room_too_hallway_right = |mut t: Location| {
            t = match t {
                RA1 | RA2 | RA3 | RA4 => AB,
                RB1 | RB2 | RB3 | RB4 => BC,
                RC1 | RC2 | RC3 | RC4 => CD,
                RD1 | RD2 | RD3 | RD4 => DR1,
                _ => panic!("room"),
            };
            order.iter().position(|x| *x == t).unwrap()
        };

        let i1 = if from.is_room() {
            let mut i1 = get_index_from_room_too_hallway_right(from);
            if i1 > order.iter().position(|x| *x == to).unwrap() {
                // we go to the next left position
                i1 -= 1
            }
            i1
        } else {
            order.iter().position(|x| *x == from).unwrap()
        };

        let i2 = if to.is_room() {
            let mut i2 = get_index_from_room_too_hallway_right(to);
            if order.iter().position(|x| *x == from).unwrap() < i2 {
                // we go to the next left position
                i2 -= 1
            }
            i2
        } else {
            order.iter().position(|x| *x == to).unwrap()
        };

        let mut hallway_moves = if i1 > i2 { i1 - i2 } else { i2 - i1 };
        hallway_moves *= 2;
        if matches!(from, AL2 | DR2) || matches!(to, AL2 | DR2) {
            hallway_moves -= 1;
        }

        let room = if from.is_room() { from } else { to };
        let room_moves = 2 + room
            .get_rooms_in_order_for_room()
            .iter()
            .take_while(|x| **x != room)
            .count();
        (room_moves + hallway_moves) as u32
    }

    fn move_(&self, l: Location, a: Amphipod, to: Location) -> State {
        let mut map = self.locations.clone();
        assert!(map.remove(&l).is_some());
        assert!(map.insert(to, a).is_none());
        State {
            score: self.score
                + Self::calc_moves(l, to)
                    * match a {
                        A => 1,
                        B => 10,
                        C => 100,
                        D => 1000,
                    },
            locations: map,
        }
    }

    fn solve_rec(
        &self,
        cache: &mut HashMap<(u32, Vec<(Location, Amphipod)>), Option<u32>>,
    ) -> Option<u32> {
        if self.finished() {
            return Some(self.score);
        }

        let key = (
            self.score,
            self.locations.iter().map(|x| (*x.0, *x.1)).collect_vec(),
        );
        match cache.get(&key) {
            None => {
                let mut min = None;

                for (l, a) in &self.locations {
                    for to in self.moves_for(*l) {
                        let result = self.move_(*l, *a, to).solve_rec(cache);
                        if let Some(r) = result {
                            match min {
                                None => min = Some(r),
                                Some(r2) => min = Some(r.min(r2)),
                            }
                        }
                    }
                }

                cache.insert(key, min);

                min
            }

            Some(s) => *s,
        }
    }

    fn moves_for(&self, l: Location) -> LocationSet {
        let a = self.get(l).unwrap();
        if self.is_finished(l, a) {
            EnumSet::empty()
        } else if l.is_room() {
            let rooms = l.get_rooms_in_order_for_room();
            let rooms_before_occupied = rooms
                .iter()
                .take_while(|x| **x != l)
                .any(|x| self.is_occupied(*x));

            if rooms_before_occupied {
                EnumSet::empty()
            } else {
                let possible_moves = self.all_moves_from_room();
                possible_moves
                    .iter()
                    .filter(|x| self.can_go_on_hallway(l, *x))
                    .collect()
            }
        } else {
            // to finish
            let rooms = a.target_room2().get_rooms_in_order_for_room();
            let target_room = rooms.iter().rev().find(|x| !self.is_occupied(**x)).cloned();

            let target_room = match target_room {
                None => return EnumSet::empty(),
                Some(target_room) => {
                    let all_above_are_finished = rooms
                        .iter()
                        .rev()
                        .take_while(|x| **x != target_room)
                        .all(|x| self.is_finished(*x, self.get(*x).unwrap()));
                    let all_below_are_not_occupied = rooms
                        .iter()
                        .take_while(|x| **x != target_room)
                        .all(|x| !self.is_occupied(*x));

                    if !all_above_are_finished || !all_below_are_not_occupied {
                        return EnumSet::empty();
                    }
                    target_room
                }
            };

            if self.can_go_on_hallway(l, target_room) {
                target_room.into()
            } else {
                EnumSet::empty()
            }
        }
    }

    fn is_finished(&self, l: Location, a: Amphipod) -> bool {
        Self::is_in_target_room(l, a) && {
            let rooms = l.get_rooms_in_order_for_room();
            rooms
                .iter()
                .rev()
                .take_while(|x| **x != l)
                .all(|x| match self.get(*x) {
                    None => false,
                    Some(a) => Self::is_in_target_room(*x, a),
                })
        }
    }

    fn is_in_target_room(l: Location, a: Amphipod) -> bool {
        match a {
            A => matches!(l, RA1 | RA2 | RA3 | RA4),
            B => matches!(l, RB1 | RB2 | RB3 | RB4),
            C => matches!(l, RC1 | RC2 | RC3 | RC4),
            D => matches!(l, RD1 | RD2 | RD3 | RD4),
        }
    }

    fn print(&self, l: Location) -> String {
        self.get(l)
            .map(|x| format!("{:?}", x))
            .unwrap_or(".".into())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#############
#{}{}.{}.{}.{}.{}{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########
",
            self.print(AL2),
            self.print(AL1),
            self.print(AB),
            self.print(BC),
            self.print(CD),
            self.print(DR1),
            self.print(DR2),
            self.print(RA1),
            self.print(RB1),
            self.print(RC1),
            self.print(RD1),
            self.print(RA2),
            self.print(RB2),
            self.print(RC2),
            self.print(RD2),
        )
    }
}

fn solution1(input: &str) -> Result<String> {
    let state = State::new(vec![
        (RA1, D),
        (RA2, C),
        (RB1, B),
        (RB2, A),
        (RC1, D),
        (RC2, A),
        (RD1, B),
        (RD2, C),
    ]);

    let mut map = Default::default();
    Ok(format!("{}", state.solve_rec(&mut map).unwrap()))
}

fn solution2(input: &str) -> Result<String> {
    let state = State::new(vec![
        (RA1, D),
        (RA2, D),
        (RA3, D),
        (RA4, C),
        (RB1, B),
        (RB2, C),
        (RB3, B),
        (RB4, A),
        (RC1, D),
        (RC2, B),
        (RC3, A),
        (RC4, A),
        (RD1, B),
        (RD2, A),
        (RD3, C),
        (RD4, C),
    ]);

    let mut map = Default::default();
    Ok(format!("{}", state.solve_rec(&mut map).unwrap()))
}

mod tests {
    use crate::day23::Amphipod::*;
    use crate::day23::Location::*;
    use crate::day23::{solution1, solution2, Amphipod, HashMap, Location, State};
    use crate::run_solution;
    use indoc::indoc;
    use itertools::Itertools;

    const INPUT: &'static str = "day1.txt";

    // PART 1

    #[test]
    fn test_moves() {
        assert_eq!(2, State::calc_moves(RA1, AL1));
        assert_eq!(3, State::calc_moves(RA1, AL2));
        assert_eq!(6, State::calc_moves(RA4, AL2));
        assert_eq!(2, State::calc_moves(RA1, AB));
        assert_eq!(3, State::calc_moves(RA2, AB));

        assert_eq!(8, State::calc_moves(RA1, DR1));
        assert_eq!(9, State::calc_moves(RA1, DR2));

        assert_eq!(5, State::calc_moves(RC1, DR2));
        assert_eq!(6, State::calc_moves(RC1, AL1));
        assert_eq!(7, State::calc_moves(RC1, AL2));

        assert_eq!(5, State::calc_moves(AL2, RB1));
        assert_eq!(6, State::calc_moves(AL2, RB2));
        assert_eq!(3, State::calc_moves(AB, RB2));
        assert_eq!(2, State::calc_moves(AB, RA1));

        assert_eq!(4, State::calc_moves(RC1, AB));
        assert_eq!(2, State::calc_moves(RB1, BC));
        assert_eq!(2, State::calc_moves(BC, RC1));

        assert_eq!(9, State::calc_moves(DR2, RA1));

        let state = State::new(vec![
            (RA1, B),
            (RA2, A),
            (RB1, C),
            (RB2, D),
            (RC1, B),
            (RC2, C),
            (RD1, D),
            (AL2, A),
        ]);

        assert!(!state.can_go_on_hallway(RC1, AL2));
        assert!(state.can_go_on_hallway(RC1, AL1));
        assert!(state.can_go_on_hallway(RC1, AB));
        assert!(state.can_go_on_hallway(RC1, BC));
        assert!(state.can_go_on_hallway(RC1, BC));

        assert!(state.can_go_on_hallway(RC1, DR1));
        assert!(state.can_go_on_hallway(RC1, DR2));
    }

    #[test]
    fn test_part1() {
        let state = State::new(vec![
            (RA1, B),
            (RA2, A),
            (RB1, C),
            (RB2, D),
            (RC1, B),
            (RC2, C),
            (RD1, D),
            (RD2, A),
        ]);

        assert_eq!(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
",
            format!("{}", state)
        );

        assert_eq!("EnumSet()", format!("{:?}", state.moves_for(RA2)));
        assert_eq!(
            "[AL2, AL1, AB, BC, CD, DR1, DR2]",
            format!("{:?}", state.moves_for(RA1).iter().collect_vec())
        );
        assert_eq!("EnumSet()", format!("{:?}", state.moves_for(RB2)));
        assert_eq!(
            "[AL2, AL1, AB, BC, CD, DR1, DR2]",
            format!("{:?}", state.moves_for(RB1).iter().collect_vec())
        );
        assert_eq!("EnumSet()", format!("{:?}", state.moves_for(RC2)));
        assert_eq!(
            "[AL2, AL1, AB, BC, CD, DR1, DR2]",
            format!("{:?}", state.moves_for(RC1).iter().collect_vec())
        );
        assert_eq!("EnumSet()", format!("{:?}", state.moves_for(RD2)));
        assert_eq!(
            "[AL2, AL1, AB, BC, CD, DR1, DR2]",
            format!("{:?}", state.moves_for(RD1).iter().collect_vec())
        );

        let mut map: HashMap<(u32, Vec<(Location, Amphipod)>), Option<u32>> = Default::default();
        let option = state.solve_rec(&mut map);
        let min = map.values().filter_map(|x| *x).min().unwrap();
        assert_eq!(12521, min);
        assert_eq!(Some(12521), option)
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        let state = State::new(vec![
            (RA1, B),
            (RA2, D),
            (RA3, D),
            (RA4, A),
            (RB1, C),
            (RB2, C),
            (RB3, B),
            (RB4, D),
            (RC1, B),
            (RC2, B),
            (RC3, A),
            (RC4, C),
            (RD1, D),
            (RD2, A),
            (RD3, C),
            (RD4, A),
        ]);
        let mut map: HashMap<(u32, Vec<(Location, Amphipod)>), Option<u32>> = Default::default();
        assert_eq!(Some(44169), state.solve_rec(&mut map));
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
