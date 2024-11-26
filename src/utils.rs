use std::cmp::min;

use image::Rgb;

use crate::constants;

pub fn write_text(
    text: &str,
    img: &mut image::RgbImage,
    x: i32,
    y: i32,
    font: Option<&[u8]>,
    dark_mode: bool,
    size: [usize; 2],
    color: Option<[u8; 3]>,
) -> anyhow::Result<()> {
    let f_clone = constants::FONT.clone();
    let f;
    if let Some(font) = font {
        f = ab_glyph::FontRef::try_from_slice(&font)?;
    } else {
        f = ab_glyph::FontRef::try_from_slice(&f_clone)?;
    }

    let c;
    if let Some(color) = color {
        c = Rgb(color);
    } else {
        if dark_mode {
            c = Rgb([255, 255, 255]);
        } else {
            c = Rgb([0, 0, 0]);
        }
    }

    imageproc::drawing::draw_text_mut(
        img,
        c,
        x,
        y,
        ab_glyph::PxScale {
            x: size[0] as f32 - 1.,
            y: size[1] as f32 - 1.,
        },
        &f,
        text,
    );

    anyhow::Ok(())
}

pub fn sub_image(img: &image::RgbImage, x: u32, y: u32, w: u32, h: u32) -> image::RgbImage {
    image::imageops::crop_imm(img, x, y, w, h).to_image()
}

pub fn sub_image_mean(img: &image::RgbImage, x: u32, y: u32, w: u32, h: u32) -> f64 {
    // imageproc::stats::mean_f32(&sub_image(img, x, y, w, h))
    let sub_img = sub_image(img, x, y, w, h);
    let r: u64 = sub_img
        .pixels()
        .map(|p| (p.0[0] as u64 + p.0[1] as u64 + p.0[2] as u64) as u64)
        .sum();
    let pixel_count = w * h * 3;
    let mean = r as f64 / pixel_count as f64;
    mean
}

pub fn sub_image_mean_rgb(img: &image::RgbImage, x: u32, y: u32, w: u32, h: u32) -> [f64; 3] {
    // imageproc::stats::mean_f32(&sub_image(img, x, y, w, h))
    let sub_img = sub_image(img, x, y, w, h);
    let r: u64 = sub_img.pixels().map(|p| p.0[0] as u64).sum();
    let g: u64 = sub_img.pixels().map(|p| p.0[1] as u64).sum();
    let b: u64 = sub_img.pixels().map(|p| p.0[2] as u64).sum();
    let pixel_count = w * h;
    let r_mean = r as f64 / pixel_count as f64;
    let g_mean = g as f64 / pixel_count as f64;
    let b_mean = b as f64 / pixel_count as f64;
    [r_mean, g_mean, b_mean]
}

pub fn get_index(char_set_len: usize, mean: f64) -> usize {
    let i = mean / 255.0 * char_set_len as f64;
    return min(i.round() as usize, char_set_len - 1);
}

pub fn get_blocks_num(img: &mut image::RgbImage, block_size: [usize; 2]) -> (u32, u32) {
    let block_num_in_horizontal = img.width() as u32 / block_size[0] as u32;
    let block_num_in_vertical = img.height() as u32 / block_size[1] as u32;

    // image::imageops::resize(
    //     img,
    //     block_num_in_horizontal * (block_size[0] as u32),
    //     block_num_in_vertical * (block_size[1] as u32),
    //     image::imageops::FilterType::Nearest,
    // );

    return (block_num_in_horizontal, block_num_in_vertical);
}

fn get_char_(char_set: &str, index: usize) -> char {
    char_set.chars().nth(index).unwrap()
}

pub fn get_char(char_set: &str, mean: f64) -> char {
    let index = get_index(char_set.len(), mean);
    get_char_(char_set, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_text() -> anyhow::Result<()> {
        let mut img = image::RgbImage::new(100, 100);
        write_text(
            "H",
            &mut img,
            10,
            10,
            None,
            false,
            [10, 10],
            Some([255, 0, 0]),
        )?;
        img.save("result.png")?;
        anyhow::Ok(())
    }

    #[test]
    fn crop() -> anyhow::Result<()> {
        let mut img = image::RgbImage::new(100, 100);
        write_text(
            "H",
            &mut img,
            10,
            10,
            None,
            false,
            [10, 10],
            Some([255, 255, 255]),
        )?;
        let r = sub_image_mean(&img, 10, 10, 10, 10);
        println!("{}", r);
        // let index = get_index(26 * 2, r);
        let index = get_char("AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz", r);
        println!("{}", index);
        anyhow::Ok(())
    }
}
