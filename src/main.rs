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
        match command[0].replace("\n", "").as_str() {
            "isready" => println!("readyok"),
            "uci" => (),
            "ucinewgame" => (),
            "position" => {
                if command.len() == 1 {
                    println!("position requires at least 1 argument!");
                } else {
                    match command[1].replace("\n", "").as_str() {
                        "startpos" => board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
                        fen => board.load_fen(fen.to_string()),
                    }

                    if command.len() > 2 {
                        for mov in &command[3..] {
                            board.make_move(
                                types::Move::new(
                                    board::Board::string_to_square(mov.replace("\n", "")[0..2].to_string()),
                                    board::Board::string_to_square(mov.replace("\n", "")[2..4].to_string()),
                                    if mov.replace("\n", "").len() == 5 {
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
                    board::Board::print_move(search::search(-1, &mut board));
                } else {
                    match command[1].replace("\n", "").as_str() {
                        "infinite" => board::Board::print_move(search::search(-1, &mut board)),
                        "depth" => {
                            if command.len() >= 3 {
                                let depth: i32 = command[2].replace("\n", "").parse().unwrap();
                                board::Board::print_move(search::search(depth, &mut board));
                            }
                        },
                        "perft" => {
                            if command.len() == 3 {
                                let depth: i32 = command[2].replace("\n", "").parse().unwrap();
                                println!("Nodes searched: {}", search::perft(depth, &mut board));
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
