mod interleave_iter;
mod weave_iter;

pub use crate::utils::interleave_iter::Interleave;
pub use crate::utils::weave_iter::{Weave, weave, weave_iter};

//Sample public function in a module
pub fn say_hello() {
    print!("hello");
}

pub trait IteratorExt<'i, T>: Iterator<Item = T> + 'i {
    fn interleave<U>(self, other: U) -> Interleave<Self, U::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
        Self: Sized;

    fn weave<'o, 'w, O>(self, other: O) -> Weave<'w, Self::Item>
    where
        'i: 'w,
        'o: 'w,
        O: IntoIterator<Item = Self::Item> + 'o,
        Self: Sized;
}

impl<'i, E, I> IteratorExt<'i, E> for I
where
    I: Iterator<Item = E> + 'i,
{
    fn interleave<U>(self, other: U) -> Interleave<Self, U::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
        Self: Sized,
    {
        Interleave::new(self, other.into_iter())
    }

    fn weave<'o, 'w, O>(self, other: O) -> Weave<'w, E>
    where
        'i: 'w,
        'o: 'w,
        O: IntoIterator<Item = Self::Item> + 'o,
        O::IntoIter: Iterator<Item = Self::Item>,
        Self: Sized + 'i,
    {
        Weave::<'w, Self::Item>::new(self, other.into_iter())
    }
}
