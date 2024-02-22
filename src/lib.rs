use std::io;
use ansi_escapes::*;

pub fn choose_player() -> char {
    loop {
    
        println!("\nPlayer 1: Choose X or O");
        let mut player1 = String::new();
        
        io::stdin().read_line(&mut player1)
                   .expect("Failed to read line.");
     
        let player1: char = match player1.trim().parse() {
            Ok(char) => char,
            Err(_) => { print!("\r{}", EraseScreen); // Clears the input line too!
                        print!("{}{}", CursorUp(3), EraseLine);
                        print!("Not valid. Not a single character.");  
                        continue 
            }
        };
        
        let player1 = match player1 {
            'x' => 'X',
            'X' => 'X',
            'o' => 'O',
            'O' => 'O',
            _ => {  print!("\r{}", EraseScreen);
                    print!("{}{}", CursorUp(3), EraseLine);
                    print!("\rNot valid. Not X or O.");
                    continue 
            }
        };
        break player1
    }
}

pub fn print_player_turn(current_player: char, player1: char, player2: char) {
    if current_player == player1 {
            println!("{}Player 1's turn.", EraseLine);           
    } else if current_player == player2 {
            println!("{}Player 2's turn.", EraseLine);
    } else {
            println!("NeW PhOne!? WHo DiS?");
    }
}

pub fn draw_grid(matrix: [char; 9]) {
        println!("");   
        println!(" {} │ {} │ {} ", matrix[6], matrix[7], matrix[8]);
        println!("───┼───┼───");
        println!(" {} │ {} │ {} ", matrix[3], matrix[4], matrix[5]);
        println!("───┼───┼───");
        println!(" {} │ {} │ {} ", matrix[0], matrix[1], matrix[2]);
        println!("");
}

pub fn switch_player(current_player: char) -> char {
    if current_player == 'X' {
        'O'
    } else {
        'X'
    }
}