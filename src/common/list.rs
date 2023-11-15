pub mod list {
    pub struct CdsListHead {
        pub(crate) next: Option<*mut CdsListHead>,
        prev: Option<*mut CdsListHead>,
        data: i32,
    }

    pub fn new() -> CdsListHead {
        let mut m = CdsListHead {
            next: None,
            prev: None,
            data: -1,
        };
        m.next = Some(&m as *const CdsListHead as *mut CdsListHead);
        m.prev = Some(&m as *const CdsListHead as *mut CdsListHead);
        return m;
    }
    pub fn cds_list_add(head: *mut CdsListHead, new: *mut CdsListHead) {
        unsafe {
            let mut old = (*head).next.unwrap();
            (*new).next = Some(old);
            (*new).prev = Some(head);
            (*old).prev = Some(new);
            (*head).next = Some(new);
        }
    }
    pub fn cds_list_del(head: *mut CdsListHead) {
        unsafe {
            let mut old = (*head).next.unwrap();
            let mut new = (*head).prev.unwrap();
            (*new).next = Some(old);
            (*old).prev = Some(new);
        }
    }
    pub fn cds_list_empty(head: *mut CdsListHead) -> bool {
        return unsafe { (*head).next.unwrap() == head };
    }

    pub fn cds_list_move(elem: *mut CdsListHead, head: *mut CdsListHead) {
        unsafe {
            cds_list_del(elem);
            cds_list_add(head, elem);
        }
    }

    pub fn cds_list_splice(list: *mut CdsListHead, head: *mut CdsListHead) {
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