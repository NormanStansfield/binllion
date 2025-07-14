use std::collections::VecDeque;

use crate::constants;
use crate::interfaces::Notice;
use crate::interfaces::NoticeProviderTrait;

// ステータス伝達
pub(crate) struct NoticeProvider {
    count: u8,
    queue: VecDeque<String>,
    cache: String,
}

impl NoticeProviderTrait for NoticeProvider {
    fn new() -> Self {
        Self {
            count: 0,
            queue: VecDeque::new(),
            cache: String::with_capacity(0),
        }
    }

    // メッセージキューへ追加
    fn add(&mut self, notice: Notice) {
        let message = notice.into_inner();

        self.queue.push_back(message);
    }

    // メッセージ取得
    fn get_notice(&mut self) -> Notice {
        match self.count {
            0 => {
                if let Some(message) = self.queue.pop_front() {
                    self.cache = message;
                    self.count = 1;
                } else {
                    self.cache.clear();
                }
            }
            x if x >= constants::LIMIT => {
                self.count = 0;
            }
            x => {
                self.count += 1;
            }
        }

        Notice::new(self.cache.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::LIMIT;

    #[test]
    fn test_notice_provider() {
        let mut notice_provider = NoticeProvider::new();
        let test_data_notice = "TestMessage".to_string();
        notice_provider.add(Notice::new(test_data_notice.clone()));
        (0..=LIMIT).for_each(|_| {
            let res = notice_provider.get_notice();
            assert_eq!(res.into_inner(), test_data_notice);
        });
        let res = notice_provider.get_notice();
        assert_eq!(res.into_inner(), String::from(""));
        let res = notice_provider.get_notice();
        assert_eq!(res.into_inner(), String::from(""));
    }
}
