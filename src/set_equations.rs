use std::marker::PhantomData;

pub struct SetType<T> {
    marker: PhantomData<T>,
}

impl<T: std::fmt::Debug> SetType<T> {
    pub fn empty() -> T {
        todo!()
    }

    pub fn union(t1: T, t2: T) -> T {
        todo!()
    }
    pub fn inter(t1: T, t2: T) -> T {
        todo!()
    }
    pub fn diff(t1: T, t2: T) -> T {
        todo!()
    }
    pub fn is_empty(t: T) -> bool {
        todo!()
    }
    pub fn equal(t1: T, t2: T) -> bool {
        todo!()
    }
}
