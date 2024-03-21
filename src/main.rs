mod position;
mod evaluation;


use position::{Position, Evaluation, Coordinates};
use std::io::{self, Write};


fn main() {
    clear_screen();
    println!("      ////^\\\\\\\\\n      | ^   ^ |\n     @ (o) (o) @\n      |   <   |\n      |  ___  |\n       \\_____/\n     ____|  |____\n    /    \\__/    \\\n   /              \\\n  /\\_/|        |\\_/\\   __   _____  _   _ __  __  ___  _  ___   _ _   _\n / /  |        |  \\ \\  \\ \\ / / _ \\| \\ | |  \\/  |/ _ \\| |/ / | | | \\ | |\n( <   |        |   > )  \\ V / | | |  \\| | |\\/| | | | | ' /| | | |  \\| |\n \\ \\  |        |  / /    | || |_| | |\\  | |  | | |_| | . \\| |_| | |\\  |\n  \\ \\ |________| / /     |_| \\___/|_| \\_|_|  |_|\\___/|_|\\_\\\\___/|_| \\_|\n");
    println!("やあ、僕の名はよんも君。一緒に三次元四目並べを遊ぼう！\n");

    let depth: u8;
    loop {
        println!("1. 本気モード");
        println!("2. 優しいモード");
        print!("遊ぶモードを選んでね。[1,2]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        match &input[..] {
            "1" => depth = 6,
            "2" => depth = 1,
            _ => {
                println!("１か２を選んでね。");
                continue;
            }
        }

        break;
    }


    let mut current_position = Position::new();

    let final_evaluation: Evaluation;
    let last_move_by_player: bool;

    current_position.print();
    loop {
        println!("君の番だよ。");

        let player_coordinates = get_user_coordinates(&current_position);
        current_position = current_position.play(player_coordinates);

        if current_position.is_terminal() {
            final_evaluation = current_position.get_static_evaluation();
            last_move_by_player = true;
            break;
        };

        let report = evaluation::analyse(&current_position, depth);
        current_position = current_position.play(report.best_move);

        clear_screen();
        current_position.print();
        println!("僕はここに打ったよ。{}", report.best_move);
        println!("自信係数: {:.0}%", report.evaluation.to_confidence() * 100.0);
        println!("computation time (ms):   {}", report.search_time.as_millis());
        println!("num positions traversed: {}", report.num_positions_traversed);

        if current_position.is_terminal() {
            final_evaluation = current_position.get_static_evaluation();
            last_move_by_player = false;
            break;
        };
    }

    match final_evaluation {
        Evaluation::Loss => {
            if last_move_by_player {
                println!("負けました！楽しい対局をありがとう。");
            } else {
                println!("やった、僕の勝ち！楽しい対局をありがとう。");
            }
        },
        Evaluation::Draw => println!("引き分けだね。楽しい対局をありがとう。"),
        _ => panic!("Something went wrong.")
    }
}


fn clear_screen() {
    print!("\x1B[2J\x1b[1;1H");
}


fn get_user_coordinates(position: &Position) -> Coordinates {
    loop {
        let file: usize;
        let rank: usize;
        let coordinates: Coordinates;

        loop {
            print!("縦列の番号は？[1-4]: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_string();

            match &input[..] {
                "1" => file = 0,
                "2" => file = 1,
                "3" => file = 2,
                "4" => file = 3,
                _ => {
                    println!("使える数字を打ってね。[1-4]");
                    continue;
                }
            }

            break;
        };
        
        loop {
            print!("横列の番号は？[1-4]: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_string();

            match &input[..] {
                "1" => rank = 0,
                "2" => rank = 1,
                "3" => rank = 2,
                "4" => rank = 3,
                _ => {
                    println!("使える数字を打ってね。[1-4]");
                    continue;
                }
            }

            break;
        }

        coordinates = Coordinates::new(file, rank);

        if position.can_play(coordinates) {
            return coordinates
        } else {
            println!("そこには置けないな。どこに置きたい？");
            continue;
        }
    }
}
