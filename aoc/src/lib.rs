#![feature(pattern)]

mod extensions;
pub use crate::extensions::*;
pub mod geom;
mod macros;

use std::str::FromStr;
use std::{collections::BTreeMap, fmt, hash::Hash, io};
pub trait DisplayableDayResult: fmt::Display {}

impl<T: fmt::Display> DisplayableDayResult for T {}

#[derive(Debug, PartialEq, Eq)]
pub struct DayResult<P1: DisplayableDayResult, P2: DisplayableDayResult>(pub P1, pub P2);

impl<P1: DisplayableDayResult, P2: DisplayableDayResult> fmt::Display for DayResult<P1, P2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<P1: DisplayableDayResult, P2: DisplayableDayResult> From<(P1, P2)> for DayResult<P1, P2> {
    fn from((p1, p2): (P1, P2)) -> Self {
        Self(p1, p2)
    }
}

#[derive(Default)]
pub struct DayRegistry<Key: Default + Clone + fmt::Display + Hash + Eq> {
    days: BTreeMap<Key, Box<dyn Fn(&mut dyn io::Write, &str) -> Result<(), io::Error>>>,
}

#[derive(Debug)]
pub enum RunError {
    KeyNotFound,
    WriteError(io::Error),
}

impl From<io::Error> for RunError {
    fn from(value: io::Error) -> Self {
        RunError::WriteError(value)
    }
}

pub enum DayRunOption<Key> {
    All,
    Day(Key),
}

impl<Key> FromStr for DayRunOption<Key>
where
    Key: FromStr,
{
    type Err = <Key as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "all" => Self::All,
            s @ _ => Self::Day(s.parse::<Key>()?),
        })
    }
}

impl<Key: Default + Clone + fmt::Display + Hash + Eq + Ord + 'static> DayRegistry<Key> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_day<Res: DisplayableDayResult, D: Fn(&str) -> Res + 'static>(
        &mut self,
        key: Key,
        day: D,
    ) {
        self.days.insert(
            key.clone(),
            Box::new(move |out, input| writeln!(out, "{key}: {}", day(input))),
        );
    }

    pub fn run_day(&self, out: &mut dyn io::Write, key: &Key, input: &str) -> Result<(), RunError> {
        match self.days.get(key) {
            Some(run) => run(out, input).map_err(|e| e.into()),
            None => Err(RunError::KeyNotFound),
        }
    }

    pub fn run_days<'a>(
        &self,
        out: &mut dyn io::Write,
        days: impl Iterator<Item = (&'a Key, String)>,
    ) -> Result<(), RunError> {
        for (key, input) in days {
            self.run_day(out, key, input.as_str())?;
        }
        Ok(())
    }

    pub fn run(
        &self,
        out: &mut dyn io::Write,
        option: DayRunOption<Key>,
        get_input: impl Fn(&Key) -> String,
    ) -> Result<(), RunError> {
        match option {
            DayRunOption::All => {
                self.run_days(out, self.all_days().map(|key| (key, get_input(key))))
            }
            DayRunOption::Day(key) => self.run_day(out, &key, get_input(&key).as_str()),
        }
    }

    pub fn all_days(&self) -> impl Iterator<Item = &Key> {
        self.days.keys()
    }
}

pub fn exchange<T: Clone>(storage: &mut T, value: T) -> T {
    let old = storage.clone();
    *storage = value;
    old
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mid_day(input: &str) -> String {
        input.to_owned()
    }

    #[derive(Default)]
    struct FullDay;

    fn full_day(input: &str) -> DayResult<String, usize> {
        DayResult(input.to_string(), input.len())
    }

    #[test]
    fn run_days() {
        let mut days = DayRegistry::<u8>::new();
        days.add_day(0, mid_day);
        days.add_day(1, full_day);

        let mut buf = Vec::new();
        days.run_day(&1, &mut buf, "Hello").unwrap();
        days.run_day(&0, &mut buf, "World").unwrap();

        let res = std::str::from_utf8(&buf).unwrap();
        assert_eq!(res, "1: (Hello, 5)\n0: World\n");
    }
}
