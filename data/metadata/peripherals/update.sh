#!/bin/bash

set -euxo pipefail

# This script updates all the peripheral files from source SVDs,
# after applying the relevant transforms.

# The peripheral description files should still be considered the 'source-of-truth',
# but this script allows for easy updating / checking if anything relevant has changed
# when updating the SVDs.

CURRENT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"

pushd $CURRENT_DIR/../../../
cargo run -p generator -- extract MCXA156
cargo run -p generator -- extract MCXA256
cargo run -p generator -- extract MCXA577
popd

pushd $CURRENT_DIR

# Manually curated, do not change
# cp raw/MCXA577/DMA.yaml mcxa/DMA.yaml
# cp raw/MCXA577/EDMA_TCD.yaml mcxa/EDMA_TCD.yaml
# cp raw/MCXA577/AHBSC.yaml mcxa/AHBSC.yaml

cp raw/MCXA256/FLEXIO.yaml mcxa/FLEXIO.yaml
cp raw/MCXA256/FLEXPWM.yaml mcxa/FLEXPWM.yaml
cp raw/MCXA256/SPC.yaml mcxa/SPC.yaml
cp raw/MCXA256/USB.yaml mcxa/USB.yaml

cp raw/MCXA577/ADC.yaml mcxa/ADC.yaml
cp raw/MCXA577/CDOG.yaml mcxa/CDOG.yaml
cp raw/MCXA577/CMC.yaml mcxa/CMC.yaml
cp raw/MCXA577/CRC.yaml mcxa/CRC.yaml
cp raw/MCXA577/CTIMER.yaml mcxa/CTIMER.yaml
cp raw/MCXA577/DAC.yaml mcxa/DAC.yaml
cp raw/MCXA577/FLEXSPI.yaml mcxa/FLEXSPI.yaml
cp raw/MCXA577/FMU.yaml mcxa/FMU.yaml
cp raw/MCXA577/GPIO.yaml mcxa/GPIO.yaml
cp raw/MCXA577/I3C.yaml mcxa/I3C.yaml
cp raw/MCXA577/INPUTMUX.yaml mcxa/INPUTMUX.yaml
cp raw/MCXA577/LPI2C.yaml mcxa/LPI2C.yaml
cp raw/MCXA577/LPSPI.yaml mcxa/LPSPI.yaml
cp raw/MCXA577/LPUART.yaml mcxa/LPUART.yaml
cp raw/MCXA577/MBC.yaml mcxa/MBC.yaml
cp raw/MCXA577/OSTIMER.yaml mcxa/OSTIMER.yaml
cp raw/MCXA577/PORT.yaml mcxa/PORT.yaml
cp raw/MCXA577/SCG.yaml mcxa/SCG.yaml
cp raw/MCXA577/SGI.yaml mcxa/SGI.yaml
cp raw/MCXA577/TRNG.yaml mcxa/TRNG.yaml
cp raw/MCXA577/VBAT.yaml mcxa/VBAT.yaml
cp raw/MCXA577/WWDT.yaml mcxa/WWDT.yaml

cp raw/MCXA256/RTC.yaml mcxa/RTC2xx.yaml
cp raw/MCXA577/RTC.yaml mcxa/RTC5xx.yaml

cp raw/MCXA156/MRCC.yaml mcxa/MRCC1xx.yaml
cp raw/MCXA256/MRCC.yaml mcxa/MRCC2xx.yaml
cp raw/MCXA577/MRCC.yaml mcxa/MRCC5xx.yaml

cp raw/MCXA156/SYSCON.yaml mcxa/SYSCON1xx.yaml
cp raw/MCXA256/SYSCON.yaml mcxa/SYSCON2xx.yaml
cp raw/MCXA577/SYSCON.yaml mcxa/SYSCON5xx.yaml
