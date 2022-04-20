use fltk::{
    draw::Region,
    enums::Cursor,
    image::RgbImage,
    prelude::{ImageExt, WidgetExt, WindowExt},
    window::RawHandle,
};

/// Adds builder pattern friendly versions of several setter functions
pub trait WindowBuilderExt {
    /// Makes a window modal, should be called before `show`
    fn as_modal(self, val: bool) -> Self;
    /// Makes a window fullscreen
    fn as_fullscreen(self, val: bool) -> Self;
    /// Sets the windows icon.
    /// Supported formats are bmp, jpeg, png and rgb.
    fn with_icon<T: ImageExt>(self, image: Option<T>) -> Self
    where
        Self: Sized;
    /// Sets the cursor style within the window.
    /// Needs to be called after the window is shown
    fn with_cursor(self, cursor: Cursor) -> Self;
    /// Sets whether the window has a border
    fn with_border(self, flag: bool) -> Self;
    #[doc(hidden)]
    /// Set the window associated with a raw handle.
    /// `RawHandle` is a void pointer to: (Windows: `HWND`, X11: `Xid` (`u64`), macOS: `NSWindow`)
    /// # Safety
    /// The data must be valid and is OS-dependent. The window must be shown.
    unsafe fn with_raw_handle(self, handle: RawHandle) -> Self;
    /// Set the graphical draw region of the window
    /// # Safety
    /// The data must be valid.
    unsafe fn with_region(self, region: Region) -> Self;
    /// Set the window's minimum width, minimum height, max width and max height.
    /// You can pass 0 as max_w and max_h to allow unlimited upward resize of the window.
    fn with_size_range(self, min_w: i32, min_h: i32, max_w: i32, max_h: i32) -> Self;
    /// Set the hotspot widget of the window
    fn with_hotspot<W: WidgetExt>(self, w: &W) -> Self
    where
        Self: Sized;
    /// Set the shape of the window.
    /// Supported image formats are BMP, RGB and Pixmap.
    /// The window covers non-transparent/non-black shape of the image.
    /// The image must not be scaled(resized) beforehand.
    /// The size will be adapted to the window's size
    fn with_shape<I: ImageExt>(self, image: Option<I>) -> Self
    where
        Self: Sized;
    /// Set the cursor image
    fn with_cursor_image(self, image: RgbImage, hot_x: i32, hot_y: i32) -> Self;
    /// Set the window's default cursor
    fn with_default_cursor(self, cursor: Cursor) -> Self;
    /// Set the screen number
    fn with_screen_num(self, n: i32) -> Self;
    /// Set the window's opacity,
    /// Ranges from 0.0 to 1.0, where 1.0 is fully opaque and 0.0 is fully transparent.
    /// This should be called on a shown window.
    /// On X11, opacity support depends on the window manager and can be queried:
    /// ```ignore
    /// $ xprop -root _NET_SUPPORTED | grep -o _NET_WM_WINDOW_OPACITY
    /// ```
    fn with_opacity(self, val: f64) -> Self;
    /// Set the window's XA_WM_CLASS property.
    /// This should be called before showing the window
    fn with_xclass(self, s: &str) -> Self;
    /// removes the window border and sets the window on top, by settings the NOBORDER and OVERRIDE flags
    fn with_override(self) -> Self;
    /// Forces the position of the window
    fn with_force_position(self, flag: bool) -> Self;
}

impl<E> WindowBuilderExt for E
where
    E: WindowExt,
{
    fn as_modal(mut self, val: bool) -> Self {
        self.make_modal(val);
        self
    }

    fn as_fullscreen(mut self, val: bool) -> Self {
        self.fullscreen(val);
        self
    }

    fn with_icon<T: ImageExt>(mut self, image: Option<T>) -> Self
    where
        Self: Sized,
    {
        self.set_icon(image);
        self
    }

    fn with_cursor(mut self, cursor: Cursor) -> Self {
        self.set_cursor(cursor);
        self
    }

    fn with_border(mut self, flag: bool) -> Self {
        self.set_border(flag);
        self
    }

    unsafe fn with_raw_handle(mut self, handle: RawHandle) -> Self {
        self.set_raw_handle(handle);
        self
    }

    unsafe fn with_region(mut self, region: Region) -> Self {
        self.set_region(region);
        self
    }

    fn with_size_range(mut self, min_w: i32, min_h: i32, max_w: i32, max_h: i32) -> Self {
        self.size_range(min_w, min_h, max_w, max_h);
        self
    }

    fn with_hotspot<W: WidgetExt>(mut self, w: &W) -> Self
    where
        Self: Sized,
    {
        self.hotspot(w);
        self
    }

    fn with_shape<I: ImageExt>(mut self, image: Option<I>) -> Self
    where
        Self: Sized,
    {
        self.set_shape(image);
        self
    }

    fn with_cursor_image(mut self, image: RgbImage, hot_x: i32, hot_y: i32) -> Self {
        self.set_cursor_image(image, hot_x, hot_y);
        self
    }

    fn with_default_cursor(mut self, cursor: Cursor) -> Self {
        self.default_cursor(cursor);
        self
    }

    fn with_screen_num(mut self, n: i32) -> Self {
        self.set_screen_num(n);
        self
    }

    fn with_opacity(mut self, val: f64) -> Self {
        self.set_opacity(val);
        self
    }

    fn with_xclass(mut self, s: &str) -> Self {
        self.set_xclass(s);
        self
    }

    fn with_override(mut self) -> Self {
        self.set_override();
        self
    }

    fn with_force_position(mut self, flag: bool) -> Self {
        self.force_position(flag);
        self
    }
}
