use fltk::prelude::ValuatorExt;

/// Adds builder pattern friendly versions of several setter functions
pub trait ValuatorBuilderExt {
    /// Set bounds of a valuator
    fn with_bounds(self, a: f64, b: f64) -> Self;
    /// Set the minimum bound of a valuator
    fn with_minimum(self, a: f64) -> Self;
    /// Set the maximum bound of a valuator
    fn with_maximum(self, a: f64) -> Self;
    /// Set the range of a valuator
    fn with_range(self, a: f64, b: f64) -> Self;
    /// Set change step of a valuator.
    /// Rounds to multiples of a/b, or no rounding if a is zero
    fn with_step(self, a: f64, b: i32) -> Self;
    /// Set the precision of a valuator
    fn with_precision(self, digits: i32) -> Self;
    /// Set the value of a valuator
    fn with_value(self, arg2: f64) -> Self;
}

impl<V> ValuatorBuilderExt for V where V: ValuatorExt {
    fn with_bounds(mut self, a: f64, b: f64) -> Self {
        self.set_bounds(a, b);
        self
    }

    fn with_minimum(mut self, a: f64) -> Self {
        self.set_minimum(a);
        self
    }

    fn with_maximum(mut self, a: f64) -> Self {
        self.set_maximum(a);
        self
    }

    fn with_range(mut self, a: f64, b: f64) -> Self {
        self.set_range(a, b);
        self
    }

    fn with_step(mut self, a: f64, b: i32) -> Self {
        self.set_step(a, b);
        self
    }

    fn with_precision(mut self, digits: i32) -> Self {
        self.set_precision(digits);
        self
    }

    fn with_value(mut self, arg2: f64) -> Self {
        self.set_value(arg2);
        self
    }
}