use fltk::{browser::BrowserScrollbar, prelude::BrowserExt};

/// Adds builder pattern friendly versions of several setter functions
pub trait BrowserBuilderExt {
    /// Sets the text size.
    /// Lines start at 1
    fn with_text_size(self, sz: i32) -> Self;
    /// Sets the column separator to c. This will only have an effect if you also use `with_column_widths()`.
    /// c should be ascii
    fn with_column_char(self, c: char) -> Self;
    /// Sets the current column width array
    fn with_column_widths(self, arr: &[i32]) -> Self;
    /// Sets the type of scrollbar associated with the browser
    fn with_has_scrollbar(self, mode: BrowserScrollbar) -> Self;
    /// Sets the scrollbar size
    fn with_scrollbar_size(self, new_size: i32) -> Self;
}

impl<B> BrowserBuilderExt for B
where
    B: BrowserExt,
{
    fn with_text_size(mut self, sz: i32) -> Self {
        self.set_text_size(sz);
        self
    }

    fn with_column_char(mut self, c: char) -> Self {
        self.set_column_char(c);
        self
    }

    fn with_column_widths(mut self, arr: &[i32]) -> Self {
        self.set_column_widths(arr);
        self
    }

    fn with_has_scrollbar(mut self, mode: BrowserScrollbar) -> Self {
        self.set_has_scrollbar(mode);
        self
    }

    fn with_scrollbar_size(mut self, new_size: i32) -> Self {
        self.set_scrollbar_size(new_size);
        self
    }
}
