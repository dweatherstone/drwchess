use crate::common::MoveData::{precomputed_move_data, DIRECTION_OFFSET};

use super::piece::{Piece, PieceType, PColor};
use super::board::Board;

use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub start: usize,
    pub end: usize
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum MoveAction {
    INCORRECT,
    MOVE,
    TAKE
}

impl Move {
    pub fn new(start: usize, end: usize) -> Move {
        Move {
            start,
            end
        }
    }

    pub fn is_valid(start: usize, end: usize, board: &mut Board, piece: Piece, possible_moves: &HashMap<usize, Vec<Move>>) -> MoveAction {
        if !Move::is_in_list(start, end, &possible_moves[&start]) {
            return MoveAction::INCORRECT;
        }

        let selected = board.get_square(end);

        board.set(end / 8, end % 8, Some(piece));

        match selected {
            None => MoveAction::MOVE,
            Some(p) => MoveAction::TAKE
        }
    }

    fn is_in_list(start: usize, end: usize, moves: &Vec<Move>) -> bool {
        for r#move in moves {
            println!("Move: from {} to {}", r#move.start, r#move.end);
            if r#move.start == start && r#move.end == end {
                return true;
            }
        }
        false
    }
}

pub struct MoveGenerator {
    precomputed: [[i8; 8]; 64]
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let precomputed = precomputed_move_data();

        MoveGenerator {
            precomputed
        }
    }

    pub fn generate_moves(&self, board: &Board, player_color: PColor) -> HashMap<usize, Vec<Move>> {
        let mut hash = HashMap::new();
        for square in 0..64 {
            let mut moves: Vec<Move> = Vec::new();
            let piece = match board.get_square(square) {
                None => {continue}
                Some(p) => {p}
            };
            if !piece.is_color(player_color) {continue;}

            if piece.is_sliding_piece() {
                self.generate_sliding_move(&mut moves, &piece, square, board);
            } else {

            }
            hash.insert(square, moves);
        }
        hash
    }

    fn generate_sliding_move(&self, moves: &mut Vec<Move>, piece: &Piece, square: usize, board: &Board) {
        let start_index: i32 = if piece.r#type == PieceType::BISHOP {3} else {0};
        let end_index: i32 = if piece.r#type == PieceType::ROOK {4} else {8};

        for index in start_index..end_index {
            for n in 0..self.precomputed[square][index as usize] {
                let target = (square as i8 + (DIRECTION_OFFSET[index as usize] * (n+1))) as usize;
                let s = board.get_square(target as usize);
                if s != None {
                    if piece.is_enemy(s) {
                        moves.push(Move::new(square, target as usize));
                    }
                    break;
                } else {
                    moves.push(Move::new(square, target as usize));
                }
            }
        }
    }
}