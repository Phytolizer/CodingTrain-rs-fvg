use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub fn map<T>(value: T, start1: T, stop1: T, start2: T, stop2: T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    (stop2 - start2) * ((value - start1) / (stop1 - start1)) + start2
}
