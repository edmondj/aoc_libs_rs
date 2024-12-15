pub mod twod {
    use crate::extensions::SliceExtensions;
    use core::fmt::{self, Debug};
    use std::{
        fmt::Display,
        iter,
        ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub},
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

    impl<T> Vec<T>
    where
        T: Clone + Neg<Output = T>,
    {
        pub fn rotated_left(&self) -> Self {
            Self(self.1.clone(), -self.0.clone())
        }

        pub fn rotated_right(&self) -> Self {
            Self(-self.1.clone(), self.0.clone())
        }
    }

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

    impl<T> Sub<Pos<T>> for Pos<T>
    where
        T: Sub<T, Output = T>,
    {
        type Output = Vec<T>;

        fn sub(self, rhs: Pos<T>) -> Self::Output {
            Vec(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl<T> Sub<Vec<T>> for Pos<T>
    where
        T: Sub<T, Output = T>,
    {
        type Output = Pos<T>;

        fn sub(self, rhs: Vec<T>) -> Self::Output {
            Pos(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl<T> Sub<Dir> for Pos<T>
    where
        T: Sub<T, Output = T> + Add<T, Output = T> + From<i8>,
    {
        type Output = Pos<T>;

        fn sub(self, rhs: Dir) -> Self::Output {
            self.sub(rhs.as_vec())
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

    impl<T> Sub<Vec<T>> for Vec<T>
    where
        T: Sub<T, Output = T>,
    {
        type Output = Vec<T>;

        fn sub(self, rhs: Vec<T>) -> Self::Output {
            Vec(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl<T> Mul<T> for Vec<T>
    where
        T: Mul<T, Output = T> + Clone,
    {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Self(self.0 * rhs.clone(), self.1 * rhs)
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

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

    pub const DIAGONAL_DIR: [Dir; 4] = [Dir::UpLeft, Dir::UpRight, Dir::DownRight, Dir::DownLeft];

    pub fn diagonal_dirs() -> impl Iterator<Item = Dir> {
        DIAGONAL_DIR.into_iter()
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

        pub fn rotate_left_90(self) -> Self {
            match self {
                Dir::Up => Dir::Left,
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::UpRight => Dir::UpLeft,
                Dir::DownRight => Dir::UpRight,
                Dir::DownLeft => Dir::DownRight,
                Dir::UpLeft => Dir::DownLeft,
            }
        }

        pub fn rotate_left_45(self) -> Self {
            match self {
                Dir::Up => Dir::UpLeft,
                Dir::Right => Dir::UpRight,
                Dir::Left => Dir::DownLeft,
                Dir::Down => Dir::DownRight,
                Dir::UpRight => Dir::Up,
                Dir::DownRight => Dir::Right,
                Dir::DownLeft => Dir::Down,
                Dir::UpLeft => Dir::Left,
            }
        }

        pub fn rotate_right_90(self) -> Self {
            match self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Left => Dir::Up,
                Dir::Down => Dir::Left,
                Dir::UpRight => Dir::DownRight,
                Dir::DownRight => Dir::DownLeft,
                Dir::DownLeft => Dir::UpLeft,
                Dir::UpLeft => Dir::UpRight,
            }
        }

        pub fn rotate_right_45(self) -> Self {
            match self {
                Dir::Up => Dir::UpRight,
                Dir::Right => Dir::DownRight,
                Dir::Left => Dir::UpLeft,
                Dir::Down => Dir::DownLeft,
                Dir::UpRight => Dir::Right,
                Dir::DownRight => Dir::Down,
                Dir::DownLeft => Dir::Left,
                Dir::UpLeft => Dir::Up,
            }
        }

        pub fn is_horizontal(&self) -> bool {
            self == &Dir::Left || self == &Dir::Right
        }

        pub fn is_vertical(&self) -> bool {
            self == &Dir::Up || self == &Dir::Down
        }

        pub fn is_orthogonal(&self) -> bool {
            self.is_horizontal() || self.is_vertical()
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct Map<T, Cell> {
        pub width: T,
        pub height: T,
        pub cells: std::vec::Vec<Cell>,
    }

    impl<T, Cell, FromUsizeErr, IntoUsizeErr> fmt::Display for Map<T, Cell>
    where
        FromUsizeErr: Debug,
        T: Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + TryFrom<usize, Error = FromUsizeErr>
            + PartialOrd
            + Copy,
        Cell: Clone + Display,
        IntoUsizeErr: Debug,
        usize: TryFrom<T, Error = IntoUsizeErr>,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let height = usize::try_from(self.height).unwrap();
            let width = usize::try_from(self.width).unwrap();
            for y in 0..height {
                if y > 0 {
                    write!(f, "\n")?;
                }
                for x in 0..width {
                    write!(f, "{}", self.cells[width * y + x])?;
                }
            }
            Ok(())
        }
    }

    impl<T, Cell, FromUsizeErr, IntoUsizeErr> Map<T, Cell>
    where
        FromUsizeErr: Debug,
        T: Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + TryFrom<usize, Error = FromUsizeErr>
            + PartialOrd
            + Copy,
        Cell: Clone,
        IntoUsizeErr: Debug,
        usize: TryFrom<T, Error = IntoUsizeErr>,
    {
        pub fn get(&self, p: Pos<T>) -> Option<&Cell> {
            self.offset_from_pos(p)
                .and_then(|offset| self.cells.get(offset))
        }

        pub fn get_mut(&mut self, p: Pos<T>) -> Option<&mut Cell> {
            self.offset_from_pos(p)
                .and_then(|offset| self.cells.get_mut(offset))
        }

        pub fn get_wrapping(&self, Pos(x, y): Pos<T>) -> Option<&Cell> {
            let mut x = x % self.width;
            if x < T::try_from(0).unwrap() {
                x = x + self.width;
            }
            let mut y = y % self.height;
            if y < T::try_from(0).unwrap() {
                y = y + self.height;
            }
            self.offset_from_pos(Pos(x, y))
                .and_then(|offset| self.cells.get(offset))
        }

        pub fn swap(&mut self, a: Pos<T>, b: Pos<T>) -> () {
            if let (Some(a), Some(b)) = (self.offset_from_pos(a), self.offset_from_pos(b)) {
                self.cells.swap(a, b)
            } else {
                panic!("Invalid index")
            }
        }

        fn pos_from_offset(&self, offset: T) -> Pos<T> {
            Self::pos_from_offset_width(offset, self.width)
        }

        fn pos_from_offset_width(offset: T, width: T) -> Pos<T> {
            Pos(offset % width, offset / width)
        }

        pub fn offset_from_pos(&self, Pos(x, y): Pos<T>) -> Option<usize> {
            let zero = T::try_from(0).unwrap();
            if x < zero || x >= self.width || y < zero || y >= self.height {
                None
            } else {
                match usize::try_from(x + y * self.width) {
                    Ok(offset) => Some(offset),
                    Err(_) => None,
                }
            }
        }

        pub fn new(width: T, height: T, value: Cell) -> Self {
            let cells = std::vec::Vec::from_iter(
                iter::repeat(value).take((width * height).try_into().unwrap()),
            );
            Self {
                width,
                height,
                cells,
            }
        }

        pub fn enumerate_cells<'a>(
            &'a self,
        ) -> impl DoubleEndedIterator<Item = (Pos<T>, &'a Cell)> {
            self.cells
                .iter()
                .enumerate()
                .map(|(i, c)| (self.pos_from_offset(T::try_from(i).unwrap()), c))
        }

        pub fn enumerate_cells_mut<'a>(
            &'a mut self,
        ) -> impl DoubleEndedIterator<Item = (Pos<T>, &'a mut Cell)> {
            self.cells.iter_mut().enumerate().map(|(i, c)| {
                (
                    Self::pos_from_offset_width(T::try_from(i).unwrap(), self.width),
                    c,
                )
            })
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

    pub struct MapBuilder<T, Cell>
    where
        Cell: TryFrom<char>,
    {
        width: T,
        height: T,
        cells: std::vec::Vec<Cell>,
        error: Option<<Cell as TryFrom<char>>::Error>,
    }

    impl<T, Cell, FromUsizeErr> MapBuilder<T, Cell>
    where
        T: Default
            + Add
            + PartialEq
            + TryFrom<usize, Error = FromUsizeErr>
            + AddAssign
            + Debug
            + Mul<Output = T>
            + Copy,
        Cell: TryFrom<char>,
        FromUsizeErr: Debug,
    {
        pub fn new() -> Self {
            Self {
                width: T::default(),
                height: T::default(),
                cells: std::vec::Vec::new(),
                error: None,
            }
        }

        pub fn feed_line(&mut self, line: &str) {
            if self.error.is_none() {
                for c in line.chars() {
                    match Cell::try_from(c) {
                        Ok(c) => self.cells.push(c),
                        Err(e) => {
                            self.error = Some(e);
                            return;
                        }
                    };
                }
                if self.height == T::default() {
                    self.width = self.cells.len().try_into().unwrap();
                }
                self.height += T::try_from(1).unwrap();
            }
        }

        pub fn finalize(self) -> Result<Map<T, Cell>, <Cell as TryFrom<char>>::Error> {
            match self.error {
                Some(e) => Err(e),
                None => {
                    assert_eq!(
                        self.width * self.height,
                        self.cells.len().try_into().unwrap()
                    );
                    Ok(Map {
                        width: self.width,
                        height: self.height,
                        cells: self.cells,
                    })
                }
            }
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
            let mut builder = MapBuilder::new();
            for line in s.lines() {
                builder.feed_line(line);
            }
            builder.finalize()
        }
    }
}
