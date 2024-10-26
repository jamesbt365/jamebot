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
    #[description = "Space-separated hex colour codes"]
    #[rest]
    colours: String,
) -> Result<(), Error> {
    let colour_codes: Vec<&str> = colours
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty())
        .collect();

    let block_width = 160;
    let block_height = 160;

    let image_width = block_width * colour_codes.len() as u32;
    let image_height = block_height;

    let mut combined_image = ImageBuffer::new(image_width, image_height);

    for (i, colour) in colour_codes.iter().enumerate() {
        if let Ok(rgba) = hex_to_rgba(colour) {
            for x in 0..block_width {
                for y in 0..block_height {
                    combined_image.put_pixel(i as u32 * block_width + x, y, Rgba(rgba));
                }
            }
        } else {
            ctx.say(format!("Could not parse colour: {}", colour))
                .await?;
            return Ok(());
        }
    }

    let bytes = {
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        DynamicImage::ImageRgba8(combined_image)
            .write_to(&mut cursor, image::ImageOutputFormat::Png)
            .unwrap();
        buffer
    };

    let attachment = serenity::CreateAttachment::bytes(bytes, "combined_colour.png");
    ctx.send(poise::CreateReply::default().attachment(attachment))
        .await?;

    Ok(())
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

#[must_use]
pub fn commands() -> [crate::Command; 1] {
    [hex()]
}
