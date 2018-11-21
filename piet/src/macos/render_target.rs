use math::*;
use write::*;
use brush::Brush;

pub trait RenderTarget {

    fn fill_rectangle<R, B>(&mut self, rect: R, brush: &B)
        where
            R: Into<RectF>,
            B: Brush,
    {}

    fn draw_text_layout<P, B>(
        &mut self,
        origin: P,
        layout: &TextLayout,
        brush: &B,
        options: DrawTextOptions,
        ) where
        P: Into<Point2F>,
        B: Brush,
    {}

    fn get_size(&self) -> SizeF { SizeF {width: 10.0, height: 10.0} }
}
pub struct GenericRenderTarget {}

impl RenderTarget for GenericRenderTarget {}
