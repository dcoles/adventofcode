const INPUT: u32 = 909441;

fn main() {
    assert_eq!(part1(9), "5158916779");
    assert_eq!(part1(5), "0124515891");
    assert_eq!(part1(18), "9251071085");
    assert_eq!(part1(2018), "5941429882");
    println!("After {} recipes, the scores of the next ten would be {}.", INPUT, part1(INPUT as usize));

    assert_eq!(part2("51589"), 9);
    assert_eq!(part2("01245"), 5);
    assert_eq!(part2("92510"), 18);
    assert_eq!(part2("59414"), 2018);
    println!("{} first appears after {} recipes.", INPUT, part2(&INPUT.to_string()));
}

fn part1(input: usize) -> String{
    let mut elves_idx: Vec<usize> = Vec::new();
    let mut board: Vec<u32> = Vec::new();

    board.extend_from_slice(&[3, 7]);
    elves_idx.extend_from_slice(&[0, 1]);

    while board.len() < 10 + input {
        let sum: u32 = elves_idx.iter().map(|&idx| board[idx]).sum();
        board.extend(&digits(sum));

        for elf_idx in &mut elves_idx {
            *elf_idx = (*elf_idx + 1 + board[*elf_idx] as usize) % board.len();
        }
    }

    board[input..input+10].iter().map(|&x| x.to_string()).collect()
}

fn part2(input: &str) -> usize {
    let mut elves_idx: Vec<usize> = Vec::new();
    let mut board: Vec<u32> = Vec::new();

    board.extend_from_slice(&[3, 7]);
    elves_idx.extend_from_slice(&[0, 1]);

    for _ in 0.. {
        let sum: u32 = elves_idx.iter().map(|&idx| board[idx]).sum();
        for digit in digits(sum) {
            board.push(digit);

            if board.len() >= input.len() {
                let board_str: String = board[board.len()-input.len()..]
                    .iter().map(|&x| x.to_string()).collect();
                if board_str == input {
                    return board.len() - input.len();
                }
            }
        }

        for elf_idx in &mut elves_idx {
            *elf_idx = (*elf_idx + 1 + board[*elf_idx] as usize) % board.len();
        }
    }
    unreachable!();
}

fn digits(n: u32) -> Vec<u32> {
    if n > 99 {
        panic!("Only supports 0 to 99");
    }

    let tens = n / 10 % 10;
    let ones = n % 10;
    let mut result = Vec::new();
    if tens != 0 {
        result.push(tens);
    }
    result.push(ones);

    result
}
