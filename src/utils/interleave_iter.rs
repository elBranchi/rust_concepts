pub struct Interleave<F, B> {
    front: Option<F>,
    back: Option<B>,
    use_front: bool,
}

impl<F, B> Interleave<F, B> {
    pub fn new(f: F, b: B) -> Interleave<F, B> {
        Interleave {
            front: Some(f),
            back: Some(b),
            use_front: true,
        }
    }
}
impl<F, B> Iterator for Interleave<F, B>
where
    F: Iterator,
    B: Iterator<Item = F::Item>,
{
    type Item = F::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.use_front {
            call_and_clear(&mut self.front, Iterator::next)
                .or_else(|| call_and_clear(&mut self.back, Iterator::next))
        } else {
            call_and_clear(&mut self.back, Iterator::next)
                .or_else(|| call_and_clear(&mut self.front, Iterator::next))
        };
        self.use_front = (!self.use_front && !self.front.is_none() && !self.back.is_none())
            || (self.use_front && self.back.is_none())
            || (!self.use_front && self.front.is_none());
        next
    }
}

//Inspired by similar function for chain implementation
#[inline]
fn call_and_clear<T, U>(opt: &mut Option<T>, f: impl FnOnce(&mut T) -> Option<U>) -> Option<U> {
    let result = f(opt.as_mut()?);
    if result.is_none() {
        *opt = None;
    }
    result
}
