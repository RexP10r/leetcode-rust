use std::ops::Rem;

pub fn euclidean_algo<T>(mut a: T, mut b: T) -> T
where
    T: PartialOrd + Rem<Output = T> + Default + Copy,
{
    let zero = T::default(); 
    
    while b > zero {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
