pub struct PaintCtx<'a> {
    d2d_factory: &'a piet::Factory,
    render_target: &'a mut piet::render_target::GenericRenderTarget,
}

impl<'a> PaintCtx<'a> {
    pub fn d2d_factory(&self) -> &piet::Factory {
        self.d2d_factory
    }

    pub fn render_target(&mut self) -> &mut piet::render_target::GenericRenderTarget {
        self.render_target
    }
}

