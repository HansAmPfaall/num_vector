#[cfg(test)]
mod tests;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Clone)]
pub struct NumVector<T>(Vec<T>);

impl<T> NumVector<T> {
    pub fn empty() -> Self {
        NumVector(vec![])
    }

    pub fn new(vec: Vec<T>) -> Self {
        NumVector(vec)
    }

    pub fn get(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> Add<&NumVector<T>> for &NumVector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = NumVector<T>;

    fn add(self, rhs: &NumVector<T>) -> Self::Output {
        assert!(self.len() == rhs.len());
        let mut vec = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            vec.push(self.0[i] + rhs.0[i]);
        }
        NumVector(vec)
    }
}

impl<T> Sub<&NumVector<T>> for &NumVector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = NumVector<T>;

    fn sub(self, rhs: &NumVector<T>) -> Self::Output {
        assert!(self.len() == rhs.len());
        let mut vec = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            vec.push(self.0[i] - rhs.0[i]);
        }
        NumVector(vec)
    }
}

impl<T> AddAssign<&NumVector<T>> for NumVector<T>
where
    T: std::ops::AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &NumVector<T>) {
        assert!(self.len() == rhs.len());
        for i in 0..self.len() {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<T> SubAssign<&NumVector<T>> for NumVector<T>
where
    T: std::ops::SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: &NumVector<T>) {
        assert!(self.len() == rhs.len());
        for i in 0..self.len() {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<T> Mul<T> for &NumVector<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = NumVector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut vec = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            vec.push(self.0[i] * rhs);
        }
        NumVector(vec)
    }
}

impl<T> MulAssign<T> for NumVector<T>
where
    T: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.len() {
            self.0[i] *= rhs;
        }
    }
}

impl<T> Div<T> for &NumVector<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = NumVector<T>;
    fn div(self, rhs: T) -> Self::Output {
        let mut vec = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            vec.push(self.0[i] / rhs);
        }
        NumVector(vec)
    }
}

impl<T> DivAssign<T> for NumVector<T>
where
    T: std::ops::DivAssign + Copy + std::cmp::PartialEq<i32>,
{
    fn div_assign(&mut self, rhs: T) {
        assert!(rhs != 0);
        for i in 0..self.len() {
            self.0[i] /= rhs;
        }
    }
}

impl<T> std::ops::Deref for NumVector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for NumVector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// TODO: write a Display function that displays the stuct properly.
// impl<T> std::fmt::Display for NumVector<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.capacity())
//     }
// }
