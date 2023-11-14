pub mod mutex {
    use std::sync::{Arc, Mutex, Condvar, MutexGuard};
    struct timespec {
        tv_sec: i64,
        tv_nsec: i64,
    }
    pub(crate) enum futex_stat {
        FUTEX_WAIT,
        FUTEX_WAKE,
    }

    static futex_lock: Mutex<i32> = Mutex::new(0);
    static cond_var: Arc<Condvar> = Arc::new(Condvar::new());
    pub fn compat_futex_noasync(uaddr: MutexGuard<i32>, futex_op: futex_stat, val: i32, val3: i32) -> i32 {
        let mut ret: i32 = 0;
        let mut oldval: i32 = 0;
        let mut newval: i32 = 0;
        let mut gret: i32 = 0;
        {
            ret = *futex_lock.lock().unwrap();
            if ret != 0 {
                return ret;
            }
            unsafe {
                if (futex_op == futex_stat::FUTEX_WAIT) {
                    oldval = *uaddr;
                    if oldval != val {
                        return -1;
                    }
                    *uaddr = val3;
                    cond_var.wait(futex_lock.lock().unwrap());
                    *uaddr = oldval;
                } else if (futex_op == futex_stat::FUTEX_WAKE) {
                    for i in 0..val {
                        cond_var.notify_one();
                    }
                }
                gret = -22;
            }
        }

        return gret;
    }
}