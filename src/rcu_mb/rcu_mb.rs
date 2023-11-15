pub mod rcu_mb {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::thread::current;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use std::collections::LinkedList;
    use crate::common::utils::{store_shared, load_shared, barrier};
    use crate::common::rcu_common::rcu_common::{RcuReader, RcuSync};


    static REGISTRY_LOCK: Mutex<()> = Mutex::new(());

    const RCU_GP_CTR_PHASE: usize = 0x1000;
    const RCU_NEST_MASK: usize = 0x0fff;
    const RCU_NEST_COUNT: usize = 0x1;

    lazy_static! {
        static ref RCU_SYNC: RefCell<RcuSync> = RefCell::new(RcuSync::new());
        static ref RCU_GP_LOCK: Arc<Mutex<LinkedList<RcuReader>>> = Arc::new(Mutex::new(LinkedList::new()));
        static ref RCU_GP_CTRL: AtomicUsize = AtomicUsize::new(RCU_NEST_COUNT);
    }

    pub fn rcu_reader_lock() {
        println!("rcu_reader_lock");
        let thread_id = current().id();
        let reader = RCU_SYNC.borrow_mut().reader_flags.get_mut(&thread_id).unwrap();
        let tmp = reader.ctr;
        if tmp & RCU_NEST_MASK == 0 {
            store_shared(&reader.ctr, RCU_GP_CTRL.load(Ordering::SeqCst));
            std::sync::atomic::fence(Ordering::SeqCst);
        } else {
            store_shared(&reader.ctr, tmp + RCU_NEST_COUNT)
        }
    }

    pub fn rcu_reader_unlock() {
        println!("rcu_reader_unlock");
        barrier();
        let thread_id = current().id();
        let reader = RCU_SYNC.borrow_mut().reader_flags.get_mut(&thread_id).unwrap();
        store_shared(&reader.ctr, reader.ctr - RCU_NEST_COUNT);
    }

    pub fn synchronize_rcu() {
        println!("synchronize_rcu");
        barrier();
        let _guard = RCU_GP_LOCK.lock().unwrap();
        update_counter_and_wait();
        barrier();
        update_counter_and_wait();
        barrier();
    }

    pub fn rcu_gp_ongoing(ctr: &usize) -> bool {
        let v = load_shared(ctr);
        (v & RCU_NEST_MASK) && ((v ^ RCU_GP_CTRL.load(Ordering::SeqCst)) & RCU_GP_CTR_PHASE)
    }

    pub fn update_counter_and_wait() {
        RCU_GP_CTRL.fetch_xor(RCU_GP_CTR_PHASE, Ordering::SeqCst);
        barrier();
        for reader in RCU_SYNC.borrow_mut().reader_flags.values() {
            while rcu_gp_ongoing(&reader.ctr) {
                std::thread::yield_now();
            }
        }
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
}