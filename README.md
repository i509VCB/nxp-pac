## nxp-pac crate workspace

This is the workspace for the nxp-pac crate and supporting utilities.

## Cloning

The SVD submodule intentionally declares `update = none`, so a normal
`--recursive` clone does not populate it. Before regenerating, explicitly
check out the repository-pinned SVD revision:

```sh
git submodule update --init --recursive --checkout data/mcux-soc-svd
```

Generate every supported PAC from the repository root and verify that the
committed output is current:

```sh
cargo run -p generator --locked -- generate
git diff --exit-code -- nxp-pac
```

## PAC vs MetaPAC

This crate is in transition from generating the PAC from the NXP provided SVD files, to a metapac approach where the SVD is only used to extract general peripheral definition files.
A metapac definition file then specifies which peripherals are included in the chip.
This is very useful when a vendor has used similar peripheral IPs across their portfolio.
This allows HAL authors to write drivers for these peripheral IPs, instead of having to copy-paste them for each supported chipset.

Because the crate is in transition, some of them use the PAC method, and some of them are part of the metapac.

## Supported chips

| Chip | Type |
|------|------|
| MIMXRT1011 | PAC |
| MIMXRT1062 | PAC |
| MIMXRT1064 | PAC |
| MIMXRT685S | PAC |
| LPC55S16 | PAC |
| LPC55S69 | PAC |
| MCXN947 | PAC |
| MCXA256 | MetaPAC |
| MCXA577 | MetaPAC |

## Tour

The [`data`](/data) directory contains the SVD files, board metadata, and chiptool transformations needed to
generate the nxp-pac crate. This data is used by the code generation tool.

The [`generator`](/generator) directory contains the code generation tool for generating the code in the [`nxp-pac`](/nxp-pac) crate.
If you want change the way the [`nxp-pac`](/nxp-pac) crate is generated please see this directory.

The [`nxp-pac`](/nxp-pac) directory contains the nxp-pac crate. If you are looking for a peripheral access crate for
an NXP microcontroller, please see this directory. You should never need to manually edit the source
code in this directory. The metadata will need to be updated to support new microcontrollers.

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by NXP under a BSD-3-Clause license.
