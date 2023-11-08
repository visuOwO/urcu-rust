pub mod rcu_qsbr {
    use std::sync::{Arc, Mutex};
    use std::thread::{self, ThreadId};
    use std::collections::LinkedList;
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;
    use crate::utils::list::list::{cds_list_add, cds_list_empty, cds_list_head, cds_list_move, cds_list_splice, cds_list_del};
    use crate::{rcu_qsbr, utils};
    use crate::utils::mutex::mutex::compat_futex_noasync;

    struct rcu_gp {
        pub ctr: usize,
    }

    struct rcu_qsbr_reader {
        pub tid: ThreadId,
        pub ctr: usize,
        pub registered: bool,
        pub node: Option<*mut cds_list_head>,
        waiting: i32,
    }

    enum rcu_state {
        RCU_READER_ACTIVE_CURRENT,
        RCU_READER_ACTIVE_OLD,
        RCU_READER_INACTIVE,
    }

    static rcu_gp_lock: Mutex<i32> = Mutex::new(0);
    static rcu_register_lock: Mutex<i32> = Mutex::new(0);
    static mut rcu_gp: rcu_gp = rcu_gp { ctr: 0 };
    static registry: Option<*mut cds_list_head> = None;
    static gp_futex: Mutex<i32> = Mutex::new(0);

    thread_local! {
        static rcu_qsbr_reader_local: RefCell<rcu_qsbr_reader> = RefCell::new(rcu_qsbr_reader {
            tid: thread::current().id(),
            ctr: 0,
            registered: false,
            node: node{
                data: 0,
                next: None,
                prev: None,
            },
            waiting: 0,
        });
    }

    pub fn rcu_get_state(reader: &rcu_qsbr_reader) -> rcu_state {
        println!("rcu_get_state");
        let mut gp_ctr: usize = 0;
        let mut state: rcu_state = rcu_state::RCU_READER_INACTIVE;
        if reader.ctr == gp_ctr {
            state = rcu_state::RCU_READER_ACTIVE_CURRENT;
        } else if reader.ctr == gp_ctr - 1 {
            state = rcu_state::RCU_READER_ACTIVE_OLD;
        } else {
            state = rcu_state::RCU_READER_INACTIVE;
        }
        return state;
        // not sure if this is correct
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
            {
                let mut _num = rcu_register_lock.lock().unwrap();
                rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                    rcu_qsbr_reader.borrow_mut().ctr = 1;
                    *rcu_qsbr_reader.borrow_mut().registered = true;
                });
                rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                    unsafe {
                        registry.cds_list_add(rcu_qsbr_reader.borrow_mut().node);
                    }
                });
            }
        }
    }
    pub fn rcu_thread_unregister() {
        {
            let mut _num = rcu_register_lock.lock().unwrap();
            println!("rcu_thread_unregister");
            rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                *rcu_qsbr_reader.borrow_mut().ctr = 0;
                *rcu_qsbr_reader.borrow_mut().registered = false;
            });
            rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                unsafe {
                    cds_list_del(rcu_qsbr_reader.borrow_mut().node.unwrap());
                }
            });
        }
    }
    pub fn synchronize_rcu() {
        println!("synchronize_rcu");
        let mut qsreaders = utils::list::list::cds_list_head::new();
        let was_online = rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
            let mut reader = rcu_qsbr_reader.borrow_mut();
            return reader.ctr;
        });

        wait_for_readers(registry.unwrap(), *qsreaders);
    }

    pub fn rcu_thread_offline() {
        println!("rcu_thread_offline");
        let mut qsreaders = utils::list::list::cds_list_head::new();
        let was_online = rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
            let mut reader = rcu_qsbr_reader.borrow_mut();
            return reader.ctr;
        });
        if (was_online) {
            rcu_qsbr_reader_local.with(|rcu_qsbr_reader| {
                let mut reader = rcu_qsbr_reader.borrow_mut();
                reader.ctr = 0;
            });
            wake_up_gp();
        }
    }

    pub fn wake_up_gp() {
        println!("wake_up_gp");
        // do nothing
    }

    pub fn wait_for_readers(input_reader: *mut cds_list_head,  qsreaders: *mut cds_list_head) {
        println!("wait_for_readers");
        let mut wait_loops = 0;
        loop {
            wait_loops += 1;
            if (wait_loops > 1000) {
                let mut index = registry.unwrap();
                loop {
                    let mut reader = index.unwrap();
                    reader.waiting = 1;
                    if (index.unwrap().next == index) {
                        break;
                    }
                }
            }
            loop {
                let mut reader = input_reader.unwrap();
                if (reader.next == input_reader) {
                    break;
                }
                let mut state = rcu_get_state(reader);
                if (state == rcu_state::RCU_READER_ACTIVE_CURRENT) {
                    break;
                } else if (state == rcu_state::RCU_READER_ACTIVE_OLD) {
                    break;
                } else {
                    if (reader.waiting == 0) {
                        unsafe {
                            cds_list_move(reader, qsreaders);
                        }
                    }
                }
            }
            if (cds_list_empty(registry.unwrap())) {
                *gp_futex.lock().unwrap() = 0;
break;
            } else {
                wait_gp();
            }
        }
        cds_list_splice(qsreaders, registry.unwrap());
    }

    pub fn wait_gp() {
        println!("wait_gp");
        {
            let num = gp_futex.lock().unwrap();
            if (*num == -1) {
                compat_futex_noasync(*gp_futex.lock().unwrap(),
                                     utils::mutex::mutex::futex_stat::FUTEX_WAIT, -1,
                                     0);
            }
        }
    }
}
