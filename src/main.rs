use gif::{Encoder, Frame, Repeat};
use image::{Rgb, RgbImage};
use std::fs::File;
use clap::Parser;


#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    start_color: String,
    end_color: String,

    width: u32,
    height: u32,

    duration: u32,
}

fn parse_rgb(s: &str) -> Result<[u8; 3], String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("Must be in R, G, B format".to_string());
    }

    let nums: Result<Vec<u8>, _> = parts.iter().map(|p| p.trim().parse()).collect();
    nums.map_err(|_| "Each value must be a number between 0 - 255".to_string())?
        .try_into()
        .map_err(|_| "Invalid RGB length".to_string())
}

fn interpolate_color(color1: [u8; 3], color2: [u8; 3], ratio: f32) -> [u8; 3] {
    [
        ((1.0 - ratio) * color1[0] as f32 + ratio * color2[0] as f32).round() as u8,
        ((1.0 - ratio) * color1[1] as f32 + ratio * color2[1] as f32).round() as u8,
        ((1.0 - ratio) * color1[2] as f32 + ratio * color2[2] as f32).round() as u8,
    ]
}

fn generate_gradient_frame(
    width: u32,
    height: u32,
    progress: f32,
    color1: [u8; 3],
    color2: [u8; 3],
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

    let color1 = parse_rgb(&args.start_color)?;
    let color2 = parse_rgb(&args.end_color)?;

    let width = &args.width;
    let height = &args.height;


    let duration_seconds = &args.duration;
    
    let fps = 30;

    let total_frames = duration_seconds * fps;
    let delay = (100.0 / fps as f32).round() as u16;

    let file = File::create("gradient")?;
    let mut encoder = Encoder::new(file, *width as u16, *height as u16, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    for i in 0..total_frames {
        let mut progress = (i as f32 / total_frames as f32) * 2.0;
        if progress > 1.0 {
            progress = 2.0 - progress;
        }

        let img = generate_gradient_frame(*width, *height, progress, color1, color2);
        let mut frame = Frame::from_rgb(*width as u16, *height as u16, &img);
        frame.delay = delay;
        encoder.write_frame(&frame)?;
    }

    Ok(())
}
