use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    println!("GOAL: type the coordinate of the square selected");
    let mut board: [[char; 8]; 8] = [[' '; 8]; 8];
    init_board(&mut board);
    pick_square(&mut board, &mut rng);
    print_board(board, &mut rng);
}

fn pick_square(board: &mut [[char; 8]; 8], rng: &mut ThreadRng) {
    let rank = rng.gen_range(0..8);
    let file = rng.gen_range(0..8);

    board[rank][file] = '\u{1F3AF}';
}

fn init_board(board: &mut [[char; 8]; 8]) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j] = if (i as i32 + j as i32) % 2 == 1 { '\u{2B1B}' } else { '\u{2B1C}' }
        }
    }
}

fn print_board(board: [[char; 8]; 8], rng: &mut ThreadRng) {
    let teams: [&str; 2] =
        if rng.gen::<f32>() > 0.5 {
            ["white", "black"]
        } else { ["black", "white"] };
    println!("{}", teams[0]);
    for rank in board {
        for square in rank {
            print!("{}", square)
        }
        println!()
    }
    println!("{}", teams[1])
}