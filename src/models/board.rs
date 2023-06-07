use super::piece::PColor;
use super::piece::Piece;
use super::piece::PieceTextures;

use crate::common::canvas_display;
use crate::common::misc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Board<'a> {
    pub size: usize,
    pub board: Vec<Option<Piece>>,
    pub white: Color,
    pub black: Color,
    pub piece_textures: PieceTextures<'a>,
}

impl Board<'_> {
    pub fn new(renderer: &TextureCreator<WindowContext>) -> Board<'_> {
        let mut board: Vec<Option<Piece>> = Vec::new();

        // initializing board
        for _ in 0..64 {
            board.push(None);
        }

        Board {
            size: 8,
            board,
            white: Color::RGBA(234, 203, 164, 255),
            black: Color::RGBA(185, 112, 68, 255),
            piece_textures: Piece::create_piece_textures(renderer),
        }
    }

    pub fn get(&self, y: usize, x: usize) -> Option<Piece> {
        if y >= self.size || x >= self.size {
            return None;
        }
        self.board[y * self.size + x]
    }

    pub fn get_square(&self, square: usize) -> Option<Piece> {
        if square >= self.size * self.size {
            return None;
        }
        self.board[square]
    }

    pub fn set(&mut self, y: usize, x: usize, value: Option<Piece>) {
        if y >= self.size || x >= self.size {
            return;
        }
        self.board[y * self.size + x] = value;
    }

    pub fn set_square(&mut self, square: usize, value: Option<Piece>) {
        if square >= self.size * self.size {
            return;
        }
        self.board[square] = value;
    }

    pub fn init(&mut self) {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        self.fen_init(fen); //String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    }

    pub fn draw_board(&self, canvas: &mut WindowCanvas, width: i32, height: i32) {
        let mut alternate: u8 = 0;

        let case_height: i32 = height / self.size as i32;
        let case_width: i32 = width / self.size as i32;

        for y in 0..self.size {
            for x in 0..self.size {
                if x != 0 {
                    alternate = 1 - alternate;
                }

                match alternate {
                    0 => canvas.set_draw_color(self.white),
                    1 => canvas.set_draw_color(self.black),
                    _ => {}
                }
                canvas_display::canvas_fill(
                    canvas,
                    Rect::new(
                        x as i32 * case_width,
                        y as i32 * case_height,
                        case_width as u32,
                        case_height as u32,
                    ),
                );
            }
        }
    }

    pub fn draw_pieces(&self, canvas: &mut WindowCanvas, width: i32, height: i32) {
        let case_height: i32 = height / self.size as i32;
        let case_width: i32 = width / self.size as i32;

        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(p) = self.get(y, x) {
                    let rect = Rect::new(
                        x as i32 * case_width,
                        y as i32 * case_height,
                        case_width as u32,
                        case_height as u32,
                    );
                    match p.color {
                        PColor::White => {
                            canvas_display::canvas_copy(
                                canvas,
                                self.piece_textures.white_textures.get(&p.r#type).unwrap(),
                                None,
                                Some(rect),
                            );
                        }
                        PColor::Black => {
                            canvas_display::canvas_copy(
                                canvas,
                                self.piece_textures.black_textures.get(&p.r#type).unwrap(),
                                None,
                                Some(rect),
                            );
                        }
                    }
                }
            }
        }
    }

    // -------------------------------------------
    // ------------ PRIVATE FUNCTIONS ------------
    // -------------------------------------------

    fn fen_init(&mut self, notation: String) {
        let mut index: usize = 0;
        for c in notation.chars() {
            if c == '/' {
                println!("Skipped char /");
                continue;
            }
            let tmp = misc::to_digit(c);

            if let Some(digit) = tmp {
                println!("char {} is number: {}", c, digit);
                index += digit as usize;
            } else {
                match Piece::new(c) {
                    None => {
                        println!("{} is not a valid symbol for a chess piece !", c)
                    }
                    p => {
                        self.board[index] = p;
                        println!("Generated new piece with symbol: {}", c);
                    }
                }
                index += 1;
            }
        }
    }
}
