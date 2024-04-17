#[cfg(test)]
mod list_tests {
    use crate::linked_list::{linked_list_insert_after, LinkedList};
    use crate::linked_list::{linked_list_insert_before, linked_list_remove, Linkable, ListEntry};
    use core::marker::Copy;

    #[derive(Copy, Clone)]
    pub struct Page {
        value: i32,
        entry: ListEntry<Page>,
    }

    impl Page {
        pub fn new(value: i32) -> Self {
            Self {
                value: value,
                entry: ListEntry::new(),
            }
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
        let mut pages = [Page::new(11); 10];

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

    #[test]
    fn insert_after_test() {
        let mut page_list: LinkedList<Page> = LinkedList::new();
        assert!(page_list.is_empty() == true);

        let p1 = &mut Page::new(1);
        let p2 = &mut Page::new(3);
        let p3 = &mut Page::new(5);

        page_list.insert_head(p1);
        page_list.insert_head(p2);
        page_list.insert_head(p3);

        linked_list_insert_after(p1, &mut Page::new(0));
        linked_list_insert_after(p2, &mut Page::new(2));
        linked_list_insert_after(p3, &mut Page::new(4));

        for i in (0..6).rev() {
            let head = page_list.first();
            unsafe {
                assert!((*head).value == i);
            }
            linked_list_remove(head);
        }

        assert!(page_list.is_empty() == true);
    }

    #[test]
    fn complex_operations_test() {
        let mut page_list: LinkedList<Page> = LinkedList::new();

        let mut pages = (1..6).map(|v| Page::new(v)).collect::<Vec<_>>();
        for page in pages.iter_mut() {
            page_list.insert_head(page);
        }

        let values: Vec<_> = page_list.iter().map(|p| p.value).collect();
        assert_eq!(values, vec![5, 4, 3, 2, 1]);

        let p_before = &mut Page::new(10);
        linked_list_insert_before(&mut pages[2], p_before);

        let values: Vec<_> = page_list.iter().map(|p| p.value).collect();
        assert_eq!(values, vec![5, 4, 10, 3, 2, 1]);

        let p_after = &mut Page::new(20);
        linked_list_insert_after(&mut pages[1], p_after);

        let values: Vec<_> = page_list.iter().map(|p| p.value).collect();
        assert_eq!(values, vec![5, 4, 10, 3, 2, 20, 1]);

        let head = page_list.first();
        linked_list_remove(head);
        let values: Vec<_> = page_list.iter().map(|p| p.value).collect();
        assert_eq!(values, vec![4, 10, 3, 2, 20, 1]);

        linked_list_remove(&mut pages[2]);
        let values: Vec<_> = page_list.iter().map(|p| p.value).collect();
        assert_eq!(values, vec![4, 10, 2, 20, 1]);

        assert!(!page_list.is_empty());

        while !page_list.is_empty() {
            let head = page_list.first();
            linked_list_remove(head);
        }

        assert!(page_list.is_empty());
    }
}
