pub struct TextFormat {}

impl TextFormat {
    pub fn create(factory: &Factory) -> TextFormatBuilder {
        TextFormatBuilder {}
    }

}

pub struct TextLayout {}

impl TextLayout {
    pub fn create(factory: &Factory) -> TextLayoutBuilder {
        TextLayoutBuilder {}
    }
}

pub struct Factory {}

impl Factory {
    pub fn new() -> Option<Factory> {
        Some(Factory {})
    }
}

pub enum DrawTextOptions {
    NONE,
}


pub struct TextLayoutBuilder
{
}

impl TextLayoutBuilder
{
    pub fn with_text(&self, label: &str) -> &Self { &self }
    pub fn with_font(&self, font: &TextFormat) -> &Self { &self }
    pub fn with_width(&self, width: f32) -> &Self { &self }
    pub fn with_height(&self, height: f32) -> &Self { &self }
    pub fn build(&self) -> Option<TextLayout> { Some(TextLayout {}) }
}

pub struct TextFormatBuilder
{
}

impl TextFormatBuilder
{

    pub fn with_family(&self, family: &str) -> &Self { &self }
    pub fn with_size(&self, size: f32) -> &Self { &self }

    pub fn build(&self) -> Option<TextFormat> { Some(TextFormat {}) }
}
