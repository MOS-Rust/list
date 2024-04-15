#[cfg(test)]
mod list_tests {
    use crate::linked_list::{Linkable, ListEntry};
    use core::marker::Copy;

    #[derive(Copy, Clone)]
    pub struct Page {
        value: i32,
        entry: ListEntry<Page>,
    }

    unsafe impl Linkable<Page> for Page {
        fn link(&self) -> ListEntry<Page> {
            self.entry
        }
    }

    #[test]
    fn insert_test() {
        
    }
}