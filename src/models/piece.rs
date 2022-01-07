use crate::common::Misc;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PColor {
    WHITE,
    BLACK,
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
    pub fn new<'a>(symbol: char) -> Option<Piece> {
        let color = if !Misc::islowercase(symbol) {
            PColor::WHITE
        } else {
            PColor::BLACK
        };

        let t = match symbol.to_lowercase().last().unwrap() {
            'p' => (PieceType::PAWN, 1),
            'n' => (PieceType::KNIGHT, 2),
            'b' => (PieceType::BISHOP, 3),
            'r' => (PieceType::ROOK, 4),
            'q' => (PieceType::QUEEN, 6),
            'k' => (PieceType::KING, 5),
            _ => return None,
        };
        let id: u8 = if color == PColor::BLACK { 8 } else { 16 } | t.1;
        Some(Piece {
            r#type: t.0,
            color: color,
            id: id,
            can_castle: t.0 == PieceType::KING || t.0 == PieceType::ROOK,
            can_en_passant: 0,
        })
    }

    pub fn create_piece_textures<'a>(
        renderer: &'a TextureCreator<WindowContext>,
    ) -> PieceTextures<'a> {
        let tmp_black = HashMap::from([
            (PieceType::PAWN, "textures/pieces/black_pawn.png"),
            (PieceType::KNIGHT, "textures/pieces/black_knight.png"),
            (PieceType::BISHOP, "textures/pieces/black_bishop.png"),
            (PieceType::ROOK, "textures/pieces/black_rook.png"),
            (PieceType::QUEEN, "textures/pieces/black_queen.png"),
            (PieceType::KING, "textures/pieces/black_king.png"),
        ]);

        let tmp_white = HashMap::from([
            (PieceType::PAWN, "textures/pieces/white_pawn.png"),
            (PieceType::KNIGHT, "textures/pieces/white_knight.png"),
            (PieceType::BISHOP, "textures/pieces/white_bishop.png"),
            (PieceType::ROOK, "textures/pieces/white_rook.png"),
            (PieceType::QUEEN, "textures/pieces/white_queen.png"),
            (PieceType::KING, "textures/pieces/white_king.png"),
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
        self.r#type == PieceType::QUEEN
            || self.r#type == PieceType::BISHOP
            || self.r#type == PieceType::ROOK
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
