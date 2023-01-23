use image::{Rgb, RgbImage};
use num::complex::Complex;
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug, Copy, Clone)]
enum FractalType {
    Julia,
    Mandelbrot,
}

#[derive(Deserialize, Debug)]
struct Config {
    height: u32,
    width: u32,
    scale_fac: f64,
    fractal_type: FractalType,
    julia_r: f64,
    julia_i: f64,
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let config_file = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_file).expect("Error! ");
    println!("Rendering a fractal with the following settings...");
    println!("{:#?}", config);

    let mut output = RgbImage::new(config.width, config.height);
    output = fill(output, config);

    println!("{:.2?} elapsed", now.elapsed());

    println!("Writing to file...");
    output.save("output.png").unwrap();
}

fn fill(mut a: RgbImage, config: Config) -> RgbImage {
    let (mut z_bright, mut z, c);
    let (mut fx, mut fy): (f64, f64);
    let (xmax, xmin, ymax, ymin) = (
        2.0 * config.scale_fac,
        -2.0 * config.scale_fac,
        2.0 * config.scale_fac,
        -2.0 * config.scale_fac,
    );
    match config.fractal_type {
        FractalType::Julia => {
            c = Complex::new(config.julia_r, config.julia_i);
        }
        FractalType::Mandelbrot => {
            c = Complex::new(0.0, 0.0);
        }
    }
    for y in 0..config.height {
        fy = y as f64 / config.height as f64 * (ymax - ymin) + ymin;
        for x in 0..config.width {
            fx = x as f64 / config.width as f64 * (xmax - xmin) + xmin;
            z = Complex::new(fx, fy);
            match config.fractal_type {
                FractalType::Julia => z_bright = julia(z, c),
                FractalType::Mandelbrot => z_bright = mandelbrot(z),
            }
            a.put_pixel(x, y, Rgb([z_bright, z_bright, z_bright]));
        }
    }
    a
}

fn mandelbrot(z: Complex<f64>) -> u8 {
    let iterations = 200;
    let mut v: Complex<f64> = Complex::new(0.0, 0.0);
    for n in 0..iterations {
        v = v * v + z;
        if v.norm() > 2.0 {
            return n;
        };
    }
    255
}

fn julia(mut z: Complex<f64>, c: Complex<f64>) -> u8 {
    let iterations = 200;
    for n in 0..iterations {
        z = z * z + c;
        if z.norm() >= 2.0 {
            return n + 1 - (z.norm().ln().log2() as u8);
            // return n;
        }
    }
    0
}
