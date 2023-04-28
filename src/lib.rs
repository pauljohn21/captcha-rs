use image::DynamicImage;
use imageproc::noise::{gaussian_noise_mut, salt_and_pepper_noise_mut};

use crate::captcha::{cyclic_write_character, draw_interference_ellipse, get_image, to_base64_str};

mod captcha;

pub struct Captcha {
    pub text: String,
    pub image: DynamicImage,
    pub compression: u8,
}

impl Captcha {
    pub fn to_base64(&self) -> String {
        to_base64_str(&self.image, self.compression)
    }
}

#[derive(Default)]
pub struct CaptchaBuilder {
    text: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    complexity: Option<u32>,
    compression: Option<u8>,
}

impl CaptchaBuilder {
    pub fn new() -> Self {
        CaptchaBuilder {
            text: None,
            width: None,
            height: None,
            complexity: None,
            compression: Some(40),
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn length(mut self, length: usize) -> Self {
        // Generate an array of captcha characters
        let res = captcha::get_captcha(length);
        self.text = Some(res.join(""));
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn complexity(mut self, complexity: u32) -> Self {
        let mut complexity = complexity;
        if complexity > 10 {
            complexity = 10;
        }
        if complexity < 1 {
            complexity = 1;
        }
        self.complexity = Some(complexity);
        self
    }

    pub fn compression(mut self, compression: u8) -> Self {
        self.compression = Some(compression);
        self
    }

    pub fn build(self) -> Captcha {
        let text = self.text.unwrap_or(captcha::get_captcha(5).join(""));
        let width = self.width.unwrap_or(130);
        let height = self.height.unwrap_or(40);
        let complexity = self.complexity.unwrap_or(1);

        // Create a white background image
        let mut image = get_image(width, height);

        let res: Vec<String> = text.chars().map(|x| x.to_string()).collect();

        // Loop to write the verification code string into the background image
        cyclic_write_character(&res, &mut image);

        // Draw a distraction circle
        draw_interference_ellipse(2, &mut image);
        draw_interference_ellipse(2, &mut image);

        if complexity > 1 {
            gaussian_noise_mut(&mut image, (complexity - 1) as f64, ((5 * complexity) - 5) as f64, ((5 * complexity) - 5) as u64);
            salt_and_pepper_noise_mut(&mut image, (0.002 * complexity as f64) - 0.002, (0.5 * complexity as f64) as u64);
        }

        Captcha {
            text,
            image: DynamicImage::ImageRgb8(image),
            compression: 40,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CaptchaBuilder;

    #[test]
    fn it_generates_captcha_using_builder() {
        let captcha = CaptchaBuilder::new().length(4).width(200).height(70).complexity(5).compression(40).build();

        let base_img = captcha.to_base64();
        println!("text: {}", captcha.text);
        println!("base_img: {:#?}", base_img);
    }
}
