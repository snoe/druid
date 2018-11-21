pub struct TextFormat {}

impl TextFormat {
    pub fn create(factory: &Factory) -> TextFormat {
        TextFormat {}
    }

    pub fn with_family(&self, family: &str) -> &Self { &self }
    pub fn with_size(&self, size: f32) -> &Self { &self }

    pub fn build(&self) -> Option<&Self> { Some(&self) }
}

pub struct TextLayout {}

impl TextLayout {
    pub fn create(factory: &Factory) -> TextLayout {
        TextLayout {}
    }
    pub fn with_text(&self, label: &str) -> &Self { &self }
    pub fn with_font(&self, font: &TextFormat) -> &Self { &self }
    pub fn with_width(&self, width: f32) -> &Self { &self }
    pub fn with_height(&self, height: f32) -> &Self { &self }
    pub fn build(&self) -> Option<&Self> { Some(&self) }
}

pub struct Factory {}

pub enum DrawTextOptions {
    NONE,
}
