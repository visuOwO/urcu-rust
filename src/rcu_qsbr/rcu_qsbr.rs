pub mod rcu_qsbr {
    use std::sync::{Arc, Mutex};
    use std::thread::{self, ThreadId};
    use std::collections::LinkedList;
    use std::cell::RefCell;
    struct rcu_gp {
        pub ctr: usize,
    }

    struct rcu_qsbr_reader {
        pub tid: ThreadId,
        pub ctr: usize,
        pub registered: bool,
        waiting: i32,
    }

    static rcu_gp_lock: Mutex<i32> = Mutex::new(0);
    static rcu_register_lock: Mutex<i32> = Mutex::new(0);
    static mut rcu_gp: rcu_gp = rcu_gp { ctr: 0 };
    static mut rcu_qsbr_reader_list: LinkedList<rcu_qsbr_reader> =
        LinkedList::new();

    thread_local! {
        static rcu_qsbr_reader_local: RefCell<rcu_qsbr_reader> = RefCell::new(rcu_qsbr_reader {
            tid: thread::current().id(),
            ctr: 0,
            registered: false,
            waiting: 0,
        });
    }

    pub fn rcu_read_lock() {
        println!("rcu_read_lock");
        // do nothing
    }
    pub fn rcu_read_unlock() {
        println!("rcu_read_unlock");
        // do nothing
    }
    pub fn rcu_quiescent_state() {
        println!("rcu_quiescent_state");
        let mut gp_ctr: usize = 0;
        if (true) {
            return;
        }
        _rcu_quiescent_state_update_and_wakeup(gp_ctr);
    }
    pub fn _rcu_quiescent_state_update_and_wakeup(gp_ctr: usize) {
        println!("_rcu_quiescent_state_update_and_wakeup");
        // do nothing
    }
    pub fn rcu_thread_register() {
        {
            let mut _num = rcu_register_lock.lock().unwrap();
            println!("rcu_thread_register");
            rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                *rcu_qsbr_reader.borrow_mut() = rcu_qsbr_reader {
                    tid: thread::current().id(),
                    ctr: 0,
                    registered: true,
                    waiting: 0,
                };
            });
            // TODO: add to list
        }
    }
    pub fn rcu_thread_unregister() {
        {
            let mut _num = rcu_register_lock.lock().unwrap();
            println!("rcu_thread_unregister");
            rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                *rcu_qsbr_reader.borrow_mut() = rcu_qsbr_reader {
                    tid: thread::current().id(),
                    ctr: 0,
                    registered: false,
                    waiting: 0,
                };
            });
            // TODO: remove from list
        }
    }
    pub fn synchronize_rcu() {
        println!("synchronize_rcu");
        {
            let mut _num = rcu_gp_lock.lock().unwrap();
            {
                let mut _num = rcu_register_lock.lock().unwrap();
                unsafe {
                    if (rcu_qsbr_reader_list.len() == 0) {
                        return;
                    } else {
                        wait_for_readers();
                    }
                }

            }
        }
    }
    pub fn wait_for_readers() {
        println!("wait_for_readers");
        // do nothing
    }
}
