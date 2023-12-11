pub mod twod {
    use crate::extensions::SliceExtensions;
    use core::fmt::Debug;
    use std::{
        ops::{Add, AddAssign, Div, Mul, Rem},
        str::FromStr,
    };

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
    pub struct Pos<T>(pub T, pub T);

    impl<T, U> From<(U, U)> for Pos<T>
    where
        T: From<U>,
    {
        fn from((x, y): (U, U)) -> Self {
            Pos(T::from(x), T::from(y))
        }
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
    pub struct Vec<T>(pub T, pub T);

    impl<T> Add<Vec<T>> for Pos<T>
    where
        T: Add<T, Output = T>,
    {
        type Output = Pos<T>;

        fn add(self, rhs: Vec<T>) -> Self::Output {
            Pos(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl<T> Add<Dir> for Pos<T>
    where
        T: Add<T, Output = T> + From<i8>,
    {
        type Output = Pos<T>;

        fn add(self, rhs: Dir) -> Self::Output {
            self.add(rhs.as_vec())
        }
    }

    impl<T> AddAssign<Vec<T>> for Pos<T>
    where
        T: AddAssign<T>,
    {
        fn add_assign(&mut self, rhs: Vec<T>) {
            self.0 += rhs.0;
            self.1 += rhs.1;
        }
    }

    impl<T> AddAssign<Dir> for Pos<T>
    where
        T: AddAssign<T> + From<i8> + Add<T, Output = T>,
    {
        fn add_assign(&mut self, rhs: Dir) {
            self.add_assign(rhs.as_vec())
        }
    }

    impl<T> Add<Vec<T>> for Vec<T>
    where
        T: Add<T, Output = T>,
    {
        type Output = Vec<T>;

        fn add(self, rhs: Vec<T>) -> Self::Output {
            Vec(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl<T, U> From<(U, U)> for Vec<T>
    where
        T: From<U>,
    {
        fn from((x, y): (U, U)) -> Self {
            Vec(T::from(x), T::from(y))
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Dir {
        Up,
        Right,
        Left,
        Down,
        UpRight,
        DownRight,
        DownLeft,
        UpLeft,
    }

    pub const ALL_DIR: [Dir; 8] = [
        Dir::Up,
        Dir::Right,
        Dir::Left,
        Dir::Down,
        Dir::UpRight,
        Dir::DownRight,
        Dir::DownLeft,
        Dir::UpLeft,
    ];

    pub fn all_dirs() -> impl Iterator<Item = Dir> {
        ALL_DIR.into_iter()
    }

    pub const ORTHO_DIR: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Left, Dir::Down];

    pub fn ortho_dirs() -> impl Iterator<Item = Dir> {
        ORTHO_DIR.into_iter()
    }

    impl Dir {
        pub fn as_vec<T: From<i8> + Add<T, Output = T>>(&self) -> Vec<T> {
            match self {
                Dir::Up => (0, -1).into(),
                Dir::Right => (1, 0).into(),
                Dir::Down => (0, 1).into(),
                Dir::Left => (-1, 0).into(),
                Dir::UpRight => Dir::Up.as_vec() + Dir::Right.as_vec(),
                Dir::DownRight => Dir::Down.as_vec() + Dir::Right.as_vec(),
                Dir::DownLeft => Dir::Down.as_vec() + Dir::Left.as_vec(),
                Dir::UpLeft => Dir::Up.as_vec() + Dir::Left.as_vec(),
            }
        }

        pub fn opposite(self) -> Self {
            match self {
                Dir::Up => Dir::Down,
                Dir::Right => Dir::Left,
                Dir::Left => Dir::Right,
                Dir::Down => Dir::Up,
                Dir::UpRight => Dir::DownLeft,
                Dir::DownRight => Dir::UpLeft,
                Dir::DownLeft => Dir::UpRight,
                Dir::UpLeft => Dir::DownRight,
            }
        }
    }

    pub struct Map<T, Cell> {
        pub width: T,
        pub height: T,
        pub cells: std::vec::Vec<Cell>,
    }

    impl<T, Cell, FromUsizeErr, IntoUsizeErr> Map<T, Cell>
    where
        FromUsizeErr: Debug,
        T: Mul<Output = T>
            + Add<Output = T>
            + TryFrom<usize, Error = FromUsizeErr>
            + PartialOrd
            + Copy,
        IntoUsizeErr: Debug,
        usize: TryFrom<T, Error = IntoUsizeErr>,
    {
        pub fn get(&self, Pos(x, y): Pos<T>) -> Option<&Cell> {
            if x >= T::try_from(0).unwrap()
                && x < self.width
                && y >= T::try_from(0).unwrap()
                && y < self.height
            {
                Some(&self.cells[usize::try_from(x + y * self.width).unwrap()])
            } else {
                None
            }
        }
    }

    impl<T, Cell, FromUsizeErr> Map<T, Cell>
    where
        Cell: PartialEq,
        FromUsizeErr: Debug,
        T: Div<Output = T> + TryFrom<usize, Error = FromUsizeErr> + Rem<Output = T> + Copy,
    {
        pub fn pos_of(&self, c: &Cell) -> Option<Pos<T>> {
            self.cells.index_of(c).map(|index| {
                Pos(
                    T::try_from(index).unwrap() % self.width,
                    T::try_from(index).unwrap() / self.width,
                )
            })
        }
    }

    impl<T, FromUsizeErr, Cell> FromStr for Map<T, Cell>
    where
        Cell: TryFrom<char>,
        FromUsizeErr: Debug,
        T: Default
            + Add
            + PartialEq
            + TryFrom<usize, Error = FromUsizeErr>
            + AddAssign
            + Debug
            + Mul<Output = T>
            + Copy,
    {
        type Err = <Cell as TryFrom<char>>::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut width = T::default();
            let mut height = T::default();
            let mut cells = std::vec::Vec::new();

            for line in s.lines() {
                for c in line.chars() {
                    cells.push(c.try_into()?);
                }
                if height == T::default() {
                    width = cells.len().try_into().unwrap();
                }
                height += T::try_from(1).unwrap();
            }
            assert_eq!(width * height, cells.len().try_into().unwrap());
            Ok(Self {
                width,
                height,
                cells,
            })
        }
    }
}
