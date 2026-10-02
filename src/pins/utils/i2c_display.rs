use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_10X20},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
    primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment, PrimitiveStyle},
};

use ssd1306::{ prelude::*, I2CDisplayInterface, Ssd1306Async};


//----------------------------------------



pub async fn new_display(i2c_bus: esp_hal::i2c::master::I2c<'_, esp_hal::Async>)
-> Ssd1306Async<
I2CInterface<esp_hal::i2c::master::I2c<'_, esp_hal::Async>>
, DisplaySize128x32
, ssd1306::mode::BufferedGraphicsModeAsync<DisplaySize128x32>
>
{
    let interface = I2CDisplayInterface::new_custom_address(i2c_bus, crate::pins::configs::i2c0::device1_128x32_diplay::DEVICE_ADDRESS);

    let mut display = Ssd1306Async::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_buffered_graphics_mode();
    display.init().await.unwrap_or_default();

    display
}


//----------------------------------------

/// Clears the entire display.
#[inline]
pub fn clear<D>(display: &mut D) -> Result<(), D::Error>
where
D: DrawTarget<Color = BinaryColor>,
{
    display.clear(BinaryColor::Off)
}


/// Writes text using the 6x10 font.
#[inline]
pub fn write_text<D>(
    display: &mut D,
    text: &str,
    x: i32,
    y: i32,
) -> Result<(), D::Error>
where
D: DrawTarget<Color = BinaryColor>,
{
    let style = MonoTextStyle::new(
        &FONT_6X10,
        BinaryColor::On,
    );

    Text::new(
        text,
        Point::new(x, y + 6),
        style,
    )
    .draw(display)?;

    Ok(())
}


/// Writes text using the 10x20 font.
#[inline]
pub fn write_text_large<D>(
    display: &mut D,
    text: &str,
    x: i32,
    y: i32,
) -> Result<(), D::Error>
where
D: DrawTarget<Color = BinaryColor>,
{
    let style = MonoTextStyle::new(
        &FONT_10X20,
        BinaryColor::On,
    );

    Text::new(
        text,
        Point::new(x, y + 10),
        style,
    )
    .draw(display)?;

    Ok(())
}



pub fn draw_border<D>(display: &mut D) -> Result<(), D::Error>
where
D: DrawTarget<Color = BinaryColor>,
{
    let style = PrimitiveStyleBuilder::new()
    .stroke_color(BinaryColor::On)
    .stroke_width(1)
    .stroke_alignment(StrokeAlignment::Inside) // rămâne în interiorul zonei
    .build();

    let bb = display.bounding_box();
    bb.into_styled(style).draw(display)?;
    Ok(())
}


pub fn write_text_boxed_white_background<D>(
    display: &mut D,
    text: &str,
    y: i32,
    height: u32,
) -> Result<(), D::Error>
where
D: DrawTarget<Color = BinaryColor>,
{
    let width = display.bounding_box().size.width;
    let rect = Rectangle::new(Point::new(0, y), Size::new(width, height));

    rect.into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(display)?;

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
    Text::with_baseline(text, Point::new(3, y), style, Baseline::Top)
    .draw(display)?;

    Ok(())
}


pub fn write_text_boxed_border<D>(
    display: &mut D,
    text: &str,
    y: i32,
    height: u32,
) -> Result<(), D::Error>
where
D: DrawTarget<Color = BinaryColor>,
{
    let width = display.bounding_box().size.width;

    let border = PrimitiveStyleBuilder::new()
    .stroke_color(BinaryColor::On)
    .stroke_width(1)
    .stroke_alignment(StrokeAlignment::Inside)
    .build();

    Rectangle::new(Point::new(0, y), Size::new(width, height))
    .into_styled(border)
    .draw(display)?;

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::with_baseline(text, Point::new(3, y + 1), text_style, Baseline::Top)
    .draw(display)?;

    Ok(())
}


//////////////////// formaters //////////////

pub fn format_text_with_u32_2inline<'a>(
    prefix: &str,
    number: f32,
    prefix_2: &str,
    number_2: f32,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 24];
    let num_str = fmt_f32(number, 2, &mut num_buf);

    let mut num_buf_2 = [0u8; 24];
    let num_str_2 = fmt_f32(number_2, 2, &mut num_buf_2);
    concat_4(prefix, num_str, prefix_2, num_str_2, out)
}


pub fn concat_4<'a>(
    a: &str,
    b: &str,
    c: &str,
    d: &str,
    out: &'a mut [u8],
) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let c_bytes = c.as_bytes();
    let d_bytes = d.as_bytes();

    let total_len = a_bytes.len() + b_bytes.len() + c_bytes.len()+ d_bytes.len();

    if total_len > out.len() {
        return "-";
    }

    let mut pos = 0;

    out[pos..pos + a_bytes.len()].copy_from_slice(a_bytes);
    pos += a_bytes.len();

    out[pos..pos + b_bytes.len()].copy_from_slice(b_bytes);
    pos += b_bytes.len();

    out[pos..pos + c_bytes.len()].copy_from_slice(c_bytes);
    pos += c_bytes.len();

    out[pos..pos + d_bytes.len()].copy_from_slice(d_bytes);
    pos += d_bytes.len();

    core::str::from_utf8(&out[..pos]).unwrap_or("-")
}


pub fn format_text_with_f32<'a>(
    prefix: &str,
    number: f32,
    suffix: &str,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 24];
    let num_str = fmt_f32(number, 2, &mut num_buf);
    concat_3(prefix, num_str, suffix, out)
}


pub fn format_text_with_u32<'a>(
    prefix: &str,
    number: u32,
    suffix: &str,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 10];
    let num_str = fmt_u32(number, &mut num_buf);
    concat_3(prefix, num_str, suffix, out)
}


pub fn concat_3<'a>(
    a: &str,
    b: &str,
    c: &str,
    out: &'a mut [u8],
) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let c_bytes = c.as_bytes();

    let total_len = a_bytes.len() + b_bytes.len() + c_bytes.len();

    if total_len > out.len() {
        return "-";
    }

    let mut pos = 0;

    out[pos..pos + a_bytes.len()].copy_from_slice(a_bytes);
    pos += a_bytes.len();

    out[pos..pos + b_bytes.len()].copy_from_slice(b_bytes);
    pos += b_bytes.len();

    out[pos..pos + c_bytes.len()].copy_from_slice(c_bytes);
    pos += c_bytes.len();

    core::str::from_utf8(&out[..pos]).unwrap_or("-")
}




pub fn fmt_u32( n: u32, buf: &mut [u8; 10]) -> &str {
    if n == 0 {
        buf[0] = b'0';
        return core::str::from_utf8(&buf[..1]).unwrap();
    }
	let mut n = n;
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    core::str::from_utf8(&buf[i..]).unwrap()
}
#[inline]
pub fn fmt_u16(n: u16, buf: &mut [u8; 10]) -> &str {
    fmt_u32(n as u32, buf)
}



pub fn fmt_f32(value: f32, decimals: u32, buf: &mut [u8; 24]) -> &str {
    let mut pos = 0usize;
    let mut v = value;

    if v.is_sign_negative() {
        buf[pos] = b'-';
        pos += 1;
        v = -v;
    }

    let scale = match decimals {
        0 => 1u32,
        1 => 10u32,
        2 => 100u32,
        3 => 1000u32,
        4 => 10000u32,
        _ => 100u32, // fallback
    };

    let scaled = (v * scale as f32 + 0.5) as u32;
    let int_part = scaled / scale;
    let frac_part = scaled % scale;

    // partea întreagă
    let mut int_buf = [0u8; 10];
    let int_str = fmt_u32(int_part, &mut int_buf);
    let int_bytes = int_str.as_bytes();

    buf[pos..pos + int_bytes.len()].copy_from_slice(int_bytes);
    pos += int_bytes.len();

    if decimals > 0 {
        buf[pos] = b'.';
        pos += 1;

        // scriem fracția cu zero-padding
        let mut div = scale / 10;
        while div > 0 {
            buf[pos] = b'0' + ((frac_part / div) % 10) as u8;
            pos += 1;
            div /= 10;
        }
    }

    core::str::from_utf8(&buf[..pos]).unwrap_or("-")
}

