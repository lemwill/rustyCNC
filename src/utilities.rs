#[macro_export]
macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            f32::min($x, utilities::min!( $($xs),+ ))
        }
    };
}
pub use min;
