fn file_diff(pos1: i32, pos2: i32) -> i32 {
    i32::abs(pos2 % 8 - pos1 % 8)
}

fn rank_diff(pos1: i32, pos2: i32) -> i32 {
    i32::abs(pos2 / 8 - pos1 / 8)
}

fn generate_king_attacks() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    let offsets: [i32; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
    for square in 0..64 {
        for offset in offsets {
            let pos = square as i32 + offset;
            if pos >= 0 && pos <= 63 
                && (i32::abs(square as i32 % 8 - pos % 8) <= 1
                    && i32::abs(square as i32 / 8 - pos / 8) <= 1) {
                
                bitboards[square] |= 1 << pos;
            }
        }
    }

    bitboards
}

fn generate_white_pawn_pushes() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 8..56 {
        bitboards[square] = 1 << (square + 8);
        if square / 8 == 1 {
            bitboards[square] |= 1 << (square + 16);
        }
    }

    bitboards
}

fn generate_black_pawn_pushes() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 8..56 {
        bitboards[square] = 1 << (square - 8);
        if square / 8 == 6 {
            bitboards[square] |= 1 << (square - 16);
        }
    }

    bitboards
}

fn generate_white_pawn_attacks() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    let offsets: [i32; 2] = [7, 9];
    for square in 8..56 {
        for offset in offsets {
            let pos = square + offset as usize;
            if pos <= 63 && i32::abs((square % 8) as i32 - (pos % 8) as i32) == 1 {
                bitboards[square] |= 1 << pos;
            }
        }
    }

    bitboards
}

fn generate_black_pawn_attacks() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    let offsets: [i32; 2] = [-7, -9];
    for square in 8..56 {
        for offset in offsets {
            let pos: i32 = square as i32 + offset;
            if pos >= 0 && i32::abs((square % 8) as i32 - (pos % 8) as i32) == 1 {
                bitboards[square] |= 1 << pos;
            }
        }
    }

    bitboards
}

fn generate_east_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = 1;
        let mut i = 1;
        while (square + offset * i) / 8 - square / 8 == 0 {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }

    bitboards
}

fn generate_north_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = 8;
        let mut i = 1;
        while square + offset * i < 64 {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }

    bitboards
}

fn generate_west_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = -1;
        let mut i = 1;
        while square + offset * i >= 0 && (square + offset * i) / 8 - square / 8 == 0 {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }

    bitboards
}

fn generate_south_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = -8;
        let mut i = 1;
        while square + offset * i >= 0 {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }

    bitboards
}

fn generate_north_east_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = 9;
        let mut i = 1;
        while square + offset * i < 64 && file_diff(square, square + offset * i) == rank_diff(square, square + offset * i) {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }
    
    bitboards
}

fn generate_north_west_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = 7;
        let mut i = 1;
        while square + offset * i < 64 && file_diff(square, square + offset * i) == rank_diff(square, square + offset * i) {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }
    
    bitboards
}

fn generate_south_east_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = -7;
        let mut i = 1;
        while square + offset * i >= 0 && file_diff(square, square + offset * i) == rank_diff(square, square + offset * i) {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }
    
    bitboards
}

fn generate_south_west_rays() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    for square in 0..64 {
        let offset = -9;
        let mut i = 1;
        while square + offset * i >= 0 && file_diff(square, square + offset * i) == rank_diff(square, square + offset * i) {
            bitboards[square as usize] |= 1 << (square + offset * i);
            i += 1;
        }
    }
    
    bitboards
}

fn get_squares(bitboard: u64) -> Vec<usize> {
    let mut result = Vec::new();
    for square in 0..64 {
        if (1 << square) & bitboard > 0 {
            result.push(square);
        }
    }
    result
}

fn main() {
    let black_pawn_pushes = generate_black_pawn_pushes();
    // let rays: [u64; 64] = generate_east_rays();
    // let rays: [u64; 64] = generate_north_rays();
    // let rays: [u64; 64] = generate_west_rays();
    // let rays: [u64; 64] = generate_south_rays();
    // let rays: [u64; 64] = generate_north_east_rays();
    // let rays: [u64; 64] = generate_north_west_rays();
    // let rays: [u64; 64] = generate_south_east_rays();
    // let rays: [u64; 64] = generate_south_west_rays();
    for push in black_pawn_pushes {
        println!("{:?}", get_squares(push));
    }
    println!("{:?}", black_pawn_pushes);
}

