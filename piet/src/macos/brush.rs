use render_target;

pub trait Brush {}


pub struct SolidColorBrush {}

impl SolidColorBrush {
    pub fn create<'a, R>(context: &'a R) -> SolidColorBrushBuilder<'a, R>
    where
        R: render_target::RenderTarget + 'a,
    {
        SolidColorBrushBuilder {context: context}
    }
}

pub struct SolidColorBrushBuilder<'a, R>
where
    R: render_target::RenderTarget + 'a,
{
    context: &'a R,
}

impl<'a, R> SolidColorBrushBuilder<'a, R>
where
    R: render_target::RenderTarget + 'a,
{

    pub fn with_color(&self, _ : i32) -> &Self { self }
    pub fn build(&self) -> Option<SolidColorBrush> { 
        Some(SolidColorBrush {})
    }

}

impl Brush for SolidColorBrush {}
