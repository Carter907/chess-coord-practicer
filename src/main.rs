use rand::prelude::*;
use colored::*;

const TARGET: char = '\u{1F3AF}';
const BLACK_SQUARE: char = '\u{2B1B}';
const WHITE_SQUARE: char = '\u{2B1C}';


fn main() {
    let mut rng = thread_rng();
    println!("GOAL: type the coordinate of the square selected");
    let mut board: [[char; 8]; 8] = [[' '; 8]; 8];
    init_board(&mut board);
    loop {
        let picked_board = pick_square(board.clone(), &mut rng);
        print_board(picked_board);

        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("you failed to read the line");
        answer = answer.trim().to_string();
        if answer == "exit" {
            std::process::exit(0);
        }
        if is_guess_correct(&answer, &picked_board) {
            let msg = "correct!".blue();
            println!("{msg}");
        } else {
            let msg = "false".red();
            println!("{msg}");
        }
    }
}

fn is_guess_correct(guess: &String, board: &[[char; 8]; 8]) -> bool {
    let rank = 8 - guess.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
    let file = (guess.chars().nth(0).unwrap() as i32 - 97) as usize;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == TARGET {
            }
        }
    }

    board[rank][file] == TARGET
}

fn pick_square(mut board: [[char; 8]; 8], rng: &mut ThreadRng) -> [[char; 8]; 8] {
    let rank = rng.gen_range(0..8);
    let file = rng.gen_range(0..8);

    board[rank][file] = TARGET;

    board
}

fn init_board(board: &mut [[char; 8]; 8]) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j] = if (i as i32 + j as i32) % 2 == 1 { BLACK_SQUARE } else { WHITE_SQUARE }
        }
    }
}

fn print_board(board: [[char; 8]; 8]) {

    for rank in board {
        for square in rank {
            print!("{}", square)
        }
        println!()
    }
}