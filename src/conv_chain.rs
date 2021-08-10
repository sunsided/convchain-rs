pub struct Pattern {
    pub size: u32,
    data: Vec<bool>,
}

impl Pattern {
    pub fn new_from_function<F>(size: u32, f: F) -> Self
    where
        F: Fn(u32, u32) -> bool,
    {
        let data = vec![false; (size * size) as usize];
        let mut value = Self { size, data };
        value.set(f);
        value
    }

    pub fn new_from_pattern(
        field_width: u32,
        field_height: u32,
        field: &[bool],
        x: i64,
        y: i64,
        size: u32,
    ) -> Self {
        let mut value = Self::new_from_function(size, |i, j| {
            let fx = (x + i as i64 + field_width as i64) % (field_width as i64);
            let fy = (y + j as i64 + field_height as i64) % (field_height as i64);
            debug_assert!((fx >= 0) && (fy >= 0));
            let index = fy * (field_width as i64) + fx;
            field[index as usize]
        });
        value
    }

    pub fn rotated(&self) -> Self {
        Self::new_from_function(self.size, |x, y| {
            let index = self.array_index((self.size - 1 - y), x);
            self.data[index]
        })
    }

    pub fn reflected(&self) -> Self {
        Self::new_from_function(self.size, |x, y| {
            let index = self.array_index(self.size - 1 - x, y);
            self.data[index]
        })
    }

    pub fn index(&self) -> usize {
        let mut result = 0;
        let size = self.size;
        for y in 0..size {
            for x in 0..size {
                let index = self.array_index(x, y);
                result += if self.data[index] { 1 << index } else { 0 };
            }
        }
        result
    }

    fn set<F>(&mut self, f: F)
    where
        F: Fn(u32, u32) -> bool,
    {
        let size = self.size;
        for y in 0..size {
            for x in 0..size {
                let index = self.array_index(x, y);
                self.data[index] = f(x, y);
            }
        }
    }

    #[inline]
    fn array_index(&self, x: u32, y: u32) -> usize {
        (y * self.size + x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pattern {
        use super::*;

        #[test]
        fn new_from_function_works() {
            let pattern = Pattern::new_from_function(4, |x, y| (x + y) % 4 == 0);
            assert_eq!(
                pattern.data,
                [
                    true, false, false, false, //
                    false, false, false, true, //
                    false, false, true, false, //
                    false, true, false, false, //
                ]
            );
        }

        #[test]
        fn rotated_works() {
            let pattern = Pattern::new_from_function(4, |x, y| (x + y) % 4 == 0).rotated();
            assert_eq!(
                pattern.data,
                [
                    false, true, false, false, //
                    false, false, true, false, //
                    false, false, false, true, //
                    true, false, false, false, //
                ]
            );
        }

        #[test]
        fn reflected_works() {
            let pattern = Pattern::new_from_function(4, |x, y| (x + y) % 4 == 0).reflected();
            assert_eq!(
                pattern.data,
                [
                    false, false, false, true, //
                    true, false, false, false, //
                    false, true, false, false, //
                    false, false, true, false, //
                ]
            );
        }
    }
}
