use std::collections::VecDeque;

const INPUT: usize = 909441;

fn main() {
    let mut elves_idx: Vec<usize> = Vec::new();
    let mut board: Vec<u32> = Vec::new();

    board.extend_from_slice(&[3, 7]);
    elves_idx.extend_from_slice(&[0, 1]);

    while board.len() < 10 + INPUT {
        let mut sum: u32 = elves_idx.iter().map(|&idx| board[idx]).sum();
        let mut digits = VecDeque::new();
        loop {
            digits.push_front(sum % 10);
            sum /= 10;
            if sum == 0 {
                break;
            }
        }
        board.extend(&digits);
        //println!("{:?}", board);

        for elf_idx in &mut elves_idx {
            *elf_idx = (*elf_idx + 1 + board[*elf_idx] as usize) % board.len();
        }
    }

    let digits: String = board[INPUT..INPUT+10].iter().map(|&x| x.to_string()).collect();
    println!("Digits: {}", digits);
}
