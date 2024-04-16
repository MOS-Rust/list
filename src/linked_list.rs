//! list data structure used in mos
//! from linux, rewrite in rust

use core::ptr;

/// anything organized by linked list should implement this trait
/// this trait ensures type T in linked list has pointer field
pub unsafe trait Linkable<T: Linkable<T>>: Copy {
    fn link(&mut self) -> &mut ListEntry<T>;
}

/// linked list head
#[derive(Copy, Clone, Debug)]  
pub struct LinkedList<T: Linkable<T>> {
    head: *mut T,
}

/// linked list entry(pointer field)
#[derive(Clone, Copy, Debug)]
pub struct ListEntry<T: Linkable<T>> {
    pub prev: *mut *mut T,
    pub next: *mut T,
}

/// list entry init method
impl<T: Linkable<T>> ListEntry<T> {
    pub fn new() -> Self {
        Self { prev: ptr::null_mut(), next: ptr::null_mut() }
    }
}

impl<T: Linkable<T>> LinkedList<T> {
    /// linked list init method
    pub const fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    /// return if list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// insert elm to the head of the list
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

    /// get the first item of the list(list head)
    pub fn first(&self) -> *mut T {
        self.head
    }
}

/// insert elm before listelm
/// ! this function has NOT been tested 
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

/// insert elm after listelm
/// ! this function has NOT been tested
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

/// remove elm from list
pub fn linked_list_remove<T: Linkable<T>>(elm: *mut T) {
    unsafe {
        if !(*elm).link().next.is_null() {
            (*(*elm).link().next).link().prev = (*elm).link().prev;
        }
        (*((*elm).link().prev)) = (*elm).link().next;
    }
}

// TODO implement iterator feature for linked list