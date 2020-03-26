use std::io;
use std::io::Write;

fn check_board(&board: &[char; 9]) -> bool {
    // Diagonal
    if (board[4] == board[0] && board[4] == board[8]
        || board[4] == board[2] && board[4] == board[6])
        && board[4] != ' '
    {
        return true;
    }

    // Vertical
    if ((board[0] == board[3] && board[0] == board[6]) && board[0] != ' ')
        || ((board[1] == board[4] && board[1] == board[7]) && board[1] != ' ')
        || ((board[2] == board[5] && board[2] == board[8]) && board[2] != ' ')
    {
        return true;
    }

    // Horizonal
    if board[0..3].iter().all(|&i| i == board[0]) && board[0] != ' '
        || board[3..6].iter().all(|&i| i == board[3]) && board[3] != ' '
        || board[6..9].iter().all(|&i| i == board[6]) && board[6] != ' '
    {
        return true;
    }

    false
}

fn main() {
    let mut board: [char; 9] = [' '; 9];
    let mut current_turn = 1;
    let fire: char = '🔥';
    let water: char = '💧';
    let valid_cells = 1..10;

    loop {
        print!("\nPlease input your movement: ");
        io::stdout().flush().unwrap();

        let mut movement = String::new();

        io::stdin()
            .read_line(&mut movement)
            .expect("Failed to read line!");

        let movement: usize = match movement.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };

        match valid_cells.contains(&movement) {
            true => {
                board[movement - 1] = if current_turn % 2 == 0 { fire } else { water };
            }
            _ => {
                println!("That's not a valid cell!");
                continue;
            }
        }

        for (i, x) in board.iter().enumerate() {
            print!("| {}  ", x);
            if (i + 1) % 3 == 0 {
                println!("|\n");
            }
        }

        if check_board(&board) {
            println!(
                "Game over! {}  wins!",
                if current_turn % 2 == 0 { fire } else { water }
            );
            break;
        }

        if !board.contains(&' ') {
            println!("Game over! It's a tie!");
            break;
        }

        current_turn += 1;
    }
}
