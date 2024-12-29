use std::collections::HashMap;
use std::sync::Mutex;
use std::ffi::CString;
use std::os::raw::c_char;

//use lazy_static::lazy_static;


// Global linked list registry
lazy_static::lazy_static! {
    static ref LINKED_LISTS: Mutex<HashMap<i32, LinkedList>> = Mutex::new(HashMap::new());
}

#[derive(Default)]
pub struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn add_at_beginning(&mut self, value: i32) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn add_at_end(&mut self, value: i32) {
        let new_node = Box::new(Node { data: value, next: None });
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }
        let mut current = self.head.as_mut().unwrap();
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(new_node);
    }

    fn remove_from_beginning(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }

    fn remove_from_end(&mut self) {
        if self.head.is_none() {
            return; // List is empty
        }
    
        let mut current = self.head.as_mut().unwrap();
    
        if current.next.is_none() {
            // If there's only one element, remove it by setting `head` to None
            self.head = None;
            return;
        }
    
        while let Some(next_node) = current.next.take() {
            if next_node.next.is_none() {
                // We've reached the last node; break the borrow chain and remove it
                current.next = None;
                return;
            } else {
                // Restore the next_node link and move forward
                current.next = Some(next_node);
                current = current.next.as_mut().unwrap();
            }
        }
    }
    
    
    

    fn print_list(&self) -> String {
        let mut result = String::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.data));
            current = &node.next;
        }
        result.push_str("None");
        result
    }
}

#[no_mangle]
pub extern "C" fn linked_list_create(id: i32) {
    let mut lists = LINKED_LISTS.lock().unwrap();
    lists.insert(id, LinkedList::default());
}

#[no_mangle]
pub extern "C" fn linked_list_add_at_beginning(id: i32, value: i32) {
    let mut lists = LINKED_LISTS.lock().unwrap();
    if let Some(list) = lists.get_mut(&id) {
        list.add_at_beginning(value);
    }
}

#[no_mangle]
pub extern "C" fn linked_list_add_at_end(id: i32, value: i32) {
    let mut lists = LINKED_LISTS.lock().unwrap();
    if let Some(list) = lists.get_mut(&id) {
        list.add_at_end(value);
    }
}

#[no_mangle]
pub extern "C" fn linked_list_remove_from_beginning(id: i32) {
    let mut lists = LINKED_LISTS.lock().unwrap();
    if let Some(list) = lists.get_mut(&id) {
        list.remove_from_beginning();
    }
}

#[no_mangle]
pub extern "C" fn linked_list_remove_from_end(id: i32) {
    let mut lists = LINKED_LISTS.lock().unwrap();
    if let Some(list) = lists.get_mut(&id) {
        list.remove_from_end();
    }
}

#[no_mangle]
pub extern "C" fn linked_list_print(id: i32) -> *mut c_char {
    let lists = LINKED_LISTS.lock().unwrap();
    if let Some(list) = lists.get(&id) {
        let result = CString::new(list.print_list()).unwrap();
        return result.into_raw();
    }
    CString::new("List not found").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn linked_list_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}
