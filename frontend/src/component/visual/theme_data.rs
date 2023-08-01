use plotters::style::RGBColor;

#[derive(Clone)]
pub struct Color (pub u8, pub u8, pub u8);

#[derive(Debug, thiserror::Error)]
pub enum HexCodeError {
    #[error("Invalid Hex Code first character, expected #, found {0}")]
    InvalidFirstCharacter(char),
    #[error("Invalid Hex Code length, expected 4 or 7 character including header, found {0}")]
    InvalidLength(usize),
    #[error("Invalid Hex Code character, expected characters in 0..=F, found {0}")]
    InvalidCharacter(char)
}

type HCE = HexCodeError;


impl Color {
    pub fn from_hex_code(code: &str) -> Result<Color, HexCodeError> {
        match code.chars().nth(0) {
            Some(c) => if c != '#' {
                return Err(HCE::InvalidFirstCharacter(c))
            },
            None => return Err(HCE::InvalidLength(0)),
        }

        match code.len() {
            4 => {
                let numbers = code.chars().skip(1).enumerate().try_fold((0u8, 0u8, 0u8), |mut acc, c| {
                    match u8::from_str_radix(c.1.to_string().as_str(), 16) {
                        Ok(n) => match c.0 {
                            0 => acc.0 = n*16+n,
                            1 => acc.1 = n*16+n,
                            2 => acc.2 = n*16+n,
                            _ => panic!("We have already confirmed the length is exactly 3 characters after skipping the leading #.")
                        },
                        Err(_) => return Err(HCE::InvalidCharacter(c.1)),
                    }
                    Ok(acc)
                })?;
                return Ok(Color(numbers.0, numbers.1, numbers.2))
            },
            7 => {
                let numbers = code.chars().skip(1).enumerate().try_fold((0u8, 0u8, 0u8), |mut acc, c| {
                    match u8::from_str_radix(c.1.to_string().as_str(), 16) {
                        Ok(n) => match c.0 {
                            0 => acc.0 = n*16,
                            1 => acc.0 += n,
                            2 => acc.1 = n*16,
                            3 => acc.1 += n,
                            4 => acc.2 = n*16,
                            5 => acc.2 += n,
                            _ => panic!("We have already confirmed the length is exactly 6 characters after skipping the leading #.")
                        },
                        Err(_) => return Err(HCE::InvalidCharacter(c.1)),
                    }
                    Ok(acc)
                })?;
                return Ok(Color(numbers.0, numbers.1, numbers.2))
            },
            _ => {
                return Err(HCE::InvalidLength(code.len()))
            }
        }
    }
}





pub struct ThemeData {
    pub theme_primary: Color,
    pub theme_secondary: Color,
    pub theme_background_primary: Color,
    pub theme_background_secondary: Color,
    pub theme_background_tertiary: Color,
    pub theme_text: Color,
}

impl From<Color> for RGBColor {
    fn from(value: Color) -> Self {
        RGBColor(value.0, value.1, value.2)
    }
}