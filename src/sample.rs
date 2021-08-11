use std::ops::Index;

pub struct ConvChainSample<'a> {
    sample: &'a [bool],
    pub width: u32,
    pub height: u32,
}

impl<'a> ConvChainSample<'a> {
    pub fn new(sample: &'a [bool], width: u32, height: u32) -> Self {
        assert_eq!(sample.len(), (width * height) as usize);
        Self {
            sample,
            width,
            height,
        }
    }
}

impl<'a> Index<usize> for ConvChainSample<'a> {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.sample[index]
    }
}

impl<'a> Index<(u32, u32)> for ConvChainSample<'a> {
    type Output = bool;

    #[inline]
    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let (x, y) = index;
        let index = (x % self.width) + (y % self.height) * self.width;
        &self.sample[index as usize]
    }
}
