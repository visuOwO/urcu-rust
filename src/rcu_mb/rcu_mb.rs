pub mod rcu_mb {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::thread::ThreadId;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    struct RcuSync {
        reader_flags: HashMap<ThreadId, AtomicBool>,
        total_threads: AtomicUsize,
    }

    static REGISTRY_LOCK: Mutex<()> = Mutex::new(());

    impl RcuSync {
        fn new() -> RcuSync {
            RcuSync {
                reader_flags: HashMap::new(),
                total_threads: AtomicUsize::new(0),
            }
        }

        fn set_reader_flag(&mut self, thread_id: ThreadId, value: bool) {
            let flag = self.reader_flags.entry(thread_id).or_insert_with(|| AtomicBool::new(false));
            flag.store(value, Ordering::Release);
        }

        fn all_readers_inactive(&self) -> bool {
            self.reader_flags.values().all(|flag| !flag.load(Ordering::Acquire))
        }

        fn remove_reader_flag(&mut self, thread_id: &ThreadId) {
            self.reader_flags.remove(thread_id);
        }
    }

    lazy_static! {
        static ref RCU_SYNC: RefCell<RcuSync> = RefCell::new(RcuSync::new());
    }

    thread_local! {
        static RCU_READER: RefCell<rcu_reader> = std::cell::RefCell::new(rcu_reader {
            ctr: AtomicBool.new(false),
            tid: std::thread::current().id(),
            id: -1,
        });
    }

    struct RcuReader {
        ctr: AtomicBool,
        tid: ThreadId,
        id: usize,
        registered: bool,
    }

    impl RcuReader {
        fn new() -> Self {
            Self {
                ctr: AtomicBool::new(false),
                tid: std::thread::current().id(),
                id: usize::MAX,
                registered: false,
            }
        }
    }

    pub fn rcu_reader_lock() {
        println!("rcu_reader_lock");
        let tid = std::thread::current().id();
        RCU_SYNC.set_reader_flag(tid, true);
    }

    pub fn rcu_reader_unlock() {
        println!("rcu_reader_unlock");
        let tid = std::thread::current().id();
        RCU_SYNC.set_reader_flag(tid, false);
    }

    pub fn synchronize_rcu() {
        println!("synchronize_rcu");
        while !RCU_SYNC.all_reader_inactive() {
            std::thread::yield_now();
        }
    }

    pub fn rcu_register_thread() {
        println!("rcu_register_thread");
        let mut guard = REGISTRY_LOCK.lock().unwrap();
        let tid = std::thread::current().id();
        RCU_SYNC.set_reader_flag(tid, false);
        guard.deref();
    }

    pub fn rcu_unregister_thread() {
        println!("rcu_unregister_thread");
        let mut guard = REGISTRY_LOCK.lock().unwrap();
        let tid = std::thread::current().id();
        RCU_SYNC.remove_reader_flag(&tid);
        guard.deref();
    }
}