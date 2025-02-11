#[derive(Clone, Debug)]
pub struct ImageFilter {
    pub filter: Vec<ImageFilterValue>,
}

impl ImageFilter {
    pub fn sharpen() -> ImageFilter {
        ImageFilter {
            filter: vec![
                ImageFilterValue::new(-1, 0, -1.0),
                ImageFilterValue::new(1, 0, -1.0),
                ImageFilterValue::new(0, -1, -1.0),
                ImageFilterValue::new(0, 1, -1.0),
                ImageFilterValue::new(0, 0, 5.0),
            ]
        }
    }

    pub fn sobel() -> ImageFilter {
        let mut entries = ImageFilter::sobel_x().filter;
        entries.extend(ImageFilter::sobel_y().filter);

        ImageFilter {
            filter: entries
        }
    }

    pub fn sobel_x() -> ImageFilter {
        ImageFilter {
            filter: vec![
                ImageFilterValue::new(-1, -1, 1.0),
                ImageFilterValue::new(-1, 0, 2.0),
                ImageFilterValue::new(-1, 1, 1.0),
                ImageFilterValue::new(1, -1, -1.0),
                ImageFilterValue::new(1, 0, -2.0),
                ImageFilterValue::new(1, 1, -1.0),
            ]
        }
    }

    pub fn sobel_y() -> ImageFilter {
        ImageFilter {
            filter: vec![
                ImageFilterValue::new(-1, -1, 1.0),
                ImageFilterValue::new(0, -1, 2.0),
                ImageFilterValue::new(1, -1, 1.0),
                ImageFilterValue::new(-1, 1, -1.0),
                ImageFilterValue::new(0, 1, -2.0),
                ImageFilterValue::new(1, 1, -1.0),
            ]
        }
    }

    pub fn prewit() -> ImageFilter {
        let mut entries = ImageFilter::prewit_x().filter;
        entries.extend(ImageFilter::prewit_y().filter);

        ImageFilter {
            filter: entries
        }
    }

    pub fn prewit_x() -> ImageFilter {
        ImageFilter {
            filter: vec![
                ImageFilterValue::new(-1, -1, 1.0),
                ImageFilterValue::new(-1, 0, 1.0),
                ImageFilterValue::new(-1, 1, 1.0),
                ImageFilterValue::new(1, -1, -1.0),
                ImageFilterValue::new(1, 0, -1.0),
                ImageFilterValue::new(1, 1, -1.0),
            ]
        }
    }

    pub fn prewit_y() -> ImageFilter {
        ImageFilter {
            filter: vec![
                ImageFilterValue::new(-1, -1, 1.0),
                ImageFilterValue::new(0, -1, 1.0),
                ImageFilterValue::new(1, -1, 1.0),
                ImageFilterValue::new(-1, 1, -1.0),
                ImageFilterValue::new(0, 1, -1.0),
                ImageFilterValue::new(1, 1, -1.0),
            ]
        }
    }

    pub fn normalize(&mut self) {
        let mut acc = 0.0;
        for val in &self.filter {
            acc += val.weight;
        }

        for val in self.filter.iter_mut() {
            val.weight /= acc;
        }
    }

    pub fn flip(&mut self) {
        for val in self.filter.iter_mut() {
            let temp = val.offset_x;
            val.offset_x = val.offset_y;
            val.offset_y = temp;
        }
    }

    pub fn flipped(mut self) -> ImageFilter {
        self.flip();
        self
    }

    pub fn radius_x(&self) -> u32 {
        let mut largest = 0;
        for val in &self.filter {
            if val.offset_x.abs() > largest {
                largest = val.offset_x.abs();
            }
        }

        largest as u32
    }

    pub fn radius_y(&self) -> u32 {
        let mut largest = 0;
        for val in &self.filter {
            if val.offset_y.abs() > largest {
                largest = val.offset_y.abs();
            }
        }

        largest as u32
    }
}

#[derive(Clone, Debug)]
pub struct ImageFilterValue {
    pub offset_x: i32,
    pub offset_y: i32,
    pub weight: f32,
}

impl ImageFilterValue {
    pub fn new(x: i32, y: i32, weight: f32) -> ImageFilterValue {
        ImageFilterValue {
            offset_x: x,
            offset_y: y,
            weight,
        }
    }
}

