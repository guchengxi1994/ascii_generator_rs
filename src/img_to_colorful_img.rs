use crate::utils::{self, get_blocks_num};
use rayon::prelude::*;

pub fn img_to_colorful_img(
    input_path: &str,
    output_path: &str,
    char_set: &str,
    block_size: Option<[usize; 2]>,
) -> anyhow::Result<()> {
    let bs = if let Some(block_size) = block_size {
        block_size
    } else {
        [12, 12]
    };

    let mut img = image::open(input_path)?.into_rgb8();
    let blocks = get_blocks_num(&mut img, bs);
    let mut new_img = image::RgbImage::new(img.width(), img.height());

    for i in 0..blocks.0 {
        for j in 0..blocks.1 {
            let rgb = utils::sub_image_mean_rgb(
                &img,
                i as u32 * bs[0] as u32,
                j as u32 * bs[1] as u32,
                bs[0] as u32,
                bs[1] as u32,
            );

            let mean = (rgb[0] + rgb[1] + rgb[2]) / 3.0;
            let char = utils::get_char(char_set, mean);
            for x in 0..bs[0] {
                for y in 0..bs[1] {
                    new_img.put_pixel(
                        i as u32 * bs[0] as u32 + x as u32,
                        j as u32 * bs[1] as u32 + y as u32,
                        image::Rgb([rgb[0] as u8, rgb[1] as u8, rgb[2] as u8]),
                    );
                }
            }
            utils::write_text(
                &char.to_string(),
                &mut new_img,
                i as i32 * bs[0] as i32,
                j as i32 * bs[1] as i32,
                None,
                false,
                [bs[0], bs[1]],
                Some([0, 0, 0]),
            )?;
        }
    }
    new_img.save(output_path)?;

    Ok(())
}

fn process_image_with_rayon(
    img: &image::RgbImage,
    blocks: (u32, u32),
    bs: [usize; 2],
    char_set: &str,
    new_img: &mut image::RgbImage,
) -> anyhow::Result<()> {
    let (block_width, block_height) = (bs[0], bs[1]);

    // 创建一个 Mutex 包裹的图像以避免多线程写入冲突
    let new_img = std::sync::Mutex::new(new_img);

    // 外层循环并行化
    (0..blocks.0).into_par_iter().for_each(|i| {
        for j in 0..blocks.1 {
            let rgb = utils::sub_image_mean_rgb(
                img,
                i as u32 * block_width as u32,
                j as u32 * block_height as u32,
                block_width as u32,
                block_height as u32,
            );

            let mean = (rgb[0] + rgb[1] + rgb[2]) / 3.0;
            let char = utils::get_char(char_set, mean);

            // 填充像素
            for x in 0..block_width {
                for y in 0..block_height {
                    let pixel_x = i as u32 * block_width as u32 + x as u32;
                    let pixel_y = j as u32 * block_height as u32 + y as u32;
                    let mut img_lock = new_img.lock().unwrap();
                    img_lock.put_pixel(
                        pixel_x,
                        pixel_y,
                        image::Rgb([rgb[0] as u8, rgb[1] as u8, rgb[2] as u8]),
                    );
                }
            }

            // 写入字符
            {
                let mut img_lock = new_img.lock().unwrap();
                utils::write_text(
                    &char.to_string(),
                    &mut *img_lock,
                    i as i32 * block_width as i32,
                    j as i32 * block_height as i32,
                    None,
                    false,
                    [block_width, block_height],
                    Some([0, 0, 0]),
                )
                .expect("Failed to write text");
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_img_to_img() -> anyhow::Result<()> {
        img_to_colorful_img(
            r"test.jpg",
            "result2.png",
            "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz",
            Some([12, 12]),
        )
    }
}
