use std::collections::HashMap;

const N_PLAYERS: u32 = 429;
const TOP_MARBLE: u32 = 70901;

fn main() {
    let mut circle: Vec<u32> = Vec::new();
    circle.push(0);

    let mut score: HashMap<u32, u32> = HashMap::new();

    let mut idx: usize = 0;
    for n in 1..=TOP_MARBLE {
        let player = ((n - 1) % N_PLAYERS) + 1;
        if n % 23 == 0 {
            idx = (idx + circle.len() - 7) % circle.len();
            *score.entry(player).or_default() += n + circle.remove(idx);
            idx = idx % circle.len();
        } else {
            idx = (idx + 2) % circle.len();
            if idx == 0 {
                idx = circle.len();
            }
            circle.insert(idx, n);
        }

        if n % (TOP_MARBLE/100) == 0 {
            println!("{}%", 100*n/TOP_MARBLE);
        }

        //println!("{}: {:?}  (current: {})", player, circle, &circle[idx]);
    }

    let mut top_scores: Vec<_> = score.values().collect();
    top_scores.sort();
    println!("Top Score: {:#?}", top_scores.pop().unwrap());
}
