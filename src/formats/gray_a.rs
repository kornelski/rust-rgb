#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale + Alpha` pixel.
pub struct GrayA<T, A = T>(
    /// Grayscale Component
    pub T,
    /// Alpha Component
    pub A,
);
