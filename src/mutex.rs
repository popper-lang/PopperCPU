use core::cell::UnsafeCell;

pub struct AtomicRefCounter {
    count: core::sync::atomic::AtomicUsize,
}

impl AtomicRefCounter {
    pub fn new() -> Self {
        AtomicRefCounter {
            count: core::sync::atomic::AtomicUsize::new(1),
        }
    }

    pub fn clone(&self) {
        self.count.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }

    pub fn drop(&self) {
        if self.count.fetch_sub(1, core::sync::atomic::Ordering::Release) == 1 {
            core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        }
    }
}


pub struct Mutex<T> {
    data: UnsafeCell<T>,
    lock: AtomicRefCounter,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex {
            data: UnsafeCell::new(data),
            lock: AtomicRefCounter::new(),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.lock.clone(); // Incrémente le compteur de référence
        MutexGuard {
            data: unsafe { &mut *self.data.get() },
            lock: &self.lock,
        }
    }
}

#[allow(dead_code)]
pub struct MutexGuard<'a, T> {
    data: &'a mut T,
    lock: &'a AtomicRefCounter,
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.drop();
    }
}