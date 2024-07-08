
# 0.8.43/0.8.44

* Reverted changes to `ComponentBytes`. `bytemuck` is still recommended instead.
* Disabling `as-bytes` feature fixes soundness issues with `RGBA<T, AddsPadding>` types.

# 0.8.42

* breaking: added `Copy` trait bound on `fn new` functions.

* Enabled all unusual pixel types by default (they have minimal impact on compile times).
* Added `new_bgra`/`new_argb` functions to pixel types with non-RGB component order.
* Changed `ComponentBytes` to defer to `bytemuck`. You can use `bytemuck::cast_slice()`, etc. directly.

# 0.8.41

* Renamed `.alpha(new_val)` method to `.with_alpha(new_val)`.
* Added Rust-idiomatic name aliases `Rgb`/`Rgba` for the `RGB`/`RGBA` structs.

# 0.8.40

* Added `.checked_add`/`sub` methods.

* Added a `Deref` trick that makes `GrayAlpha` tuple type have `.v` (value) and `.a` (alpha) fields. You should migrate to the new field names wherever possible.
  Unfortunately, `Gray` can't have that in a backward-compatible way, since it already derefs to the only field it has.

* The `Pod` trait is implemented for `RGBA<T, A>` types where `T != A`, but this is unsound if the fields have a different size or alignment that causes padding. In the future only `RGBA<T, T>` will support casting from/to raw bytes.

