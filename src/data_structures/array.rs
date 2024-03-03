use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct VectorOwn<T> {
    // A vector because fuck allocations
    arr: Vec<T>,
    size: usize,
    index: usize,
}
impl<T> VectorOwn<T> {
    pub fn new(size: usize) -> Self {
        Self {
            arr: Vec::with_capacity(size),
            index: 0,
            size,
        }
    }
    pub fn append(&mut self, e: T) {
        //
    }
}
impl<T: PartialEq> PartialEq<VectorOwn<T>> for VectorOwn<T> {
    fn eq(&self, other: &VectorOwn<T>) -> bool {
        if self.index != other.index {
            return false;
        }
        for i in 0..self.index {
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }
        true
    }
}
impl<T> Index<usize> for VectorOwn<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}
impl<T> IndexMut<usize> for VectorOwn<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}
#[cfg(test)]
mod tests {
    //
}
