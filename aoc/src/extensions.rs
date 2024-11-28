use std::{mem::MaybeUninit, str::pattern::Pattern};

pub trait VecExtensions {
    type Item;

    fn remove_if<F>(&mut self, predicate: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> bool;
}

impl<T> VecExtensions for Vec<T> {
    type Item = T;

    fn remove_if<F>(&mut self, predicate: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> bool,
    {
        for (i, item) in self.iter().enumerate() {
            if predicate(item) {
                return Some(self.remove(i));
            }
        }
        None
    }
}

pub trait SliceExtensions {
    type Item;

    fn index_of(&self, item: &Self::Item) -> Option<usize>;
}

impl<T> SliceExtensions for [T]
where
    T: PartialEq,
{
    type Item = T;

    fn index_of(&self, item: &Self::Item) -> Option<usize> {
        self.iter().position(|v| v == item)
    }
}

pub trait StrExtensions {
    fn match_advance(&self, pattern: &Self) -> Option<&Self>;
    fn consume_until<'a, P: Pattern>(&'a self, pattern: P) -> (&'a Self, &'a Self);
    fn split_exact<'a, const N: usize, P: Pattern>(&'a self, pattern: P) -> Option<[&'a Self; N]>;
}

impl StrExtensions for str {
    fn match_advance(&self, pattern: &Self) -> Option<&Self> {
        if self.starts_with(pattern) {
            Some(&self[pattern.len()..])
        } else {
            None
        }
    }

    fn consume_until<'a, P: Pattern>(&'a self, pattern: P) -> (&'a Self, &'a Self) {
        let found = self.find(pattern).unwrap_or(self.len());
        (&self[0..found], &self[found..])
    }

    fn split_exact<'a, const N: usize, P: Pattern>(&'a self, pattern: P) -> Option<[&'a Self; N]> {
        let mut arr: [MaybeUninit<&'a Self>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut splitted = self.split(pattern);

        for i in 0..N {
            match splitted.next() {
                Some(s) => {
                    arr[i].write(s);
                }
                None => {
                    for j in 0..i {
                        unsafe { arr[j].assume_init_drop() };
                        return None;
                    }
                }
            }
        }

        let arr = unsafe { arr.transpose().assume_init() };

        if splitted.next().is_some() {
            None
        } else {
            Some(arr)
        }
    }
}
