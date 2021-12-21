use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::io;
use std::io::Write;

type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Player {
        Player { position, score: 0 }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Game {
    dice: usize,
    dice_size: usize,
    board_size: usize,
    dice_rolls_count: usize,
}

impl Game {
    fn roll(&mut self, p: &mut Player) -> bool {
        let mut pos_d = 0;
        pos_d += self.dice;
        self.dice += 1;
        pos_d += self.dice;
        self.dice += 1;
        pos_d += self.dice;
        self.dice += 1;
        self.dice_rolls_count += 3;

        p.position += pos_d;
        p.position = p.position % self.board_size;

        self.dice = self.dice % self.dice_size;

        p.score += p.position + 1;

        p.score >= 1000
    }

    fn roll_game(mut g: Game, mut p: Player, turn: usize) -> (Game, Player) {
        g.dice_rolls_count += 1;

        p.position += g.dice;
        p.position = p.position % g.board_size;

        if turn == 2 || turn == 5 {
            p.score += p.position + 1;
        }

        (g, p)
    }
}

struct GameCache {
    cache: HashMap<(usize, Game, Player, Player), (u64, u64)>,
}

impl GameCache {
    fn play_rec(
        &mut self,
        game: Game,
        mut p1: Player,
        mut p2: Player,
        mut turn: usize,
    ) -> (u64, u64) {
        let cache_key = (turn, game, p1, p2);
        match self.cache.get(&cache_key) {
            None => {
                let win_at = 21;

                if turn < 3 {
                    let (game, p1_new) = Game::roll_game(game, p1, turn);
                    p1 = p1_new;
                    if p1.score >= win_at {
                        return (1, 0);
                    }
                } else {
                    let (game, p2_new) = Game::roll_game(game, p2, turn);
                    p2 = p2_new;
                    if p2.score >= win_at {
                        return (0, 1);
                    }
                }

                turn = turn + 1;
                turn = turn % 6;

                let mut p1_cnt = 0;
                let mut p2_cnt = 0;
                for x in 1..=3 {
                    let mut game = game.clone();
                    game.dice = x;
                    let r = self.play_rec(game, p1, p2, turn);
                    p1_cnt += r.0;
                    p2_cnt += r.1;
                }
                self.cache.insert(cache_key, (p1_cnt, p2_cnt));
                (p1_cnt, p2_cnt)
            }
            Some(x) => *x,
        }
    }
}

fn solution1(input: &str) -> Result<String> {
    let mut p1 = Player::new(8 - 1);
    let mut p2 = Player::new(7 - 1);
    let mut game = Game {
        dice: 1,
        dice_size: 100,
        board_size: 10,
        dice_rolls_count: 0,
    };

    let p = &mut p1;
    loop {
        if game.roll(&mut p1) || game.roll(&mut p2) {
            break;
        }
        dbg!(
            game.dice,
            p1.position + 1,
            p1.score,
            p2.position + 1,
            p2.score
        );
    }

    Ok(format!(
        "{}",
        p1.score.min(p2.score) * game.dice_rolls_count
    ))
}

fn solution2(input: &str) -> Result<String> {
    let p1 = Player::new(8 - 1);
    let p2 = Player::new(7 - 1);

    let mut r = (0, 0);
    for x in 1..=3 {
        let game = Game {
            dice: x,
            dice_size: 3,
            board_size: 10,
            dice_rolls_count: 0,
        };

        let mut cache = GameCache {
            cache: Default::default(),
        };
        let rr = cache.play_rec(game, p1, p2, 0);
        r.0 += rr.0;
        r.1 += rr.1;
    }

    Ok(format!("{}", r.0.max(r.1)))
}

mod tests {
    use crate::day21::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;

    const INPUT: &'static str = "dayXX.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("739785", solution1(indoc!("")).unwrap());
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
