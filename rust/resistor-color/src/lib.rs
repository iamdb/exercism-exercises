use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntEnum, Sequence, PartialOrd, Ord)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    let color: Result<ResistorColor, int_enum::IntEnumError<ResistorColor>> =
        ResistorColor::from_int(value);

    if let Ok(color) = color {
        format!("{:?}", color)
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut all_colors = all::<ResistorColor>().collect::<Vec<_>>();
    all_colors.sort();

    all_colors
}
