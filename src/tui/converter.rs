// 変換処理系

// crosstermクレート
use ratatui::text::Line;

use crate::constants;

// コンバーター
// pub(super) struct Converter;

// ratatuiのLines向けに変換
pub(crate) fn convert_to_lines<F: Converter>(buf: &[u8], address: usize) -> Vec<Line> {
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

// 16進数変換
pub(super) struct ForHex;

// Ascii変換
pub(super) struct ForAscii;

pub(super) trait Converter {
    fn convert(buf: &[u8]) -> String;
}

impl Converter for ForHex {
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
impl Converter for ForAscii {
    // Asciiへ変換
    fn convert(buf: &[u8]) -> String {
        buf.iter().fold(String::new(), |mut output, &x| {
            let _ = write!(output, "{}", to_printable_char(x));
            output
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, text::Text, Terminal};

    #[test]
    fn test_to_printable_char() {
        let num = 0x0;
        let res = super::to_printable_char(num);
        assert_eq!(res, '.');

        let num = 0x1F;
        let res = super::to_printable_char(num);
        assert_eq!(res, '.');

        let num = 0x40;
        let res = super::to_printable_char(num);
        assert_eq!(res, '@');

        let num = 0x7A;
        let res = super::to_printable_char(num);
        assert_eq!(res, 'z');

        let num = 0x7F;
        let res = super::to_printable_char(num);
        assert_eq!(res, '.');

        let num = 0xFF;
        let res = super::to_printable_char(num);
        assert_eq!(res, '.');
    }

    #[test]
    fn test_convert_to_lines_for_hex() {
        let mut bin_data = Vec::<u8>::new();
        let _ = (0..139).fold(0, |_acc, val| {
            bin_data.push(val);
            val
        });

        let lines = super::convert_to_lines::<ForHex>(&bin_data, 3 * constants::LINE_LEN);
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(Text::from(lines), frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_convert_to_lines_for_ascii() {
        let mut bin_data = Vec::<u8>::new();
        let _ = (0..139).fold(0, |_acc, val| {
            bin_data.push(val);
            val
        });

        let lines = super::convert_to_lines::<ForAscii>(&bin_data, 3 * constants::LINE_LEN);
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(Text::from(lines), frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
