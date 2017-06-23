use num_complex::Complex64;

pub fn mandelbrot(num: Complex64, iterations: u64) -> Option<u64> {
    let mut test = num;
    let mut i: u64 = 0;
    while i < iterations && test.norm() <= 2f64 {
        test = test.powf(2f64) + num;
        i += 1;
    }
    if i >= iterations {
        return None
    }
    return Some(i)
}
