use colosseum::{sync::Arena as SyncArena, unsync::Arena as UnsyncArena};

// Abstract over sync and unsync arena
pub trait Arena<T> {
    fn new() -> Self;

    fn with_capacity(n: usize) -> Self;

    fn alloc(&self, t: T) -> &mut T;

    fn alloc_extend<I: Iterator<Item = T>>(&self, iterable: I) -> &mut [T];
}

impl<T> Arena<T> for UnsyncArena<T> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(n: usize) -> Self {
        Self::with_capacity(n)
    }
    fn alloc(&self, t: T) -> &mut T {
        self.alloc(t)
    }
    fn alloc_extend<I: Iterator<Item = T>>(&self, iterable: I) -> &mut [T] {
        self.alloc_extend(iterable)
    }
}

impl<T> Arena<T> for SyncArena<T> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(n: usize) -> Self {
        Self::with_capacity(n)
    }
    fn alloc(&self, t: T) -> &mut T {
        self.alloc(t)
    }
    fn alloc_extend<I: Iterator<Item = T>>(&self, iterable: I) -> &mut [T] {
        self.alloc_extend(iterable)
    }
}