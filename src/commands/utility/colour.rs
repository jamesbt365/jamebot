use crate::{Context, Error};
//use poise::serenity_prelude as serenity;
use image::{DynamicImage, ImageBuffer, Rgba};

#[poise::command(prefix_command, slash_command, category = "Utility", user_cooldown = "5")]
pub async fn hex(
    ctx: Context<'_>,
    #[description = "Hex colour code"] colour: String,
) -> Result<(), Error> {
    let color_image = create_color_image(&colour)?;

    let image_path = "colour.png";
    color_image.save(image_path)?;


    ctx.send(|e| {
        e.attachment(image_path.into())
    })
    .await?;

    Ok(())
}

fn create_color_image(hex_color: &str) -> Result<DynamicImage, Error> {
    let rgba_color = hex_to_rgba(hex_color)?;

    let mut img = ImageBuffer::new(160, 160);

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba(rgba_color);
    }

    Ok(DynamicImage::ImageRgba8(img))
}

fn hex_to_rgba(hex_color: &str) -> Result<[u8; 4], Error> {
    let hex_color = hex_color.trim_start_matches('#');

    let trimmed_hex_color = if hex_color.len() > 6 {
        &hex_color[0..6]
    } else {
        hex_color
    };

    let normalized_hex_color = if trimmed_hex_color.len() < 6 {
        format!("{:0<6}", trimmed_hex_color)
    } else {
        trimmed_hex_color.to_string()
    };

    let r = u8::from_str_radix(&normalized_hex_color[0..2], 16)?;
    let g = u8::from_str_radix(&normalized_hex_color[2..4], 16)?;
    let b = u8::from_str_radix(&normalized_hex_color[4..6], 16)?;

    Ok([r, g, b, 255]) // Set alpha to 255 (fully opaque)
}
