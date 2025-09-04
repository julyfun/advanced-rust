use std::ops::Add;

#[derive(Debug, Clone)]
struct Vec2<T>(Vec<T>);

impl<T> Add for Vec2<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.0.len().min(rhs.0.len());
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            result.push(self.0[i].clone() + rhs.0[i].clone());
        }
        Vec2(result)
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = Vec2(vec![1, 2, 3]);
        let b = Vec2(vec![1, 2, 3]);
        let c = a + b;
        assert_eq!(c.0, vec![2, 4, 6]);
    }
}

