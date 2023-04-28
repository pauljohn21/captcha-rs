use base64::engine::general_purpose;
use base64::Engine;
use image::DynamicImage;
use image::ImageOutputFormat::Jpeg;
use image::{ImageBuffer, Rgb};
use imageproc::drawing::{draw_hollow_ellipse_mut, draw_text_mut};
use rand::{thread_rng, Rng};
use rusttype::{Font, Scale};
use std::io::Cursor;

// 默认随机颜色
pub const LIGHT_BASIC_COLOR: [[u8; 3]; 5] = [[214, 14, 50], [240, 181, 41], [176, 203, 40], [105, 137, 194], [242, 140, 71]];

// 默认背景颜色
pub const LIGHT: [u8; 3] = [224, 238, 253];

// 默认字体大小
pub const SCALE_SM: Scale = Scale { x: 38.0, y: 35.0 };
pub const SCALE_MD: Scale = Scale { x: 45.0, y: 42.0 };
pub const SCALE_LG: Scale = Scale { x: 53.0, y: 50.0 };

/***
 * 生成随机号
 * 最小4位
 */
pub fn get_rnd(num: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..=num)
}

/**
 * 生成验证码为String
 * 最小位4位
 */
pub fn get_captcha(num: usize) -> Vec<String> {
    let mut res = vec![];
    for _ in 0..num {
        let rnd = get_rnd(num);
        res.push(rnd.to_string())
    }
    res
}

/**
 * 获取颜色
 */
pub fn get_color() -> Rgb<u8> {
    let rnd = get_rnd(4);
    Rgb(LIGHT_BASIC_COLOR[rnd])
}

/**
 * 获取字体
 */
pub fn get_font() -> Font<'static> {
    let font = Vec::from(include_bytes!("../../fonts/arial.ttf") as &[u8]);
    Font::try_from_vec(font).unwrap()
}

/**
 * 获取背景颜色
 */
pub fn get_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |_, _| Rgb(LIGHT))
}

/**
 * 背景图像上写入验证码
 */
pub fn cyclic_write_character(res: &[String], image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let c = (image.width() - 10) / res.len() as u32;
    let y = image.height() / 2 - 15;

    let scale = match res.len() {
        1..=3 => SCALE_LG,
        4..=5 => SCALE_MD,
        _ => SCALE_SM,
    };

    for (i, _) in res.iter().enumerate() {
        let text = &res[i];

        draw_text_mut(image, get_color(), 5 + (i as u32 * c) as i32, y as i32, scale, &get_font(), text);
    }
}

/**
 * 绘制一个圆形干扰线
 */
pub fn draw_interference_ellipse(num: usize, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for _ in 0..num {
        let w = (10 + get_rnd(5)) as i32;
        let x = get_rnd((image.width() - 25) as usize) as i32;
        let y = get_rnd((image.height() - 15) as usize) as i32;
        draw_hollow_ellipse_mut(image, (x, y), w, w, get_color());
    }
}

/**
 * 转换图片为BASE64
 */
pub fn to_base64_str(image: &DynamicImage, compression: u8) -> String {
    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, Jpeg(compression)).unwrap();
    let res_base64 = general_purpose::STANDARD.encode(buf.into_inner());
    format!("data:image/jpeg;base64,{}", res_base64)
}
