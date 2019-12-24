use std::path::Path;
use std::fs;

type Signed = i128;
type Unsigned = u128;
type Vector2<T> = [T; 2];

const DEAL_NEW_STACK: &str = "deal into new stack";
const CUT: &str = "cut ";
const DEAL_WITH_INCREMENT: &str = "deal with increment ";
const N_CARDS: Unsigned = 10007;
const M_CARDS: Unsigned = 119315717514047;
const M_SHUFFLES: Unsigned = 101741582076661;

fn main() {
    let techniques = read_input("input.txt");

    // Part 1
    let pos = 2019;
    for c in 0..M_CARDS {
        if Shuffle::from_techniques(&techniques, N_CARDS).evaluate(c) == 2019 {
            println!("Part 1: Position of card {}: {}", pos, c);
            break;
        }
    }

    // Part 2
    let pos = 2020;
    let card = Shuffle::from_techniques(&techniques, M_CARDS).repeat(M_SHUFFLES - 1).evaluate(pos);
    println!("Part 2: After shuffling {} cards {} times, the card at position {} is: {}", M_CARDS, M_SHUFFLES, pos, card);
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

/// Represents a shuffle as a linear equation `v`, modulo `n_cards`
/// Formula: `ax + b (mod m)`; where `v = [a, b]`
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Shuffle {
    v: Vector2<Mod>,
}

impl Shuffle {
    fn from_techniques(techniques: &[Technique], n_cards: Unsigned) -> Self {
        // Since we're trying to find card `Cx` at position `x`,
        // we need to follow it backwards through the shuffling techniques
        let mut v = [Mod::new(1, n_cards), Mod::new(0, n_cards)];
        for n in (0..techniques.len()).rev() {
            let technique = techniques[n];

            use Technique::*;
            match technique {
                DealNewStack => {
                    v = Shuffle::undeal_new_stack(v);
                },
                Cut(n) => {
                    v = Shuffle::uncut(n, v);
                },
                DealWithIncrement(n) => {
                    v = Shuffle::undeal_with_increment(n, v);
                },
            }
        }
        Shuffle { v }
    }

    /// Find the card at `pos`, given a `shuffle`
    fn evaluate(&self, pos: Unsigned) -> Unsigned {
        // Formula: ax + b (mod m);  where v = [a, b]
        (self.v[0] * pos + self.v[1]).into()
    }

    /// Repeat thus shuffle `n` times
    fn repeat(&self, n: Unsigned) -> Shuffle {
        // Calculate the powers-of-2 repeats less than `n`
        let mut powers = vec![*self];
        while 1 << (powers.len() as Unsigned - 1) < n {
            // Apply the currently largest shuffle to itself
            let shuffle = powers[powers.len() - 1];
            powers.push(shuffle.apply(shuffle));
        }

        let mut shuffle = *self;
        let mut n = n;
        for (i, &power) in powers.iter().enumerate().rev() {
            let p = 1 << i as Unsigned;
            while n >= p {
                shuffle = power.apply(shuffle);
                n -= p;
            }
        }

        shuffle
    }

    /// Apply this shuffle to `other` shuffle`
    fn apply(&self, other: Shuffle) -> Shuffle {
        // Substitute `x = cx + d` into `ax + b` => `a(cx + d) + b = acx + ad + b`
        let [a, b] = self.v;
        let [c, d] = other.v;
        let v = [a * c, a * d + b];

        Shuffle { v }
    }

    /// Undo cut
    fn uncut(n: Signed, [a, b]: Vector2<Mod>) -> Vector2<Mod> {
        // i: 0  1  2  3  4  5  6  7  8  9
        // | c1 c2 c3 c4 c5 c6 c7*c8 c9 c0 |  Cut 1
        // | c0 c1 c2 c3 c4 c5 c6 c7*c8 c9 |
        //
        // Formula: x + n (mod m)
        [a, b + n]
    }

    /// Undo deal with increment
    fn undeal_with_increment(n: Unsigned, [a, b]: Vector2<Mod>) -> Vector2<Mod> {
        // i: 0  1  2  3  4  5  6  7  8  9
        // | c0 c7*c4 c1 c8 c5 c2 c9 c6 c3 |  Deal with increment 3
        // | c0 c1 c2 c3 c4 c5 c6 c7*c8 c9 |
        //
        // This requires us to find the modular multiplicative inverse
        // Note: These multiplications may result in some *very* big numbers
        //
        // Formula: n⁻¹x (mod m)
        let n = Mod::new(n, a.m);
        let ninv = n.inverse();
        [ninv * a, ninv * b]
    }

    /// Undo deal new stack
    fn undeal_new_stack([a, b]: Vector2<Mod>) -> Vector2<Mod> {
        // i: 0  1  2  3  4  5  6  7  8  9
        // | c9 c8 c7*c6 c5 c4 c3 c2 c1 c0 |  Deal new stack
        // | c0 c1 c2 c3 c4 c5 c6 c7*c8 c9 |
        //
        // Formula: -(x + 1) (mod m)
        [-a, -(b + 1u128)]
    }
}

/// Modular arithmetic
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Mod {
    a: Unsigned,
    m: Unsigned,
}

impl Mod {
    /// New `a (mod m)`
    fn new(a: Unsigned, m: Unsigned) -> Self {
        Mod { a: a % m, m }
    }

    /// Multiplicative inverse via extended Euclidean algorithm
    /// see: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
    fn inverse(self) -> Mod {
        assert_ne!(self.a, 0, "Zero has no multiplicative inverse");

        let mut a = self.a as Signed;
        let mut m = self.m as Signed;
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

        Mod { a: modulo(x, self.m), m: self.m }
    }
}

impl std::ops::Add<Mod> for Mod {
    type Output = Mod;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.m, rhs.m, "Must have same modulus");
        Mod { a: (self.a + rhs.a) % self.m, m: self.m }
    }
}

impl std::ops::Add<Unsigned> for Mod {
    type Output = Mod;

    fn add(self, rhs: Unsigned) -> Self::Output {
        Mod { a: (self.a + rhs) % self.m, m: self.m }
    }
}

impl std::ops::Add<Signed> for Mod {
    type Output = Mod;

    fn add(self, rhs: Signed) -> Self::Output {
        Mod { a: modulo(self.a as Signed + rhs, self.m), m: self.m }
    }
}

impl std::ops::Mul<Mod> for Mod {
    type Output = Mod;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.m, rhs.m, "Must have same modulus");
        Mod { a: (self.a * rhs.a) % self.m, m: self.m }
    }
}

impl std::ops::Mul<Unsigned> for Mod {
    type Output = Mod;

    fn mul(self, rhs: Unsigned) -> Self::Output {
        Mod { a: (self.a * rhs) % self.m, m: self.m }
    }
}

impl std::ops::Neg for Mod {
    type Output = Mod;

    fn neg(self) -> Self::Output {
        Mod { a: modulo(-(self.a as Signed), self.m), m: self.m }
    }
}

impl Into<Signed> for Mod {
    fn into(self) -> Signed {
        self.a as Signed
    }
}

impl Into<Unsigned> for Mod {
    fn into(self) -> Unsigned {
        self.a
    }
}

/// Calculate `x mod m`
/// The `%` operator in Rust is remainder, thus behaves differently on negative numbers
fn modulo(a: Signed, m: Unsigned) -> Unsigned {
    (m as Signed + a % m as Signed) as Unsigned % m
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

        let shuffle = Shuffle::from_techniques(&[DealNewStack], n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    #[test]
    fn test_cut1() {
        let shuffled = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&[Cut(1)], n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    #[test]
    fn test_cut3() {
        let shuffled = &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&[Cut(3)], n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    #[test]
    fn test_negativecut1() {
        let shuffled = &[9, 0, 1, 2, 3, 4, 5, 6, 7, 8];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&[Cut(-1)], n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    #[test]
    fn test_negativecut4() {
        let shuffled = &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&[Cut(-4)], n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    #[test]
    fn test_dealwithincrement1() {
        let shuffled = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&[DealWithIncrement(1)], n_cards);
        for i in 0..10 {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    #[test]
    fn test_dealwithincrement3() {
        let shuffled = &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&[DealWithIncrement(3)], n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    const SAMPLE_1: [Technique; 3] = [DealWithIncrement(7), DealNewStack, DealNewStack];

    #[test]
    fn test_sample1() {
        let shuffled = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&SAMPLE_1, n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    const SAMPLE_2: [Technique; 3] = [Cut(6), DealWithIncrement(7), DealNewStack];

    #[test]
    fn test_sample2() {
        let shuffled = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        let n_cards = shuffled.len() as Unsigned;

        let shuffle = Shuffle::from_techniques(&SAMPLE_2, n_cards);
        for i in 0..n_cards {
            assert_eq!(shuffle.evaluate(i), shuffled[i as usize]);
        }
    }

    const SAMPLE_4: [Technique; 10] = [DealNewStack, Cut(-1), DealWithIncrement(7), Cut(8), Cut(-4), DealWithIncrement(7), Cut(3), DealWithIncrement(9), DealWithIncrement(3), Cut(-1)];

    #[test]
    fn test_identities() {
        let n_cards = 10u128;
        for i in 0..n_cards {
            // Cut n
            let val: Unsigned = (Mod::new(7, n_cards) + i).into();
            assert_eq!(val, Shuffle::from_techniques(&[Cut(7)], n_cards).evaluate(i));

            // DealWithIncrement m
            let val: Unsigned = (Mod::new(7, n_cards).inverse() * i).into();
            assert_eq!(val, Shuffle::from_techniques(&[DealWithIncrement(7)], n_cards).evaluate(i));

            // DealNewStack
            let val: Unsigned = (-(Mod::new(1, n_cards) + i)).into();
            assert_eq!(val, Shuffle::from_techniques(&[DealNewStack], n_cards).evaluate(i));
        }
    }

    #[test]
    fn test_repeat0() {
        assert_eq!(Shuffle::from_techniques(&SAMPLE_4, M_CARDS),
                   Shuffle::from_techniques(&SAMPLE_4, M_CARDS).repeat(0));
    }

    #[test]
    fn test_repeat1() {
        let repeated: Vec<_> = SAMPLE_4.iter().copied().cycle().take(2 * SAMPLE_4.len()).collect();  // Repeated once

        assert_eq!(Shuffle::from_techniques(&repeated, M_CARDS),
                   Shuffle::from_techniques(&SAMPLE_4, M_CARDS)
                       .apply(Shuffle::from_techniques(&SAMPLE_4, M_CARDS)));

        assert_eq!(Shuffle::from_techniques(&repeated, M_CARDS),
                   Shuffle::from_techniques(&SAMPLE_4, M_CARDS).repeat(1));
    }

    #[test]
    fn test_repeat4() {
        let repeated: Vec<_> = SAMPLE_4.iter().copied().cycle().take(5 * SAMPLE_4.len()).collect();  // Repeated 4 times

        assert_eq!(Shuffle::from_techniques(&repeated, M_CARDS),
                   Shuffle::from_techniques(&SAMPLE_4, M_CARDS).repeat(4));
    }
}
