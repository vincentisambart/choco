#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    pub fn new(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> Self {
        Self {
            origin: CGPoint::new(x, y),
            size: CGSize::new(width, height),
        }
    }

    // For most of the methods below, I tried to have the generated assembly the closest to the one of the Apple provided methods.

    pub fn min_x(self) -> CGFloat {
        self.origin.x + self.size.width.min(0.)
    }

    pub fn mid_x(self) -> CGFloat {
        self.min_x() + self.size.width.abs() / 2.
    }

    pub fn max_x(self) -> CGFloat {
        self.min_x() + self.size.width.abs()
    }

    pub fn min_y(self) -> CGFloat {
        self.origin.y + self.size.height.min(0.)
    }

    pub fn mid_y(self) -> CGFloat {
        self.min_y() + self.size.height.abs() / 2.
    }

    pub fn max_y(self) -> CGFloat {
        self.min_y() + self.size.height.abs()
    }

    pub fn width(self) -> CGFloat {
        self.size.width.abs()
    }

    pub fn height(self) -> CGFloat {
        self.size.height.abs()
    }
}
