mod interleave_iter;
mod weave_iter;

pub use crate::utils::interleave_iter::Interleave;
pub use crate::utils::weave_iter::{Weave, weave, weave_iter};

//Sample public function in a module
pub fn say_hello() {
    print!("hello");
}

pub trait IteratorExt<T>: Iterator<Item = T> {
    fn interleave<U>(self, other: U) -> Interleave<Self, U::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
        Self: Sized;

    fn weave<U>(self, other: U) -> Weave<Self, U::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
        Self: Sized;
}


impl<E, I> IteratorExt<E> for I
where
    I: Iterator<Item = E>,
{
    fn interleave<U>(self, other: U) -> Interleave<Self, U::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
        Self: Sized,
    {
        Interleave::new(self, other.into_iter())
    }

    fn weave<U>(self, other: U) -> Weave<Self, U::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
        U::IntoIter: Iterator<Item = Self::Item>,
        Self: Sized,
    {
        Weave::new(self, other.into_iter())
    }
}