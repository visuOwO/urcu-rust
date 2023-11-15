pub mod rcu_qsbr {
    use std::sync::Mutex;
    use std::thread::{self, yield_now};
    use std::cell::RefCell;
    use std::ops::Deref;
    use lazy_static::lazy_static;
    use crate::common::rcu_common::rcu_common::RcuReader;
    use crate::common::utils::{barrier, load_shared, store_shared};

    static RCU_GP_LOCK: Mutex<i32> = Mutex::new(0);
    static REGISTRY_LOCK: Mutex<()> = Mutex::new(());
    static GP_FUTEX: Mutex<i32> = Mutex::new(0);
    static GP_CTR: usize = 1;
    static RCU_GP_CTR: usize = 0x2;
    static RCU_GP_ONLINE: usize = 0x1;

    lazy_static! {
        static ref RCU_SYNC: RefCell<RcuSync> = RefCell::new(RcuSync::new());
        static ref RCU_REGISTER_LOCK: Mutex<i32> = Mutex::new(0);
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
        let reader = RCU_SYNC.borrow_mut().read_flags.get_mut(&thread::current().id()).unwrap();
        store_shared(&reader.ctr, GP_CTR);
    }

    pub fn rcu_register_thread() {
        println!("rcu_register_thread");
        let mut guard = REGISTRY_LOCK.lock().unwrap();
        let tid = std::thread::current().id();
        RCU_SYNC.borrow_mut().reader_flags.insert(tid, RcuReader::new());
        guard.deref();
    }

    pub fn rcu_unregister_thread() {
        println!("rcu_unregister_thread");
        let mut guard = REGISTRY_LOCK.lock().unwrap();
        let tid = std::thread::current().id();
        RCU_SYNC.borrow_mut().reader_flags.remove(&tid);
        guard.deref();
    }

    fn rcu_gp_ongoing(x: &usize) -> bool {
        let v = load_shared(x);
        v && usize::from((v != GP_CTR))
    }

    pub fn update_counter_and_wait() {
        let reader: RcuReader = RCU_SYNC.borrow_mut().read_flags.get_mut(&thread::current().id()).unwrap();
        store_shared(&reader.ctr, GP_CTR +RCU_GP_CTR);
        barrier();
        drop(reader);
        for reader in RCU_SYNC.borrow_mut().reader_flags.values() {
            while rcu_gp_ongoing(&reader.ctr) {
                yield_now();
            }
        }
    }

    pub fn synchronize_rcu() {
        println!("synchronize_rcu");
        let reader: RcuReader = RCU_SYNC.borrow_mut().read_flags.get_mut(&thread::current().id()).unwrap();
        let was_online = reader.ctr;
        if was_online {
            barrier();
            store_shared(&reader.ctr, 0);
        }

        let _guard = GP_FUTEX.lock().unwrap();
        update_counter_and_wait();
        drop(_guard);
        if was_online {
            barrier();
            store_shared(&reader.ctr, RCU_GP_ONLINE);
        }
        barrier();
    }
}
