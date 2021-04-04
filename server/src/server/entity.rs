use std::sync::atomic::{AtomicU64, Ordering};

lazy_static::lazy_static! {
    static ref UUID_ALLOCATOR: AtomicU64 = AtomicU64::new(1);
}

pub fn get_uuid() -> u64 {
    UUID_ALLOCATOR.fetch_add(1, Ordering::SeqCst)
}
