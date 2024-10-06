// convert_to_ascii

#![feature(ascii_char)]
#![feature(ascii_char_variants)]

// #[warn(dead_code)]

fn main() {}

// 16進数へ変換
pub(crate) fn to_hex(buf: &[u8]) -> String {
    let sep = String::from(" ");
    let hex = buf
        .iter()
        .map(|x| format!("{:02X}", x))
        .collect::<Vec<_>>()
        .join(&sep);
    hex
}

// ratatui向け Lineを使わないように修正
pub(crate) fn to_lines(buf: &[u8], len: usize) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", to_hex(x), width = 8)));
    vec
}

// char::from(u8)
pub(crate) fn to_ascii_1(buf: &[u8]) -> String {
    let res = buf
        .iter()
        .fold(String::new(), |acc, x| format!("{}{}", acc, char::from(*x)));
    res
}

// as_ascii
pub(crate) fn to_ascii_2(buf: &[u8]) -> String {
    let res = buf.iter().fold(String::new(), |acc, x| {
        format!(
            "{}{}",
            acc,
            x.as_ascii().unwrap_or(std::ascii::Char::FullStop)
        )
    });
    res
}

// encoding_rs #1
use encoding_rs::mem::decode_latin1;

pub(crate) fn to_ascii_3(buf: &[u8]) -> String {
    let res = decode_latin1(&buf);
    res.into()
}

// encoding_rs #2
use encoding_rs::UTF_8;

pub(crate) fn to_ascii_4(buf: &[u8]) -> String {
    let (res, _, _) = UTF_8.decode(&buf);
    res.into()
}

// ascii crate #1
use ascii::{AsciiChar, AsciiStr, IntoAsciiString};

pub(crate) fn to_ascii_5(buf: &[u8]) -> String {
    // let ascii_data = data.clone().into_ascii_string().unwrap_or_default();
    let res = &buf.into_ascii_string().unwrap_or(AsciiChar::Dot.into());
    res.to_string()
}

// ascii crate #2
pub(crate) fn to_ascii_6(buf: &[u8]) -> String {
    let res: String = buf
        .iter()
        .map(|x| {
            AsciiChar::from_ascii(*x)
                .unwrap_or(AsciiChar::Dot)
                .as_printable_char()
        })
        .collect();
    res
}

// ascii crate #3
pub(crate) fn to_ascii_7(buf: &[u8]) -> String {
    let res = buf.iter().fold(String::new(), |mut acc, &x| {
        acc.push(
            AsciiChar::from_ascii(x)
                .unwrap_or(AsciiChar::Dot)
                .as_printable_char(),
        );
        acc
    });
    res
}

// ascii crate #4
pub(crate) fn to_ascii_8(buf: &[u8]) -> String {
    let res = unsafe { AsciiStr::from_ascii_unchecked(&buf) };
    res.to_string()
}

// Original #1
pub(crate) fn to_ascii_9(buf: &[u8]) -> String {
    let res: String = buf
        .iter()
        .map(|&x| format!("{}", ToPrintableAscii::as_printable_char(x)))
        .collect();
    res
}

struct ToPrintableAscii;

impl ToPrintableAscii {
    pub fn as_printable_char(num: u8) -> char {
        const DUMMY_CHAR: char = '.';
        match num as u8 {
            0x0..=0x1f => DUMMY_CHAR,
            0x7f.. => DUMMY_CHAR,
            _ => char::from(num),
        }
    }
}

struct ToHex;
struct ToAscii;

// 関数バージョン
trait ConvertTrait {
    fn convert(buf: &[u8]) -> String;
}

// メソッドバージョン
trait ConvertTraitMethod {
    fn convert(&self, buf: &[u8]) -> String;
}

impl ConvertTrait for ToHex {
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

impl ConvertTraitMethod for ToHex {
    fn convert(&self, buf: &[u8]) -> String {
        let sep = String::from(" ");
        let hex = buf
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join(&sep);
        hex
    }
}

impl ConvertTrait for ToAscii {
    fn convert(buf: &[u8]) -> String {
        let res: String = buf
            .iter()
            .map(|&x| format!("{}", ToPrintableAscii::as_printable_char(x)))
            .collect();
        res
    }
}

impl ConvertTraitMethod for ToAscii {
    fn convert(&self, buf: &[u8]) -> String {
        let res: String = buf
            .iter()
            .map(|&x| format!("{}", ToPrintableAscii::as_printable_char(x)))
            .collect();
        res
    }
}

// to_hex()が置き換わっただけ
pub(crate) fn to_lines2(buf: &[u8], len: usize) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", to_ascii_9(x), width = 8)));
    vec
}

// トレイト境界を使う
pub(crate) fn to_lines3<F: Fn(&[u8]) -> String>(buf: &[u8], len: usize, func: F) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", func(x), width = 8)));
    vec
}

// 別のやり方でトレイト境界を使う
pub(crate) fn to_lines4(buf: &[u8], len: usize, func: impl Fn(&[u8]) -> String) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", func(x), width = 8)));
    vec
}

// 型制約を使う
pub(crate) fn to_lines5(buf: &[u8], len: usize, func: fn(&[u8]) -> String) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", func(x), width = 8)));
    vec
}

// 自作トレイトでトレイト境界を使う 関数バージョン
pub(crate) fn to_lines6<F: ConvertTrait>(buf: &[u8], len: usize) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", F::convert(x), width = 8)));
    vec
}

// 自作トレイトででトレイト境界を使う メソッドバージョン
pub(crate) fn to_lines7<F: ConvertTraitMethod>(buf: &[u8], len: usize, func: F) -> Vec<String> {
    let mut vec = Vec::new();
    buf.chunks(len)
        .for_each(|x| vec.push(format!("{:width$} {}", " ", func.convert(x), width = 8)));
    vec
}

// enumでの実装
#[derive(Debug)]
enum Jojo {
    Dio,
    Terence,
    Joseph,
    Tim,
}

impl Jojo {
    pub fn question(&self) -> &str {
        match self {
            Jojo::Dio => {
                let question = "お前は今まで食ったパンの枚数を覚えているのか？";
                println!("{:?}: {}", &self, question);
                question
            }
            Jojo::Terence => {
                let question = "もしかしてオラオラですかーッ！？";
                println!("{:?}: {}", &self, question);
                question
            }
            Jojo::Joseph => {
                let question = "ノックしてもしもお～し";
                println!("{:?}: {}", &self, question);
                question
            }
            Jojo::Tim => {
                let question = "おっと 会話が成り立たないアホがひとり登場〜〜 質問文に対し質問文で答えるとテスト0点なの知ってたか？マヌケ ";
                println!("{:?}: {}", &self, question);
                question
            }
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::to_hex;
    // use crate::to_ascii_1;
    use crate::*;

    // #[test]
    fn setup() -> &'static [u8] {
        let bin_data: &[u8] = &[
            0x01, 0x02, 0x03, 0x00, 0x63, 0x71, 0x00, 0x61, 0x62, 0x0f, 0x01, 0x02, 0x03, 0x00,
            0x63, 0x71, 0x0f, 0x61, 0x62, 0x63, 0x01, 0xff, 0x03, 0x00, 0x63, 0x71, 0x0f, 0x61,
            0x62, 0x63,
        ];
        bin_data
    }
    fn setup_to_hex() -> String {
        let expect = "01 02 03 00 63 71 00 61 62 0F 01 02 03 00 63 71 0F 61 62 63 01 FF 03 00 63 71 0F 61 62 63".to_string();
        expect
    }
    fn setup_to_ascii() -> Vec<String> {
        let expect = vec![
            "         ....cq.a".into(),
            "         b.....cq".into(),
            "         .abc....".into(),
            "         cq.abc".into(),
        ];
        expect
    }
    fn setup_to_lines_hex() -> Vec<String> {
        let expect = vec![
            "         01 02 03 00 63 71 00 61".into(),
            "         62 0F 01 02 03 00 63 71".into(),
            "         0F 61 62 63 01 FF 03 00".into(),
            "         63 71 0F 61 62 63".into(),
        ];
        expect
    }

    #[test]
    fn test_to_hex() {
        // 仮データ
        let bin_data = setup();
        let expect = setup_to_hex();
        let res = to_hex(bin_data);
        dbg!(&res);
        assert_eq!(res, expect);
    }
    #[test]
    fn test_to_ascii_1() {
        // char::from(u8)
        let bin_data = setup();
        let res = to_ascii_1(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_1", res);
    }
    #[test]
    fn test_to_ascii_2() {
        // as_ascii
        let bin_data = setup();
        let res = to_ascii_2(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_2", res);
    }
    #[test]
    fn test_to_ascii_3() {
        // encoding_rs #1
        let bin_data = setup();
        let res = to_ascii_3(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_3", res);
    }
    #[test]
    fn test_to_ascii_4() {
        // encoding_rs #2
        let bin_data = setup();
        let res = to_ascii_4(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_4", res);
    }
    #[test]
    fn test_to_ascii_5() {
        // ascii crate #1
        let bin_data = setup();
        let res = to_ascii_5(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_5", res);
    }
    #[test]
    fn test_to_ascii_6() {
        // ascii crate #2
        let bin_data = setup();
        let res = to_ascii_6(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_6", res);
    }
    #[test]
    fn test_to_ascii_7() {
        // ascii crate #3
        let bin_data = setup();
        let res = to_ascii_7(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_7", res);
    }
    #[test]
    fn test_to_ascii_8() {
        // ascii crate #4
        let bin_data = setup();
        let res = to_ascii_8(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_8", res);
    }
    #[test]
    fn test_to_ascii_9() {
        // Original #1
        let bin_data = setup();
        let res = to_ascii_9(bin_data);
        dbg!(&res);
        println!("{}: {}", "to_ascii_9", res);
    }
    #[test]
    fn test_to_lines() {
        let bin_data = setup();
        let res = to_lines(bin_data, 8);
        dbg!(&res);
        println!("{}: {:?}", "to_lines", &res);
    }
    #[test]
    fn test_to_lines_with_ascii_1() {
        let bin_data = setup();
        let expect = setup_to_ascii();
        let res = to_lines2(bin_data, 8);
        dbg!(&res);
        println!("{}: {:?}", "to_lines", &res);
        assert_eq!(res, expect);
    }
    #[test]
    fn test_to_lines_with_ascii_and_hex_1() {
        let bin_data = setup();
        let expect = setup_to_ascii();
        let res = to_lines3(bin_data, 8, to_ascii_9);
        dbg!(&res);
        println!("{}: {:?}", "to_ascii_9", &res);
        assert_eq!(res, expect);

        let expect = setup_to_lines_hex();
        let res = to_lines3(bin_data, 8, to_hex);
        dbg!(&res);
        println!("{}: {:?}", "to_hex", &res);
        assert_eq!(res, expect);
    }
    #[test]
    fn test_to_lines_with_ascii_and_hex_2() {
        let bin_data = setup();
        let expect = setup_to_ascii();
        let res = to_lines4(bin_data, 8, to_ascii_9);
        dbg!(&res);
        println!("{}: {:?}", "to_ascii_9", &res);
        assert_eq!(res, expect);

        let expect = setup_to_lines_hex();
        let res = to_lines4(bin_data, 8, to_hex);
        dbg!(&res);
        println!("{}: {:?}", "to_hex", &res);
        assert_eq!(res, expect);
    }
    #[test]
    fn test_to_lines_with_hello_1() {
        let bin_data = setup();
        // let expect = setup_to_ascii();
        let res_1 = to_lines3(bin_data, 8, |_| -> String { "hello".to_string() });
        dbg!(&res_1);
        println!("{}: {:?}", "hello", &res_1);

        let res_2 = to_lines4(bin_data, 8, |_| -> String { "hello".to_string() });
        dbg!(&res_2);
        println!("{}: {:?}", "hello", &res_2);
        assert_eq!(res_1, res_2);
    }
    #[test]
    fn test_to_lines_with_hello_2() {
        let bin_data = setup();
        let expect = setup_to_ascii();
        let res = to_lines5(bin_data, 8, to_ascii_9);
        dbg!(&res);
        println!("{}: {:?}", "to_ascii_9", &res);
        assert_eq!(res, expect);

        let expect = setup_to_lines_hex();
        let res = to_lines5(bin_data, 8, to_hex);
        dbg!(&res);
        println!("{}: {:?}", "to_hex", &res);
        assert_eq!(res, expect);

        let res_1 = to_lines5(bin_data, 8, |_| -> String { "hello".to_string() }); // 外部変数をキャプチャしないと関数にキャストされる
                                                                                   /*
                                                                                   let res_1 = to_lines5(bin_data, 8, |_| -> String {bin_data;"hello".to_string()});  // 外部変数をキャプチャするとクロージャとして扱われて型がマッチせずエラー
                                                                                   */
        dbg!(&res_1);
        println!("{}: {:?}", "hello", &res_1);

        let res_2 = to_lines4(bin_data, 8, |_| -> String { "hello".to_string() });
        dbg!(&res_2);
        println!("{}: {:?}", "hello", &res_2);
        assert_eq!(res_1, res_2);
    }
    #[test]
    fn test_to_lines_with_ascii_and_hex_3() {
        let bin_data = setup();
        let expect = setup_to_ascii();
        let res = to_lines6::<ToAscii>(bin_data, 8);
        dbg!(&res);
        println!("{}: {:?}", "to_ascii_9", &res);
        assert_eq!(res, expect);

        let expect = setup_to_lines_hex();
        let res = to_lines6::<ToHex>(bin_data, 8);
        dbg!(&res);
        println!("{}: {:?}", "to_hex", &res);
        assert_eq!(res, expect);
    }
    #[test]
    fn test_to_lines_with_ascii_and_hex_4() {
        let bin_data = setup();
        let expect = setup_to_ascii();
        let res = to_lines7(bin_data, 8, ToAscii);
        dbg!(&res);
        println!("{}: {:?}", "to_ascii_9", &res);
        assert_eq!(res, expect);

        let to_ascii = ToAscii;
        let res = to_lines7(bin_data, 8, to_ascii);
        dbg!(&res);
        println!("{}: {:?}", "to_ascii_9", &res);
        assert_eq!(res, expect);

        let expect = setup_to_lines_hex();
        let res = to_lines7(bin_data, 8, ToHex);
        dbg!(&res);
        println!("{}: {:?}", "to_hex", &res);
        assert_eq!(res, expect);
    }
    #[test]
    fn test_jojo_question() {
        Jojo::Dio.question();
        Jojo::Joseph.question();
        Jojo::Terence.question();
        Jojo::Tim.question();
    }
}
