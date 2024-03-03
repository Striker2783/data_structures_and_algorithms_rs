// Just use a vector, this is shit.

// use std::ops::{Index, IndexMut};

// #[derive(Debug, Default)]
// pub struct VectorOwn<T> {
//     // A vector because fuck allocations
//     arr: Vec<T>,
//     size: usize,
//     index: usize,
// }
// impl<T: Sized> VectorOwn<T> {
//     pub fn new(size: usize) -> Self {
//         Self {
//             arr: {
//                 let mut vec = Vec::with_capacity(size);
//                 vec.push(T::default());
//                 vec
//             },
//             index: 0,
//             size,
//         }
//     }
//     fn resize(&mut self, size: usize) {
//         self.arr.resize_with(size, Default::default());
//     }
//     pub fn append(&mut self, e: T) {
//         if self.index == self.size {
//             self.resize(self.size * 2);
//         }
//     }
// }
// impl<T: PartialEq> PartialEq<VectorOwn<T>> for VectorOwn<T> {
//     fn eq(&self, other: &VectorOwn<T>) -> bool {
//         if self.index != other.index {
//             return false;
//         }
//         !(0..self.index).any(|i| self.arr[i] != other.arr[i])
//     }
// }
// impl<T> Index<usize> for VectorOwn<T> {
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.arr[index]
//     }
// }
// impl<T> IndexMut<usize> for VectorOwn<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.arr[index]
//     }
// }
// #[cfg(test)]
// mod tests {
//     //
// }
