use std::vec;
//good luck understanding this mess, hope i never to have to read it again
fn main() {
    let mut positions = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]; //the board is actually a 3x3 matrix (or an vector made of 3 more vectors) :')
    let mut Xturn = true; //true = X, false = O
    loop {
        //gameplay loop

        //get user input
        let mut boardnow = positionstostring(&positions); //saves the board to a string
        let mut chooseR: i32;
        let mut chooseC: i32;
        let mut passone: bool; //pass used if user enters invalid input ex 4 or 0
        let mut passtwou: bool; //pass used if user enters a taken spot
        loop {
            loop {
                passone = true; //the passes are defaulted to true, but they are changed to false if the user enters a valid input
                passtwou = true; // when changed to false the loop restarts, this makes sure that they will be true again in the start of the loop and will only be false if the user enters an invalid input
                println!("{}", boardnow); //prints the board
                println!("Enter a row"); //asking for user input (row)
                let mut inputrow = String::new();
                std::io::stdin()
                    .read_line(&mut inputrow)
                    .expect("Failed to read line");
                println!("Enter a column"); //asking for user input (column)
                let mut inputcolumn = String::new();
                std::io::stdin()
                    .read_line(&mut inputcolumn)
                    .expect("Failed to read line");
                chooseR = inputrow.trim().parse().expect("Please type a number"); //converts the user input to an integer
                chooseC = inputcolumn.trim().parse().expect("Please type a number"); //same as above
                                                                                     //offset magic to make row and column work with index 1
                chooseR = chooseR - 1;
                chooseC = chooseC - 1;
                if chooseR > 2 || chooseR < 0 || chooseC > 2 || chooseC < 0 {
                    //here we check if the user input is valid, if not the pass will be false then well restart the loop
                    println!("Please enter a number between 1 and 3");
                    passone = false;
                }
                if passone == true {
                    break;
                }
            } //if the first pass is true, we break out of the loop
            if positions[chooseR as usize][chooseC as usize] != 0 {
                //here we check if the spot is taken, if it is we will restart the loop
                println!("That spot is taken");
                passtwou = false;
            }
            if passtwou == true {
                break;
            }
        } //if the second pass is true, we break out of the loop

        if Xturn == true {
            positions = mark(&positions, chooseR, chooseC, 'x'); //marks the spot with an x if it is X's turn
        } else {
            positions = mark(&positions, chooseR, chooseC, 'o'); // marks the spot with an o if it is O's turn
        }
        Xturn = !Xturn;
        let wincheck = checkwin(&positions); //checks if someone has won an saves to wincheck var
        if wincheck == 1 {
            //if its 1, X wins, we print the board and exit the loop
            println!("X wins!");
            println!("{}", positionstostring(&positions));
            break;
        } else if wincheck == 2 {
            //if its -2, O wins, we print the board and exit the loop
            println!("O wins!");
            println!("{}", positionstostring(&positions));
            break;
        } else if wincheck == 0 {
            //if its 0, nobody has won, and we dont break out of the loop
            continue;
        }
    }
}
fn positionstostring(positions: &Vec<Vec<i32>>) -> String {
    //converts the board (matrix/vectors) to a string using emojis to look cool :P
    let mut string = String::new();
    for i in 0..3 {
        string.push_str("\n");
        for j in 0..3 {
            string.push_str(&positions[i][j].to_string());
        }
    }
    let result = string
        .replace("0", "⬜")
        .replace("-1", "⭕")
        .replace("1", "❌"); //replaces the -1(O marks) and 1(X marks) with the emojis
    result
}
fn mark(positions: &Vec<Vec<i32>>, x: i32, y: i32, turn: char) -> Vec<Vec<i32>> {
    //receives the board, the row and column, and the turn (X or O) and marks the spot
    let mut newpositions = positions.clone();
    if turn == 'x' {
        newpositions[(x) as usize][(y) as usize] = 1;
    } else {
        newpositions[(x) as usize][(y) as usize] = -1;
    }
    newpositions
}
fn checkwin(positions: &Vec<Vec<i32>>) -> u32 {
    //checks if the sum of roll/column/diagonal is 3(X wins return 1) or -3(O wins return 2), if none, return 0
    let sumfirst = positions[0][0] + positions[1][0] + positions[2][0];
    if sumfirst == 3 {
        return 1;
    } else if sumfirst == -3 {
        return 2;
    }
    let sumsecond = positions[0][1] + positions[1][1] + positions[2][1];
    if sumsecond == 3 {
        return 1;
    } else if sumsecond == -3 {
        return 2;
    }
    let sumthird = positions[0][2] + positions[1][2] + positions[2][2];
    if sumthird == 3 {
        return 1;
    } else if sumthird == -3 {
        return 2;
    }
    let sumfirstdiagonal = positions[0][0] + positions[1][1] + positions[2][2];
    if sumfirstdiagonal == 3 {
        return 1;
    } else if sumfirstdiagonal == -3 {
        return 2;
    }
    let sumseconddiagonal = positions[0][2] + positions[1][1] + positions[2][0];
    if sumseconddiagonal == 3 {
        return 1;
    } else if sumseconddiagonal == -3 {
        return 2;
    } else if !positions[0].contains(&0) && !positions[1].contains(&0) && !positions[2].contains(&0)
    {
        return 0;
    } else {
        return 3;
    }
}
