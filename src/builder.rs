use super::grid::Grid;
// NOTE: the actual Command API does not use owned Strings;
// this is a simplified version.

pub struct GridBuilder<T> {
    cells: Option<Vec<T>>,
    width: Option<usize>,
    height: Option<usize>,
}

impl<T> GridBuilder<T> {
    pub fn new() -> Self {
        GridBuilder {
            cells: None,
            width: None,
            height: None,
        }
    }

    pub fn from(mut self, cells: Vec<T>) -> Self {
        self.cells = Some(cells);
        self
    }
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn build(self) -> Grid<T> {
        let GridBuilder {
            cells,
            width,
            height,
        } = self;
        match (cells, width, height) {
            (Some(cells), Some(width), Some(height)) => Grid {
                cells,
                width,
                height,
            },
            _ => panic!("Grid could not be build"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid() {
        assert_eq!(
            GridBuilder::new()
                .from(vec![0; 100])
                .width(10)
                .height(10)
                .build(),
            Grid {
                width: 10,
                height: 10,
                cells: vec![0; 100]
            }
        );
    }
}
