pub mod misc {
    pub fn islowercase(car: char) -> bool {
        car.to_lowercase().last().unwrap() == car
    }

    pub fn to_digit(car: char) -> Option<u8> {
        match car {
            '0'..='9' => Some(car as u8 - b'0'),
            _ => None,
        }
    }

    pub fn min(x: isize, y: isize) -> isize {
        (x + y - abs(x - y)) / 2
    }

    pub fn abs(x: isize) -> isize {
        if x < 0 {
            return -x;
        }
        x
    }
}

pub mod move_data {
    use crate::common::misc::min;

    pub const NORTH: usize = 0;
    pub const SOUTH: usize = 1;
    pub const EAST: usize = 3;
    pub const WEST: usize = 2;

    // pub const NORTH_EAST: usize = 6;
    // pub const NORTH_WEST: usize = 4;
    // pub const SOUTH_EAST: usize = 5;
    // pub const SOUTH_WEST: usize = 7;

    // first 4 digits, move for column and row movements
    // last 4 digits, move for diagonls movements
    pub const DIRECTION_OFFSET: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];

    pub fn precomputed_move_data() -> [[i8; 8]; 64] {
        let mut data: [[i8; 8]; 64] = [[0; 8]; 64];

        for height in 0..8 {
            for width in 0..8 {
                let num_north: i8 = height;
                let num_south: i8 = 7 - height;
                let num_west: i8 = width;
                let num_east: i8 = 7 - width;

                let square_index: usize = height as usize * 8 + width as usize;

                data[square_index] = [
                    num_north,
                    num_south,
                    num_west,
                    num_east,
                    min(num_north as isize, num_west as isize) as i8,
                    min(num_south as isize, num_east as isize) as i8,
                    min(num_north as isize, num_east as isize) as i8,
                    min(num_south as isize, num_west as isize) as i8,
                ];
            }
        }
        data
    }
}

pub mod canvas_display {
    use sdl2::rect::Rect;
    use sdl2::render::{Texture, WindowCanvas};

    pub fn canvas_fill(canvas: &mut WindowCanvas, rect: Rect) {
        match canvas.fill_rect(rect) {
            Ok(_) => {}
            Err(msg) => {
                println!("Error: {}", msg)
            }
        }
    }
    pub fn canvas_copy(
        canvas: &mut WindowCanvas,
        texture: &Texture,
        rect1: Option<Rect>,
        rect2: Option<Rect>,
    ) {
        match canvas.copy(texture, rect1, rect2) {
            Ok(_) => {}
            Err(msg) => {
                println!("Error: {}", msg)
            }
        }
    }
}
