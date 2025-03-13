pub struct Weave<I, O> {
    start: Option<I>,
    other: Vec<Option<O>>,
    next_from: WeaveNextIterator,
}

#[derive(Clone, Copy, PartialEq)]
enum WeaveNextIterator {
    Initial,
    NextAt(usize),
}

impl WeaveNextIterator {
    fn get_next(self) -> Self {
        let next = match self {
            WeaveNextIterator::Initial => WeaveNextIterator::NextAt(0),
            WeaveNextIterator::NextAt(idx) => WeaveNextIterator::NextAt(idx + 1),
        };
        next
    }
    fn get_next_wrap(self, len: usize) -> Self {
        let next = self.get_next();
        match next {
            WeaveNextIterator::NextAt(idx) => {
                if idx >= len {
                    WeaveNextIterator::Initial
                } else {
                    next
                }
            }
            _ => next,
        }
    }
}

// missing Iterator implementation for Weave, not sure
// if there is a nice way to chain more than one iterator and get the results
// in the same manner as in functions weave or weave_iter
//
impl<I, O> Weave<I, O> {
    pub fn new(start: I, to_weave: O) -> Weave<I, O> {
        Weave {
            start: Some(start),
            other: vec![Some(to_weave)],
            next_from: WeaveNextIterator::Initial,
        }
    }
    pub fn weave<It0, It1>(mut weave: Weave<It0, It1>, next_to_weave: It1) -> Weave<It0, It1>
    where
        It0: Iterator,
        It1: Iterator<Item = It0::Item>,
    {
        weave.other.push(Some(next_to_weave));
        weave
    }

}

impl<I, O> Iterator for Weave<I, O>
where
    I: Iterator,
    O: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = Option::<Self::Item>::None;

        let current_iter = self.next_from;
        let mut next_iter = current_iter;
        while next.is_none() {

            match next_iter {
                WeaveNextIterator::Initial => { next = get_next_or_fuse(&mut self.start); },
                WeaveNextIterator::NextAt(idx) => { next = get_next_or_fuse(&mut self.other[idx]);},
            }
            next_iter = next_iter.get_next_wrap(self.other.len());

            if next_iter == current_iter {
                break;
            }
        }
        self.next_from = next_iter;
        next
    }
}

pub fn weave<'a, T>(collections: &'a [&'a [T]]) -> impl Iterator<Item = &'a T> {
    let item_count: usize = collections.iter().map(|c| c.len()).sum();
    let iters: Vec<_> = collections.iter().map(|c| c.iter()).collect();

    let weave = (0..item_count).scan(iters, |iters, idx| {
        let mut curr_idx = idx % iters.len();

        let mut next_value = Option::<&T>::None;
        while curr_idx < iters.len() {
            next_value = iters[curr_idx].next();
            match next_value {
                None => {
                    curr_idx = (curr_idx + 1) % iters.len();
                }
                _ => {
                    break;
                }
            }
        }

        next_value
    });

    weave
}

pub fn weave_iter<'a, C, I, E>(coll_iter: C) -> impl Iterator<Item = &'a E>
where
    C: Iterator<Item = I>,
    I: IntoIterator<Item = &'a E> + 'a,
    E: 'a + ?Sized,
{
    let iters: Vec<_> = coll_iter.map(|i| i.into_iter()).collect();

    let weave = (0..).scan(iters, |state, idx| {
        let offset = idx % state.len();
        let mut next_value = Option::<&E>::None;

        let mut idx = 0;
        while idx < state.len() {
            let curr_idx = (offset + idx) % state.len();
            next_value = state[curr_idx].next();

            match next_value {
                None => {
                    idx += 1;
                }
                _ => {
                    break;
                }
            }
        }
        next_value
    });

    weave
}

#[inline]
fn get_next_or_fuse<T>(iter: &mut Option<T>) -> Option<T::Item>
where T: Iterator
{
    let next = iter.as_mut()?.next();
    if next.is_none() {
        *iter = None
    }
    next
    
}