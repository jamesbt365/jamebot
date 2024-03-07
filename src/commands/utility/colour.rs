use crate::{Context, Error};
use image::{DynamicImage, ImageBuffer, Rgba};
use poise::serenity_prelude as serenity;

#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    user_cooldown = "5"
)]
pub async fn hex(
    ctx: Context<'_>,
    #[description = "Hex colour code"] colour: String,
) -> Result<(), Error> {

    let colour_img = if let Ok(rgba) = hex_to_rgba(&colour) {
        create_color_image(rgba)
    } else {
        ctx.say("Could not parse colour!").await?;
        return Ok(());
    };

    let bytes = {
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        colour_img
            .write_to(&mut cursor, image::ImageOutputFormat::Png)
            .unwrap();
        buffer
    };

    let attachment = serenity::CreateAttachment::bytes(bytes, "colour.png");
    ctx.send(poise::CreateReply::default().attachment(attachment))
        .await?;

    Ok(())
}

fn create_color_image(rgba: [u8; 4]) -> image::DynamicImage {
    let mut img = ImageBuffer::new(160, 160);

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba(rgba);
    }

    DynamicImage::ImageRgba8(img)
}

fn hex_to_rgba(hex_color: &str) -> Result<[u8; 4], Error> {
    let hex_color = hex_color.trim_start_matches('#');

    let trimmed_hex_color = if hex_color.len() > 6 {
        &hex_color[0..6]
    } else {
        hex_color
    };

    let normalized_hex_color = if trimmed_hex_color.len() < 6 {
        format!("{trimmed_hex_color:0<6}")
    } else {
        trimmed_hex_color.to_string()
    };

    let r = u8::from_str_radix(&normalized_hex_color[0..2], 16)?;
    let g = u8::from_str_radix(&normalized_hex_color[2..4], 16)?;
    let b = u8::from_str_radix(&normalized_hex_color[4..6], 16)?;

    Ok([r, g, b, 255]) // Set alpha to 255 (fully opaque)
}
