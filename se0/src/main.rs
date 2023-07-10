mod mandelbrot;
use mandelbrot::Mandelbrot;

fn main() {
  let mandelbrot = Mandelbrot::new(1000, (-2.0, -1.0), (1.0, 1.0), (100, 24));
  let vectors = mandelbrot.calculate();
  Mandelbrot::render(vectors);
}
