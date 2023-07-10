use num_complex::Complex;

// [from] https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-mandelbrot/src/main.rs

pub struct Mandelbrot {
  max_iters: usize,
  min: (f64, f64),
  max: (f64, f64),
  size: (usize, usize),
}

impl Mandelbrot {
  pub fn new(max_iters: usize, min: (f64, f64), max: (f64, f64), size: (usize, usize)) -> Self {
    Mandelbrot {
      max_iters,
      min,
      max,
      size,
    }
  }

  pub fn render(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
      let mut line = String::with_capacity(row.len());
      for column in row {
        let val = match column {
          0..=2 => ' ',
          3..=5 => '.',
          6..=10 => '·',
          11..=30 => '*',
          32..=100 => '+',
          101..=200 => 'x',
          201..=400 => '$',
          401..=700 => '#',
          _ => '%',
        };
        line.push(val);
      }
      println!("{}", line);
    }
  }

  pub fn calculate(&self) -> Vec<Vec<usize>> {
    let dx = self.max.0 - self.min.0;
    let dy = self.max.1 - self.min.1;
    let mut rows: Vec<_> = Vec::with_capacity(self.size.0);

    for img_y in 0..self.size.1 {
      let mut row: Vec<usize> = Vec::with_capacity(self.size.1);
      for img_x in 0..self.size.0 {
        let x_percent = img_x as f64 / self.size.0 as f64;
        let y_percent = img_y as f64 / self.size.1 as f64;
        let cx = self.min.0 + dx * x_percent;
        let cy = self.min.1 + dy * y_percent;
        let escaped_at = self.at_point(cx, cy);
        row.push(escaped_at);
      }
      rows.push(row);
    }
    rows
  }

  fn at_point(&self, cx: f64, cy: f64) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);
    for i in 0..=self.max_iters {
      if z.norm() > 2.0 {
        return i;
      }
      z = z * z + c;
    }
    self.max_iters
  }
}
