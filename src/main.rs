use std::io;
use ansi_escapes::*;

use tic_tac_toe::*;

fn main() {
    print!("{}", EraseScreen);
    println!("Let's play Tic-Tac-Toe!");
    let mut matrix = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut section;
    let mut move_counter = 0;
    let player1 = choose_player();
    let player2 = if player1 == 'X' {
        'O'
    } else {
        'X'
    };

    println!("Player 1 is {}.", player1);
    println!("Player 2 is {}.", player2);

    let mut current_player = player1;
    println!("Grid sections are labeled like a numberpad:");
    println!("");
    println!(" 7 │ 8 │ 9 ");
    println!("───┼───┼───");
    println!(" 4 │ 5 │ 6 ");
    println!("───┼───┼───");
    println!(" 1 │ 2 │ 3 ");       
    println!("              ");
    println!("              ");

    loop {
        draw_grid(matrix);
        if move_counter == 9 {
            println!("{}It's a tie!", EraseLine);   
            println!("{}", EraseLine);         
            break
        }
        print_player_turn(current_player, player1, player2);
        println!("Enter move as a number: ");
        // Player chooses a move.
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");   

        // match if user input is a valid choice.
        section = match input.trim().parse() {
            Ok(1) if matrix[0] == ' ' => 0,
            Ok(2) if matrix[1] == ' ' => 1,
            Ok(3) if matrix[2] == ' ' => 2,
            Ok(4) if matrix[3] == ' ' => 3,
            Ok(5) if matrix[4] == ' ' => 4,
            Ok(6) if matrix[5] == ' ' => 5,
            Ok(7) if matrix[6] == ' ' => 6,
            Ok(8) if matrix[7] == ' ' => 7,
            Ok(9) if matrix[8] == ' ' => 8,
            Ok(_) => {  print!("{}", CursorUp(11));
                        println!("That move is taken, or out of range.");
                        continue },
            Err(_) => { print!("{}", CursorUp(11));
                        println!("Not valid. Please input a section number.");
                        continue }
        };
        
        // Rewrite section value according to user input.
        if current_player == 'X' {
            matrix[section] = 'X'
        } else {
            matrix[section] = 'O'
        }

        print!("{}", CursorUp(11)); 
        println!("{}", EraseLine);
        
        // Check for win or draw.
        match matrix {
            ['X', 'X', 'X', _, _, _, _, _, _] |
            [_, _, _, 'X', 'X', 'X', _, _, _] |
            [_, _, _, _, _, _, 'X', 'X', 'X'] |
            ['X', _, _, 'X', _, _, 'X', _, _] |
            [_, 'X', _, _, 'X', _, _, 'X', _] |          
            [_, _, 'X', _, _, 'X', _, _, 'X'] |
            ['X', _, _, _, 'X', _, _, _, 'X'] |
            [_, _, 'X', _, 'X', _, 'X', _, _] |
            ['O', 'O', 'O', _, _, _, _, _, _] |
            [_, _, _, 'O', 'O', 'O', _, _, _] |
            [_, _, _, _, _, _, 'O', 'O', 'O'] |
            ['O', _, _, 'O', _, _, 'O', _, _] |
            [_, 'O', _, _, 'O', _, _, 'O', _] |          
            [_, _, 'O', _, _, 'O', _, _, 'O'] |
            ['O', _, _, _, 'O', _, _, _, 'O'] |
            [_, _, 'O', _, 'O', _, 'O', _, _] => {  println!("");   
                                                    println!(" {} │ {} │ {} ", matrix[6], matrix[7], matrix[8]);
                                                    println!("───┼───┼───");
                                                    println!(" {} │ {} │ {} ", matrix[3], matrix[4], matrix[5]);
                                                    println!("───┼───┼───");
                                                    println!(" {} │ {} │ {} ", matrix[0], matrix[1], matrix[2]);
                                                    println!("");
                                                    println!("{}{} wins!", EraseLine, current_player);
                                                    println!("{}", EraseLine);
                                                    break },
            _ => false,
        };
            
        move_counter += 1;       

        // Switch current player.
        current_player = switch_player(current_player);
    };
}