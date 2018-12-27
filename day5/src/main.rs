use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
       .expect("Failed to read input");
    let input = input.trim();

    let result = reduce(&input, None).len();
    println!("Number of units: {}", result);
    assert_eq!(result, 10450);

    for c in (b'a'..= b'z').map(|c| c as char) {
        let result = reduce(&input, Some(c));
        println!("{}: {}", c, result.len());
    }
}

fn reduce(input: &str, ignore: Option<char>) -> String {
    let mut chars = Vec::new();
    let mut new_chars: Vec<char> = input.chars().collect();

    // While we still can reduce the input
    while chars.len() != new_chars.len() {
        chars = new_chars;
        new_chars = Vec::new();
        let mut chars_iter = chars.iter().peekable();
        while let Some(c1) = chars_iter.next() {
            if let Some(&c2) = chars_iter.peek() {
                if is_opposite(*c1, *c2) {
                    // Drop this character and the next
                    chars_iter.next();
                    continue;
                }
            }

            if let Some(ignore) = ignore {
                if c1.eq_ignore_ascii_case(&ignore) {
                    continue;
                }
            }

            new_chars.push(*c1);
        }
    }

    // Return as String
    new_chars.into_iter().collect()
}

fn is_opposite(c1: char, c2: char) -> bool {
    c1.eq_ignore_ascii_case(&c2) && (c1.is_uppercase() ^ c2.is_uppercase())
}
