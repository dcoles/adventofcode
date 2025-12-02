//! Vector type.

use std::ops::{Add, AddAssign, Index, IndexMut, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vector<T: Copy + Default, const N: usize>([T; N]);

impl<T: Copy + Default, const N: usize> Vector<T, N> {
    pub const fn new(vec: [T; N]) -> Self {
        Vector(vec)
    }

    pub const fn dimensions(self) -> usize {
        N
    }
}

impl<T: Copy + Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Vector([T::default(); N])
    }
}

impl<T: Copy + Default, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Copy + Default, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Add<Output=T> + Copy + Default, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (v, v_rhs) in self.0.iter_mut().zip(&rhs.0) {
            *v = *v + *v_rhs;
        }

        self
    }
}

impl<T: Add<Output=T> + Copy + Default, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for (v, v_rhs) in self.0.iter_mut().zip(&rhs.0) {
            *v = *v + *v_rhs;
        }
    }
}

impl<T: Sub<Output=T> + Copy + Default, const N: usize> Sub for Vector<T, N> {
    type Output = Vector<T, N>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for (v, v_rhs) in self.0.iter_mut().zip(&rhs.0) {
            *v = *v - *v_rhs;
        }

        self
    }
}

impl<T: Sub<Output=T> + Copy + Default, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (v, v_rhs) in self.0.iter_mut().zip(&rhs.0) {
            *v = *v - *v_rhs;
        }
    }
}

impl<T: Neg<Output=T> + Copy + Default, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<T, N>;

    fn neg(mut self) -> Self::Output {
        for v in self.0.iter_mut() {
            *v = -*v;
        }

        self
    }
}
