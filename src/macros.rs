//! Unsorted macros


/// Use in impl block.
macro_rules! setter {
    ($( $field:ident : $t:ty),+) => {
        $(
            pub fn $field(mut self, $field: $t) -> Self {
                self.$field = $field;
                self
            }
        )+
    };
}