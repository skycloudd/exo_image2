use base64::{engine::general_purpose, Engine as _};
use exolvl::{Colour, Exolvl, Object, ObjectProperty, Read as _, Vec2, Write as _};
use flate2::{write::GzEncoder, Compression};
use image::{imageops::FilterType, io::Reader, DynamicImage, GenericImageView};
use std::io::{BufReader, Cursor, Write};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(
    image_data_url: &str,
    should_resize: bool,
    width: u32,
    height: u32,
    level_name: &str,
) -> Result<Vec<u8>, String> {
    let input = include_bytes!("../default.exolvl");

    let img = image_data_url
        .split(',')
        .nth(1)
        .ok_or("invalid image data url")?;

    let img = general_purpose::STANDARD
        .decode(img)
        .map_err(|e| e.to_string())?;

    let img = Reader::new(Cursor::new(img))
        .with_guessed_format()
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let mut level =
        Exolvl::read(&mut BufReader::new(&input[..])).map_err(|e| format!("{:?}", e))?;

    let created_time = chrono::Utc::now();

    level.local_level.level_id = Uuid::new_v4().to_string();
    level.local_level.level_name = level_name.to_string();
    level.local_level.creation_date = created_time;
    level.local_level.update_date = created_time;

    process_image(&mut level, img, should_resize, width, height);

    let mut output = Vec::new();

    level.write(&mut output).map_err(|e| e.to_string())?;

    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write_all(&output).map_err(|e| e.to_string())?;

    Ok(e.finish().map_err(|e| e.to_string())?)
}

fn process_image(
    level: &mut Exolvl,
    mut img: DynamicImage,
    should_resize: bool,
    width: u32,
    height: u32,
) {
    if should_resize {
        img = img.resize(width, height, FilterType::Lanczos3)
    };

    let pixels = img.pixels();

    let layer = level.level_data.layers.get_mut(0).unwrap();

    let mut entity_id = 0;

    for pixel in pixels {
        let obj = Object {
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
        };

        level.level_data.objects.push(obj);

        layer.children.push(entity_id);

        entity_id += 1;
    }

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
