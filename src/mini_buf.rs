use crate::interfaces::{CharCode, HexData, Index, MiniBufTrait};

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

    fn add(&mut self, char_code: CharCode) {
        let value = char_code.into_inner();
        self.buf[self.index] = value.to_ascii_uppercase();
        self.index = (self.index + 1) % 2;
    }

    fn updata(&mut self, value: HexData) {
        let value = value.into_inner();
        // 16進数へ変換
        let str = format!("{:02X}", value);

        self.index = 0;
        for char_code in str.chars() {
            self.add(CharCode::new(char_code));
        }
        self.index = 0;

    }

    fn to_hex(&self) -> Result<HexData, std::num::ParseIntError> {
        let str: String = self.buf.iter().collect();
        let res = u8::from_str_radix(&str, 16);
        match res {
            Ok(value) => Ok(HexData::new(value)),
            Err(value) => Err(value),
        }
    }
}
