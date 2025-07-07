// 変換処理系

// crosstermクレート
use ratatui::text::Line;

use crate::constants;

// コンバーター
pub(super) struct Converter;

impl Converter {
    // ratatuiのLines向けに変換
    pub(crate) fn convert_to_lines<F: ConverterTrait>(buf: &[u8], address: usize) -> Vec<Line> {
        let mut vec = Vec::new();
        let mut address = address;
        buf.chunks(constants::LINE_LEN).for_each(|x| {
            vec.push(Line::from(format!(
                "{address:#08X} {value}",
                address = address,
                value = F::convert(x),
            )));
            address += constants::LINE_LEN;
        });
        // dbg!(&vec);
        vec
    }

    // 制御文字等もDUMMY_CHARに変換して読めるようにする
    pub(crate) fn to_printable_char(num: u8) -> char {
        const DUMMY_CHAR: char = '.';
        match num {
            0x0..=0x1f => DUMMY_CHAR,
            0x7f.. => DUMMY_CHAR,
            _ => char::from(num),
        }
    }
}

// 16進数変換
pub(super) struct ForHex;

// Ascii変換
pub(super) struct ForAscii;

pub(super) trait ConverterTrait {
    fn convert(buf: &[u8]) -> String;
}

impl ConverterTrait for ForHex {
    // 16進数へ変換
    fn convert(buf: &[u8]) -> String {
        let sep = String::from(" ");
        let hex = buf
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join(&sep);
        hex
    }
}

use std::fmt::Write;
impl ConverterTrait for ForAscii {
    // Asciiへ変換
    fn convert(buf: &[u8]) -> String {
        buf.iter().fold(String::new(), |mut output, &x| {
            let _ = write!(output, "{}", Converter::to_printable_char(x));
            output
        })
    }
}
