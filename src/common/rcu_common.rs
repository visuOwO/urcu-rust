pub(crate) mod rcu_common {
    use std::collections::HashMap;
    use std::sync::atomic::AtomicUsize;
    use std::thread::ThreadId;
    use lazy_static::lazy_static;

    pub(crate) struct RcuReader {
        pub(crate) ctr: usize,
        tid: ThreadId,
        id: usize,
        registered: bool,
    }

    lazy_static!(
        pub(crate) static ref RCU_SYNC: RcuSync = RcuSync::new();
        pub(crate) static ref REGISTRY_LOCK: Mutex<()> = Mutex::new(());
    );

    impl RcuReader {
        pub(crate) fn new() -> Self {
            Self {
                ctr: 0,
                tid: std::thread::current().id(),
                id: usize::MAX,
                registered: false,
            }
        }
    }

    pub(crate) struct RcuSync {
        pub(crate) reader_flags: HashMap<ThreadId, RcuReader>,
        total_threads: AtomicUsize,
    }

    impl RcuSync {
        fn new() -> RcuSync {
            RcuSync {
                reader_flags: HashMap::new(),
                total_threads: AtomicUsize::new(0),
            }
        }
    }
}