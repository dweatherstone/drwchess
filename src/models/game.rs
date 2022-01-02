use super::board::Board;
use super::piece::Piece;
use super::piece::PColor;

use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

pub struct Game<'a> {
    pub board: Board<'a>,
    pub current_player: PColor,
    pub piece_hold: Option<Piece>,
    pub x: i32,
    pub y: i32
}

impl Game<'_> {
    pub fn new<'a>(renderer: &'a TextureCreator<WindowContext>) -> Game<'a> {
        let mut board = Board::new(renderer);
        board.init();
        Game {
            board: board,
            current_player: PColor::WHITE,
            piece_hold: None,
            x: -1,
            y: -1,
        }
    }

    pub fn select_piece(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let i: usize = self.board.size * y as usize / height as usize;
        let j: usize = self.board.size * x as usize / width as usize;

        let selected = self.board.get(i, j);
        println!("found coordinate: ({}, {})", j, i);
        match selected {
            None => {},
            Some(p) => {
                if p.color == self.current_player {
                    self.piece_hold = selected;
                    self.x = j as i32;
                    self.y = i as i32;
                    self.board.set(i, j, None);
                }
            }
        }
    }

    pub fn make_move(&mut self, x: i32, y: i32, width: u32, height: u32) {
        if self.piece_hold == None {
            return;
        }
        let i: usize = self.board.size * y as usize / height as usize;
        let j: usize = self.board.size * x as usize / width as usize;

        let selected = self.board.get(i, j);
        let mut move_made: bool = false;
        match selected {
            None => {
                self.board.set(i, j, self.piece_hold);
                move_made = self.x as usize != j || self.y as usize != i;
            },
            Some(p) => {
                if (p.color != self.current_player) {
                    self.board.set(i, j, self.piece_hold);
                    move_made = true;
                } else {
                    self.board.set(self.y as usize, self.x as usize, self.piece_hold);
                }
            }
        }
        self.piece_hold = None;
        self.x = -1;
        self.y = -1;

        if move_made {
            self.switch_player();
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, width: i32, height: i32, mouse_x: i32, mouse_y: i32) {
        self.board.draw(canvas, width, height);
        self.draw_hold(canvas, width, height, mouse_x, mouse_y);
    }

    // -------------------------------------------
    // ------------ PRIVATE FUNCTIONS ------------
    // -------------------------------------------

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == PColor::WHITE {
            PColor::BLACK
        } else {
            PColor::WHITE
        };
    }

    fn draw_hold(&self, canvas: &mut WindowCanvas, width: i32, height: i32, mouse_x: i32, mouse_y: i32) {
        match self.piece_hold {
            None => {},
            Some(p) => {
                let case_height: i32 = height / self.board.size as i32;
                let case_width: i32 = width / self.board.size as i32;

                let rect = Rect::new(mouse_x - case_width / 2,
                                    mouse_y - case_height / 2,
                                    case_width as u32,
                                    case_height as u32);

                match p.color {
                    PColor::WHITE => {
                        canvas.copy(self.board.piece_textures.white_textures.get(&p.r#type).unwrap(),
                                    None, Some(rect));
                    },
                    PColor::BLACK => {
                        canvas.copy(self.board.piece_textures.black_textures.get(&p.r#type).unwrap(),
                                    None, Some(rect));
                    }
                }
            }
        }
    }
}