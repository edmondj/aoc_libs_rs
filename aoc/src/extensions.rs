use std::str::pattern::Pattern;

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
    fn consume_until<'a, P: Pattern<'a>>(&'a self, pattern: P) -> (&'a Self, &'a Self);
}

impl StrExtensions for str {
    fn match_advance(&self, pattern: &Self) -> Option<&Self> {
        if self.starts_with(pattern) {
            Some(&self[pattern.len()..])
        } else {
            None
        }
    }

    fn consume_until<'a, P: Pattern<'a>>(&'a self, pattern: P) -> (&'a Self, &'a Self) {
        let found = self.find(pattern).unwrap_or(self.len());
        (&self[0..found], &self[found..])
    }
}
