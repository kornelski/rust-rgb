#![allow(deprecated)]

//! Comprehensive API coverage tests for the rgb crate v0.8.
//!
//! Exercises every public type, trait, method, conversion, and operator.
//! Goal: 100% of the v0.8 public API surface is tested here.

use rgb::alt::{ABGR, ARGB, BGR, BGRA, GRB};
use rgb::alt::{BGR8, BGR16, BGRA8, BGRA16, ABGR8, ABGR16, ARGB8, ARGB16, GRB8};
use rgb::alt::{GRAY8, GRAY16, GRAYA8, GRAYA16};
use rgb::{Gray, GrayAlpha, GrayA};
use rgb::{Rgb, Rgba, Bgr, Bgra, Argb, Abgr, Grb};
use rgb::{RGB, RGBA, RGB8, RGB16, RGBA8, RGBA16};
use rgb::{ComponentSlice, ComponentMap, AsPixels, FromSlice};
use rgb::prelude::*;
#[cfg(feature = "as-bytes")]
use rgb::ComponentBytes;

// ============================================================
// Type aliases
// ============================================================

mod type_aliases {
    use super::*;

    #[test]
    fn sized_u8() {
        let _: RGB8 = RGB { r: 0u8, g: 0, b: 0 };
        let _: RGBA8 = RGBA { r: 0u8, g: 0, b: 0, a: 0 };
        let _: BGR8 = BGR { b: 0u8, g: 0, r: 0 };
        let _: BGRA8 = BGRA { b: 0u8, g: 0, r: 0, a: 0 };
        let _: ARGB8 = ARGB { a: 0u8, r: 0, g: 0, b: 0 };
        let _: ABGR8 = ABGR { a: 0u8, b: 0, g: 0, r: 0 };
        let _: GRB8 = GRB { g: 0u8, r: 0, b: 0 };
    }

    #[test]
    fn sized_u16() {
        let _: RGB16 = RGB { r: 0u16, g: 0, b: 0 };
        let _: RGBA16 = RGBA { r: 0u16, g: 0, b: 0, a: 0 };
        let _: BGR16 = BGR { b: 0u16, g: 0, r: 0 };
        let _: BGRA16 = BGRA { b: 0u16, g: 0, r: 0, a: 0 };
        let _: ABGR16 = ABGR { a: 0u16, b: 0, g: 0, r: 0 };
        let _: ARGB16 = ARGB { a: 0u16, r: 0, g: 0, b: 0 };
    }

    #[test]
    fn gray_aliases() {
        let _: GRAY8 = Gray::new(0u8);
        let _: GRAY16 = Gray::new(0u16);
        let _: GRAYA8 = GrayAlpha::new(0u8, 0);
        let _: GRAYA16 = GrayAlpha::new(0u16, 0);
    }

    #[test]
    fn legacy_uppercase() {
        let _: BGR<u8> = Bgr { b: 0u8, g: 0, r: 0 };
        let _: BGRA<u8> = Bgra { b: 0u8, g: 0, r: 0, a: 0 };
        let _: ARGB<u8> = Argb { a: 0u8, r: 0, g: 0, b: 0 };
        let _: ABGR<u8> = Abgr { a: 0u8, b: 0, g: 0, r: 0 };
        let _: GRB<u8> = Grb { g: 0u8, r: 0, b: 0 };
        let _: RGB<u8> = Rgb { r: 0u8, g: 0, b: 0 };
        let _: RGBA<u8> = Rgba { r: 0u8, g: 0, b: 0, a: 0 };
    }
}

// ============================================================
// Construction
// ============================================================

mod construction {
    use super::*;

    #[test]
    fn rgb_new() {
        let a = RGB { r: 1u8, g: 2, b: 3 };
        let b = RGB::new(1u8, 2, 3);
        assert_eq!(a, b);
        const C: RGB8 = RGB::new(1, 2, 3);
        assert_eq!(C.r, 1);
    }

    #[test]
    fn bgr_new_bgr() {
        let a = BGR { b: 1u8, g: 2, r: 3 };
        let b = Bgr::new_bgr(1u8, 2, 3);
        assert_eq!(a, b);
        assert_eq!(a.b, 1);
        assert_eq!(a.g, 2);
        assert_eq!(a.r, 3);
    }

    #[test]
    fn grb_new_grb() {
        let a = GRB { g: 1u8, r: 2, b: 3 };
        let b = Grb::new_grb(1u8, 2, 3);
        assert_eq!(a, b);
    }

    #[test]
    fn rgba_new() {
        let a = RGBA { r: 1u8, g: 2, b: 3, a: 4 };
        let b = RGBA::new(1u8, 2, 3, 4);
        assert_eq!(a, b);
    }

    #[test]
    fn rgba_new_alpha() {
        let a = RGBA::new_alpha(1u8, 2u8, 3u8, 4u16);
        assert_eq!(a.r, 1u8);
        assert_eq!(a.a, 4u16);
    }

    #[test]
    fn bgra_new_bgra() {
        let a = Bgra::new_bgra(1u8, 2, 3, 4);
        assert_eq!(a.b, 1);
        assert_eq!(a.g, 2);
        assert_eq!(a.r, 3);
        assert_eq!(a.a, 4);
    }

    #[test]
    fn argb_new_argb() {
        let a = Argb::new_argb(1u8, 2, 3, 4);
        assert_eq!(a.a, 1);
        assert_eq!(a.r, 2);
        assert_eq!(a.g, 3);
        assert_eq!(a.b, 4);
    }

    #[test]
    fn abgr_new_abgr() {
        let a = Abgr::new_abgr(1u8, 2, 3, 4);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
        assert_eq!(a.g, 3);
        assert_eq!(a.r, 4);
    }

    #[test]
    fn gray_new() {
        let a = Gray(100u8);
        let b = Gray::new(100u8);
        assert_eq!(a, b);
        assert_eq!(a.0, 100);
    }

    #[test]
    fn gray_alpha_new() {
        let a = GrayAlpha(100u8, 200);
        let b = GrayAlpha::new(100u8, 200);
        assert_eq!(a, b);
        assert_eq!(a.0, 100);
        assert_eq!(a.1, 200);
    }

    #[test]
    fn graya_new() {
        let a = GrayA { v: 100u8, a: 200 };
        let b = GrayA::new(100u8, 200);
        assert_eq!(a, b);
    }
}

// ============================================================
// Inherent methods
// ============================================================

mod inherent_methods {
    use super::*;

    // Gray
    #[test]
    fn gray_value() {
        assert_eq!(Gray::new(42u8).value(), 42);
    }

    #[test]
    fn gray_value_mut() {
        let mut g = Gray::new(10u8);
        *g.value_mut() = 20;
        assert_eq!(g.value(), 20);
    }

    #[test]
    fn gray_deref() {
        let g = Gray::new(42u8);
        assert_eq!(*g, 42);
    }

    #[test]
    fn gray_with_alpha() {
        let g = Gray::new(42u8);
        assert_eq!(g.with_alpha(200), GrayAlpha(42, 200));
    }

    // GrayAlpha
    #[test]
    fn gray_alpha_value() {
        assert_eq!(GrayAlpha::new(10u8, 20).value(), 10);
    }

    #[test]
    fn gray_alpha_gray() {
        assert_eq!(GrayAlpha::new(10u8, 20).gray(), Gray::new(10));
    }

    #[test]
    fn gray_alpha_gray_mut() {
        let mut ga = GrayAlpha::new(10u8, 20);
        *ga.gray_mut() = Gray::new(50);
        assert_eq!(ga.0, 50);
    }

    #[test]
    fn gray_alpha_with_alpha() {
        assert_eq!(GrayAlpha::new(10u8, 20).with_alpha(99), GrayAlpha(10, 99));
    }

    #[test]
    fn gray_alpha_map_alpha() {
        assert_eq!(GrayAlpha::new(10u8, 20).map_alpha(|a| a as u16 * 2), GrayAlpha(10u8, 40u16));
    }

    #[test]
    fn gray_alpha_map_gray() {
        assert_eq!(GrayAlpha::new(10u8, 20).map_gray(|g| g as u16 * 2), GrayAlpha(20u16, 20u8));
    }

    #[test]
    fn gray_alpha_deref_to_graya() {
        let ga = GrayAlpha::new(10u8, 20);
        assert_eq!(ga.v, 10);
        assert_eq!(ga.a, 20);
    }

    #[test]
    fn gray_alpha_deref_mut() {
        let mut ga = GrayAlpha::new(10u8, 20);
        ga.v = 50;
        ga.a = 60;
        assert_eq!(ga.0, 50);
        assert_eq!(ga.1, 60);
    }

    // RGB with_alpha / iter
    #[test]
    fn rgb_with_alpha() {
        assert_eq!(RGB::new(1u8, 2, 3).with_alpha(255), RGBA::new(1, 2, 3, 255));
    }

    #[test]
    fn rgb_iter() {
        let v: Vec<u8> = RGB::new(1u8, 2, 3).iter().collect();
        assert_eq!(v, vec![1, 2, 3]);
    }

    // BGR with_alpha / iter
    #[test]
    fn bgr_with_alpha() {
        let bgr = BGR { b: 1u8, g: 2, r: 3 };
        assert_eq!(bgr.with_alpha(255), BGRA { b: 1, g: 2, r: 3, a: 255 });
    }

    #[test]
    fn bgr_iter() {
        let bgr = BGR { b: 1u8, g: 2, r: 3 };
        let v: Vec<u8> = bgr.iter().collect();
        assert_eq!(v, vec![1, 2, 3]); // memory order
    }

    // GRB iter
    #[test]
    fn grb_iter() {
        let grb = GRB { g: 1u8, r: 2, b: 3 };
        let v: Vec<u8> = grb.iter().collect();
        assert_eq!(v, vec![1, 2, 3]);
    }

    // RGBA methods
    #[test]
    fn rgba_rgb() {
        assert_eq!(RGBA::new(1u8, 2, 3, 4).rgb(), RGB::new(1, 2, 3));
    }

    #[test]
    fn rgba_bgr() {
        assert_eq!(RGBA::new(1u8, 2, 3, 4).bgr(), BGR { b: 3, g: 2, r: 1 });
    }

    #[test]
    fn rgba_rgb_mut() {
        let mut p = RGBA::new(1u8, 2, 3, 4);
        p.rgb_mut().r = 10;
        assert_eq!(p.r, 10);
        assert_eq!(p.a, 4);
    }

    #[test]
    fn rgba_with_alpha() {
        assert_eq!(RGBA::new(1u8, 2, 3, 4).with_alpha(99), RGBA::new(1, 2, 3, 99));
    }

    #[test]
    fn rgba_map_alpha() {
        assert_eq!(RGBA::new(1u8, 2, 3, 100).map_alpha(|a| a as u16).a, 100u16);
    }

    #[test]
    fn rgba_iter() {
        let v: Vec<u8> = RGBA::new(1u8, 2, 3, 4).iter().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    // BGRA methods
    #[test]
    fn bgra_bgr() {
        assert_eq!(BGRA { b: 1u8, g: 2, r: 3, a: 4 }.bgr(), BGR { b: 1, g: 2, r: 3 });
    }

    #[test]
    fn bgra_bgr_mut() {
        let mut p = BGRA { b: 1u8, g: 2, r: 3, a: 4 };
        p.bgr_mut().r = 10;
        assert_eq!(p.r, 10);
    }

    #[test]
    fn bgra_with_alpha() {
        let p = BGRA { b: 1u8, g: 2, r: 3, a: 4 };
        assert_eq!(p.with_alpha(99).a, 99);
    }

    #[test]
    fn bgra_map_alpha() {
        let p = BGRA { b: 1u8, g: 2, r: 3, a: 100 };
        assert_eq!(p.map_alpha(|a| a as u16).a, 100u16);
    }

    #[test]
    fn bgra_iter() {
        let v: Vec<u8> = BGRA { b: 1u8, g: 2, r: 3, a: 4 }.iter().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    // ARGB methods
    #[test]
    fn argb_rgb() {
        assert_eq!(ARGB { a: 1u8, r: 2, g: 3, b: 4 }.rgb(), RGB::new(2, 3, 4));
    }

    #[test]
    fn argb_bgr() {
        assert_eq!(ARGB { a: 1u8, r: 2, g: 3, b: 4 }.bgr(), BGR { b: 4, g: 3, r: 2 });
    }

    #[test]
    fn argb_with_alpha() {
        let p = ARGB { a: 1u8, r: 2, g: 3, b: 4 };
        assert_eq!(p.with_alpha(99).a, 99);
    }

    #[test]
    fn argb_map_alpha() {
        let p = ARGB { a: 100u8, r: 1, g: 2, b: 3 };
        assert_eq!(p.map_alpha(|a| a as u16).a, 100u16);
    }

    #[test]
    fn argb_iter() {
        let v: Vec<u8> = ARGB { a: 1u8, r: 2, g: 3, b: 4 }.iter().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    // ABGR methods
    #[test]
    fn abgr_bgr() {
        assert_eq!(ABGR { a: 1u8, b: 2, g: 3, r: 4 }.bgr(), BGR { b: 2, g: 3, r: 4 });
    }

    #[test]
    fn abgr_with_alpha() {
        let p = ABGR { a: 1u8, b: 2, g: 3, r: 4 };
        assert_eq!(p.with_alpha(99).a, 99);
    }

    #[test]
    fn abgr_map_alpha() {
        let p = ABGR { a: 100u8, b: 1, g: 2, r: 3 };
        assert_eq!(p.map_alpha(|a| a as u16).a, 100u16);
    }

    #[test]
    fn abgr_iter() {
        let v: Vec<u8> = ABGR { a: 1u8, b: 2, g: 3, r: 4 }.iter().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}

// ============================================================
// ComponentMap trait
// ============================================================

mod component_map {
    use super::*;

    #[test]
    fn rgb_map() {
        let p = RGB::new(1u8, 2, 3);
        assert_eq!(p.map(|c| c as u16 * 2), RGB::new(2u16, 4, 6));
    }

    #[test]
    fn bgr_map() {
        let p = BGR { b: 1u8, g: 2, r: 3 };
        assert_eq!(p.map(|c| c + 10), BGR { b: 11, g: 12, r: 13 });
    }

    #[test]
    fn grb_map() {
        let p = GRB { g: 1u8, r: 2, b: 3 };
        assert_eq!(p.map(|c| c + 10), GRB { g: 11, r: 12, b: 13 });
    }

    #[test]
    fn rgba_map() {
        let p = RGBA::new(1u8, 2, 3, 4);
        assert_eq!(p.map(|c| c as u16 * 2), RGBA::new(2u16, 4, 6, 8));
    }

    #[test]
    fn bgra_map() {
        let p = BGRA { b: 1u8, g: 2, r: 3, a: 4 };
        assert_eq!(p.map(|c| c + 10), BGRA { b: 11, g: 12, r: 13, a: 14 });
    }

    #[test]
    fn argb_map() {
        let p = ARGB { a: 1u8, r: 2, g: 3, b: 4 };
        assert_eq!(p.map(|c| c + 10), ARGB { a: 11, r: 12, g: 13, b: 14 });
    }

    #[test]
    fn abgr_map() {
        let p = ABGR { a: 1u8, b: 2, g: 3, r: 4 };
        assert_eq!(p.map(|c| c + 10), ABGR { a: 11, b: 12, g: 13, r: 14 });
    }

    #[test]
    fn gray_map() {
        assert_eq!(Gray::new(10u8).map(|c| c as u16 * 2), Gray(20u16));
    }

    #[test]
    fn gray_alpha_map() {
        assert_eq!(GrayAlpha::new(10u8, 20).map(|c| c + 1), GrayAlpha(11, 21));
    }
}

// ============================================================
// ColorComponentMap trait
// ============================================================

mod color_component_map {
    use super::*;

    #[test]
    fn rgb_map_colors() {
        let p = RGB::new(1u8, 2, 3);
        assert_eq!(p.map_colors(|c| c as u16 * 2), RGB::new(2u16, 4, 6));
    }

    #[test]
    fn rgba_map_colors_skips_alpha() {
        let p = RGBA::new(1u8, 2, 3, 100);
        let mapped: RGBA<u16, u8> = p.map_colors(|c| c as u16 * 2);
        assert_eq!(mapped.r, 2u16);
        assert_eq!(mapped.a, 100u8);
    }

    #[test]
    fn bgra_map_colors_skips_alpha() {
        let p = BGRA { b: 1u8, g: 2, r: 3, a: 100 };
        let mapped: BGRA<u16, u8> = p.map_colors(|c| c as u16);
        assert_eq!(mapped.r, 3u16);
        assert_eq!(mapped.a, 100u8);
    }

    #[test]
    fn argb_map_colors_skips_alpha() {
        let p = ARGB { a: 100u8, r: 1, g: 2, b: 3 };
        let mapped: ARGB<u16, u8> = p.map_colors(|c| c as u16);
        assert_eq!(mapped.r, 1u16);
        assert_eq!(mapped.a, 100u8);
    }

    #[test]
    fn abgr_map_colors_skips_alpha() {
        let p = ABGR { a: 100u8, b: 1, g: 2, r: 3 };
        let mapped: ABGR<u16, u8> = p.map_colors(|c| c as u16);
        assert_eq!(mapped.r, 3u16);
        assert_eq!(mapped.a, 100u8);
    }

    #[test]
    fn gray_map_colors() {
        assert_eq!(Gray::new(42u8).map_colors(|c| c as u16), Gray(42u16));
    }

    #[test]
    fn gray_alpha_map_colors_skips_alpha() {
        let ga = GrayAlpha::new(10u8, 20);
        let mapped: GrayAlpha<u16, u8> = ga.map_colors(|c| c as u16);
        assert_eq!(mapped.0, 10u16);
        assert_eq!(mapped.1, 20u8);
    }

    #[test]
    fn map_c_alias() {
        let p = RGB::new(1u8, 2, 3);
        assert_eq!(p.map_c(|c| c + 10), RGB::new(11, 12, 13));
    }

    #[test]
    fn bgr_map_colors() {
        let p = BGR { b: 1u8, g: 2, r: 3 };
        assert_eq!(p.map_colors(|c| c as u16), BGR { b: 1u16, g: 2, r: 3 });
    }

    #[test]
    fn grb_map_colors() {
        let p = GRB { g: 1u8, r: 2, b: 3 };
        assert_eq!(p.map_colors(|c| c as u16), GRB { g: 1u16, r: 2, b: 3 });
    }
}

// ============================================================
// ComponentSlice trait
// ============================================================

mod component_slice {
    use super::*;

    #[test]
    fn rgb_as_slice() {
        assert_eq!(RGB::new(1u8, 2, 3).as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn rgb_as_mut_slice() {
        let mut p = RGB::new(1u8, 2, 3);
        p.as_mut_slice()[1] = 20;
        assert_eq!(p.g, 20);
    }

    #[test]
    fn rgba_as_slice() {
        assert_eq!(RGBA::new(1u8, 2, 3, 4).as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn bgr_as_slice() {
        assert_eq!((BGR { b: 1u8, g: 2, r: 3 }).as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn bgra_as_slice() {
        assert_eq!((BGRA { b: 1u8, g: 2, r: 3, a: 4 }).as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn argb_as_slice() {
        assert_eq!((ARGB { a: 1u8, r: 2, g: 3, b: 4 }).as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn abgr_as_slice() {
        assert_eq!((ABGR { a: 1u8, b: 2, g: 3, r: 4 }).as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn grb_as_slice() {
        assert_eq!((GRB { g: 1u8, r: 2, b: 3 }).as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn gray_as_slice() {
        assert_eq!(Gray::new(42u8).as_slice(), &[42]);
    }

    #[test]
    fn gray_alpha_as_slice() {
        assert_eq!(GrayAlpha::new(10u8, 20).as_slice(), &[10, 20]);
    }

    #[test]
    fn rgb_slice_as_slice() {
        let pixels = [RGB::new(1u8, 2, 3), RGB::new(4, 5, 6)];
        assert_eq!(<[RGB<u8>]>::as_slice(&pixels[..]), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn rgba_slice_as_slice() {
        let pixels = [RGBA::new(1u8, 2, 3, 4)];
        assert_eq!(<[RGBA<u8>]>::as_slice(&pixels[..]), &[1, 2, 3, 4]);
    }

    #[test]
    fn gray_slice_as_slice() {
        let pixels = [Gray::new(1u16), Gray::new(2)];
        assert_eq!(<[Gray<u16>]>::as_slice(&pixels[..]), &[1, 2]);
    }

    #[test]
    fn gray_alpha_slice_as_slice() {
        let pixels = [GrayAlpha::new(1u16, 2), GrayAlpha::new(3, 4)];
        assert_eq!(<[GrayAlpha<u16>]>::as_slice(&pixels[..]), &[1, 2, 3, 4]);
    }
}

// ============================================================
// ComponentBytes trait (feature as-bytes)
// ============================================================

#[cfg(feature = "as-bytes")]
mod component_bytes {
    use super::*;

    #[test]
    fn rgb_as_bytes() {
        assert_eq!([RGB::new(1u8, 2, 3), RGB::new(4, 5, 6)].as_bytes(), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn rgba_as_bytes() {
        assert_eq!([RGBA::new(1u8, 2, 3, 4)].as_bytes(), &[1, 2, 3, 4]);
    }

    #[test]
    fn bgr_as_bytes() {
        assert_eq!([BGR { b: 1u8, g: 2, r: 3 }].as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn bgra_as_bytes() {
        assert_eq!([BGRA { b: 1u8, g: 2, r: 3, a: 4 }].as_bytes(), &[1, 2, 3, 4]);
    }

    #[test]
    fn argb_as_bytes() {
        assert_eq!([ARGB { a: 1u8, r: 2, g: 3, b: 4 }].as_bytes(), &[1, 2, 3, 4]);
    }

    #[test]
    fn abgr_as_bytes() {
        assert_eq!([ABGR { a: 1u8, b: 2, g: 3, r: 4 }].as_bytes(), &[1, 2, 3, 4]);
    }

    #[test]
    fn grb_as_bytes() {
        assert_eq!([GRB { g: 1u8, r: 2, b: 3 }].as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn gray_as_bytes() {
        assert_eq!([Gray::new(42u8), Gray::new(43)].as_bytes(), &[42, 43]);
    }

    #[test]
    fn gray_alpha_as_bytes() {
        assert_eq!([GrayAlpha::new(10u8, 20)].as_bytes(), &[10, 20]);
    }

    #[test]
    fn as_bytes_mut() {
        let mut pixels = [RGBA::new(1u8, 2, 3, 4)];
        pixels.as_bytes_mut()[3] = 99;
        assert_eq!(pixels[0].a, 99);
    }

    #[test]
    fn rgb16_as_bytes() {
        let pixels = [RGB::new(0x0102u16, 0x0304, 0x0506)];
        assert_eq!(pixels.as_bytes().len(), 6);
    }
}

// ============================================================
// FromSlice trait
// ============================================================

mod from_slice {
    use super::*;

    #[test]
    fn as_rgb() {
        assert_eq!([1u8, 2, 3, 4, 5, 6].as_rgb(), &[RGB::new(1, 2, 3), RGB::new(4, 5, 6)]);
    }

    #[test]
    fn as_rgba() {
        assert_eq!([1u8, 2, 3, 4, 5, 6, 7, 8].as_rgba(), &[RGBA::new(1, 2, 3, 4), RGBA::new(5, 6, 7, 8)]);
    }

    #[test]
    fn as_bgr() {
        assert_eq!([1u8, 2, 3].as_bgr(), &[BGR { b: 1, g: 2, r: 3 }]);
    }

    #[test]
    fn as_bgra() {
        assert_eq!([1u8, 2, 3, 4].as_bgra(), &[BGRA { b: 1, g: 2, r: 3, a: 4 }]);
    }

    #[test]
    fn as_argb() {
        assert_eq!([1u8, 2, 3, 4].as_argb(), &[ARGB { a: 1, r: 2, g: 3, b: 4 }]);
    }

    #[test]
    fn as_abgr() {
        assert_eq!([1u8, 2, 3, 4].as_abgr(), &[ABGR { a: 1, b: 2, g: 3, r: 4 }]);
    }

    #[test]
    fn as_gray() {
        assert_eq!([10u8, 20].as_gray(), &[Gray::new(10), Gray::new(20)]);
    }

    #[test]
    fn as_gray_alpha() {
        assert_eq!([10u8, 20, 30, 40].as_gray_alpha(), &[GrayAlpha::new(10, 20), GrayAlpha::new(30, 40)]);
    }

    #[test]
    fn as_rgb_mut() {
        let mut data = [1u8, 2, 3, 4, 5, 6];
        data.as_rgb_mut()[0].r = 10;
        assert_eq!(data[0], 10);
    }

    #[test]
    fn as_rgba_mut() {
        let mut data = [1u8, 2, 3, 4];
        data.as_rgba_mut()[0].a = 99;
        assert_eq!(data[3], 99);
    }

    #[test]
    fn as_argb_mut() {
        let mut data = [1u8, 2, 3, 4];
        data.as_argb_mut()[0].a = 99;
        assert_eq!(data[0], 99);
    }

    #[test]
    fn as_bgr_mut() {
        let mut data = [1u8, 2, 3];
        data.as_bgr_mut()[0].r = 99;
        assert_eq!(data[2], 99);
    }

    #[test]
    fn as_bgra_mut() {
        let mut data = [1u8, 2, 3, 4];
        data.as_bgra_mut()[0].a = 99;
        assert_eq!(data[3], 99);
    }

    #[test]
    fn as_abgr_mut() {
        let mut data = [1u8, 2, 3, 4];
        data.as_abgr_mut()[0].a = 99;
        assert_eq!(data[0], 99);
    }

    #[test]
    fn as_gray_mut() {
        let mut data = [10u8, 20];
        data.as_gray_mut()[0] = Gray::new(50);
        assert_eq!(data[0], 50);
    }

    #[test]
    fn as_gray_alpha_mut() {
        let mut data = [10u8, 20];
        data.as_gray_alpha_mut()[0] = GrayAlpha::new(50, 60);
        assert_eq!(data, [50, 60]);
    }

    #[test]
    fn excess_bytes_ignored() {
        assert_eq!([1u8, 2, 3, 4, 5].as_rgb().len(), 1);
        assert_eq!([1u8, 2, 3, 4, 5].as_rgba().len(), 1);
        assert_eq!([1u8, 2, 3].as_gray_alpha().len(), 1);
    }
}

// ============================================================
// AsPixels trait
// ============================================================

mod as_pixels {
    use super::*;

    #[test]
    fn as_pixels_rgb() {
        let data = [1u8, 2, 3, 4, 5, 6];
        let pixels: &[RGB<u8>] = data.as_pixels();
        assert_eq!(pixels, &[RGB::new(1, 2, 3), RGB::new(4, 5, 6)]);
    }

    #[test]
    fn as_pixels_rgba() {
        let data = [1u8, 2, 3, 4];
        let pixels: &[RGBA<u8>] = data.as_pixels();
        assert_eq!(pixels, &[RGBA::new(1, 2, 3, 4)]);
    }

    #[test]
    fn as_pixels_bgr() {
        let data = [1u8, 2, 3];
        let pixels: &[BGR<u8>] = data.as_pixels();
        assert_eq!(pixels.len(), 1);
    }

    #[test]
    fn as_pixels_gray() {
        let data = [10u8, 20];
        let pixels: &[Gray<u8>] = data.as_pixels();
        assert_eq!(pixels, &[Gray::new(10), Gray::new(20)]);
    }

    #[test]
    fn as_pixels_gray_alpha() {
        let data = [10u8, 20, 30, 40];
        let pixels: &[GrayAlpha<u8>] = data.as_pixels();
        assert_eq!(pixels, &[GrayAlpha::new(10, 20), GrayAlpha::new(30, 40)]);
    }

    #[test]
    fn as_pixels_argb() {
        let data = [1u8, 2, 3, 4];
        let pixels: &[ARGB<u8>] = data.as_pixels();
        assert_eq!(pixels.len(), 1);
    }

    #[test]
    fn as_pixels_abgr() {
        let data = [1u8, 2, 3, 4];
        let pixels: &[ABGR<u8>] = data.as_pixels();
        assert_eq!(pixels.len(), 1);
    }

    #[test]
    fn as_pixels_bgra() {
        let data = [1u8, 2, 3, 4];
        let pixels: &[BGRA<u8>] = data.as_pixels();
        assert_eq!(pixels.len(), 1);
    }

    #[test]
    fn as_pixels_grb() {
        let data = [1u8, 2, 3];
        let pixels: &[GRB<u8>] = data.as_pixels();
        assert_eq!(pixels.len(), 1);
    }

    #[test]
    fn as_pixels_mut() {
        let mut data = [1u8, 2, 3, 4, 5, 6];
        let pixels: &mut [RGB<u8>] = data.as_pixels_mut();
        pixels[0] = RGB::new(10, 20, 30);
        assert_eq!(data, [10, 20, 30, 4, 5, 6]);
    }
}

// ============================================================
// From/Into conversions
// ============================================================

mod conversions {
    use super::*;

    // Layout reordering: 3-component
    #[test]
    fn rgb_bgr_bidirectional() {
        let rgb = RGB::new(1u8, 2, 3);
        let bgr: BGR<u8> = rgb.into();
        assert_eq!(bgr, BGR { b: 3, g: 2, r: 1 });
        let back: RGB<u8> = bgr.into();
        assert_eq!(back, rgb);
    }

    #[test]
    fn rgb_grb_bidirectional() {
        let rgb = RGB::new(1u8, 2, 3);
        let grb: GRB<u8> = rgb.into();
        assert_eq!(grb, GRB { g: 2, r: 1, b: 3 });
        let back: RGB<u8> = grb.into();
        assert_eq!(back, rgb);
    }

    // Layout reordering: 4-component
    #[test]
    fn rgba_argb_bidirectional() {
        let rgba = RGBA::new(1u8, 2, 3, 4);
        let argb: ARGB<u8> = rgba.into();
        assert_eq!(argb, ARGB { a: 4, r: 1, g: 2, b: 3 });
        let back: RGBA<u8> = argb.into();
        assert_eq!(back, rgba);
    }

    #[test]
    fn rgba_bgra_bidirectional() {
        let rgba = RGBA::new(1u8, 2, 3, 4);
        let bgra: BGRA<u8> = rgba.into();
        assert_eq!(bgra, BGRA { b: 3, g: 2, r: 1, a: 4 });
        let back: RGBA<u8> = bgra.into();
        assert_eq!(back, rgba);
    }

    #[test]
    fn rgba_abgr_bidirectional() {
        let rgba = RGBA::new(1u8, 2, 3, 4);
        let abgr: ABGR<u8> = rgba.into();
        assert_eq!(abgr, ABGR { a: 4, b: 3, g: 2, r: 1 });
        let back: RGBA<u8> = abgr.into();
        assert_eq!(back, rgba);
    }

    #[test]
    fn argb_abgr_bidirectional() {
        let argb = ARGB { a: 4u8, r: 1, g: 2, b: 3 };
        let abgr: ABGR<u8> = argb.into();
        let back: ARGB<u8> = abgr.into();
        assert_eq!(back, argb);
    }

    #[test]
    fn bgra_argb_bidirectional() {
        let bgra = BGRA { b: 3u8, g: 2, r: 1, a: 4 };
        let argb: ARGB<u8> = bgra.into();
        let back: BGRA<u8> = argb.into();
        assert_eq!(back, bgra);
    }

    #[test]
    fn bgra_abgr_bidirectional() {
        let bgra = BGRA { b: 3u8, g: 2, r: 1, a: 4 };
        let abgr: ABGR<u8> = bgra.into();
        let back: BGRA<u8> = abgr.into();
        assert_eq!(back, bgra);
    }

    // RGB to RGBA with opaque alpha
    #[test]
    fn rgb_to_rgba_u8() {
        let rgba: RGBA<u8> = RGB::new(1u8, 2, 3).into();
        assert_eq!(rgba, RGBA::new(1, 2, 3, 255));
    }

    #[test]
    fn rgb_to_rgba_u16() {
        let rgba: RGBA<u8, u16> = RGB::new(1u8, 2, 3).into();
        assert_eq!(rgba.a, 65535u16);
    }

    #[test]
    fn rgb_to_bgra() {
        let bgra: BGRA<u8> = RGB::new(1u8, 2, 3).into();
        assert_eq!(bgra, BGRA { b: 3, g: 2, r: 1, a: 255 });
    }

    #[test]
    fn bgr_to_rgba() {
        let rgba: RGBA<u8> = (BGR { b: 3u8, g: 2, r: 1 }).into();
        assert_eq!(rgba, RGBA::new(1, 2, 3, 255));
    }

    #[test]
    fn bgr_to_bgra() {
        let bgra: BGRA<u8> = (BGR { b: 1u8, g: 2, r: 3 }).into();
        assert_eq!(bgra.a, 255);
    }

    #[test]
    fn rgb_to_argb() {
        let argb: ARGB<u8> = RGB::new(1u8, 2, 3).into();
        assert_eq!(argb.a, 255);
    }

    #[test]
    fn rgb_to_abgr() {
        let abgr: ABGR<u8> = RGB::new(1u8, 2, 3).into();
        assert_eq!(abgr.a, 255);
    }

    #[test]
    fn bgr_to_argb() {
        let argb: ARGB<u8> = (BGR { b: 3u8, g: 2, r: 1 }).into();
        assert_eq!(argb.a, 255);
    }

    #[test]
    fn bgr_to_abgr() {
        let abgr: ABGR<u8> = (BGR { b: 3u8, g: 2, r: 1 }).into();
        assert_eq!(abgr.a, 255);
    }

    // Type widening
    #[test]
    fn rgb_u8_to_u16() {
        let b: RGB<u16> = RGB::new(1u8, 2, 3).into();
        assert_eq!(b, RGB::new(1u16, 2, 3));
    }

    #[test]
    fn rgb_u8_to_f32() {
        let b: RGB<f32> = RGB::new(1u8, 2, 3).into();
        assert_eq!(b, RGB::new(1.0f32, 2.0, 3.0));
    }

    #[test]
    fn rgb_u8_to_f64() {
        let b: RGB<f64> = RGB::new(1u8, 2, 3).into();
        assert_eq!(b, RGB::new(1.0f64, 2.0, 3.0));
    }

    #[test]
    fn rgb_u16_to_f32() {
        let b: RGB<f32> = RGB::new(1u16, 2, 3).into();
        assert_eq!(b, RGB::new(1.0f32, 2.0, 3.0));
    }

    #[test]
    fn rgba_u8_to_u16() {
        let b: RGBA<u16> = RGBA::new(1u8, 2, 3, 4).into();
        assert_eq!(b, RGBA::new(1u16, 2, 3, 4));
    }

    #[test]
    fn rgba_u8_to_f32() {
        let b: RGBA<f32> = RGBA::new(1u8, 2, 3, 4).into();
        assert_eq!(b, RGBA::new(1.0f32, 2.0, 3.0, 4.0));
    }

    // Gray conversions
    #[test]
    fn t_into_gray() {
        let g: Gray<u8> = 42u8.into();
        assert_eq!(g, Gray::new(42));
    }

    #[test]
    fn gray_into_rgb() {
        let rgb: RGB<u8> = Gray::new(100u8).into();
        assert_eq!(rgb, RGB::new(100, 100, 100));
    }

    #[test]
    fn gray_into_rgba() {
        let rgba: RGBA<u8> = Gray::new(100u8).into();
        assert_eq!(rgba, RGBA::new(100, 100, 100, 255));
    }

    #[test]
    fn gray_into_gray_alpha_u8() {
        let ga: GrayAlpha<u8, u8> = Gray::new(100u8).into();
        assert_eq!(ga, GrayAlpha::new(100, 255));
    }

    #[test]
    fn gray_into_gray_alpha_u16() {
        let ga: GrayAlpha<u8, u16> = Gray::new(100u8).into();
        assert_eq!(ga, GrayAlpha(100u8, 65535u16));
    }

    #[test]
    fn gray_alpha_into_rgba() {
        let rgba: RGBA<u8> = GrayAlpha::new(100u8, 200).into();
        assert_eq!(rgba, RGBA::new(100, 100, 100, 200));
    }

    // Array conversions
    #[test]
    fn array_to_rgb_and_back() {
        let rgb: RGB<u8> = [1u8, 2, 3].into();
        assert_eq!(rgb, RGB::new(1, 2, 3));
        let arr: [u8; 3] = rgb.into();
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn array_to_rgba_and_back() {
        let rgba: RGBA<u8> = [1u8, 2, 3, 4].into();
        assert_eq!(rgba, RGBA::new(1, 2, 3, 4));
        let arr: [u8; 4] = rgba.into();
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn array_to_bgr_and_back() {
        let bgr: BGR<u8> = [1u8, 2, 3].into(); // memory order b=1,g=2,r=3
        assert_eq!(bgr, BGR { b: 1, g: 2, r: 3 });
        let arr: [u8; 3] = bgr.into();
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn array_to_bgra_and_back() {
        let bgra: BGRA<u8> = [1u8, 2, 3, 4].into();
        assert_eq!(bgra, BGRA { b: 1, g: 2, r: 3, a: 4 });
        let arr: [u8; 4] = bgra.into();
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    // Tuple conversions
    #[test]
    fn tuple_rgb_bidirectional() {
        let rgb: RGB<u8> = (1u8, 2, 3).into();
        assert_eq!(rgb, RGB::new(1, 2, 3));
        let tup: (u8, u8, u8) = rgb.into();
        assert_eq!(tup, (1, 2, 3));
    }

    #[test]
    fn tuple_bgr_bidirectional() {
        let bgr: BGR<u8> = (1u8, 2, 3).into(); // b=1,g=2,r=3
        assert_eq!(bgr, BGR { b: 1, g: 2, r: 3 });
        let tup: (u8, u8, u8) = bgr.into();
        assert_eq!(tup, (1, 2, 3));
    }

    #[test]
    fn tuple_grb_bidirectional() {
        let grb: GRB<u8> = (1u8, 2, 3).into();
        assert_eq!(grb, GRB { g: 1, r: 2, b: 3 });
        let tup: (u8, u8, u8) = grb.into();
        assert_eq!(tup, (1, 2, 3));
    }

    #[test]
    fn tuple_rgba_bidirectional() {
        let rgba: RGBA<u8> = (1u8, 2, 3, 4).into();
        assert_eq!(rgba, RGBA::new(1, 2, 3, 4));
        let tup: (u8, u8, u8, u8) = rgba.into();
        assert_eq!(tup, (1, 2, 3, 4));
    }

    #[test]
    fn tuple_argb() {
        let argb: ARGB<u8> = (1u8, 2, 3, 4).into(); // a=1,r=2,g=3,b=4
        assert_eq!(argb, ARGB { a: 1, r: 2, g: 3, b: 4 });
    }

    #[test]
    fn tuple_bgra() {
        let bgra: BGRA<u8> = (1u8, 2, 3, 4).into();
        assert_eq!(bgra, BGRA { b: 1, g: 2, r: 3, a: 4 });
    }

    #[test]
    fn tuple_abgr() {
        let abgr: ABGR<u8> = (1u8, 2, 3, 4).into();
        assert_eq!(abgr, ABGR { a: 1, b: 2, g: 3, r: 4 });
    }

    #[test]
    fn tuple_gray_alpha_bidirectional() {
        let ga: GrayAlpha<u8> = (10u8, 20).into();
        assert_eq!(ga, GrayAlpha::new(10, 20));
        let tup: (u8, u8) = ga.into();
        assert_eq!(tup, (10, 20));
    }

    #[test]
    fn tuple_gray_to_tuple() {
        let g = Gray::new(42u8);
        let tup: (u8,) = g.into();
        assert_eq!(tup, (42,));
    }
}

// ============================================================
// AsRef / AsMut
// ============================================================

mod as_ref_impls {
    use super::*;

    #[test]
    fn rgb_as_ref_slice() {
        let p = RGB::new(1u8, 2, 3);
        let s: &[u8] = p.as_ref();
        assert_eq!(s, &[1, 2, 3]);
    }

    #[test]
    fn rgb_as_mut_slice() {
        let mut p = RGB::new(1u8, 2, 3);
        let s: &mut [u8] = p.as_mut();
        s[0] = 10;
        assert_eq!(p.r, 10);
    }

    #[test]
    fn rgba_as_ref_slice() {
        let p = RGBA::new(1u8, 2, 3, 4);
        let s: &[u8] = p.as_ref();
        assert_eq!(s, &[1, 2, 3, 4]);
    }

    #[test]
    fn rgba_as_mut_slice() {
        let mut p = RGBA::new(1u8, 2, 3, 4);
        let s: &mut [u8] = p.as_mut();
        s[3] = 99;
        assert_eq!(p.a, 99);
    }

    #[test]
    fn gray_as_ref() {
        let g = Gray::new(42u8);
        let v: &u8 = g.as_ref();
        assert_eq!(*v, 42);
    }

    #[test]
    fn gray_as_mut() {
        let mut g = Gray::new(42u8);
        let v: &mut u8 = g.as_mut();
        *v = 99;
        assert_eq!(g.0, 99);
    }

    #[test]
    fn gray_alpha_as_ref() {
        let ga = GrayAlpha::new(10u8, 20);
        let v: &u8 = ga.as_ref();
        assert_eq!(*v, 10);
    }

    #[test]
    fn gray_alpha_as_mut() {
        let mut ga = GrayAlpha::new(10u8, 20);
        let v: &mut u8 = ga.as_mut();
        *v = 99;
        assert_eq!(ga.0, 99);
    }

    #[test]
    fn argb_as_ref_array() {
        let p = ARGB { a: 1u8, r: 2, g: 3, b: 4 };
        let arr: &[u8; 4] = p.as_ref();
        assert_eq!(arr, &[1, 2, 3, 4]);
    }

    #[test]
    fn bgra_as_ref_array() {
        let p = BGRA { b: 1u8, g: 2, r: 3, a: 4 };
        let arr: &[u8; 4] = p.as_ref();
        assert_eq!(arr, &[1, 2, 3, 4]);
    }

    #[test]
    fn abgr_as_ref_array() {
        let p = ABGR { a: 1u8, b: 2, g: 3, r: 4 };
        let arr: &[u8; 4] = p.as_ref();
        assert_eq!(arr, &[1, 2, 3, 4]);
    }
}

// ============================================================
// Operators
// ============================================================

mod operators {
    use super::*;
    use core::num::Wrapping;

    // Add
    #[test]
    fn rgb_add() { assert_eq!(RGB::new(1u8, 2, 3) + RGB::new(4, 5, 6), RGB::new(5, 7, 9)); }
    #[test]
    fn rgb_add_scalar() { assert_eq!(RGB::new(1u8, 2, 3) + 10, RGB::new(11, 12, 13)); }
    #[test]
    fn rgba_add() { assert_eq!(RGBA::new(1u8, 2, 3, 4) + RGBA::new(5, 6, 7, 8), RGBA::new(6, 8, 10, 12)); }
    #[test]
    fn rgba_add_scalar() { assert_eq!(RGBA::new(1u16, 2, 3, 4) + 10, RGBA::new(11, 12, 13, 14)); }
    #[test]
    fn grb_add() { assert_eq!(GRB { g: 1u8, r: 2, b: 3 } + GRB { g: 4, r: 5, b: 6 }, GRB { g: 5, r: 7, b: 9 }); }
    #[test]
    fn gray_add() { assert_eq!(Gray::new(10u8) + Gray::new(20), Gray::new(30)); }
    #[test]
    fn gray_add_scalar() { assert_eq!(Gray::new(10u8) + 5, Gray::new(15)); }
    #[test]
    fn gray_alpha_add() { assert_eq!(GrayAlpha::new(10u8, 20) + GrayAlpha::new(5, 10), GrayAlpha::new(15, 30)); }
    #[test]
    fn argb_add() {
        let a = ARGB { a: 1u8, r: 2, g: 3, b: 4 };
        let b = ARGB { a: 5, r: 6, g: 7, b: 8 };
        assert_eq!(a + b, ARGB { a: 6, r: 8, g: 10, b: 12 });
    }

    // AddAssign
    #[test]
    fn rgb_add_assign() { let mut p = RGB::new(1u8, 2, 3); p += RGB::new(4, 5, 6); assert_eq!(p, RGB::new(5, 7, 9)); }
    #[test]
    fn rgb_add_assign_scalar() { let mut p = RGB::new(1u8, 2, 3); p += 10; assert_eq!(p, RGB::new(11, 12, 13)); }
    #[test]
    fn rgba_add_assign() { let mut p = RGBA::new(1u8, 2, 3, 4); p += RGBA::new(5, 6, 7, 8); assert_eq!(p, RGBA::new(6, 8, 10, 12)); }
    #[test]
    fn rgba_add_assign_scalar() { let mut p = RGBA::new(1u16, 2, 3, 4); p += 10; assert_eq!(p, RGBA::new(11, 12, 13, 14)); }

    // Sub
    #[test]
    fn rgb_sub() { assert_eq!(RGB::new(10u8, 20, 30) - RGB::new(1, 2, 3), RGB::new(9, 18, 27)); }
    #[test]
    fn rgb_sub_scalar() { assert_eq!(RGB::new(10u8, 20, 30) - 5, RGB::new(5, 15, 25)); }
    #[test]
    fn rgba_sub() { assert_eq!(RGBA::new(10u8, 20, 30, 40) - RGBA::new(1, 2, 3, 4), RGBA::new(9, 18, 27, 36)); }
    #[test]
    fn rgba_sub_scalar() { assert_eq!(RGBA::new(10u16, 20, 30, 40) - 5, RGBA::new(5, 15, 25, 35)); }

    // SubAssign
    #[test]
    fn rgb_sub_assign() { let mut p = RGB::new(10u8, 20, 30); p -= RGB::new(1, 2, 3); assert_eq!(p, RGB::new(9, 18, 27)); }
    #[test]
    fn rgb_sub_assign_scalar() { let mut p = RGB::new(10u8, 20, 30); p -= 5; assert_eq!(p, RGB::new(5, 15, 25)); }
    #[test]
    fn rgba_sub_assign() { let mut p = RGBA::new(10u8, 20, 30, 40); p -= RGBA::new(1, 2, 3, 4); assert_eq!(p, RGBA::new(9, 18, 27, 36)); }
    #[test]
    fn rgba_sub_assign_scalar() { let mut p = RGBA::new(10u16, 20, 30, 40); p -= 5; assert_eq!(p, RGBA::new(5, 15, 25, 35)); }

    // Mul
    #[test]
    fn rgb_mul_scalar() { assert_eq!(RGB::new(1u8, 2, 3) * 2, RGB::new(2, 4, 6)); }
    #[test]
    fn rgb_mul_rgb() { assert_eq!(RGB::new(2u8, 3, 4) * RGB::new(5, 6, 7), RGB::new(10, 18, 28)); }
    #[test]
    fn rgba_mul_scalar() { assert_eq!(RGBA::new(1u8, 2, 3, 4) * 2, RGBA::new(2, 4, 6, 8)); }

    // MulAssign
    #[test]
    fn rgb_mul_assign_scalar() { let mut p = RGB::new(1u16, 2, 3); p *= 10; assert_eq!(p, RGB::new(10, 20, 30)); }
    #[test]
    fn rgb_mul_assign_rgb() { let mut p = RGB::new(2.0f32, 3.0, 4.0); p *= RGB::new(1.5, 2.0, 0.5); assert_eq!(p, RGB::new(3.0, 6.0, 2.0)); }
    #[test]
    fn rgba_mul_assign_scalar() { let mut p = RGBA::new(1u16, 2, 3, 4); p *= 10; assert_eq!(p, RGBA::new(10, 20, 30, 40)); }

    // Div
    #[test]
    fn rgb_div_scalar() { assert_eq!(RGB::new(10u8, 20, 30) / 2, RGB::new(5, 10, 15)); }
    #[test]
    fn rgba_div_scalar() { assert_eq!(RGBA::new(10u8, 20, 30, 40) / 2, RGBA::new(5, 10, 15, 20)); }

    // DivAssign
    #[test]
    fn rgb_div_assign_scalar() { let mut p = RGB::new(10u8, 20, 30); p /= 2; assert_eq!(p, RGB::new(5, 10, 15)); }

    // Sum
    #[test]
    fn rgb_sum() {
        let s: RGB<u8> = [RGB::new(1u8, 1, 1), RGB::new(2, 3, 4)].iter().copied().sum();
        assert_eq!(s, RGB::new(3, 4, 5));
    }
    #[test]
    fn rgba_sum() {
        let s: RGBA<u16> = [RGBA::new(1u16, 1, 1, 1), RGBA::new(2, 3, 4, 5)].iter().copied().sum();
        assert_eq!(s, RGBA::new(3, 4, 5, 6));
    }
    #[test]
    fn rgba_sum_wrapping() {
        let s: RGBA<u16, Wrapping<u16>> = [
            RGBA::new_alpha(1u16, 1, 1, Wrapping(1u16)),
            RGBA::new_alpha(2, 3, 4, Wrapping(5)),
        ].iter().copied().sum();
        assert_eq!(s, RGBA::new_alpha(3, 4, 5, Wrapping(6)));
    }
    #[test]
    fn grb_sum() {
        let s: GRB<u8> = [GRB { g: 1u8, r: 1, b: 1 }, GRB { g: 2, r: 3, b: 4 }].iter().copied().sum();
        assert_eq!(s, GRB { g: 3, r: 4, b: 5 });
    }
    #[test]
    fn gray_sum() {
        let s: Gray<u8> = [Gray::new(10u8), Gray::new(20)].iter().copied().sum();
        assert_eq!(s, Gray::new(30));
    }
    #[test]
    fn gray_alpha_sum() {
        let s: GrayAlpha<u8> = [GrayAlpha::new(10u8, 20), GrayAlpha::new(5, 10)].iter().copied().sum();
        assert_eq!(s, GrayAlpha::new(15, 30));
    }
    #[test]
    fn argb_sum() {
        let s: ARGB<u8> = [ARGB { a: 1u8, r: 2, g: 3, b: 4 }, ARGB { a: 5, r: 6, g: 7, b: 8 }].iter().copied().sum();
        assert_eq!(s, ARGB { a: 6, r: 8, g: 10, b: 12 });
    }

    // Float ops
    #[test]
    fn float_sub() {
        assert_eq!(
            RGBA { r: 3.5f64, g: -0.5, b: -2.0, a: 0.0 } - RGBA { r: 1.0, g: 1.0, b: -2.0, a: -5.0 },
            RGBA { r: 2.5, g: -1.5, b: 0.0, a: 5.0 }
        );
    }

    #[test]
    fn float_mul() {
        assert_eq!(RGB::new(1.0f32, 3.0, 5.0) * 0.5, RGB::new(0.5, 1.5, 2.5));
    }

    // checked_fns
    #[cfg(feature = "checked_fns")]
    #[test]
    fn checked_add_rgb() {
        assert_eq!(RGB::new(255u8, 255, 255).checked_add(RGB::new(1, 0, 0)), None);
        assert_eq!(RGB::new(1u8, 2, 3).checked_add(RGB::new(4, 5, 6)), Some(RGB::new(5, 7, 9)));
    }

    #[cfg(feature = "checked_fns")]
    #[test]
    fn checked_sub_rgb() {
        assert_eq!(RGB::new(0u8, 0, 0).checked_sub(RGB::new(1, 0, 0)), None);
        assert_eq!(RGB::new(10u8, 20, 30).checked_sub(RGB::new(1, 2, 3)), Some(RGB::new(9, 18, 27)));
    }

    #[cfg(feature = "checked_fns")]
    #[test]
    fn checked_add_rgba() {
        assert_eq!(RGBA::new(255u8, 0, 0, 0).checked_add(RGBA::new(1, 0, 0, 0)), None);
        assert_eq!(RGBA::new(1u8, 2, 3, 4).checked_add(RGBA::new(5, 6, 7, 8)), Some(RGBA::new(6, 8, 10, 12)));
    }
}

// ============================================================
// Display and Hex formatting
// ============================================================

mod formatting {
    use super::*;

    #[test]
    fn rgb_display() { assert_eq!(format!("{}", RGB::new(1u8, 2, 3)), "rgb(1,2,3)"); }
    #[test]
    fn bgr_display() { assert_eq!(format!("{}", BGR { b: 1u8, g: 2, r: 3 }), "bgr(1,2,3)"); }
    #[test]
    fn rgba_display() { assert_eq!(format!("{}", RGBA::new(1u8, 2, 3, 4)), "rgba(1,2,3,4)"); }
    #[test]
    fn bgra_display() { assert_eq!(format!("{}", BGRA { b: 1u8, g: 2, r: 3, a: 4 }), "bgra(3,2,1,4)"); }

    #[test]
    fn rgb_upper_hex() { assert_eq!(format!("{:X}", RGB::new(255u8, 0, 0)), "RGB { #FF0000 }"); }
    #[test]
    fn rgb_lower_hex() { assert_eq!(format!("{:x}", RGB::new(255u8, 0, 0)), "RGB { #ff0000 }"); }
    #[test]
    fn bgr_upper_hex() { assert_eq!(format!("{:X}", BGR { b: 0u8, g: 0, r: 255 }), "BGR { #0000FF }"); }
    #[test]
    fn bgr_lower_hex() { assert_eq!(format!("{:x}", BGR { b: 0u8, g: 0, r: 255 }), "BGR { #0000ff }"); }

    #[test]
    fn rgb16_display() { assert_eq!(format!("{}", RGB::new(6699i16, 1, 15)), "rgb(6699,1,15)"); }
    #[test]
    fn rgb16_upper_hex() { assert_eq!(format!("{:X}", RGB::new(0x1A2Bu16, 1, 0xF)), "RGB { #1A2B010F }"); }
    #[test]
    fn rgb16_lower_hex() { assert_eq!(format!("{:x}", RGB::new(0x1a2bu16, 1, 0xf)), "RGB { #1a2b010f }"); }
}

// ============================================================
// Derive impls
// ============================================================

mod derive_impls {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn clone_copy() {
        let a = RGB::new(1u8, 2, 3);
        let b = a;
        let c = a.clone();
        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn debug() {
        let s = format!("{:?}", RGB::new(1u8, 2, 3));
        assert!(s.contains("1"));
    }

    #[test]
    fn default() {
        assert_eq!(RGB::<u8>::default(), RGB::new(0, 0, 0));
        assert_eq!(RGBA::<u8>::default(), RGBA::new(0, 0, 0, 0));
    }

    #[test]
    fn partial_ord_ord() {
        assert!(RGB::new(1u8, 1, 2) < RGB::new(2, 1, 1));
        assert!(RGBA::new(0i32, 0, 0, 0) > RGBA::new(-1, 0, 0, 0));
    }

    #[test]
    fn hash() {
        let mut set = HashSet::new();
        set.insert(RGB::new(1u8, 2, 3));
        assert!(set.contains(&RGB::new(1, 2, 3)));
        assert!(!set.contains(&RGB::new(3, 2, 1)));
    }

    #[test]
    fn eq_ne() {
        assert_eq!(RGB::new(1u8, 2, 3), RGB::new(1, 2, 3));
        assert_ne!(RGB::new(1u8, 2, 3), RGB::new(3, 2, 1));
    }
}

// ============================================================
// FromIterator
// ============================================================

mod from_iterator {
    use super::*;

    #[test]
    fn rgb_collect() {
        let rgb: RGB<u8> = [1u8, 2, 3].iter().copied().collect();
        assert_eq!(rgb, RGB::new(1, 2, 3));
    }

    #[test]
    fn rgba_collect() {
        let rgba: RGBA<u8> = [1u8, 2, 3, 4].iter().copied().collect();
        assert_eq!(rgba, RGBA::new(1, 2, 3, 4));
    }
}

// ============================================================
// Bytemuck
// ============================================================

#[cfg(feature = "bytemuck")]
mod bytemuck_tests {
    use super::*;
    use rgb::bytemuck;

    #[test]
    fn rgb_pod() {
        let bytes = [1u8, 2, 3];
        let rgb: &RGB<u8> = bytemuck::from_bytes(&bytes);
        assert_eq!(rgb, &RGB::new(1, 2, 3));
    }

    #[test]
    fn rgba_pod() {
        let bytes = [1u8, 2, 3, 4];
        let rgba: &RGBA<u8> = bytemuck::from_bytes(&bytes);
        assert_eq!(rgba, &RGBA::new(1, 2, 3, 4));
    }

    #[test]
    fn cast_slice_round_trip() {
        let pixels = [RGB::new(1u8, 2, 3), RGB::new(4, 5, 6)];
        let bytes: &[u8] = bytemuck::cast_slice(&pixels);
        assert_eq!(bytes, &[1, 2, 3, 4, 5, 6]);
        let back: &[RGB<u8>] = bytemuck::cast_slice(bytes);
        assert_eq!(back, &pixels);
    }

    #[test]
    fn gray_pod() {
        let bytes = [42u8];
        let g: &Gray<u8> = bytemuck::from_bytes(&bytes);
        assert_eq!(g.0, 42);
    }

    #[test]
    fn gray_alpha_pod() {
        let bytes = [10u8, 20];
        let ga: &GrayAlpha<u8> = bytemuck::from_bytes(&bytes);
        assert_eq!(ga.0, 10);
        assert_eq!(ga.1, 20);
    }
}

// ============================================================
// Memory layout
// ============================================================

mod memory_layout {
    use super::*;

    #[test]
    fn sizes() {
        assert_eq!(core::mem::size_of::<RGB<u8>>(), 3);
        assert_eq!(core::mem::size_of::<RGB<u16>>(), 6);
        assert_eq!(core::mem::size_of::<RGBA<u8>>(), 4);
        assert_eq!(core::mem::size_of::<RGBA<u16>>(), 8);
        assert_eq!(core::mem::size_of::<BGR<u8>>(), 3);
        assert_eq!(core::mem::size_of::<BGRA<u8>>(), 4);
        assert_eq!(core::mem::size_of::<ARGB<u8>>(), 4);
        assert_eq!(core::mem::size_of::<ABGR<u8>>(), 4);
        assert_eq!(core::mem::size_of::<GRB<u8>>(), 3);
        assert_eq!(core::mem::size_of::<Gray<u8>>(), 1);
        assert_eq!(core::mem::size_of::<GrayAlpha<u8>>(), 2);
        assert_eq!(core::mem::size_of::<GrayA<u8>>(), 2);
    }
}

// ============================================================
// Serde
// ============================================================

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;

    #[test]
    fn rgb_round_trip() {
        let p = RGB::new(1u8, 2, 3);
        let json = serde_json::to_string(&p).unwrap();
        let p2: RGB<u8> = serde_json::from_str(&json).unwrap();
        assert_eq!(p, p2);
    }

    #[test]
    fn rgba_round_trip() {
        let p = RGBA::new(1u8, 2, 3, 4);
        let json = serde_json::to_string(&p).unwrap();
        let p2: RGBA<u8> = serde_json::from_str(&json).unwrap();
        assert_eq!(p, p2);
    }

    #[test]
    fn gray_round_trip() {
        let p = Gray::new(42u8);
        let json = serde_json::to_string(&p).unwrap();
        let p2: Gray<u8> = serde_json::from_str(&json).unwrap();
        assert_eq!(p, p2);
    }

    #[test]
    fn gray_alpha_round_trip() {
        let p = GrayAlpha::new(10u8, 20);
        let json = serde_json::to_string(&p).unwrap();
        let p2: GrayAlpha<u8> = serde_json::from_str(&json).unwrap();
        assert_eq!(p, p2);
    }

    #[test]
    fn bgr_round_trip() {
        let p = BGR { b: 1u8, g: 2, r: 3 };
        let json = serde_json::to_string(&p).unwrap();
        let p2: BGR<u8> = serde_json::from_str(&json).unwrap();
        assert_eq!(p, p2);
    }
}

// ============================================================
// Prelude
// ============================================================

mod prelude_tests {
    use rgb::prelude::*;
    use rgb::RGB;

    #[test]
    fn component_map_via_prelude() {
        let p = RGB::new(1u8, 2, 3);
        let mapped: RGB<u16> = ComponentMap::map(&p, |c| c as u16 * 2);
        assert_eq!(mapped, RGB::new(2u16, 4, 6));
    }

    #[test]
    fn color_component_map_via_prelude() {
        let p = rgb::RGBA::new(1u8, 2, 3, 100);
        let mapped: rgb::RGBA<u16, u8> = ColorComponentMap::map_colors(&p, |c| c as u16);
        assert_eq!(mapped.r, 1u16);
        assert_eq!(mapped.a, 100u8);
    }
}
