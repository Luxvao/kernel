use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T> {
    var: UnsafeCell<T>,
    locked: AtomicBool,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

unsafe impl<T> Send for Mutex<T> {}

unsafe impl<T> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(var: T) -> Mutex<T> {
        Mutex {
            var: UnsafeCell::new(var),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            if let Ok(locked) =
                self.locked
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            {
                if !locked {
                    break;
                }
            }
        }

        MutexGuard { mutex: self }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.var.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.var.get() }
    }
}
