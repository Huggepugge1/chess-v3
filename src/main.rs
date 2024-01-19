mod board;
mod consts;
mod types;
mod fen_reader;
mod move_generator;
mod attack_bitboards;
mod search;

fn main() {
    let mut board: board::Board = consts::EMPTY_BOARD;
    board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let command = line.split(" ").collect::<Vec<&str>>();
        match command[0].replace("\n", "").replace("\r", "").as_str() {
            "isready" => println!("readyok"),
            "uci" => (),
            "printboard" => board.print_board(),
            "ucinewgame" => (),
            "position" => {
                if command.len() == 1 {
                    println!("position requires at least 1 argument!");
                } else {
                    match command[1].replace("\n", "").replace("\r", "").as_str() {
                        "startpos" => board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
                        _ => board.load_fen(command[1..].join(" ").replace("\n", "").replace("\r", "")),
                    }

                    if command[1].replace("\n", "").replace("\r", "") == "startpos" && command.len() > 2 {
                        for mov in &command[3..] {
                            board.make_move(
                                types::Move::new(
                                    board::Board::string_to_square(mov.replace("\n", "").replace("\r", "")[0..2].to_string()),
                                    board::Board::string_to_square(mov.replace("\n", "").replace("\r", "")[2..4].to_string()),
                                    if mov.replace("\n", "").replace("\r", "").len() == 5 {
                                        match mov.chars().nth(4).unwrap() {
                                            'r' => types::PieceType::Rook,
                                            'n' => types::PieceType::Knight,
                                            'b' => types::PieceType::Bishop,
                                            'q' => types::PieceType::Queen,
                                            x   => panic!("Unknown piece: {x}"),
                                        }
                                    } else {
                                        types::PieceType::Empty
                                    }
                                ));
                        }
                    }
                }
            }
            "go" => {
                if command.len() == 1 {
                    println!("{}", board::Board::print_move(&search::search(-1, &mut board)));
                } else {
                    match command[1].replace("\n", "").replace("\r", "").as_str() {
                        "infinite" => println!("{}", board::Board::print_move(&search::search(-1, &mut board))),
                        "depth" => {
                            if command.len() >= 3 {
                                println!("{:?}", command);
                                let depth: i32 = command[2].replace("\n", "").replace("\r", "").parse().unwrap();
                                println!("{}", board::Board::print_move(&search::search(depth, &mut board)));
                            }
                        },
                        "perft" => {
                            if command.len() == 3 {
                                let depth: i32 = command[2].replace("\n", "").replace("\r", "").parse().unwrap();
                                println!("Nodes searched: {}", search::perft(depth, depth, &mut board));
                            } else {
                                println!("\"go perft\" needs **ONE** argument");
                            }
                        }
                        x => println!("{x} is either not implemented or not a valid argument for \"go\""),
                    }
                }
            },
            x => println!("{x} is either not implemented or not a valid UCI command"),
        }
    }
}
