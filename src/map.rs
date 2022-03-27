const DEFAULT_MAP: &str = "0000222222220000\
                           1              0\
                           1      11111   0\
                           1     0        0\
                           0     0  1110000\
                           0     3        0\
                           0   10000      0\
                           0   3   11100  0\
                           5   4   0      0\
                           5   4   1  00000\
                           0       1      0\
                           2       1      0\
                           0       0      0\
                           0 0000000      0\
                           0              0\
                           0002222222200000";

const DEFAULT_WIDTH: usize = 16;
const DEFAULT_HEIGHT: usize = 16;

pub struct Map {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn default() -> Self {
        Map::new(DEFAULT_MAP, DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }

    pub fn new(map: &str, width: usize, height: usize) -> Self {
        assert!(map.len() == width * height);
        Self {
            map: map.chars().collect(),
            width,
            height,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<usize> {
        self.map[i + j * self.width]
            .to_digit(10)
            .map(|x| x as usize)
    }

    pub fn is_empty(&self, i: usize, j: usize) -> bool {
        self.map[i + j * self.width] == ' '
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
