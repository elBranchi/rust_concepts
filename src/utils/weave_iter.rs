// While adding sample tests, noticed that the Weave iterator wasn't behaving in the expected
// way (chained weave still behaved as chained interleaves).
// So the implementation needs to be changed
type WrappedNext<'a, E> = Box<dyn FnMut() -> Option<E> + 'a>;
pub struct Weave<'w, E> {
    fibers: Vec<WrappedNext<'w, E>>,
    next_from: usize,
}


// missing Iterator implementation for Weave, not sure
// if there is a nice way to chain more than one iterator and get the results
// in the same manner as in functions weave or weave_iter
//
impl<'w, E> Weave<'w, E> {
    pub fn new<'f1, 'f2, F1, F2>(first: F1, second: F2) -> Weave<'w, E>
    where
        F1: Iterator<Item = E> + 'f1,
        F2: Iterator<Item = E> + 'f2,
        'f1: 'w,
        'f2: 'w,
    {
        let mut weave = Weave {
            fibers: Vec::new(),
            next_from: 0,
        };

        weave.fibers.push(Weave::<'w, E>::wrapped_next(first));
        weave.fibers.push(Weave::<'w, E>::wrapped_next(second));
        weave
    }

    fn wrapped_next<'a, T, IT>(mut iter: IT) -> WrappedNext<'a, T>
    where
        IT: Iterator<Item = T> + 'a,
    {
        let mut fused = false;
        let wrap = move || {
            let mut next = Option::<T>::None;
            if !fused {
                next = iter.next();
                if next.is_none() {
                    fused = true;
                }
            }
            next
        };
        Box::new(wrap)
    }
    pub fn continue_weave<'f, F1>(mut self, fiber: F1) -> Self
    where
        F1: Iterator<Item = E> + 'f,
        'f: 'w,
    {
        self.fibers.push(Self::wrapped_next(fiber));
        self
    }
}

impl<'w, E> Iterator for Weave<'w, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let start_on = self.next_from;

        let mut next_found = false;
        let mut next = Option::<E>::None;

        while !next_found {
            next = self.fibers[self.next_from]();
            self.next_from = (self.next_from + 1) % self.fibers.len();

            next_found = !next.is_none() || self.next_from == start_on;
        }
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

#[cfg(test)]
mod tests {
    use crate::utils::IteratorExt;

    use super::*;

    #[test]
    fn test_weave_iterator() {
        let a_s = (0..=4).map(|v| v * 2); // 0, 2, 4, 6, 8
        let b_s = (1..3).map(|v| v * 5); // 5, 10
        let c_s = (1..=6).map(|v| v * 7); // 7, 14, 21, 28, 35, 42

        let weaved = a_s.weave(b_s).continue_weave(c_s);
        assert_eq!(
            weaved.collect::<Vec<_>>(),
            [0, 5, 7, 2, 10, 14, 4, 21, 6, 28, 8, 35, 42]
        );
    }

    #[test]
    fn test_weave_function() {
        let ones = [1; 3];
        let twos = [2; 6];
        let threes = [3; 2];

        let param = [&ones[..], &twos, &threes];
        let result = weave(&param[..]).map(|r| *r).collect::<Vec<_>>();
        assert_eq!(result, [1, 2, 3, 1, 2, 3, 1, 2, 2, 2, 2]);
    }

    #[test]
    fn test_weave_iter_function() {
        let ones = [1; 3];
        let twos = [2; 6];
        let threes = [3; 2];

        let param = [ones[..].iter(), twos.iter(), threes.iter()];

        let result = weave_iter(param.into_iter())
            .map(|v| *v)
            .collect::<Vec<_>>();

        assert_eq!(result, [1, 2, 3, 1, 2, 3, 1, 2, 2, 2, 2]);
    }
}
