//! Vector type.

use std::ops::{Add, AddAssign, Index, Neg, Sub, SubAssign};

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

impl<T: Add<Output=T> + Copy + Default, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut vec = [T::default(); N];
        for n in 0..N {
            vec[n] = self.0[n] + rhs.0[n];
        }

        Vector(vec)
    }
}

impl<T: Add<Output=T> + Copy + Default, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self.0[n] = self.0[n] + rhs.0[n];
        }
    }
}

impl<T: Sub<Output=T> + Copy + Default, const N: usize> Sub for Vector<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut vec = [T::default(); N];
        for n in 0..N {
            vec[n] = self.0[n] - rhs.0[n];
        }

        Vector(vec)
    }
}

impl<T: Sub<Output=T> + Copy + Default, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self.0[n] = self.0[n] - rhs.0[n];
        }
    }
}

impl<T: Neg<Output=T> + Copy + Default, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        let mut vec = [T::default(); N];
        for n in 0..N {
            vec[n] = -self.0[n];
        }

        Vector(vec)
    }
}
