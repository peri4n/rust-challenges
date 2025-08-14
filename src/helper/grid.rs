use std::fs;

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

pub trait Parse {
    fn parse(c: char) -> Self;
}

impl Parse for char {
    fn parse(c: char) -> Self {
        c
    }
}

impl Parse for u32 {
    fn parse(c: char) -> Self {
        c.to_digit(10).unwrap()
    }
}

impl<T: Parse + Copy> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Grid { grid }
    }

    pub fn from_string(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| Parse::parse(c)).collect())
            .collect();
        Grid::new(grid)
    }

    pub fn from_file(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Unable to read grid file");
        Grid::from_string(&content)
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            current: Cursor::new(self),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.grid.len() && y < self.grid[x].len() {
            Some(&self.grid[x][y])
        } else {
            None
        }
    }

    pub fn get_cursor(&self, x: usize, y: usize) -> Cursor<T> {
        Cursor { x, y, grid: self }
    }

    pub fn columns(&self) -> usize {
        if self.grid.is_empty() {
            return 0;
        }

        self.grid[0].len()
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }
}

pub struct GridIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
}

impl <'a, T: Copy> Iterator for GridIterator<'a, T> {
    type Item = Cursor<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < self.current.grid.grid.len() {
            let current = self.current;
            if self.current.x < self.current.grid.grid[0].len() - 1 {
                self.current.x += 1;
            } else {
                self.current.x = 0;
                self.current.y += 1;
            }
            Some(current)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cursor<'a, T: Copy> {
    x: usize,
    y: usize,
    grid: &'a Grid<T>,
}

impl<'a, T: Copy> Cursor<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Cursor { x: 0, y: 0, grid }
    }

    pub fn get(&self) -> &T {
        &self.grid.grid[self.y][self.x]
    }

    pub fn up_iter(&self) -> UpwardIterator<'a, T> {
        UpwardIterator {
            current: *self,
            done: false,
        }
    }

    pub fn up_right_iter(&self) -> UpRightIterator<'a, T> {
        UpRightIterator {
            current: *self,
            done: false,
        }
    }

    pub fn right_iter(&self) -> RightIterator<'a, T> {
        RightIterator { current: *self }
    }

    pub fn down_right_iter(&self) -> DownRightIterator<'a, T> {
        DownRightIterator { current: *self }
    }

    pub fn down_iter(&self) -> DownwardIterator<'a, T> {
        DownwardIterator { current: *self }
    }

    pub fn down_left_iter(&self) -> DownLeftIterator<'a, T> {
        DownLeftIterator {
            current: *self,
            done: false,
        }
    }

    pub fn left_iter(&self) -> LeftIterator<'a, T> {
        LeftIterator {
            current: *self,
            done: false,
        }
    }

    pub fn up_left_iter(&self) -> UpLeftIterator<'a, T> {
        UpLeftIterator {
            current: *self,
            done: false,
        }
    }
}

pub struct UpwardIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
    done: bool,
}

impl<'a, T: Copy> Iterator for UpwardIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.current.y == 0 {
            self.done = true;
        }

        let current = *self.current.get();

        if self.current.y > 0 {
            self.current.y -= 1;
        }
        Some(current)
    }
}

pub struct UpRightIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
    done: bool,
}

impl<'a, T: Copy> Iterator for UpRightIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // you hit the top or the right edge of the grid
        if self.current.y == 0 || self.current.x == self.current.grid.grid[0].len() - 1 {
            self.done = true;
        }

        let current = *self.current.get();

        if self.current.x < self.current.grid.grid[0].len() && self.current.y > 0 {
            self.current.x += 1;
            self.current.y -= 1;
        }

        Some(current)
    }
}

pub struct RightIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
}

impl<'a, T: Copy> Iterator for RightIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x < self.current.grid.grid[0].len() {
            let current = *self.current.get();
            self.current.x += 1;
            Some(current)
        } else {
            None
        }
    }
}

pub struct DownRightIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
}

impl<'a, T: Copy> Iterator for DownRightIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x < self.current.grid.grid[0].len()
            && self.current.y < self.current.grid.grid.len()
        {
            let current = *self.current.get();
            self.current.x += 1;
            self.current.y += 1;
            Some(current)
        } else {
            None
        }
    }
}

pub struct DownwardIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
}

impl<'a, T: Copy> Iterator for DownwardIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < self.current.grid.grid.len() {
            let current = *self.current.get();
            self.current.y += 1;
            Some(current)
        } else {
            None
        }
    }
}

pub struct DownLeftIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
    done: bool,
}

impl<'a, T: Copy> Iterator for DownLeftIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // you hit the bottom or the left edge of the grid
        if self.current.y == self.current.grid.grid.len() - 1 || self.current.x == 0 {
            self.done = true;
        }

        let current = *self.current.get();

        if self.current.x > 0 && self.current.y < self.current.grid.grid.len() {
            self.current.x -= 1;
            self.current.y += 1;
        }

        Some(current)
    }
}

pub struct LeftIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
    done: bool,
}

impl<'a, T: Copy> Iterator for LeftIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.current.x == 0 {
            self.done = true;
        }

        let current = *self.current.get();
        if self.current.x > 0 {
            self.current.x -= 1;
        }
        Some(current)
    }
}

pub struct UpLeftIterator<'a, T: Copy> {
    current: Cursor<'a, T>,
    done: bool,
}

impl<'a, T: Copy> Iterator for UpLeftIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // you hit the top or the left edge of the grid
        if self.current.y == 0 || self.current.x == 0 {
            self.done = true;
        }

        let current = *self.current.get();

        if self.current.x > 0 && self.current.y > 0 {
            self.current.x -= 1;
            self.current.y -= 1;
        }

        Some(current)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_be_constructed_from_a_string() {
        let grid: Grid<u32> = Grid::from_string("123\n456\n789\n123");

        assert_eq!(grid.rows(), 4);
        assert_eq!(grid.columns(), 3);
    }

    #[test]
    fn can_iterate_over_the_entire_grid() {
        let grid: Grid<u32> = Grid::from_string("123\n456\n789\n123");

        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some(Cursor { x: 0, y: 0, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 1, y: 0, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 2, y: 0, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 0, y: 1, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 1, y: 1, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 2, y: 1, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 0, y: 2, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 1, y: 2, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 2, y: 2, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 0, y: 3, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 1, y: 3, grid: &grid }));
        assert_eq!(iter.next(), Some(Cursor { x: 2, y: 3, grid: &grid }));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_up() {
        let grid = Grid::from_string("123\n456\n789");
        let mut iter = grid.get_cursor(1, 2).up_iter();

        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_up_right() {
        let grid = Grid::from_string("123\n456\n789\n123");
        let mut iter = grid.get_cursor(0, 3).up_right_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_right() {
        let grid = Grid::from_string("123\n456\n789");
        let mut iter = grid.get_cursor(0, 0).right_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_down_right() {
        let grid = Grid::from_string("123\n456\n789\n123");
        let mut iter = grid.get_cursor(0, 0).down_right_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_down() {
        let grid = Grid::from_string("123\n456\n789");
        let mut iter = grid.get_cursor(1, 0).down_iter();

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_down_left() {
        let grid = Grid::from_string("123\n456\n789");
        let mut iter = grid.get_cursor(2, 0).down_left_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_left() {
        let grid = Grid::from_string("123\n456\n789");
        let mut iter = grid.get_cursor(2, 0).left_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_iterates_up_left() {
        let grid = Grid::from_string("123\n456\n789");
        let mut iter = grid.get_cursor(2, 2).up_left_iter();

        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn correctly_collects_the_first_row() {
        let grid: Grid<char> = Grid::from_string("123\n456\n789");
        let first_row = grid.get_cursor(0, 0).right_iter().collect::<String>();

        assert_eq!(first_row, "123");
    }
}
