use std::path::Path;
use std::fs;
use std::convert::TryFrom;

type Signed = i128;
type Unsigned = u128;
type Vector = [Unsigned; 2];

const DEAL_NEW_STACK: &str = "deal into new stack";
const CUT: &str = "cut ";
const DEAL_WITH_INCREMENT: &str = "deal with increment ";
const N_CARDS: Unsigned = 10007;
const M_CARDS: Unsigned = 119315717514047;
const N_SHUFFLES: Unsigned = 101741582076661;
const X: Vector = [1, 0];

fn main() {
    let techniques = read_input("input.txt");

    // Part 1
    let pos = 2019;
    for c in 0..M_CARDS {
        if evaluate(shuffle(&techniques, N_CARDS), c, N_CARDS) == 2019 {
            println!("Part 1: Position of card {}: {}", pos, c);
            break;
        }
    }

    // Part 2
    let pos = 2020;
    let card = evaluate(repeat(shuffle(&techniques, M_CARDS), N_SHUFFLES - 1, M_CARDS), pos, M_CARDS);
    println!("Part 2: After shuffling {} cards {} times, the card at position {} is: {}", M_CARDS, N_SHUFFLES, pos, card);
}

/// Find the card at `pos`, given a `shuffle`
fn evaluate(shuffle: Vector, pos: Unsigned, n_cards: Unsigned) -> Unsigned {
    // Formula: ax + b;  where v = [a, b]
    (shuffle[0] * pos + shuffle[1]) % n_cards
}

/// Repeat a shuffle `v`, `n` times.
fn repeat(shuffle: Vector, n: Unsigned, n_cards: Unsigned) -> Vector {
    // Calculate the powers-of-2 repeats less than `n`
    let mut powers = vec![shuffle];
    while 1 << (powers.len() as Unsigned - 1) < n {
        let v = powers[powers.len() - 1];
        powers.push(apply(v, v, n_cards));
    }

    let mut v = shuffle;
    let mut n = n;
    for (i, &power) in powers.iter().enumerate().rev() {
        let p = 1 << i as Unsigned;
        while n >= p {
            v = apply(power, v, n_cards);
            n -= p;
        }
    }

    v
}

/// Apply shuffle `[a, b]` to shuffle `[c, d]`
fn apply([a, b]: Vector, [c, d]: Vector, n_cards: Unsigned) -> Vector {
    // Substitute `x = cx + d` into `ax + b` => `a(cx + d) + b = acx + ad + b`
    [(a * c) % n_cards, (a * d + b) % n_cards]
}

/// Shuffle `n_cards` using `techniques`
fn shuffle(techniques: &[Technique], n_cards: Unsigned) -> Vector {
    shuffle_(X, techniques, n_cards)
}

/// Shuffle `n_cards` using `techniques` on an existing `shuffle`
fn shuffle_(mut shuffle: Vector, techniques: &[Technique], n_cards: Unsigned) -> Vector {
    // Since we're trying to find card `X` at position `x`,
    // we need to follow it backwards through the shuffling techniques
    for n in (0..techniques.len()).rev() {
        let technique = techniques[n];

        use Technique::*;
        match technique {
            DealNewStack => {
                shuffle = undeal_new_stack(shuffle, n_cards);
            },
            Cut(n) => {
                shuffle = uncut(n, shuffle, n_cards);
            },
            DealWithIncrement(n) => {
                shuffle = undeal_with_increment(n, shuffle, n_cards);
            },
        }
    }

    shuffle
}

/// Undo cut
fn uncut(n: Signed, v: Vector, n_cards: Unsigned) -> Vector {
    // i: 0  1  2  3  4  5  6  7  8  9
    // | c1 c2 c3 c4 c5 c6 c7*c8 c9 c0 |  Cut 1
    // | c0 c1 c2 c3 c4 c5 c6 c7*c8 c9 |
    //
    // Formula: x + n mod m
    [modulo(v[0], n_cards), modulo(v[1] as Signed + n as Signed, n_cards)]
}

/// Undo deal with increment
fn undeal_with_increment(n: Unsigned, v: Vector, n_cards: Unsigned) -> Vector {
    // i: 0  1  2  3  4  5  6  7  8  9
    // | c0 c7*c4 c1 c8 c5 c2 c9 c6 c3 |  Deal with increment 3
    // | c0 c1 c2 c3 c4 c5 c6 c7*c8 c9 |
    //
    // This requires us to find the modular multiplicative inverse
    // Formula: n⁻¹x mod m
    let ninv = inverse(n, n_cards);
    [modulo(ninv * v[0], n_cards), modulo(ninv * v[1], n_cards)]
}

/// Undo deal new stack
fn undeal_new_stack(v: Vector, n_cards: Unsigned) -> Vector {
    // i: 0  1  2  3  4  5  6  7  8  9
    // | c9 c8 c7*c6 c5 c4 c3 c2 c1 c0 |  Deal new stack
    // | c0 c1 c2 c3 c4 c5 c6 c7*c8 c9 |
    //
    // Formula: -(x + 1) mod m
    [modulo(-(v[0] as Signed), n_cards), modulo(-(v[1] as Signed) - 1, n_cards)]
}

/// Calculate `x mod m`
/// (In Rust `%` is the `rem` operator, so behaves differently on negative numbers`)
fn modulo<T>(x: T, m: Unsigned) -> Unsigned
    where T: std::ops::Add<Output=T> + Copy + Clone,
          T: std::convert::TryFrom<Unsigned>,
          <T as std::convert::TryFrom<Unsigned>>::Error: std::fmt::Debug,
          Unsigned: std::convert::TryFrom<T>,
          <Unsigned as std::convert::TryFrom<T>>::Error: std::fmt::Debug
{
    Unsigned::try_from(T::try_from(m).unwrap() + x).unwrap() % m
}

/// Inverse via extended Euclidean algorithm
/// see: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
fn inverse(a: Unsigned, m: Unsigned) -> Unsigned {
    let m0 = m;
    let mut a = a as Signed;
    let mut m = m as Signed;
    let mut y = 0;
    let mut x = 1;

    while a > 1 {
        let q = a / m;

        let t = m;

        m = a % m;
        a = t;
        let t = y;

        y = x - q * y;
        x = t;
    }

    (m0 as Signed + x) as Unsigned % m0
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Technique> {
    let contents = fs::read_to_string(path).expect("Failed to read input");

    let mut techniques = Vec::new();
    for line in contents.lines() {
        let technique = match line {
            DEAL_NEW_STACK => Technique::DealNewStack,
            line if line.starts_with(CUT) => {
                let n = line[CUT.len()..].parse().expect("Failed to parse value");

                Technique::Cut(n)
            },
            line if line.starts_with(DEAL_WITH_INCREMENT) => {
                let n = line[DEAL_WITH_INCREMENT.len()..].parse().expect("Failed to parse value");

                Technique::DealWithIncrement(n)
            }
            line => panic!("Unknown line: {}", line),
        };

        techniques.push(technique);
    }

    techniques
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Technique {
    DealNewStack,
    Cut(Signed),
    DealWithIncrement(Unsigned),
}

mod tests {
    use super::*;
    use Technique::*;

    #[test]
    fn test_shuffle_dealnewstack() {
        let shuffled = &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let n_cards = shuffled.len() as Unsigned;

        let v = shuffle(&[DealNewStack], n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    #[test]
    fn test_cut1() {
        let shuffled = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let n_cards = shuffled.len() as Unsigned;

        let v = shuffle(&[Cut(1)], n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    #[test]
    fn test_cut3() {
        let shuffled = &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let n_cards = shuffled.len() as Unsigned;

        let v= shuffle(&[Cut(3)], n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    #[test]
    fn test_negativecut1() {
        let shuffled = &[9, 0, 1, 2, 3, 4, 5, 6, 7, 8];
        let n_cards = shuffled.len() as Unsigned;

        let v= shuffle(&[Cut(-1)], n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    #[test]
    fn test_negativecut4() {
        let deck = &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        let n_cards = deck.len() as Unsigned;

        let v= shuffle(&[Cut(-4)], n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(deck[i as usize], pos);
        }
    }

    #[test]
    fn test_dealwithincrement1() {
        let shuffled = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let n_cards = shuffled.len() as Unsigned;

        let v= shuffle(&[DealWithIncrement(1)], n_cards);
        for i in 0..10 {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    #[test]
    fn test_dealwithincrement3() {
        let shuffled = &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        let n_cards = shuffled.len() as Unsigned;

        let v= shuffle(&[DealWithIncrement(3)], n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    const SAMPLE_1: [Technique; 3] = [DealWithIncrement(7), DealNewStack, DealNewStack];

    #[test]
    fn test_sample1() {
        let shuffled = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        let n_cards = shuffled.len() as Unsigned;

        let v= shuffle(&SAMPLE_1, n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    const SAMPLE_2: [Technique; 3] = [Cut(6), DealWithIncrement(7), DealNewStack];

    #[test]
    fn test_sample2() {
        let shuffled = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        let n_cards = shuffled.len() as Unsigned;

        let v= shuffle(&SAMPLE_2, n_cards);
        for i in 0..n_cards {
            let pos = evaluate(v, i, n_cards);
            assert_eq!(shuffled[i as usize], pos);
        }
    }

    const SAMPLE_4: [Technique; 10] = [DealNewStack, Cut(-1), DealWithIncrement(7), Cut(8), Cut(-4), DealWithIncrement(7), Cut(3), DealWithIncrement(9), DealWithIncrement(3), Cut(-1)];

    #[test]
    fn test_identities() {
        let n_cards = 10;
        for i in 0..10 {
            // Cut n
            assert_eq!((i + 7) % n_cards, evaluate(shuffle(&[Cut(7)], n_cards), i, n_cards));
            assert_eq!(evaluate(uncut(7, X, n_cards), i, n_cards),
                       evaluate(shuffle(&[Cut(7)], n_cards), i, n_cards));

            // DealWithIncrement m
            assert_eq!((inverse(7, n_cards) * i) % n_cards, evaluate(shuffle(&[DealWithIncrement(7)], n_cards), i, n_cards));
            assert_eq!(evaluate(undeal_with_increment(7, X, n_cards), i, n_cards),
                       evaluate(shuffle(&[DealWithIncrement(7)], n_cards), i, n_cards));

            // DealNewStack
            assert_eq!((n_cards - i - 1) % n_cards, evaluate(shuffle(&[DealNewStack], n_cards), i, n_cards));
            assert_eq!(evaluate(undeal_new_stack(X, n_cards), i, n_cards),
                       evaluate(shuffle(&[DealNewStack], n_cards), i, n_cards));
        }
    }

    #[test]
    fn test_repeat0() {
        assert_eq!(shuffle(&SAMPLE_4, M_CARDS),
                   repeat(shuffle(&SAMPLE_4, M_CARDS), 0, M_CARDS));
    }

    #[test]
    fn test_repeat1() {
        let repeated: Vec<_> = SAMPLE_4.iter().copied().cycle().take(2 * SAMPLE_4.len()).collect();  // Repeated once

        assert_eq!(shuffle(&repeated, M_CARDS),
                   shuffle_(shuffle(&SAMPLE_4, M_CARDS), &SAMPLE_4, M_CARDS));

        assert_eq!(shuffle(&repeated, M_CARDS),
                   repeat(shuffle(&SAMPLE_4, M_CARDS), 1, M_CARDS));
    }

    #[test]
    fn test_repeat4() {
        let repeated: Vec<_> = SAMPLE_4.iter().copied().cycle().take(5 * SAMPLE_4.len()).collect();  // Repeated 4 times

        assert_eq!(shuffle(&repeated, M_CARDS),
                   repeat(shuffle(&SAMPLE_4, M_CARDS), 4, M_CARDS));
    }
}
