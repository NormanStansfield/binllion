use crate::interfaces::{CharCode, HexData, MiniBufPosition, MiniBufTrait};

#[derive(Debug)]
pub(crate) struct MiniBuf {
    buf: [char; 2],
    index: usize,
}

impl MiniBufTrait for MiniBuf {
    fn new() -> Self {
        let buf: [char; 2] = ['0'; 2];
        let index = 0;
        Self { buf, index }
    }

    fn add(&mut self, char_code: CharCode) -> MiniBufPosition {
        let value = char_code.into_inner();
        self.buf[self.index] = value.to_ascii_uppercase();
        self.index = (self.index + 1) % 2;

        self.get_position()
    }

    fn updata(&mut self, value: HexData) {
        let value = value.into_inner();
        // 16進数へ変換
        let str = format!("{:02X}", value);

        self.index = 0;
        for char_code in str.chars() {
            let _ = self.add(CharCode::new(char_code));
        }
        self.index = 0;
    }

    fn to_hex(&self) -> Result<HexData, std::num::ParseIntError> {
        let str: String = self.buf.iter().collect();
        let res = u8::from_str_radix(&str, 16);
        match res {
            Ok(value) => Ok(HexData::new(value)),
            Err(err) => Err(err),
        }
    }

    fn get_position(&self) -> MiniBufPosition {
        match &self.index {
            0 => MiniBufPosition::Head,
            1 => MiniBufPosition::Tail,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_buf() {
        let mut mini_buf = MiniBuf::new();

        // 初期状態のテスト
        let res = mini_buf.get_position();
        assert_eq!(res, MiniBufPosition::Head);

        let res = mini_buf.to_hex();
        match res {
            Ok(val) => {
                assert_eq!(val.into_inner(), 0u8)
            }
            Err(val) => {
                panic!("{}", val);
            }
        }

        // 9を追加
        let char_code = '9';
        mini_buf.add(CharCode::new(char_code));
        let res = mini_buf.to_hex();
        match res {
            Ok(val) => {
                assert_eq!(val.into_inner(), 0x90u8)
            }
            Err(val) => {
                panic!("{}", val);
            }
        }

        let res = mini_buf.get_position();
        assert_eq!(res, MiniBufPosition::Tail);

        // Fを追加
        let char_code = 'F';
        mini_buf.add(CharCode::new(char_code));
        let res = mini_buf.to_hex();
        match res {
            Ok(val) => {
                assert_eq!(val.into_inner(), 0x9Fu8)
            }
            Err(val) => {
                panic!("{}", val);
            }
        }

        let res = mini_buf.get_position();
        assert_eq!(res, MiniBufPosition::Head);

        // 45で上書き
        let val = 45;
        mini_buf.updata(HexData::new(val));
        let res = mini_buf.to_hex();
        match res {
            Ok(val) => {
                assert_eq!(val.into_inner(), 45u8)
            }
            Err(val) => {
                panic!("{}", val);
            }
        }

        let res = mini_buf.get_position();
        assert_eq!(res, MiniBufPosition::Head);
    }
}
