use fltk::{
    enums::{Align, Color, Font},
    prelude::DisplayExt,
    text::{Cursor, StyleTableEntry, StyleTableEntryExt, TextBuffer, WrapMode},
};

/// Adds builder pattern friendly versions of several setter functions
pub trait DisplayBuilderExt {
    /// Sets the associated `TextBuffer`
    fn with_buffer<B: Into<Option<TextBuffer>>>(self, buffer: B) -> Self;
    /// Sets the text font
    fn with_text_font(self, font: Font) -> Self;
    /// Sets the text color
    fn with_text_color(self, color: Color) -> Self;
    /// Sets the text size
    fn with_text_size(self, sz: i32) -> Self;
    /// Sets the style of the text widget
    fn with_highlight_data<B: Into<Option<TextBuffer>>>(
        self,
        style_buffer: B,
        entries: Vec<StyleTableEntry>,
    ) -> Self;
    /// Sets the style of the text widget
    fn with_highlight_data_ext<B: Into<Option<TextBuffer>>>(
        self,
        style_buffer: B,
        entries: Vec<StyleTableEntryExt>,
    ) -> Self;
    /// Sets the cursor style
    fn with_cursor_style(self, style: Cursor) -> Self;
    /// Sets the cursor color
    fn with_cursor_color(self, color: Color) -> Self;
    /// Sets the scrollbar size in pixels
    fn with_scrollbar_size(self, size: i32) -> Self;
    /// Sets the scrollbar alignment
    fn with_scrollbar_align(self, align: Align) -> Self;
    /// Sets the linenumber width
    fn with_linenumber_width(self, w: i32) -> Self;
    /// Sets the linenumber font
    fn with_linenumber_font(self, font: Font) -> Self;
    /// Sets the linenumber size
    fn with_linenumber_size(self, size: i32) -> Self;
    /// Sets the linenumber foreground color
    fn with_linenumber_fgcolor(self, color: Color) -> Self;
    /// Sets the linenumber background color
    fn with_linenumber_bgcolor(self, color: Color) -> Self;
    /// Sets the linenumber alignment
    fn with_linenumber_align(self, align: Align) -> Self;
    /// Sets the wrap mode of the Display widget.
    /// If the wrap mode is `AtColumn`, wrap margin is the column.
    /// If the wrap mode is `AtPixel`, wrap margin is the pixel.
    /// For more [info](https://www.fltk.org/doc-1.4/classFl__Text__Display.html#ab9378d48b949f8fc7da04c6be4142c54)
    fn with_wrap_mode(self, wrap: WrapMode, wrap_margin: i32) -> Self;
    /// Set the grammar underline color
    fn with_grammar_underline_color(self, color: Color) -> Self;
    /// Set the spelling underline color
    fn with_spelling_underline_color(self, color: Color) -> Self;
    /// Set the secondary selection color
    fn with_secondary_selection_color(self, color: Color) -> Self;
}

impl<D> DisplayBuilderExt for D
where
    D: DisplayExt,
{
    fn with_buffer<B: Into<Option<TextBuffer>>>(mut self, buffer: B) -> Self {
        self.set_buffer(buffer);
        self
    }

    fn with_text_font(mut self, font: Font) -> Self {
        self.set_text_font(font);
        self
    }

    fn with_text_color(mut self, color: Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn with_text_size(mut self, sz: i32) -> Self {
        self.set_text_size(sz);
        self
    }

    fn with_highlight_data<B: Into<Option<TextBuffer>>>(
        mut self,
        style_buffer: B,
        entries: Vec<StyleTableEntry>,
    ) -> Self {
        self.set_highlight_data(style_buffer, entries);
        self
    }

    fn with_highlight_data_ext<B: Into<Option<TextBuffer>>>(
        mut self,
        style_buffer: B,
        entries: Vec<StyleTableEntryExt>,
    ) -> Self {
        self.set_highlight_data_ext(style_buffer, entries);
        self
    }

    fn with_cursor_style(mut self, style: Cursor) -> Self {
        self.set_cursor_style(style);
        self
    }

    fn with_cursor_color(mut self, color: Color) -> Self {
        self.set_cursor_color(color);
        self
    }

    fn with_scrollbar_size(mut self, size: i32) -> Self {
        self.set_scrollbar_size(size);
        self
    }

    fn with_scrollbar_align(mut self, align: Align) -> Self {
        self.set_scrollbar_align(align);
        self
    }

    fn with_linenumber_width(mut self, w: i32) -> Self {
        self.set_linenumber_width(w);
        self
    }

    fn with_linenumber_font(mut self, font: Font) -> Self {
        self.set_linenumber_font(font);
        self
    }

    fn with_linenumber_size(mut self, size: i32) -> Self {
        self.set_linenumber_size(size);
        self
    }

    fn with_linenumber_fgcolor(mut self, color: Color) -> Self {
        self.set_linenumber_fgcolor(color);
        self
    }

    fn with_linenumber_bgcolor(mut self, color: Color) -> Self {
        self.set_linenumber_bgcolor(color);
        self
    }

    fn with_linenumber_align(mut self, align: Align) -> Self {
        self.set_linenumber_align(align);
        self
    }

    fn with_wrap_mode(mut self, wrap: WrapMode, wrap_margin: i32) -> Self {
        self.wrap_mode(wrap, wrap_margin);
        self
    }

    fn with_grammar_underline_color(mut self, color: Color) -> Self {
        self.set_grammar_underline_color(color);
        self
    }

    fn with_spelling_underline_color(mut self, color: Color) -> Self {
        self.set_spelling_underline_color(color);
        self
    }

    fn with_secondary_selection_color(mut self, color: Color) -> Self {
        self.set_secondary_selection_color(color);
        self
    }
}
