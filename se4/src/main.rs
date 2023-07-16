use std::{cell::RefCell, fmt};

#[derive(Clone, Copy, Debug)]
pub struct Vec2<T: PartialEq + PartialOrd + fmt::Display = f32> {
  pub x: T,
  pub y: T,
}

impl<T: PartialEq + PartialOrd + fmt::Display> fmt::Display for Vec2<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vec2 ({}, {})", self.x, self.y)
  }
}

fn main() {
  let p1 = Vec2 { x: 2., y: 3. };
  let p2 = p1;
  println!("p1: {}, p2: {}", p1, p2);

  let rc = RefCell::new(Vec2 { x: 3., y: 45. });

  let mut rc_mut = rc.borrow_mut();
  rc_mut.x = 32.2;
  rc_mut.y = 32.2;

  drop(rc_mut);
  let rc_read = rc.borrow();
  println!("Hello {}", rc_read);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
  fn from(n: f64) -> Self {
    if n >= 1.0 {
      Q7(127)
    } else if n <= -1.0 {
      Q7(-128)
    } else {
      Q7((n * 128.0) as i8)
    }
  }
}

impl From<Q7> for f64 {
  fn from(n: Q7) -> Self {
    (n.0 as f64) * 2_f64.powf(-7.0)
  }
}

impl From<f32> for Q7 {
  fn from(n: f32) -> Self {
    Q7::from(n as f64)
  }
}

impl From<Q7> for f32 {
  fn from(n: Q7) -> Self {
    f64::from(n) as f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn out_of_boudns() {
    assert_eq!(Q7::from(10.), Q7::from(1.));
    assert_eq!(Q7::from(-10.), Q7::from(-1.));
  }

  #[test]
  fn f32_to_q7() {
    assert_eq!(Q7::from(0.7), Q7(89));
    assert_eq!(Q7::from(-0.4), Q7(-51));
  }

  #[test]
  fn q7_to_f32() {
    assert_eq!(f32::from(Q7::from(0.7_f32)), 0.6953125_f32);
  }
}
