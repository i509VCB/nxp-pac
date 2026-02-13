## nxp-pac crate workspace

This is the workspace for the nxp-pac crate and supporting utilities.

## Cloning

If you want to regenerate the pac, you need to clone with `--recursive`.

If you forgot this, you can use `git submodule update --checkout --init` to fetch the submodules.

## Tour

The `data` directory contains the SVD files, board metadata, and chiptool transformations needed to
generate the nxp-pac crate. This data is used by the code generation tool.

The `generate` directory contains the code generation tool for generating the code in the `nxp-pac` crate.
If you want change the way the `nxp-pac` crate is generated please see this directory.

The `nxp-pac` directory contains the nxp-pac crate. If you are looking for a peripheral access crate for
an NXP microcontroller, please see this directory. You should never need to manually edit the source
code in this directory. The metadata will need to be updated to support new microcontrollers.

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by NXP under a BSD-3-Clause license.
