pub fn gcd<T>(mut x: T, mut y: T) -> T
where
    T: Copy
        + std::cmp::PartialOrd
        + std::default::Default
        + std::ops::Rem<Output = T>,
{
    loop {
        if x > y {
            (x, y) = (y, x);
        }
        if x == T::default() {
            return y;
        }
        (x, y) = (y % x, x);
    }
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: Copy
        + std::cmp::PartialOrd
        + std::default::Default
        + std::ops::Rem<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>,
{
    x / gcd(x, y) * y
}
