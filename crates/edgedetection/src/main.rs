use anyhow::{Context, Result};
use core::ops::Add;
use image::{
    io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbaImage,
};
use log::{debug, info};
use std::env;

use nalgebra::{matrix, vector, ArrayStorage, Const, DMatrix, Matrix, Matrix3, Vector3};

use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.y,
            y: self.y + other.y,
        }
    }
}

fn convolve_dotproducts(img: DynamicImage, kernel: &Matrix3<f32>, threshold: f32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut out = RgbaImage::new(width, height);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let neighbours = Matrix3::from_fn(|i, j| {
                let pixel = img.get_pixel(x + i as u32 - 1, y + j as u32 - 1);
                let norm =
                    Vector3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32).normalize();

                if norm.x.is_nan() {
                    Vector3::new(0.0, 0.0, -1.0)
                } else {
                    norm
                }
            });

            let center =
                Vector3::new(neighbours.m22[0], neighbours.m22[1], neighbours.m22[2]).normalize();

            let mut result = neighbours.zip_fold(kernel, 1.0, |cur, neighbour, weight| {

                let dotproduct = center.dot(&neighbour);

                if neighbour != center {
                    debug!(
                        "x: {}, y: {}, Dotproduct: {}, weight: {}, \ncenter: {}, neighbour: {}, lessthan: {}, greaterthan: {}",
                        x,
                        y,
                        dotproduct,
                        weight,
                        center,
                        neighbour,
                        dotproduct < threshold,
                        dotproduct > threshold
                    );
                    //panic!("Fin");
                }
                f32::min(cur,dotproduct)
            });

            debug!("Res: {:?}, x: {}, y: {}", result, x, y);

            result = (1.0 - result).abs() / 2.0;

            if result < threshold {
                result = 0.0;
            }

            result *= 256.0;

            let out_pixel = Pixel::from_channels(result as u8, result as u8, result as u8, u8::MAX);

            out.put_pixel(x, y, out_pixel);
        }
    }

    out
}

fn convolve<F>(img: DynamicImage, kernel: &Matrix3<f32>, pixelizer: F) -> RgbaImage
where
    F: Fn((f32, f32, f32)) -> (u8, u8, u8, u8),
{
    let (width, height) = img.dimensions();
    let mut out = RgbaImage::new(width, height);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let neighbours =
                Matrix3::from_fn(|i, j| img.get_pixel(x + i as u32 - 1, y + j as u32 - 1));

            let result = neighbours.zip_fold(kernel, (0.0, 0.0, 0.0), |cur, pixel, weight| {
                (
                    cur.0 + weight * (pixel[0] as f32 / 255.0),
                    cur.1 + weight * (pixel[1] as f32 / 255.0),
                    cur.2 + weight * (pixel[2] as f32 / 255.0),
                )
            });

            let pixelized = pixelizer(result);

            let out_pixel =
                Pixel::from_channels(pixelized.0, pixelized.1, pixelized.2, pixelized.3);

            out.put_pixel(x, y, out_pixel);
        }
    }

    out
}

fn filter3x3(img: DynamicImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut out = RgbaImage::from_pixel(width, height, image::Rgba([255, 0, 0, 1]));

    let mut offsets = matrix!(
        vector!(-1, -1),
        vector!(0, -1),
        vector!(1, -1),
        vector!(-1, 0),
        vector!(0, 0),
        vector!(1, 0),
        vector!(-1, 1),
        vector!(0, 1),
        vector!(1, 1)
    );

    let edge = matrix!(-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0);

    for y in 1..height - 1 {
        info!("Processing row {}", y);
        for x in 1..width - 1 {
            let mut center = img.get_pixel(x, y);
            let neighbours = offsets.add_scalar(vector!(x as i32, y as i32));
            let mut value = 0.0;

            for (i, neighbour) in neighbours.iter().enumerate() {
                let pixel = img.get_pixel(neighbour.x as u32, neighbour.y as u32);
                value += pixel[0] as f32 * edge[i];
            }

            let val: u8 = value as u8;

            let out_pixel = Pixel::from_channels(val, val, val, u8::MAX);

            out.put_pixel(x, y, out_pixel);
        }
    }

    out
}

lazy_static! {
    static ref EDGE_DETECTION: Matrix3<f32> =
        Matrix3::new(-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0);
    static ref BOXBLUR: Matrix3<f32> = Matrix3::from_element(1.0 / 9.0);
    static ref DOTPRODUCTS: Matrix3<f32> =
        Matrix3::new(1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0);
}
fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).context("Failed to get input path argument")?;
    let output = args.get(2).context("Failed to get output path argument")?;

    info!("Loading image at: {}", input);

    let img = ImageReader::open(input)?.decode()?;

    //let filtered = filter3x3(img);
    let filtered = convolve(img, &EDGE_DETECTION, |(x, y, z)| {
        let mut max = (x.max(y.max(z))) * 255.0;
        if max <= 0.01 {
            max = 0.0;
        } else {
            debug!("{}", max);
        }
        let max = max as u8;
        (max, max, max, 255)
    });
    //let filtered = convolve_dotproducts(img, &EDGE_DETECTION, 0.0);
    //let filtered = convolve_dotproducts(img, &DOTPRODUCTS, 0.04);
    filtered.save(output)?;

    Ok(())
}
