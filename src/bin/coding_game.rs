use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_idx = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nb_games = parse_input!(input_line, i32);

    // game loop
    loop {
        let mut mv : String = "RIGHT".to_string();
        for i in 0..3 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let score_info = input_line.trim_matches('\n').to_string();
        }
        for i in 0..nb_games as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let gpu = inputs[0].trim().to_string();
            let mut regs:Vec<i32> = vec![];
            for i in 1..=7 as usize{
                regs.push(parse_input!(inputs[i], i32));
            }
            eprintln!("the reg 1 {}", regs[1]);
            eprintln!("the gpu is {}", gpu);
            // let reg_0 = parse_input!(inputs[1], i32);
            // let reg_1 = parse_input!(inputs[2], i32);
            // let reg_2 = parse_input!(inputs[3], i32);
            // let reg_3 = parse_input!(inputs[4], i32);
            // let reg_4 = parse_input!(inputs[5], i32);
            // let reg_5 = parse_input!(inputs[6], i32);
            // let reg_6 = parse_input!(inputs[7], i32);
            if (gpu == "GAME_OVER"){
                continue;
            }
            let player_pos_reg = regs[player_idx as usize];
            let player_stun_reg = regs[(player_idx + 3) as usize];
        }
        println!("{}", mv);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }
}