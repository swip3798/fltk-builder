use fltk::{
    enums::{Color, FrameType},
    prelude::TableExt,
};
/// Adds builder pattern friendly versions of several setter functions
pub trait TableBuilderExt {
    /// Sets the table frame
    fn with_table_frame(self, frame: FrameType) -> Self;
    /// Sets the number of rows
    fn with_rows(self, val: i32) -> Self;
    /// Sets the number of columns
    fn with_cols(self, val: i32) -> Self;
    /// Sets a row to be resizable
    fn with_row_resize(self, flag: bool) -> Self;
    /// Sets a column to be resizable
    fn with_col_resize(self, flag: bool) -> Self;
    /// Sets the current column minimum resize value.
    fn with_col_resize_min(self, val: i32) -> Self;
    /// Sets the current row minimum resize value.
    fn with_row_resize_min(self, val: i32) -> Self;
    /// Sets whether a row headers are enabled or not
    fn with_row_header(self, flag: bool) -> Self;
    /// Sets whether a column headers are enabled or not
    fn with_col_header(self, flag: bool) -> Self;
    /// Sets the column header height
    fn with_col_header_height(self, height: i32) -> Self;
    /// Sets the row header width
    fn with_row_header_width(self, width: i32) -> Self;
    /// Sets the row header color
    fn with_row_header_color(self, val: Color) -> Self;
    /// Sets the column header color
    fn with_col_header_color(self, val: Color) -> Self;
    /// Sets the row's height
    fn with_row_height(self, row: i32, height: i32) -> Self;
    /// Sets the column's width
    fn with_col_width(self, col: i32, width: i32) -> Self;
    /// Sets all rows height
    fn with_row_height_all(self, height: i32) -> Self;
    /// Sets all column's width
    fn with_col_width_all(self, width: i32) -> Self;
    /// Sets the row's position
    fn with_row_position(self, row: i32) -> Self;
    /// Sets the column's position
    fn with_col_position(self, col: i32) -> Self;
    /// Sets the top row
    fn with_top_row(self, row: i32) -> Self;
    /// Sets the scrollbar size
    fn with_scrollbar_size(self, new_size: i32) -> Self;
    /// Sets whether tab key cell navigation is enabled
    fn with_tab_cell_nav(self, val: bool) -> Self;
}

impl<T> TableBuilderExt for T
where
    T: TableExt,
{
    fn with_table_frame(mut self, frame: FrameType) -> Self {
        self.set_table_frame(frame);
        self
    }

    fn with_rows(mut self, val: i32) -> Self {
        self.set_rows(val);
        self
    }

    fn with_cols(mut self, val: i32) -> Self {
        self.set_cols(val);
        self
    }

    fn with_row_resize(mut self, flag: bool) -> Self {
        self.set_row_resize(flag);
        self
    }

    fn with_col_resize(mut self, flag: bool) -> Self {
        self.set_col_resize(flag);
        self
    }

    fn with_col_resize_min(mut self, val: i32) -> Self {
        self.set_col_resize_min(val);
        self
    }

    fn with_row_resize_min(mut self, val: i32) -> Self {
        self.set_row_resize_min(val);
        self
    }

    fn with_row_header(mut self, flag: bool) -> Self {
        self.set_row_header(flag);
        self
    }

    fn with_col_header(mut self, flag: bool) -> Self {
        self.set_col_header(flag);
        self
    }

    fn with_col_header_height(mut self, height: i32) -> Self {
        self.set_col_header_height(height);
        self
    }

    fn with_row_header_width(mut self, width: i32) -> Self {
        self.set_row_header_width(width);
        self
    }

    fn with_row_header_color(mut self, val: Color) -> Self {
        self.set_row_header_color(val);
        self
    }

    fn with_col_header_color(mut self, val: Color) -> Self {
        self.set_col_header_color(val);
        self
    }

    fn with_row_height(mut self, row: i32, height: i32) -> Self {
        self.set_row_height(row, height);
        self
    }

    fn with_col_width(mut self, col: i32, width: i32) -> Self {
        self.set_col_width(col, width);
        self
    }

    fn with_row_height_all(mut self, height: i32) -> Self {
        self.set_row_height_all(height);
        self
    }

    fn with_col_width_all(mut self, width: i32) -> Self {
        self.set_col_width_all(width);
        self
    }

    fn with_row_position(mut self, row: i32) -> Self {
        self.set_row_position(row);
        self
    }

    fn with_col_position(mut self, col: i32) -> Self {
        self.set_col_position(col);
        self
    }

    fn with_top_row(mut self, row: i32) -> Self {
        self.set_top_row(row);
        self
    }

    fn with_scrollbar_size(mut self, new_size: i32) -> Self {
        self.set_scrollbar_size(new_size);
        self
    }

    fn with_tab_cell_nav(mut self, val: bool) -> Self {
        self.set_tab_cell_nav(val);
        self
    }
}
