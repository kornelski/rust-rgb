
# 0.8.92

## Display and Hex formatting fixes

* Hex formatting no longer wraps output in the type name. `format!("{:x}", RGB::new(255u8, 0, 0))` now returns `#ff0000` instead of `RGB { #ff0000 }`.
* BGR/BGRA hex formatting now outputs in logical RGB order. Previously, `BGR { b: 0, g: 0, r: 255 }` (a red pixel) would format as `#0000FF`, which looks like blue. It now correctly outputs `#FF0000`.
* Hex formatting for `u16` components now pads each component to 4 hex digits, matching the component width. Previously it used a fixed 2-digit minimum, making it impossible to tell where one component ended and the next began (e.g. `#1A2B010F` is now `#1A2B0001000F`).
* BGRA `Display` now prints values in field order to match the label: `bgra(b,g,r,a)`. Previously it printed `bgra(r,g,b,a)`, which was inconsistent with the format name.

## Trait changes

* The `prelude` now additionally exports `Pixel`, `HetPixel`, `GainAlpha`, `HasAlpha`, and `ArrayLike`. The previous `ComponentMap` and `ColorComponentMap` exports are preserved for backwards compatibility.

* `map_colors()` is now on the `HetPixel` trait (included in the prelude) instead of `ColorComponentMap`. Code using method syntax (`.map_colors()`) is unaffected. Code using trait-qualified calls needs updating:

  ```rust
  // Before — no longer compiles
  use rgb::prelude::*;
  let p = RGBA::new(1u8, 2, 3, 255);
  let mapped: RGBA<u16, u8> = ColorComponentMap::map_colors(&p, |c| c as u16);

  // After
  use rgb::prelude::*;
  let p = RGBA::new(1u8, 2, 3, 255);
  let mapped: RGBA<u16, u8> = HetPixel::map_colors(&p, |c| c as u16);
  ```

* The `unstable-experimental` feature flag has been removed. Code gated on `#[cfg(feature = "unstable-experimental")]` is now dead code. `Gray` remains `Gray_v08` and `GrayAlpha` remains `GrayAlpha_v08`; the new `Gray_v09` and `GrayA` types are available separately.

## Migrating to the new Gray types

`Gray_v09` and `GrayA` use named fields (`.v`, `.a`) instead of tuple fields (`.0`, `.1`), and support the new `Pixel`/`HetPixel` traits. To opt into them now, alias at the import site:

```rust
use rgb::Gray_v09 as Gray;
use rgb::GrayA;
```

Key differences from the legacy types:

| | `Gray` (v08) | `Gray_v09` |
|---|---|---|
| Construction | `Gray(100)` | `Gray { v: 100 }` or `Gray::new(100)` |
| Field access | `.0` | `.v` |
| Deref | `*g` yields `T` | No deref |
| `.value()` / `.value_mut()` | Yes | Yes |

| | `GrayAlpha` (v08) | `GrayA` |
|---|---|---|
| Construction | `GrayAlpha(100, 255)` | `GrayA { v: 100, a: 255 }` or `GrayA::new(100, 255)` |
| Field access | `.0`, `.1` | `.v`, `.a` |
| `.value()` | Yes | Yes |

Both old and new types support `::new()`, `.value()`, and `.value_mut()`, so code using those methods works with either type unchanged.

# 0.8.41

* Renamed `.alpha(new_val)` method to `.with_alpha(new_val)`.
* Added Rust-idiomatic name aliases `Rgb`/`Rgba` for the `RGB`/`RGBA` structs.

# 0.8.40

* Added `.checked_add`/`sub` methods.

* Added a `Deref` trick that makes `GrayAlpha` tuple type have `.v` (value) and `.a` (alpha) fields. You should migrate to the new field names wherever possible.
  Unfortunately, `Gray` can't have that in a backward-compatible way, since it already derefs to the only field it has.

* The `Pod` trait is implemented for `RGBA<T, A>` types where `T != A`, but this is unsound if the fields have a different size or alignment that causes padding. In the future only `RGBA<T, T>` will support casting from/to raw bytes.

