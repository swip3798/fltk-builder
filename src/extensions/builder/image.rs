use fltk::prelude::ImageExt;

/// Adds builder pattern friendly versions of several setter functions
pub trait ImageBuilderExt {
    /// Scales the image
    fn scaled(self, width: i32, height: i32, proportional: bool, can_expand: bool) -> Self;
}

impl<I> ImageBuilderExt for I
where
    I: ImageExt,
{
    fn scaled(mut self, width: i32, height: i32, proportional: bool, can_expand: bool) -> Self {
        self.scale(width, height, proportional, can_expand);
        self
    }
}
