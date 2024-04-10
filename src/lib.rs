use base64::{engine::general_purpose, Engine as _};
use exolvl::{Colour, Exolvl, Object, ObjectProperty, Pattern, Read as _, Vec2, Write as _};
use flate2::{write::GzEncoder, Compression};
use image::{
    codecs::{gif::GifDecoder, png::PngEncoder},
    imageops::FilterType,
    io::Reader,
    AnimationDecoder, DynamicImage, Frames, GenericImageView, ImageEncoder, Rgba,
};
use rand::Rng;
use std::{
    borrow::Cow,
    error::Error,
    io::{BufReader, Cursor, Write},
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

const LEVELFILE: &[u8; 432] = include_bytes!("default.exolvl");

#[wasm_bindgen]
pub fn convert_gif(image_data_url: &str, level_name: &str) -> Result<Vec<u8>, String> {
    convert_gif_inner(image_data_url, level_name).map_err(|e| e.to_string())
}

fn convert_gif_inner(image_data_url: &str, level_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut level = read_level()?;

    let img = read_image_bytes(image_data_url)?;

    let decoder = GifDecoder::new(Cursor::new(img))?;

    let frames = decoder.into_frames();

    let pattern = pattern(frames)?;

    level.level_data.patterns.push(pattern);

    update_level_properties(&mut level, level_name);

    set_theme(&mut level);

    write_level(&level)
}

#[wasm_bindgen]
pub fn convert(
    image_data_url: &str,
    should_resize: bool,
    width: u32,
    height: u32,
    level_name: &str,
) -> Result<Vec<u8>, String> {
    convert_inner(image_data_url, should_resize, width, height, level_name)
        .map_err(|e| e.to_string())
}

fn convert_inner(
    image_data_url: &str,
    should_resize: bool,
    width: u32,
    height: u32,
    level_name: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut level = read_level()?;

    let img = read_image_bytes(image_data_url)?;

    let img = Reader::new(Cursor::new(img))
        .with_guessed_format()?
        .decode()?;

    process_image(&mut level, &img, should_resize, width, height)?;

    update_level_properties(&mut level, level_name);

    set_theme(&mut level);

    write_level(&level)
}

fn update_level_properties(level: &mut Exolvl, level_name: &str) {
    let created_time = chrono::Utc::now();

    level.local_level.level_id = Uuid::new_v4().to_string();
    level.local_level.level_name = level_name.to_string();
    level.local_level.creation_date = created_time;
    level.local_level.update_date = created_time;
}

fn write_level(level: &Exolvl) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output = Vec::new();

    level.write(&mut output)?;

    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write_all(&output)?;

    Ok(e.finish()?)
}

fn read_level() -> Result<Exolvl, Box<dyn Error>> {
    Ok(Exolvl::read(&mut BufReader::new(&LEVELFILE[..]))?)
}

fn read_image_bytes(image_data_url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image_data_url
        .split(',')
        .nth(1)
        .ok_or("unexpected image data url format")?;

    Ok(general_purpose::STANDARD.decode(img)?)
}

fn pattern(gif_frames: Frames) -> Result<Pattern, Box<dyn Error>> {
    let mut pattern_frames = Vec::new();

    for frame in gif_frames.collect::<Result<Vec<_>, _>>()? {
        let img = DynamicImage::ImageRgba8(frame.into_buffer());

        let img = img.resize_exact(512, 512, FilterType::Lanczos3);

        let mut img_buf = Vec::new();

        let encoder = PngEncoder::new(&mut img_buf);

        encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color())?;

        pattern_frames.push(exolvl::Image(img_buf));
    }

    let mut rng = rand::thread_rng();

    Ok(Pattern {
        pattern_id: rng.gen::<i32>(),
        pattern_frames,
    })
}

fn process_image(
    level: &mut Exolvl,
    img: &DynamicImage,
    should_resize: bool,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn Error>> {
    let img = if should_resize {
        Cow::Owned(img.resize(width, height, FilterType::Lanczos3))
    } else {
        Cow::Borrowed(img)
    };

    let pixels = img.pixels();

    let layer = level
        .level_data
        .layers
        .get_mut(0)
        .ok_or("level file doesn't have any layers")?;

    for (entity_id, pixel) in pixels.enumerate() {
        let entity_id = entity_id.try_into()?;

        let obj = pixel_object(entity_id, pixel, &img);

        level.level_data.objects.push(obj);

        layer.children.push(entity_id);
    }

    Ok(())
}

fn pixel_object(entity_id: i32, pixel: (u32, u32, Rgba<u8>), img: &DynamicImage) -> Object {
    Object {
        entity_id,
        tile_id: 113491821,
        prefab_entity_id: 0,
        prefab_id: 0,
        position: Vec2 {
            x: pixel.0 as f32 + 0.5,
            y: (img.height() - pixel.1) as f32 + 0.5,
        },
        scale: Vec2 { x: 1.0, y: 1.0 },
        rotation: 0.0,
        tag: String::new(),
        properties: vec![ObjectProperty::Colour(Colour {
            r: pixel.2[0] as f32 / 255.,
            g: pixel.2[1] as f32 / 255.,
            b: pixel.2[2] as f32 / 255.,
            a: pixel.2[3] as f32 / 255.,
        })],
        in_layer: 1,
        in_group: 0,
        group_members: vec![],
    }
}

fn set_theme(level: &mut Exolvl) {
    level.level_data.theme = "custom".to_string();

    level.level_data.custom_terrain_colour = Colour {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };

    level.level_data.custom_terrain_border_colour = Colour {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };

    level.level_data.custom_background_colour = Colour {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
}
