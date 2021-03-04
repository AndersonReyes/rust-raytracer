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

	/// Get length of vector, returns sqrt(x^2 + y^2 + z^2)
	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

    /// Dot product
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z + other.z
    }

    /// Normalize vector
    pub fn normalize(&self) -> Vector3 {
        let len = self.length();
        Vector3 {x: self.x / len, y: self.y / len, z: self.z / len}
    }

    /// Scale vector, by {t}
    pub fn scale(&self, t: f64) -> Vector3 {
        Vector3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t
        }
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

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl ops::Div for Vector3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }

}


/// Aliases
use Vector3 as Point3;
use Vector3 as Color3;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::internal;

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
    fn test_sub() {
        let vec = Vector3::new(1, 2, 3);
        let two = Vector3::new(4, 5, 6);

        let output = vec - two;
        assert_eq!(Vector3::new(-3, -3, -3), output);
        assert_eq!(output.x, -3.0);
        assert_eq!(output.y, -3.0);
        assert_eq!(output.z, -3.0);
    }

    #[test]
    fn test_mult() {
        let vec = Vector3::new(1, 2, 3);
        let two = Vector3::new(4, 5, 6);

        let output = vec * two;
        assert_eq!(Vector3::new(4, 10, 18), output);
        assert_eq!(output.x, 4.0);
        assert_eq!(output.y, 10.0);
        assert_eq!(output.z, 18.0);
    }

    #[test]
    fn test_div() {
        let vec = Vector3::new(1, 2, 3);
        let two = Vector3::new(4, 5, 6);

        let output = vec / two;
        assert_eq!(Vector3::new(0.25, 0.4, 0.5), output);
        assert_eq!(output.x, 0.25);
        assert_eq!(output.y, 0.4);
        assert_eq!(output.z, 0.5);
    }

    #[test]
    fn test_squared_length() {
        let res = Vector3::new(1.0, 2.0, 3.0).length_squared();
        assert_eq!(res, 14.0);
    }

    #[test]
    fn test_length() {
        let res = Vector3::new(1.0, 2.0, 3.0).length();
        assert_eq!(internal::assert_close(res, 14.0_f64.sqrt(), None), true);
    }

    #[test]
    fn test_normalize() {
        let vec = Vector3::new(1.0, 2.0, 3.0);
        let len = vec.length();
        assert_eq!(vec.scale(1.0/len), vec.normalize());
    }
}
