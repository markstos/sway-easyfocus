use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidRgb,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRgb => f.write_str("Invalid RGB"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::InvalidRgb
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Rgb {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("#");

        if s.len() == 6 {
            let r: u8 = u8::from_str_radix(&s[0..2], 16)?;
            let g: u8 = u8::from_str_radix(&s[2..4], 16)?;
            let b: u8 = u8::from_str_radix(&s[4..6], 16)?;

            Ok(Rgb { r, g, b })
        } else {
            Err(Error::InvalidRgb)
        }
    }
}
