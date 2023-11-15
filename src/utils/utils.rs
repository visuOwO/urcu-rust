use std::sync::atomic::{AtomicUsize, Ordering, fence};

// For atomic access
fn access_once_atomic<T>(value: &AtomicUsize) -> T
    where
        T: Copy,
{
    unsafe { std::mem::transmute(value.load(Ordering::SeqCst)) }
}

// For volatile access without atomicity guarantees
unsafe fn access_once_volatile<T>(value: *const T) -> T {
    std::ptr::read_volatile(value)
}

// LOAD_SHARED using atomic access
pub(crate) fn load_shared<T>(p: &T) -> T
    where
        T: Copy,
{
    access_once_atomic(p)
}

// STORE_SHARED using atomic access
pub(crate) fn store_shared<T>(x: &T, v: T)
    where
        T: Into<usize>,
{
    x.store(v.into(), Ordering::SeqCst);
}

// Memory barrier
pub fn barrier() {
    fence(Ordering::SeqCst);
}
