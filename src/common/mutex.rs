pub mod mutex {
    use std::sync::{Arc, Mutex, Condvar, MutexGuard};
    struct Timespec {
        tv_sec: i64,
        tv_nsec: i64,
    }
    pub(crate) enum FutexStat {
        FutexWait,
        FutexWake,
    }

    static FUTEX_LOCK: Mutex<i32> = Mutex::new(0);
    static COND_VAR: Arc<Condvar> = Arc::new(Condvar::new());
    pub fn compat_futex_noasync(uaddr: MutexGuard<i32>, futex_op: FutexStat, val: i32, val3: i32) -> i32 {
        let mut ret: i32 = 0;
        let mut oldval: i32 = 0;
        let mut gret: i32 = 0;
        {
            ret = *FUTEX_LOCK.lock().unwrap();
            if ret != 0 {
                return ret;
            }
            unsafe {
                if futex_op == FutexStat::FutexWait {
                    oldval = *uaddr;
                    if oldval != val {
                        return -1;
                    }
                    *uaddr = val3;
                    let _ = COND_VAR.wait(FUTEX_LOCK.lock().unwrap()).expect("TODO: panic message");
                    *uaddr = oldval;
                } else if futex_op == FutexStat::FutexWake {
                    for _i in 0..val {
                        COND_VAR.notify_one();
                    }
                }
                gret = -22;
            }
        }

        return gret;
    }
}