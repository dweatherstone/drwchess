
pub mod Misc {
    pub fn islowercase(car: char) -> bool {
        car.to_lowercase().last().unwrap() == car
    }

    pub fn to_digit(car: char) -> Option<u8> {
        match car {
            '0'..='9' => Some(car as u8 - '0' as u8),
            _ => None
        }
    }
}