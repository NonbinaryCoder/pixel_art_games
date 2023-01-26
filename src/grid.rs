use std::{
    iter,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    data: Box<[T]>,
}

impl<T> Index<UVec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: UVec2) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> IndexMut<UVec2> for Grid<T> {
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> Grid<T> {
    pub fn new(size: UVec2) -> Self
    where
        T: Default,
    {
        let area = size.x as usize * size.y as usize;
        let mut data = Vec::with_capacity(area);
        data.extend(iter::repeat_with(|| T::default()).take(area));
        Self {
            width: size.x as usize,
            data: data.into(),
        }
    }

    pub fn from_vec(data: Vec<T>, width: usize) -> Self {
        assert_eq!(data.len() % width, 0);
        Self {
            width,
            data: data.into_boxed_slice(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        (self.data.len() / self.width) as u32
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    pub fn rows(&self) -> std::slice::Chunks<T> {
        self.data.chunks(self.width)
    }
}
