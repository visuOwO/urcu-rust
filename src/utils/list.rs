pub mod list {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};

    pub struct cds_list_head {
        pub(crate) next: Option<*mut cds_list_head>,
        prev: Option<*mut cds_list_head>,
    }

    pub fn new() -> cds_list_head {
        let mut m = cds_list_head {
            next: None,
            prev: None,
        };
        m.next = Some(&m as *const cds_list_head as *mut cds_list_head);
        m.prev = Some(&m as *const cds_list_head as *mut cds_list_head);
        return m;
    }
    pub fn cds_list_add(head: *mut cds_list_head, new: *mut cds_list_head) {
        unsafe {
            let mut old = (*head).next.unwrap();
            (*new).next = Some(old);
            (*new).prev = Some(head);
            (*old).prev = Some(new);
            (*head).next = Some(new);
        }
    }
    pub fn cds_list_del(head: *mut cds_list_head) {
        unsafe {
            let mut old = (*head).next.unwrap();
            let mut new = (*head).prev.unwrap();
            (*new).next = Some(old);
            (*old).prev = Some(new);
        }
    }
    pub fn cds_list_empty(head: *mut cds_list_head) -> bool {
        return unsafe { (*head).next.unwrap() == head };
    }

    pub fn cds_list_move(elem: *mut cds_list_head, head: *mut cds_list_head) {
        unsafe {
            cds_list_del(elem);
            cds_list_add(head, elem);
        }
    }

    pub fn cds_list_splice(list: *mut cds_list_head, head: *mut cds_list_head) {
        unsafe {
            if !cds_list_empty(list) {
                let mut first = (*list).next.unwrap();
                let mut last = (*list).prev.unwrap();
                let mut at = (*head).next.unwrap();
                (*first).prev = Some(head);
                (*head).next = Some(first);
                (*last).next = Some(at);
                (*at).prev = Some(last);
            }
        }
    }

}