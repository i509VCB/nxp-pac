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
committed output is current. The status command must print nothing; unlike a
plain `git diff`, it also detects newly generated untracked files.

```sh
cargo run -p generator --locked -- generate
git status --porcelain=v1 --untracked-files=all -- nxp-pac
```

Source-only chips can lock a different revision without moving the shared
submodule or changing existing PAC inputs. Recreate and validate the MCXA156
source artifacts with:

```sh
git -C data/mcux-soc-svd fetch --depth=1 https://github.com/nxp-mcuxpresso/mcux-soc-svd cd86b0793d7e467055a2d84c441c81e3d0aef93d
git init data/source-cache/mcux-devices-mcx
git -C data/source-cache/mcux-devices-mcx remote add origin https://github.com/nxp-mcuxpresso/mcux-devices-mcx
git -C data/source-cache/mcux-devices-mcx fetch --depth=1 origin f560437f1a3b629869fb38e268fea91f553e3094
cargo run -p generator --locked -- extract-source MCXA156
git status --porcelain=v1 --untracked-files=all -- data/source-peripherals/MCXA156
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
