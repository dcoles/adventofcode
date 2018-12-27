use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
       .expect("Failed to read input");
    let input = input.trim();

    for c in (b'a'..= b'z').map(|c| c as char) {
        let result = reduce(&input, c);
        println!("{}: {}", c, result.len());
    }
}

fn reduce(polymer: &str, ignore: char) -> String {
    let mut chars = Vec::new();
    let mut new_chars: Vec<char> = polymer.chars().collect();
    while chars.len() != new_chars.len() && new_chars.len() >= 2 {
        chars = new_chars;
        new_chars = Vec::new();
        //println!("{:?}", chars);
        let mut window_iter = chars.windows(2).peekable();
        loop {
            match window_iter.next() {
                None => break,
                Some([c1, c2]) => {
                    if ! (c1.eq_ignore_ascii_case(&c2) && (c1.is_uppercase() ^ c2.is_uppercase())) {
                        push(&mut new_chars,*c1, ignore);
                        if window_iter.peek().is_none() {
                            push(&mut new_chars,*c2, ignore);
                        }
                        continue;
                    }

                    //println!("Dropping {} and {}", c1, c2);
                    if let Some([_c2, c3]) = window_iter.next() {
                        if window_iter.peek().is_none() {
                            push(&mut new_chars,*c3, ignore);
                        }
                    }
                },
                _ => (),
            }
        }
    }

    new_chars.into_iter().collect()
}

fn push(chars: &mut Vec<char>, c: char, ignore: char) {
    if c.eq_ignore_ascii_case(&ignore) {
        return;
    }
    chars.push(c)
}
