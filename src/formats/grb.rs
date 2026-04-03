#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt_03::Format), defmt(crate = defmt_03))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Green + Red + Blue` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Grb;
///
/// let pixel: Grb<u8> = Grb { g: 0, r: 0, b: 0 };
/// ```
pub struct Grb<T> {
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
    /// Blue Component
    pub b: T,
}
