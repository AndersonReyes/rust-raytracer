use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Returns a new vector from x, y, z parameters
    pub fn new<T: Into<f64> + Copy>(x: T, y: T, z: T) -> Vector3 {
        Vector3 {x: x.into(), y: y.into(), z: z.into()}
    }

    /// Get the squared length of the vector, returns x^2 + y^2 + z^2
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

impl ops::Add for Vector3 {
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
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);

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
        assert_eq!(output.x, 5.0);
        assert_eq!(output.y, 7.0);
        assert_eq!(output.z, 9.0);
    }

    #[test]
    fn test_squared_length() {
    }
}
