use clap::Parser;
use gif::{Encoder, Frame, Repeat};
use image::{Rgb, RgbImage};
use std::fs::File;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    start_color: String,
    end_color: String,
    width: u32,
    height: u32,
    duration: u32,
}

// We want use a [R,G,B] for our calculations
fn parse_input(s: &str) -> Result<[u8; 3], String> {
    if s.starts_with('#') {
        return hex_to_rgb(s);
    }

    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("Must be in R, G, B format".to_string());
    }

    let nums: Result<Vec<u8>, _> = parts.iter().map(|p| p.trim().parse()).collect();
    nums.map_err(|_| "Each value must be a number between 0 - 255".to_string())?
        .try_into()
        .map_err(|_| "Invalid RGB length".to_string())
}

fn hex_to_rgb(hex_code: &str) -> Result<[u8; 3], String> {
    let hex_code = hex_code.trim_start_matches('#');

    if hex_code.len() != 6 {
        return Err("Hex code must be 6 characters long!".to_string());
    }

    let num = u32::from_str_radix(hex_code, 16).map_err(|_| "Invalid hex code!".to_string())?;

    let r = ((num >> 16) & 0xFF) as u8;
    let g = ((num >> 8) & 0xFF) as u8;
    let b = (num & 0xFF) as u8;

    Ok([r, g, b])
}

fn interpolate_color(color1: [u8; 3], color2: [u8; 3], ratio: f32) -> [u8; 3] {
    [
        ((1.0 - ratio) * color1[0] as f32 + ratio * color2[0] as f32).round() as u8,
        ((1.0 - ratio) * color1[1] as f32 + ratio * color2[1] as f32).round() as u8,
        ((1.0 - ratio) * color1[2] as f32 + ratio * color2[2] as f32).round() as u8,
    ]
}

fn generate_gradient_frame(
    color1: [u8; 3],
    color2: [u8; 3],
    width: u32,
    height: u32,
    progress: f32,
) -> RgbImage {
    let extended_width = width * 2;
    let mut img = RgbImage::new(extended_width, height);

    for x in 0..extended_width {
        let ratio = x as f32 / extended_width as f32;
        let color = interpolate_color(color1, color2, ratio);
        for y in 0..height {
            img.put_pixel(x, y, Rgb(color));
        }
    }

    let offset = (progress * width as f32) as u32;
    image::imageops::crop(&mut img, offset, 0, width, height).to_image()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let color1 = parse_input(&args.start_color)?;
    let color2 = parse_input(&args.end_color)?;

    let width = &args.width;
    let height = &args.height;

    let duration_seconds = &args.duration;

    let fps = 30;

    let total_frames = duration_seconds * fps;
    let delay = (100.0 / fps as f32).round() as u16;

    let file = File::create(format!(
        "gradient{}x{}_{}.gif",
        width, height, duration_seconds
    ))?;
    let mut encoder = Encoder::new(file, *width as u16, *height as u16, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    for i in 0..total_frames {
        let mut progress = (i as f32 / total_frames as f32) * 2.0;
        if progress > 1.0 {
            progress = 2.0 - progress;
        }

        let img = generate_gradient_frame(color1, color2, *width, *height, progress);
        let mut frame = Frame::from_rgb(*width as u16, *height as u16, &img);
        frame.delay = delay;
        encoder.write_frame(&frame)?;
    }

    Ok(())
}

