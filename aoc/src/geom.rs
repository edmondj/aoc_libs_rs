pub mod twod {
    use crate::extensions::SliceExtensions;
    use core::fmt::{self, Debug};
    use std::{
        fmt::Display,
        iter,
        ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
        str::FromStr,
        vec,
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

    impl<T> SubAssign<Vec<T>> for Vec<T>
    where
        T: SubAssign<T>,
    {
        fn sub_assign(&mut self, rhs: Vec<T>) {
            self.0 -= rhs.0;
            self.1 -= rhs.1;
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

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Map<Cell> {
        pub width: usize,
        pub height: usize,
        pub cells: std::vec::Vec<Cell>,
    }

    impl<Cell> fmt::Display for Map<Cell>
    where
        Cell: Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (Pos(x, y), cell) in self.enumerate_cells() {
                if x == 0 && y > 0 {
                    write!(f, "\n")?;
                }
                write!(f, "{cell}")?;
            }
            Ok(())
        }
    }

    impl<Cell> Map<Cell> {
        pub fn get(&self, p: Pos<usize>) -> Option<&Cell> {
            self.offset_from_pos(p)
                .and_then(|offset| self.cells.get(offset))
        }

        pub fn get_mut(&mut self, p: Pos<usize>) -> Option<&mut Cell> {
            self.offset_from_pos(p)
                .and_then(|offset| self.cells.get_mut(offset))
        }

        pub fn get_wrapping<T>(&self, Pos(x, y): Pos<isize>) -> Option<&Cell> {
            let mut x = x % self.width as isize;
            if x < 0 {
                x = x + self.width as isize;
            }
            let mut y = y % self.height as isize;
            if y < 0 {
                y = y + self.height as isize;
            }
            self.offset_from_pos(Pos(x as usize, y as usize))
                .and_then(|offset| self.cells.get(offset))
        }

        pub fn swap(&mut self, a: Pos<usize>, b: Pos<usize>) -> () {
            if let (Some(a), Some(b)) = (self.offset_from_pos(a), self.offset_from_pos(b)) {
                self.cells.swap(a, b)
            } else {
                panic!("Invalid index")
            }
        }

        fn pos_from_offset(&self, offset: usize) -> Pos<usize> {
            Self::pos_from_offset_width(offset, self.width)
        }

        fn pos_from_offset_width(offset: usize, width: usize) -> Pos<usize> {
            Pos(offset % width, offset / width)
        }

        pub fn offset_from_pos(&self, Pos(x, y): Pos<usize>) -> Option<usize> {
            if x >= self.width || y >= self.height {
                None
            } else {
                match usize::try_from(x + y * self.width) {
                    Ok(offset) => Some(offset),
                    Err(_) => None,
                }
            }
        }

        pub fn enumerate_cells<'a>(
            &'a self,
        ) -> impl DoubleEndedIterator<Item = (Pos<usize>, &'a Cell)> {
            self.cells
                .iter()
                .enumerate()
                .map(|(i, c)| (self.pos_from_offset(i), c))
        }

        pub fn enumerate_cells_mut<'a>(
            &'a mut self,
        ) -> impl DoubleEndedIterator<Item = (Pos<usize>, &'a mut Cell)> {
            self.cells
                .iter_mut()
                .enumerate()
                .map(|(i, c)| (Self::pos_from_offset_width(i, self.width), c))
        }
    }

    impl<Cell> Map<Cell>
    where
        Cell: Clone,
    {
        pub fn new(width: usize, height: usize, value: Cell) -> Self {
            let cells = std::vec::Vec::from_iter(iter::repeat(value).take(width * height));
            Self {
                width,
                height,
                cells,
            }
        }

        pub fn rotate_right(&self) -> Self {
            let mut cells = self.cells.clone();
            for (Pos(x, y), cell) in self.enumerate_cells() {
                let new_offset = x * self.height + (self.height - y - 1);
                cells[usize::try_from(new_offset).unwrap()] = cell.clone();
            }
            Self {
                width: self.height,
                height: self.width,
                cells,
            }
        }

        pub fn horizontal_flip(&self) -> Self {
            let mut sent = self.clone();
            for y in 0..sent.height {
                for x in 0..sent.width / 2 {
                    sent.swap(Pos(x, y), Pos(self.width - x - 1, y));
                }
            }
            sent
        }

        pub fn split_regions(&self, width: usize, height: usize) -> Option<vec::Vec<Self>> {
            if self.width % width != 0 || self.height % height != 0 {
                None
            } else {
                let mut result = vec::Vec::with_capacity(self.width / width * self.height / height);
                let mut x = 0;
                let mut y = 0;
                while x < self.width && y < self.height {
                    let mut sub = vec::Vec::with_capacity(width * height);
                    for i in 0..height {
                        let start = self.offset_from_pos(Pos(x, y + i)).unwrap();
                        sub.extend_from_slice(&self.cells[start..start + width]);
                    }

                    result.push(Self {
                        width,
                        height,
                        cells: sub,
                    });

                    x += width;
                    if x >= self.width {
                        y += width;
                        x = 0;
                    }
                }
                Some(result)
            }
        }
    }

    pub trait MapCombinable {
        type Output;

        fn combine(&self, maps_per_row: usize) -> Self::Output;
    }

    impl<Cell> MapCombinable for [Map<Cell>]
    where
        Cell: Clone + Default,
    {
        type Output = Option<Map<Cell>>;

        fn combine(&self, maps_per_row: usize) -> Self::Output {
            if self.is_empty() {
                None
            } else {
                let sub_width = self[0].width;
                let sub_height = self[0].height;
                let mut result = Map::new(
                    sub_width * maps_per_row,
                    sub_height * (self.len() / maps_per_row),
                    Cell::default(),
                );

                for (i, sub) in self.iter().enumerate() {
                    if sub.width != sub_width || sub.height != sub_height {
                        return None;
                    }
                    let top_left = Pos(i % maps_per_row * sub_width, i / maps_per_row * sub_height);
                    for (Pos(x, y), cell) in sub.enumerate_cells() {
                        *result.get_mut(top_left + Vec(x, y)).unwrap() = cell.clone();
                    }
                }

                Some(result)
            }
        }
    }

    impl<Cell> Map<Cell>
    where
        Cell: PartialEq + Clone,
    {
        pub fn pos_of(&self, c: &Cell) -> Option<Pos<usize>> {
            self.cells
                .index_of(c)
                .map(|index| self.pos_from_offset(index))
        }
    }

    pub struct MapBuilder<Cell>
    where
        Cell: TryFrom<char>,
    {
        width: usize,
        height: usize,
        cells: std::vec::Vec<Cell>,
        error: Option<<Cell as TryFrom<char>>::Error>,
    }

    impl<Cell> MapBuilder<Cell>
    where
        Cell: TryFrom<char>,
    {
        pub fn new() -> Self {
            Self {
                width: 0,
                height: 0,
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
                if self.height == 0 {
                    self.width = self.cells.len();
                }
                self.height += 1;
            }
        }

        pub fn finalize(self) -> Result<Map<Cell>, <Cell as TryFrom<char>>::Error> {
            match self.error {
                Some(e) => Err(e),
                None => {
                    assert_eq!(self.width * self.height, self.cells.len());
                    Ok(Map {
                        width: self.width,
                        height: self.height,
                        cells: self.cells,
                    })
                }
            }
        }
    }

    impl<Cell> FromStr for Map<Cell>
    where
        Cell: TryFrom<char>,
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
