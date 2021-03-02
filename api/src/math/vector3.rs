use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    /// Returns a new vector from x, y, z parameters
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 {x, y, z}
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec = Vector3::new(1, 2, 3);
        assert_eq!(vec.x, 1);
        assert_eq!(vec.y, 2);
        assert_eq!(vec.z, 3);

        let vec2 = Vector3::new(1.3, 2.0, 3.5);
        assert_eq!(vec2.x, 1.3);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 3.5);
    }

    #[test]
    fn test_add() {
        let vec = Vector3::new(1, 2, 3);
        let two = Vector3::new(4, 5, 6);

        let output = vec + two;
        assert_eq!(Vector3::new(5, 7, 9), output);
        assert_eq!(output.x, 5);
        assert_eq!(output.y, 7);
        assert_eq!(output.z, 9);
    }
}
