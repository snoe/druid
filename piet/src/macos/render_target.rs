use math::*;
use write::*;
use brush::Brush;
use macos::stroke_style::StrokeStyle;

pub trait RenderTarget {

    fn fill_rectangle<R, B>(&mut self, rect: R, brush: &B)
        where
            R: Into<RectF>,
            B: Brush,
    {}

    fn draw_line<P0, P1, B>(
        &mut self,
        p0: P0,
        p1: P1,
        brush: &B,
        stroke_width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) where
        P0: Into<Point2F>,
        P1: Into<Point2F>,
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
