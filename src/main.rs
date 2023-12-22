mod board;
mod consts;
mod types;
mod fen_reader;

fn main() {
    let mut board: board::Board = consts::EMPTY_BOARD;

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let command = line.split(" ").collect::<Vec<&str>>();
        match command[0].replace("\n", "").as_str() {
            "isready" => println!("readyok"),
            "uci" => (),
            "ucinewgame" => (),
            "position" => {
                match command[1].replace("\n", "").as_str() {
                    "startpos" => board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ".to_string()),
                    fen => board.load_fen(fen.to_string()),
                }

                if command.len() > 2 {
                    for mov in &command[3..] {
                        board.make_move(
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
                        );
                    }
                }
            }
            x => todo!("{x}")
        }
    }
}
