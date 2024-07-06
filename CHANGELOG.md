
# 0.8.41

* Renamed `.alpha(new_val)` method to `.with_alpha(new_val)`.
* Added Rust-idiomatic name aliases `Rgb`/`Rgba` for the `RGB`/`RGBA` structs.

# 0.8.40

* Added `.checked_add`/`sub` methods.

* Added a `Deref` trick that makes `GrayAlpha` tuple type have `.v` (value) and `.a` (alpha) fields. You should migrate to the new field names wherever possible.
  Unfortunately, `Gray` can't have that in a backward-compatible way, since it already derefs to the only field it has.

* The `Pod` trait is implemented for `RGBA<T, A>` types where `T != A`, but this is unsound if the fields have a different size or alignment that causes padding. In the future only `RGBA<T, T>` will support casting from/to raw bytes.

