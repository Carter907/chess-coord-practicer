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
    let mut team = String::new();
    loop {
        let picked_board = pick_square(board.clone(), &mut rng);
        print_board(picked_board, &mut rng, &mut team);

        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("you failed to read the line");
        if is_guess_correct(&answer, &picked_board, &team) {
            let msg = "correct!".blue();
            println!("{msg}");
        } else {
            let msg = "false".red();
            println!("{msg}");
        }
    }
}

fn is_guess_correct(guess: &String, board: &[[char; 8]; 8], team: &String) -> bool {
    let rank = 8 - guess.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
    let file = (guess.chars().nth(0).unwrap() as i32 - 97) as usize;
    println!("file: {} rank {}", file, rank);
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == TARGET {
                println!("{}{}", j, i);
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

fn print_board(board: [[char; 8]; 8], rng: &mut ThreadRng, team: &mut String) {
    let teams: [&str; 2] =
        if rng.gen::<f32>() > 0.5 {
            ["white", "black"]
        } else { ["black", "white"] };
    team.clear();
    team.push_str(teams[1]);
    println!("{}", teams[0]);
    for rank in board {
        for square in rank {
            print!("{}", square)
        }
        println!()
    }
    println!("{}", teams[1])
}