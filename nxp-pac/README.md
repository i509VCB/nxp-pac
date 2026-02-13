# nxp-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for NXP microcontrollers.

This crate has been automatically generated from the SVD files in
[NXP's mcux-soc-svd repo](https://github.com/nxp-mcuxpresso/mcux-soc-svd), using
[chiptool](https://github.com/embassy-rs/chiptool/). Fixes are added to the SVD file to make the
crate more amenable to writing HALs with, such as converting sets of identical registers/fields to
arrays, merging identical registers and enums, etc.

This crate will (hopefully) be used for [`embassy-nxp`](https://github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the NXP microcontrollers.

## Cloning

If you want to regenerate the pac, you need to clone with `--recursive`.

If you forgot this, you can use `git submodule update --checkout --init` to fetch the submodules.

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by NXP under a BSD-3-Clause license.
