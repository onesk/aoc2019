use std::borrow::BorrowMut;
use std::default::Default;

#[derive(Debug)]
pub struct Permutations<T>(Option<T>) where T: BorrowMut<[usize]> + Default + Copy + std::fmt::Debug;

impl<T> Permutations<T> where T: BorrowMut<[usize]> + Default + Copy + std::fmt::Debug {
    pub fn new() -> Self {
        let mut perm: T = Default::default();
        let borrow = perm.borrow_mut();
        for i in 0..borrow.len() {
            borrow[i] = i;
        }

        Self(Some(perm))
    }
}

impl<T> Iterator for Permutations<T> where T: BorrowMut<[usize]> + Default + Copy + std::fmt::Debug {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret = self.0;

        self.0 = self.0.take().and_then(|mut next| {
            let a = next.borrow_mut();
            let n = a.len();

            let k: usize = (0..(n-1)).filter(|&k| a[k] < a[k+1]).max()?;
            let l: usize = ((k+1)..n).filter(|&l| a[k] < a[l]).max().expect("Always exists.");

            a.swap(k, l);
            a[(k+1)..].reverse();

            Some(next)
        });

        ret
    }
}

#[test]
fn test_permutations() {
    assert_eq!(Permutations::<[usize; 8]>::new().count(), 40320);
}
