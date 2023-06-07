use crate::common::misc;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PColor {
    White,
    Black,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Piece {
    pub r#type: PieceType, // state's name is type
    pub color: PColor,
    pub id: u8,
    pub can_castle: bool,
    pub can_en_passant: usize,
}

pub struct PieceTextures<'a> {
    // renderer: &'a TextureCreator<WindowContext>,
    pub black_textures: HashMap<PieceType, Texture<'a>>,
    pub white_textures: HashMap<PieceType, Texture<'a>>,
}

impl Piece {
    pub fn new(symbol: char) -> Option<Piece> {
        let color = if !misc::islowercase(symbol) {
            PColor::White
        } else {
            PColor::Black
        };

        let t = match symbol.to_lowercase().last().unwrap() {
            'p' => (PieceType::Pawn, 1),
            'n' => (PieceType::Knight, 2),
            'b' => (PieceType::Bishop, 3),
            'r' => (PieceType::Rook, 4),
            'q' => (PieceType::Queen, 6),
            'k' => (PieceType::King, 5),
            _ => return None,
        };
        let id: u8 = if color == PColor::Black { 8 } else { 16 } | t.1;
        Some(Piece {
            r#type: t.0,
            color,
            id,
            can_castle: t.0 == PieceType::King || t.0 == PieceType::Rook,
            can_en_passant: 0,
        })
    }

    pub fn create_piece_textures(renderer: &TextureCreator<WindowContext>) -> PieceTextures<'_> {
        let tmp_black = HashMap::from([
            (PieceType::Pawn, "textures/pieces/black_pawn.png"),
            (PieceType::Knight, "textures/pieces/black_knight.png"),
            (PieceType::Bishop, "textures/pieces/black_bishop.png"),
            (PieceType::Rook, "textures/pieces/black_rook.png"),
            (PieceType::Queen, "textures/pieces/black_queen.png"),
            (PieceType::King, "textures/pieces/black_king.png"),
        ]);

        let tmp_white = HashMap::from([
            (PieceType::Pawn, "textures/pieces/white_pawn.png"),
            (PieceType::Knight, "textures/pieces/white_knight.png"),
            (PieceType::Bishop, "textures/pieces/white_bishop.png"),
            (PieceType::Rook, "textures/pieces/white_rook.png"),
            (PieceType::Queen, "textures/pieces/white_queen.png"),
            (PieceType::King, "textures/pieces/white_king.png"),
        ]);

        let mut white: HashMap<PieceType, Texture> = HashMap::new();
        let mut black: HashMap<PieceType, Texture> = HashMap::new();

        for (piece, path) in tmp_black {
            black.insert(piece, renderer.load_texture(path).unwrap());
            println!("Loaded piece at path {}", path);
        }

        for (piece, path) in tmp_white {
            white.insert(piece, renderer.load_texture(path).unwrap());
            println!("Loaded piece at path {}", path);
        }

        PieceTextures {
            //renderer: renderer,
            black_textures: black,
            white_textures: white,
        }
    }

    pub fn is_sliding_piece(&self) -> bool {
        self.r#type == PieceType::Queen
            || self.r#type == PieceType::Bishop
            || self.r#type == PieceType::Rook
    }

    pub fn is_type(&self, r#type: PieceType) -> bool {
        self.r#type == r#type
    }

    pub fn is_color(&self, color: PColor) -> bool {
        self.color == color
    }

    pub fn is_enemy(&self, piece: Option<Piece>) -> bool {
        match piece {
            None => false,
            Some(p) => p.color != self.color,
        }
    }

    pub fn is_ally(&self, piece: Option<Piece>) -> bool {
        match piece {
            None => false,
            Some(p) => p.color == self.color,
        }
    }

    pub fn can_castle(piece: Option<Piece>) -> bool {
        match piece {
            None => false,
            Some(p) => p.can_castle,
        }
    }
}
