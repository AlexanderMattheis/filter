use image::{DynamicImage, GenericImageView};

use crate::system::defaults::types::border_handling_type::BorderHandlingType;

pub struct BorderHandling {
    pub get_pixel: fn(image: &DynamicImage, u: i32, v: i32, background_color: &[u8; 4]) -> [u8; 4]
}

impl BorderHandling {
    pub fn new(border_handling_type: &BorderHandlingType) -> BorderHandling {
        return BorderHandling {
            get_pixel: match border_handling_type {
                BorderHandlingType::PaddingConstantValue => get_constant_pixel,
                BorderHandlingType::PaddingExtend => get_extended_pixel,
                BorderHandlingType::PaddingMirror => get_mirrored_pixel,
                BorderHandlingType::PaddingPeriodically => get_periodic_pixel
            }
        };
    }
}

fn get_constant_pixel(_image: &DynamicImage, _pos_u: i32, _pos_v: i32, _background_color: &[u8; 4]) -> [u8; 4] {
    return *_background_color;
}

fn get_extended_pixel(_image: &DynamicImage, _pos_u: i32, _pos_v: i32, _background_color: &[u8; 4]) -> [u8; 4] {
    let dimensions = _image.dimensions();

    let u;
    let v;

    if _pos_u < 0 {  // left bound
        u = 0;
    } else if _pos_u >= dimensions.0 as i32 {  // right bound
        u = dimensions.0 - 1;
    } else {
        u = _pos_u as u32;
    }

    if _pos_v < 0 {
        v = 0;
    } else if _pos_v >= dimensions.1 as i32 {
        v = dimensions.1 - 1;
    } else {
        v = _pos_v as u32;
    }

    return _image.get_pixel(u, v).0;
}

fn get_mirrored_pixel(_image: &DynamicImage, _pos_u: i32, _pos_v: i32, _background_color: &[u8; 4]) -> [u8; 4] {
    let dimensions = _image.dimensions();

    let u;
    let v;

    if _pos_u < 0 {
        u = -_pos_u;
    } else if _pos_u >= dimensions.0 as i32 {
        u = (dimensions.0 - 1) as i32 - (_pos_u - (dimensions.0 - 1) as i32);
    } else {
        u = _pos_u;
    }

    if _pos_v < 0 {
        v = -_pos_v;
    } else if _pos_v >= dimensions.1 as i32 {
        v = (dimensions.1 - 1) as i32 - (_pos_v - (dimensions.1 - 1) as i32);
    } else {
        v = _pos_v;
    }

    return _image.get_pixel(u as u32, v as u32).0;
}

fn get_periodic_pixel(_image: &DynamicImage, _pos_u: i32, _pos_v: i32, _background_color: &[u8; 4]) -> [u8; 4] {
    let dimensions = _image.dimensions();

    let u;
    let v;

    if _pos_u < 0 {
        u = dimensions.0 + _pos_u as u32;
    } else if _pos_u >= dimensions.0 as i32 {
        u = _pos_u as u32 - dimensions.0;
    } else {
        u = _pos_u as u32;
    }

    if _pos_v < 0 {
        v = dimensions.1 + _pos_v as u32;
    } else if _pos_v >= dimensions.1 as i32 {
        v = _pos_v as u32 - dimensions.1;
    } else {
        v = _pos_v as u32;
    }

    return _image.get_pixel(u as u32, v as u32).0;
}