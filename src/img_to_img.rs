use image::Rgb;

use crate::utils::{self, image_to_blocks_and_reshape};

pub fn img_to_img(
    input_path: &str,
    output_path: &str,
    char_set: &str,
    block_size: Option<[usize; 2]>,
    dark_mode: bool,
) -> anyhow::Result<()> {
    let bs = if let Some(block_size) = block_size {
        block_size
    } else {
        [12, 12]
    };

    let mut img = image::open(input_path)?.into_rgb8();
    let blocks = image_to_blocks_and_reshape(&mut img, bs);
    let mut new_img;
    if dark_mode {
        new_img = image::RgbImage::new(img.width(), img.height());
    } else {
        new_img = image::RgbImage::from_pixel(img.width(), img.height(), Rgb([255, 255, 255]));
    }

    for i in 0..blocks.0 {
        for j in 0..blocks.1 {
            let mean = utils::sub_image_mean(
                &img,
                i as u32 * bs[0] as u32,
                j as u32 * bs[1] as u32,
                bs[0] as u32,
                bs[1] as u32,
            );
            let char = utils::get_char(char_set, mean);
            utils::write_text(
                &char.to_string(),
                &mut new_img,
                i as i32 * bs[0] as i32,
                j as i32 * bs[1] as i32,
                None,
                dark_mode,
                [bs[0], bs[1]],
                None,
            )?;
        }
    }

    new_img.save(output_path)?;

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_img_to_img() -> anyhow::Result<()> {
        img_to_img(
            r"test.jpg",
            "result.png",
            "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz",
            Some([12, 12]),
            false,
        )
    }
}
