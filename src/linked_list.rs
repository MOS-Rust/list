use core::ptr;

pub unsafe trait Linkable<T: Linkable<T>>: Copy {
    fn link(&self) -> ListEntry<T>;
}

#[derive(Copy, Clone)]  
pub struct LinkedList<T: Linkable<T>> {
    head: *mut T,
}

#[derive(Clone, Copy)]
pub struct ListEntry<T: Linkable<T>> {
    prev: *mut *mut T,
    next: *mut T,
}

impl<T: Linkable<T>> ListEntry<T> {
    pub fn new() -> Self {
        Self { prev: ptr::null_mut(), next: ptr::null_mut() }
    }
}

impl<T: Linkable<T>> LinkedList<T> {
    pub const fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn insert_head(&mut self, elm: *mut T) {
        unsafe {
            (*elm).link().next = self.head;
            if !(self.head).is_null() {
                (*self.head).link().prev = &mut (*elm).link().next;
            }
            self.head = elm;
            (*elm).link().prev = &mut self.head;
        }
    }
}

pub fn linked_list_insert_before<T: Linkable<T>>(
    listelm: *mut T, elm: *mut T
) {
    unsafe {
        (*elm).link().prev = (*listelm).link().prev;
        (*elm).link().next = listelm;
        *((*listelm).link().prev) = elm;
        (*listelm).link().prev = &mut ((*elm).link().next);
    }
}

pub fn linked_list_insert_after<T: Linkable<T>>(
    listelm: *mut T, elm: *mut T
) {
    unsafe {
        (*elm).link().next = (*listelm).link().next;
        if (*listelm).link().next.is_null() {
            (*(*listelm).link().next).link().prev = &mut (*elm).link().next;
        }
        (*listelm).link().prev = &mut (*listelm).link().next;
    }
}

pub fn linked_list_remove<T: Linkable<T>>(elm: *mut T) {
    unsafe {
        if !(*elm).link().next.is_null() {
            (*(*elm).link().next).link().prev = (*elm).link().prev;
        }
        (*(*elm).link().prev) = (*elm).link().next;
    }
}

// TODO implement iterator feature for linked list