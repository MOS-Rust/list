#[cfg(test)]
mod list_tests {
    use crate::linked_list::{self, linked_list_remove, Linkable, ListEntry};
    use core::marker::Copy;
    use crate::linked_list::LinkedList;

    #[derive(Copy, Clone)]
    pub struct Page {
        value: i32,
        entry: ListEntry<Page>,
    }

    impl Page {
        pub fn new(value: i32) -> Self {
            Self { value: value, entry: ListEntry::new() }
        }
    }

    unsafe impl Linkable<Page> for Page {
        fn link(&self) -> ListEntry<Page> {
            self.entry
        }
    }

    #[test]
    fn insert_test() {
        let mut page_list: LinkedList<Page> = LinkedList::new();
        let mut pages = [Page::new(11);10];
        assert!(page_list.is_empty() == true);
        for i in 0..10 {
            pages[i].value = i as i32;
            unsafe {
                page_list.insert_head(&mut pages[i]);
            }
        }
        
        for i in (0..10).rev() {
            let head = page_list.first();
            unsafe {
                assert!((*head).value == i);
            }
            linked_list_remove(head);
        
        }

    }
}