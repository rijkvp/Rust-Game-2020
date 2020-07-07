use std::ops::SubAssign;
use std::ops::AddAssign;
use std::ops;
use std::fmt;
use amethyst::core::math::Vector3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32,
}

impl Vector2
{
  pub fn new(x: f32, y: f32) -> Self
  {
    Vector2 {x, y}
  }
  
  pub fn sqr_magnitude(&self) -> f32
  {
    return self.x * self.x + self.y * self.y;
  }

  pub fn magnitude(&self) -> f32
  {
    return self.sqr_magnitude().sqrt();
  }

  pub fn to_vector3(&self) -> Vector3<f32>
  {
    return Vector3::new(self.x, self.y, 0.0);
  }

  pub fn normalized(&self) -> Vector2
  {
    let mag = self.magnitude();
    if mag == 0.0
    {
      return *self;
    }
    else
    {
      Vector2 {
          x: self.x/mag,
          y: self.y/mag,
      }
    }
  }

  pub fn lerp(a: Vector2, b: Vector2, t: f32) -> Vector2
  {
    return a + (b-a) * t;
  }

  pub fn _distance(a: Vector2, b: Vector2) -> f32
  {
    return (a-b).magnitude();
  }

  pub fn get_radians(&self) -> f32
  {
      self.x.atan2(self.y)
  }

  pub fn _get_degrees(&self) -> f32
  {
      self.x.atan2(self.y) * 180.0 / std::f32::consts::PI
  }
}

impl ops::Add<Vector2> for Vector2
{
  type Output = Vector2;
  fn add(self, rhs: Vector2) -> Vector2 {
    Vector2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::Sub<Vector2> for Vector2
{
  type Output = Vector2;
  fn sub(self, rhs: Vector2) -> Vector2 {
    Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl SubAssign for Vector2 {
  fn sub_assign(&mut self, other: Self) {
      *self = Self {
          x: self.x + other.x,
          y: self.y + other.y,
      };
  }
}

impl ops::Mul<Vector2> for Vector2
{
  type Output = Vector2;
  fn mul(self, rhs: Vector2) -> Vector2 {
    Vector2 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    }
  }
}

impl ops::Mul<f32> for Vector2
{
  type Output = Vector2;
  fn mul(self, rhs: f32) -> Vector2 {
    Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl fmt::Display for Vector2
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "X: {}, Y: {}", self.x, self.y)
  }
}