use crate::common::CanvasDisplay;

use super::board::Board;
use super::piece::PColor;
use super::piece::Piece;
use super::r#move::{Move, MoveAction, MoveGenerator};
use super::sound::Sound;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use std::collections::HashMap;

pub struct Game<'a> {
    //board structure: used for piece placement and display
    pub board: Board<'a>,
    //Color of the player that is currently playing,
    //Used for recognition of which pieces can be played
    pub current_player: PColor,
    // piece states, used in order to know which piece is currently being
    // hold by the player
    pub piece_hold: Option<Piece>,
    pub x: usize,
    pub y: usize,
    // visual fx: used in order to display the last move on the board
    pub last_move: Option<Move>,
    // algorithmic states: used to generate moves for the pieces according
    // to chess rules
    possible_moves: HashMap<usize, Vec<Move>>,
    move_generator: MoveGenerator,
}

impl Game<'_> {
    pub fn new<'a>(renderer: &'a TextureCreator<WindowContext>) -> Game<'a> {
        let mut board = Board::new(renderer);
        board.init();
        let player = PColor::WHITE;
        let generator = MoveGenerator::new();
        let possible_moves = generator.generate_moves(&mut board, player);

        Game {
            board: board,
            current_player: player,
            piece_hold: None,
            x: 0,
            y: 0,
            last_move: None,
            possible_moves: possible_moves,
            move_generator: generator,
        }
    }

    pub fn select_piece(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let i: usize = self.board.size * y as usize / height as usize;
        let j: usize = self.board.size * x as usize / width as usize;

        let selected: Option<Piece> = self.board.get(i, j);
        println!("found coordinate: ({}, {})", j, i);
        match selected {
            None => {}
            Some(p) => {
                if p.color == self.current_player {
                    self.piece_hold = selected;
                    self.x = j;
                    self.y = i;
                    println!("x: {}, y: {}", self.x, self.y);
                    self.board.set(i, j, None);
                }
            }
        }
    }

    pub fn make_move(&mut self, x: i32, y: i32, width: u32, height: u32, sound: &Sound) {
        if self.piece_hold == None {
            return;
        }
        let i: usize = self.board.size * y as usize / height as usize;
        let j: usize = self.board.size * x as usize / width as usize;

        let start: usize = self.y * self.board.size + self.x;
        let end: usize = i * self.board.size + j;

        println!("start given: {}, end given: {}", start, end);
        let move_made = Move::is_valid(
            start,
            end,
            &mut self.board,
            &mut self.piece_hold.unwrap(),
            &self.possible_moves,
        );

        match move_made {
            MoveAction::MOVE => {
                sound.play("move");
            }
            MoveAction::TAKE => {
                sound.play("take");
            }
            MoveAction::CASTLE => {
                sound.play("castle");
            }
            MoveAction::INCORRECT => {
                self.board.set(self.y, self.x, self.piece_hold);
                self.reset_hold_piece_states();
            }
        }
        if move_made != MoveAction::INCORRECT {
            self.update_after_move(start, end);
        }
    }

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        width: i32,
        height: i32,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        self.board.draw_board(canvas, width, height);
        self.draw_last_move(canvas, width, height);
        self.draw_possible_moves(canvas, width, height);
        self.board.draw_pieces(canvas, width, height);
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

    fn draw_hold(
        &self,
        canvas: &mut WindowCanvas,
        width: i32,
        height: i32,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        match self.piece_hold {
            None => {}
            Some(p) => {
                let case_height: i32 = height / self.board.size as i32;
                let case_width: i32 = width / self.board.size as i32;

                let rect = Rect::new(
                    mouse_x - case_width / 2,
                    mouse_y - case_height / 2,
                    case_width as u32,
                    case_height as u32,
                );

                match p.color {
                    PColor::WHITE => {
                        CanvasDisplay::canvas_copy(
                            canvas,
                            self.board
                                .piece_textures
                                .white_textures
                                .get(&p.r#type)
                                .unwrap(),
                            None,
                            Some(rect),
                        );
                    }
                    PColor::BLACK => {
                        CanvasDisplay::canvas_copy(
                            canvas,
                            self.board
                                .piece_textures
                                .black_textures
                                .get(&p.r#type)
                                .unwrap(),
                            None,
                            Some(rect),
                        );
                    }
                }
            }
        }
    }

    fn draw_last_move(&self, canvas: &mut WindowCanvas, width: i32, height: i32) {
        match self.last_move {
            None => {}
            Some(m) => {
                let case_height: i32 = height / self.board.size as i32;
                let case_width: i32 = width / self.board.size as i32;

                canvas.set_draw_color(Color::RGBA(0, 255, 0, 30));
                CanvasDisplay::canvas_fill(
                    canvas,
                    Rect::new(
                        (m.start % self.board.size) as i32 * case_width,
                        (m.start / self.board.size) as i32 * case_height,
                        case_width as u32,
                        case_height as u32,
                    ),
                );
                canvas.set_draw_color(Color::RGBA(255, 255, 0, 30));
                CanvasDisplay::canvas_fill(
                    canvas,
                    Rect::new(
                        (m.end % self.board.size) as i32 * case_width,
                        (m.end / self.board.size) as i32 * case_height,
                        case_width as u32,
                        case_height as u32,
                    ),
                );
            }
        }
    }

    fn draw_possible_moves(&self, canvas: &mut WindowCanvas, width: i32, height: i32) {
        if self.piece_hold == None {
            return;
        }
        let square: usize = self.y * self.board.size + self.x;
        let case_height: i32 = height / self.board.size as i32;
        let case_width: i32 = width / self.board.size as i32;

        canvas.set_draw_color(Color::RGBA(255, 0, 0, 200));
        for mv in &self.possible_moves[&square] {
            CanvasDisplay::canvas_fill(
                canvas,
                Rect::new(
                    (mv.end % self.board.size) as i32 * case_width,
                    (mv.end / self.board.size) as i32 * case_height,
                    case_width as u32,
                    case_height as u32,
                ),
            );
        }
    }

    fn update_after_move(&mut self, start: usize, end: usize) {
        self.switch_player();
        self.update_last_move(start, end);
        self.update_new_moves();
        self.reset_hold_piece_states();
    }

    fn update_new_moves(&mut self) {
        self.possible_moves = self
            .move_generator
            .generate_moves(&mut self.board, self.current_player);
    }

    fn update_last_move(&mut self, start: usize, end: usize) {
        self.last_move = Some(Move::new(start, end));
    }

    fn reset_hold_piece_states(&mut self) {
        self.piece_hold = None;
        self.x = 0;
        self.y = 0;
    }
}
