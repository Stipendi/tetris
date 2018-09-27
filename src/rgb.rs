pub fn rgb(red: u8, green: u8, blue: u8, opacity: f32) -> [f32; 4] {
    [red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, opacity]
}
