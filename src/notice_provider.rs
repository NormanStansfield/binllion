use std::collections::VecDeque;
use std::os::linux::raw::stat;

use crate::interfaces::Notice;
use crate::interfaces::NoticeProviderTrait;

// ステータス伝達
pub(crate) struct NoticeProvider {
    count: u8,
    queue: VecDeque<String>,
    cache: String,
}

// // ステータス伝達
// pub(crate) struct Notice {
//     count: Cell<u8>,
//     notice: RefCell<VecDeque<String>>,
//     cache: RefCell<String>,
// }

impl NoticeProviderTrait for NoticeProvider {
    fn new() -> Self {
        Self {
            count: 0,
            queue: VecDeque::new(),
            cache: String::with_capacity(0),
            // ::new(),
        }
    }

    // メッセージキューへ追加
    fn add(&mut self, notice: Notice) {
        // let Notice(state) = state;
        let message = notice.into_inner();

        self.queue.push_back(message);
    }

    // メッセージ取得
    fn get_notice(&mut self) -> Notice {
        const LIMIT: u8 = 2; // しばらく表示するためのアクション猶予回数

        match self.count {
            0 => {
                if let Some(message) = self.queue.pop_front() {
                    // self.cache = format!("{state}");
                    self.cache = message;
                    self.count = 1;
                } else {
                    self.cache.clear();
                }
            }
            x if x > LIMIT => {
                // self.count.set(0);
                self.count = 0;
            }
            x => {
                // self.count.set(x + 1);
                self.count += 1;
            }
        }

        // self.cache.borrow().clone()
        Notice::new(self.cache.clone())
    }

    //  fn pop_front(&self) -> String {
    //     const LIMIT: u8 = 2; // しばらく表示するためのアクション猶予回数

    //     match self.count.get() {
    //         0 => {
    //             if let Some(state) = self.notice.borrow_mut().pop_front() {
    //                 *self.cache.borrow_mut() = format!(" {state} ");
    //                 self.count.set(1);
    //             } else {
    //                 self.cache.borrow_mut().clear();
    //             }
    //         }
    //         x if x > LIMIT => {
    //             self.count.set(0);
    //         }
    //         x => {
    //             self.count.set(x + 1);
    //         }
    //     }

    //     self.cache.borrow().clone()
    // }
}
