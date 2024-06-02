use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

pub struct Lazy<T> {
    lazy_static: UnsafeCell<Option<T>>,
    get_static: fn() -> T,
}

unsafe impl<T> Send for Lazy<T> {}

unsafe impl<T> Sync for Lazy<T> {}

impl<T> Lazy<T> {
    pub const fn new(f: fn() -> T) -> Lazy<T> {
        Lazy {
            lazy_static: UnsafeCell::new(None),
            get_static: f,
        }
    }

    fn initialize(&self) {
        let lazy_static = self.lazy_static.get();

        unsafe {
            if (*lazy_static).is_none() {
                *lazy_static = Some((self.get_static)());
            }
        }
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.initialize();

        unsafe { self.lazy_static.get().as_ref_unchecked().as_ref().unwrap() }
    }
}

impl<T> DerefMut for Lazy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.initialize();

        self.lazy_static.get_mut().as_mut().unwrap()
    }
}
