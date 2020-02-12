use std::ops::SubAssign;
use std::ops::AddAssign;
use std::ops;
use sdl2::rect::{Point};

#[derive(Copy, Clone)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32,
}

impl Vector2
{
  pub fn zero() -> Vector2
  {
    Vector2 {
      x: 0.0,
      y: 0.0,
    }
  }

  pub fn sqr_magnitude(&self) -> f32
  {
    return self.x * self.x + self.y * self.y;
  }

  pub fn magnitude(&self) -> f32
  {
    return self.sqr_magnitude().sqrt();
  }
  
  #[allow(dead_code)]
  pub fn to_point(&self) -> Point
  {
    return Point::new(self.x as i32, self.y as i32);
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