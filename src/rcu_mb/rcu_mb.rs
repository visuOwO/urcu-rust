pub mod rcu_mb {
    use std::cell::RefCell;
    use std::thread::ThreadId;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize};
    use std::sync::Mutex;
    use crate::utils::list::list::cds_list_head;

    struct rcu_sync {
        reader_flags: Vec<AtomicBool>,
        list_head: Option<*mut cds_list_head>,
        total_threads: AtomicUsize,
    }

    struct rcu_reader {
        ctr: AtomicBool,
        tid: ThreadId,
        id: usize,
        registered: bool,
    }

    static regisrer_lock: Mutex<()> = Mutex::new(());

    static RCU_SYNC: rcu_sync = rcu_sync {
        reader_flags: Vec::new(),
        list_head: cds_list_head::new(),
        total_threads: AtomicUsize::new(0),
    };

    thread_local! {
        static rcu_reader_local: RefCell<rcu_reader> = std::cell::RefCell::new(rcu_reader {
            ctr: 0,
            tid: std::thread::current().id(),
            id: -1,
        });
    }
    pub fn rcu_reader_lock() {
        println!("rcu_reader_lock");
        let id = rcu_reader_local.with(|rcu_reader| {
            let mut reader = rcu_reader.borrow_mut();
            reader.ctr.store(true, std::sync::atomic::Ordering::Release);
            reader.id
        });
        assert_ne!(id, -1);
        RCU_SYNC.reader_flags[id].store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn rcu_reader_unlock() {
        println!("rcu_reader_unlock");
        let id = rcu_reader_local.with(|rcu_reader| {
            let mut reader = rcu_reader.borrow_mut();
            reader.ctr.store(false, std::sync::atomic::Ordering::Release);
            reader.id
        });
        assert_ne!(id, -1);
        RCU_SYNC.reader_flags[id].store(false, std::sync::atomic::Ordering::Release);
    }

    pub fn synchronize_rcu() {
        println!("synchronize_rcu");
        loop {
            let mut all_done = true;
            for i in 0..RCU_SYNC.total_threads.load(std::sync::atomic::Ordering::SeqCst) {
                if RCU_SYNC.reader_flags[i].load(std::sync::atomic::Ordering::Acquire) {
                    all_done = false;
                    break;
                }
            }
            if all_done {
                break;
            }
        }
    }

    pub fn rcu_register_thread() {
        println!("rcu_register_thread");
        {
            let num = regisrer_lock.lock().unwrap();
            rcu_reader_local.with(|rcu_reader| {
                let mut reader = rcu_reader.borrow_mut();
                reader.ctr.store(true, std::sync::atomic::Ordering::Release);
                reader.id = RCU_SYNC.total_threads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
        }
    }

    pub fn rcu_unregister_thread() {
        println!("rcu_unregister_thread");
        {
            let num = regisrer_lock.lock().unwrap();
            rcu_reader_local.with(|rcu_reader| {
                let mut reader = rcu_reader.borrow_mut();
                reader.ctr.store(false, std::sync::atomic::Ordering::Release);
                reader.id = 0;
                RCU_SYNC.total_threads.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                reader.registered = false;
            });
        }
    }
}