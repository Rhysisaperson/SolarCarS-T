use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub speedometer: Speedometer,
}

#[derive(Deserialize, Debug)]
pub struct Speedometer {
    // colors
    pub long_notch_color: Color,
    pub short_notch_color: Color,
    pub notch_text_color: Color,
    pub speed_bar_color: Color,
    pub speed_arc_color: Color,
    pub speed_display_text_color: Color,
    pub outer_circle_outline_color: Color,
    pub outer_circle_fill_color: Color,
    pub inner_circle_outline_color: Color,
    // idk
    pub width: i32,
    pub height: i32,
    pub long_notch_length: i8,
    pub short_notch_length: i8,
    pub notch_interval: i16,
    pub font_size: f64,
    pub speed_font_size: f64,
    pub bounds: (i16, i16)
}

//add 255 -> 1.0 conversions
#[derive(Deserialize, Debug)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64
}