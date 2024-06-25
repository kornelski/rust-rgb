pub mod internal {
    pub mod convert;
    pub mod ops;
    pub mod pixel;
    pub mod rgb;
    pub mod rgba;
}

/// BGR/BGRA alernative layouts & grayscale
///
/// BGR might be useful for some Windows or OpenGL APIs.
pub mod alt;
