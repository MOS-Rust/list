#[cfg(test)]
mod list_tests {
    use crate::linked_list::{linked_list_insert_before, linked_list_remove, Linkable, ListEntry};
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
        fn link(&mut self) -> &mut ListEntry<Page> {
            &mut self.entry
        }
    }

    #[test]
    fn simple_test() {
        let mut page_list: LinkedList<Page> = LinkedList::new();
        let mut pages = [Page::new(11);10];

        assert!(page_list.is_empty() == true);

        for i in 0..10 {
            pages[i].value = i as i32;
            page_list.insert_head(&mut pages[i]);
        }

        for i in (0..10).rev() {
            let head = page_list.first();
            unsafe {
                assert!((*head).value == i);
            }
            linked_list_remove(head);
        
        }

        assert!(page_list.is_empty() == true);

    }

    #[test]
    fn insert_before_test() {
        let mut page_list: LinkedList<Page> = LinkedList::new();
        assert!(page_list.is_empty() == true);

        let p1 = &mut Page::new(1);
        let p2 = &mut Page::new(3);
        let p3 = &mut Page::new(5);

        page_list.insert_head(p1);
        page_list.insert_head(p2);
        page_list.insert_head(p3);

        linked_list_insert_before(p1, &mut Page::new(2));
        linked_list_insert_before(p2, &mut Page::new(4));
        linked_list_insert_before(p3, &mut Page::new(6));

        for i in (1..7).rev() {
            let head = page_list.first();
            unsafe {
                assert!((*head).value == i);
            }
            linked_list_remove(head);
        }

        assert!(page_list.is_empty() == true);
    }
}