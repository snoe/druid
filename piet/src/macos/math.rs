pub struct RectF {left: f32, top: f32, right: f32, bottom: f32}

impl RectF {
#[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> RectF {
        RectF {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }
}

impl From<(f32, f32, f32, f32)> for RectF {
    #[inline]
    fn from((left, top, right, bottom): (f32, f32, f32, f32)) -> RectF {
        RectF::new(left, top, right, bottom)
    }
}

pub struct Point2F {x : f32, y : f32}

impl From<(f32, f32)> for Point2F {
    #[inline]
    fn from((x, y): (f32, f32)) -> Point2F {
        Point2F {x, y}
    }
}

pub struct SizeF {
    pub width : f32,
    pub height : f32
}

impl From<(f32, f32)> for SizeF {
    #[inline]
    fn from((width, height): (f32, f32)) -> SizeF {
        SizeF {width, height}
    }
}

