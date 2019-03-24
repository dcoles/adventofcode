use std::collections::HashMap;

const INPUT: i32 = 277678;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut state = State::RIGHT;
    let mut m = 2;
    let mut p = 1;
    loop {
        for _ in 0..m/2 {
            if p >= INPUT {
                return x.abs() + y.abs();
            }

            match &state {
                State::RIGHT => x += 1,
                State::UP => y += 1,
                State::LEFT => x -= 1,
                State::DOWN => y -= 1,
            };
            p += 1;
        }

        m += 1;
        state = state.next();
    }
}

fn part2() -> i32 {
    let mut mem = HashMap::new();
    mem.insert((0, 0), 1);

    let mut x = 0i32;
    let mut y = 0i32;
    let mut state = State::RIGHT;
    let mut m = 2;
    loop {
        for _ in 0..m/2 {
            match &state {
                State::RIGHT => x += 1,
                State::UP => y += 1,
                State::LEFT => x -= 1,
                State::DOWN => y -= 1,
            };
            let sum = sum_adj(&mem, (x, y));
            if sum > INPUT {
                return sum;
            }
            mem.insert((x, y), sum);
        }

        m += 1;
        state = state.next();
    }
}

fn sum_adj(mem: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> i32 {
    let mut sum = 0;

    for offset in &[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
        let adj = (pos.0 + offset.0, pos.1 + offset.1);
        sum += mem.get(&adj).unwrap_or(&0);
    }

    sum
}

enum State {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

impl State {
    fn next(&self) -> State {
        match self {
            State::RIGHT => State::UP,
            State::UP => State::LEFT,
            State::LEFT => State::DOWN,
            State::DOWN => State::RIGHT,
        }
    }
}
