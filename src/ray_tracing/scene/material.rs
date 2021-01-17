#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Debug, Clone)]
pub enum Material {
    SolidColor(Color)
}
