use crate::common::Misc;

use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PColor {
    WHITE,
    BLACK
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Piece {
    pub r#type: PieceType,   // state's name is type
    pub color: PColor,
}

pub struct PieceTextures<'a> {
    // renderer: &'a TextureCreator<WindowContext>,
    pub black_textures: HashMap<PieceType, Texture<'a>>,
    pub white_textures: HashMap<PieceType, Texture<'a>>
}

impl Piece {
    pub fn new(symbol: char) -> Option<Piece> {
        let color = if !Misc::islowercase(symbol) {
            PColor::WHITE
        } else {
            PColor::BLACK
        };

        let t = match symbol.to_lowercase().last().unwrap() {
            'p' => PieceType::PAWN,
            'n' => PieceType::KNIGHT,
            'b' => PieceType::BISHOP,
            'r' => PieceType::ROOK,
            'q' => PieceType::QUEEN,
            'k' => PieceType::KING,
            _   => return None
        };
        Some(Piece {
            r#type: t,
            color: color
        })
    }

    pub fn create_piece_textures<'a>(renderer: &'a TextureCreator<WindowContext>) ->
            PieceTextures<'a> {
        
        let tmp_black = HashMap::from([
            (PieceType::PAWN, "textures/pieces/black_pawn.png"),
            (PieceType::KNIGHT, "textures/pieces/black_knight.png"),
            (PieceType::BISHOP, "textures/pieces/black_bishop.png"),
            (PieceType::ROOK, "textures/pieces/black_rook.png"),
            (PieceType::QUEEN, "textures/pieces/black_queen.png"),
            (PieceType::KING, "textures/pieces/black_king.png")
        ]);

        let tmp_white = HashMap::from([
            (PieceType::PAWN, "textures/pieces/white_pawn.png"),
            (PieceType::KNIGHT, "textures/pieces/white_knight.png"),
            (PieceType::BISHOP, "textures/pieces/white_bishop.png"),
            (PieceType::ROOK, "textures/pieces/white_rook.png"),
            (PieceType::QUEEN, "textures/pieces/white_queen.png"),
            (PieceType::KING, "textures/pieces/white_king.png")
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
            white_textures: white
        }
    }
}