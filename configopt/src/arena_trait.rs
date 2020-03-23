use colosseum::{sync::Arena as SyncArena, unsync::Arena as UnsyncArena};

// Abstract over sync and unsync arena
pub trait Arena<T> {
    fn new() -> Self;

    fn with_capacity(n: usize) -> Self;

    #[allow(clippy::mut_from_ref)]
    fn alloc(&self, t: T) -> &mut T;
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
}

impl<T> Arena<T> for SyncArena<T> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(n: usize) -> Self {
        Self::with_capacity(n)
    }

    #[allow(clippy::mut_from_ref)]
    fn alloc(&self, t: T) -> &mut T {
        self.alloc(t)
    }
}
