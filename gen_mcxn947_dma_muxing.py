#!/usr/bin/env python3
"""Patch `dma_muxing` request-source metadata into data/metadata/MCXN947.json.

Usage:  ./gen_mcxn947_dma_muxing.py <rm.txt>

where <rm.txt> is a plain-text extraction of the MCX N reference manual:

    pdftotext -layout MCXNP184M150F70RM.pdf rm.txt

The manual is not redistributable, so it is not in this repo and the extraction
path is an argument. This script was written against Rev. 7, 2025-10-10.

Unlike gen_mcxn947_metadata.py, this script is *additive*: it patches the
`dma_muxing` key into peripherals that already exist and leaves every other key
(notably the hand-maintained `block`) untouched. It is safe to re-run.

The source data is RM Table 291 (DMAMUX0) and Table 292 (DMAMUX1). Those two
tables are byte-for-byte identical - all 122 rows - which is what licenses a
single flat request map shared by both eDMA controllers, and hence a single
`DmaRequest` enum in the HAL. That equality is asserted below rather than
assumed, because embassy's build.rs keys its request map on the signal name
alone and silently drops collisions: if the two controllers ever diverge, this
script must fail rather than quietly emit one controller's numbers.

The reason this script exists rather than a hand-edited JSON is NAMES below.
Turning 122 rows of a PDF table into 117 metadata entries embeds naming
judgements that are invisible in the resulting JSON, and re-deriving them is the
expensive part. They are written out as data so they show up in review.
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).parent
OUT = ROOT / "data/metadata/MCXN947.json"

# Line ranges of the two tables' data rows in the Rev. 7 extraction. Generous
# bounds; the row regex does the real work and the EXPECTED check below catches
# a range that drifted onto the wrong table.
DMAMUX0 = (48652, 48958)
DMAMUX1 = (48962, 49272)

# The `mux` field is required by data/metadata/schema.json but is read by nothing
# that consumes MCXN947 - embassy-mcxn's build.rs uses only `signal` and
# `request`. It deliberately does NOT name DMA0 or DMA1: both are real
# peripherals here, so either would read as a claim that this map is specific to
# one controller, which Table 291 == Table 292 disproves. MCXN947 has no DMAMUX
# peripheral in its metadata because the RM describes the DMAMUX as "an
# integrated component of the DMA Controller", so this names the mux itself and
# is unmistakably a placeholder.
MUX = "DMAMUX"

# RM alias -> peripheral name in MCXN947.json, for the three that differ.
# MICFIL0 is the RM's name for the block this metadata calls PDM (RM chapter 76
# is "PDM Microphone Interface (MICFIL)", and the NVIC vector is MICFIL0).
ALIAS_TO_PERIPHERAL = {
    "FlexSPI0": "FLEXSPI0",
    "FlexIO0": "FLEXIO0",
    "MICFIL0": "PDM",
}

# Requests deliberately not represented.
#
#   0            "Disabled". Not a request source but the idle encoding of
#                CH_MUX[SRC], which the driver already writes as a literal when
#                clearing the mux. A `Disabled` variant would invite passing it
#                to set_request_source and make from_number_unchecked(0) look
#                valid for an unrouted channel.
#   55,56,89,90  "Reserved". No peripheral to attach them to, and four identical
#                signal strings would collapse to one arbitrary survivor in
#                build.rs's HashMap - the failure MCXA already exhibits, where
#                EQDC0/EQDC1 both use signal "BUFFER" and request 65 silently
#                vanishes from the generated enum.
OMITTED = {0, 55, 56, 89, 90}

# request -> (peripheral, signal)
#
# Naming policy, in priority order:
#
#  1. Where the same logical source exists on MCXA, reuse MCXA's signal string
#     verbatim, so a driver ported from embassy-mcxa resolves DmaRequest::X
#     unchanged. Marked (A) below.
#  2. Otherwise <PERIPH><Function>, where <PERIPH> is the peripheral's name as
#     spelled in MCXN947.json and <Function> renders the RM's Source Description
#     as an identifier. Data movement is always Rx/Tx.
#
# The HAL variant name is convert_case's pascal case of the signal, which is not
# predictable by eye - LPI2C0Rx becomes Lpi2C0Rx while I3C0Rx stays I3C0Rx - so
# the variant spellings are confirmed by building the HAL, not by reading this.
NAMES = {
    # (A) FLEXSPI: matches MCXA, so a ported FlexSPI driver needs no edits.
    1: ("FLEXSPI0", "FLEXSPI0Rx"),
    2: ("FLEXSPI0", "FLEXSPI0Tx"),
    3: ("PINT0", "PINT0INT0"),
    4: ("PINT0", "PINT0INT1"),
    5: ("PINT0", "PINT0INT2"),
    6: ("PINT0", "PINT0INT3"),
    # (A) CTIMER match requests.
    7: ("CTIMER0", "CTIMER0M0"),
    8: ("CTIMER0", "CTIMER0M1"),
    9: ("CTIMER1", "CTIMER1M0"),
    10: ("CTIMER1", "CTIMER1M1"),
    11: ("CTIMER2", "CTIMER2M0"),
    12: ("CTIMER2", "CTIMER2M1"),
    13: ("CTIMER3", "CTIMER3M0"),
    14: ("CTIMER3", "CTIMER3M1"),
    15: ("CTIMER4", "CTIMER4M0"),
    16: ("CTIMER4", "CTIMER4M1"),
    17: ("WUU0", "WUU0WakeUpEvent"),  # (A)
    # The RM alias is MICFIL0; the peripheral is PDM. Prefixed with the
    # peripheral's own name, like every other entry here.
    18: ("PDM", "PDMFifoRequest"),
    # RM describes these as SCT0 "DMA0"/"DMA1", meaning SCT request outputs, not
    # the eDMA controllers. Named ...DmaRequest{n} so the index cannot be read as
    # a controller number, matching the CMP{n}DmaRequest spelling below.
    19: ("SCT0", "SCT0DmaRequest0"),
    20: ("SCT0", "SCT0DmaRequest1"),
    # MCXN's ADCs have two FIFOs where MCXA has one, so MCXA's single
    # ADC{n}FifoRequest cannot be reused - one signal string cannot carry two
    # request numbers. A ported ADC driver must pick a FIFO.
    21: ("ADC0", "ADC0FifoARequest"),
    22: ("ADC0", "ADC0FifoBRequest"),
    23: ("ADC1", "ADC1FifoARequest"),
    24: ("ADC1", "ADC1FifoBRequest"),
    25: ("DAC0", "DAC0FifoRequest"),  # (A)
    26: ("DAC1", "DAC1FifoRequest"),  # (A)
    27: ("DAC2", "DAC2FifoRequest"),  # (A)
    28: ("CMP0", "CMP0DmaRequest"),
    29: ("CMP1", "CMP1DmaRequest"),
    30: ("CMP2", "CMP2DmaRequest"),
    31: ("EVTG0", "EVTG0Out0A"),
    32: ("EVTG0", "EVTG0Out0B"),
    33: ("EVTG0", "EVTG0Out1A"),
    34: ("EVTG0", "EVTG0Out1B"),
    35: ("EVTG0", "EVTG0Out2A"),
    36: ("EVTG0", "EVTG0Out2B"),
    37: ("EVTG0", "EVTG0Out3A"),
    38: ("EVTG0", "EVTG0Out3B"),
    # (A) MCXA names this peripheral FLEX_PWM0 while MCXN names it PWM0. Rule 1
    # wins: this is the one signal whose prefix does not match its own
    # peripheral's metadata name, accepted so a ported eFlexPWM driver works.
    39: ("PWM0", "FlexPWM0Mcapt0"),
    40: ("PWM0", "FlexPWM0Mcapt1"),
    41: ("PWM0", "FlexPWM0Mcapt2"),
    42: ("PWM0", "FlexPWM0Mcapt3"),
    43: ("PWM0", "FlexPWM0Mval0"),
    44: ("PWM0", "FlexPWM0Mval1"),
    45: ("PWM0", "FlexPWM0Mval2"),
    46: ("PWM0", "FlexPWM0Mval3"),
    47: ("PWM1", "FlexPWM1Mcapt0"),
    48: ("PWM1", "FlexPWM1Mcapt1"),
    49: ("PWM1", "FlexPWM1Mcapt2"),
    50: ("PWM1", "FlexPWM1Mcapt3"),
    51: ("PWM1", "FlexPWM1Mval0"),
    52: ("PWM1", "FlexPWM1Mval1"),
    53: ("PWM1", "FlexPWM1Mval2"),
    54: ("PWM1", "FlexPWM1Mval3"),
    57: ("LPTMR0", "LPTMR0CounterMatchEvent"),  # (A)
    58: ("LPTMR1", "LPTMR1CounterMatchEvent"),  # (A)
    59: ("CAN0", "CAN0"),  # (A) function-less, like MCXA
    60: ("CAN1", "CAN1"),  # (A)
    # (A) One request per index, shared by "Shifter{k} OR Timer{k}". MCXA's SR
    # ("status request") spelling is deliberately agnostic about which of the two
    # fires, which is a FlexIO configuration matter rather than a mux one.
    # Encoding both halves would be two signals on one request number.
    61: ("FLEXIO0", "FLEXIO0SR0"),
    62: ("FLEXIO0", "FLEXIO0SR1"),
    63: ("FLEXIO0", "FLEXIO0SR2"),
    64: ("FLEXIO0", "FLEXIO0SR3"),
    65: ("FLEXIO0", "FLEXIO0SR4"),
    66: ("FLEXIO0", "FLEXIO0SR5"),
    67: ("FLEXIO0", "FLEXIO0SR6"),
    68: ("FLEXIO0", "FLEXIO0SR7"),
    # One Rx and one Tx per flexcomm, shared by whichever of UART/SPI/I2C
    # PSELID[PERSEL] selects - there is no per-LPUART/LPSPI/LPI2C request. Naming
    # these per function is not merely unidiomatic but impossible: three signals
    # sharing one request number is a duplicate-discriminant error. A serial
    # driver ported from MCXA must reference LpFlexcomm{n}Tx, not Lpuart{n}Tx.
    69: ("LP_FLEXCOMM0", "LP_FLEXCOMM0Rx"),
    70: ("LP_FLEXCOMM0", "LP_FLEXCOMM0Tx"),
    71: ("LP_FLEXCOMM1", "LP_FLEXCOMM1Rx"),
    72: ("LP_FLEXCOMM1", "LP_FLEXCOMM1Tx"),
    73: ("LP_FLEXCOMM2", "LP_FLEXCOMM2Rx"),
    74: ("LP_FLEXCOMM2", "LP_FLEXCOMM2Tx"),
    75: ("LP_FLEXCOMM3", "LP_FLEXCOMM3Rx"),
    76: ("LP_FLEXCOMM3", "LP_FLEXCOMM3Tx"),
    77: ("LP_FLEXCOMM4", "LP_FLEXCOMM4Rx"),
    78: ("LP_FLEXCOMM4", "LP_FLEXCOMM4Tx"),
    79: ("LP_FLEXCOMM5", "LP_FLEXCOMM5Rx"),
    80: ("LP_FLEXCOMM5", "LP_FLEXCOMM5Tx"),
    81: ("LP_FLEXCOMM6", "LP_FLEXCOMM6Rx"),
    82: ("LP_FLEXCOMM6", "LP_FLEXCOMM6Tx"),
    83: ("LP_FLEXCOMM7", "LP_FLEXCOMM7Rx"),
    84: ("LP_FLEXCOMM7", "LP_FLEXCOMM7Tx"),
    85: ("LP_FLEXCOMM8", "LP_FLEXCOMM8Rx"),
    86: ("LP_FLEXCOMM8", "LP_FLEXCOMM8Tx"),
    87: ("LP_FLEXCOMM9", "LP_FLEXCOMM9Rx"),
    88: ("LP_FLEXCOMM9", "LP_FLEXCOMM9Tx"),
    91: ("EMVSIM0", "EMVSIM0Rx"),
    92: ("EMVSIM0", "EMVSIM0Tx"),
    93: ("EMVSIM1", "EMVSIM1Rx"),
    94: ("EMVSIM1", "EMVSIM1Tx"),
    95: ("I3C0", "I3C0Rx"),  # (A)
    96: ("I3C0", "I3C0Tx"),  # (A)
    97: ("I3C1", "I3C1Rx"),  # (A)
    98: ("I3C1", "I3C1Tx"),  # (A)
    99: ("SAI0", "SAI0Rx"),
    100: ("SAI0", "SAI0Tx"),
    101: ("SAI1", "SAI1Rx"),
    102: ("SAI1", "SAI1Tx"),
    # RM describes these as "ipd_req_sinc[k] or ipd_req_alt [k]" - internal RTL
    # port names, and another OR'd pair on one request line. Named for what they
    # mean: the channel-k request.
    103: ("SINC0", "SINC0Ch0Request"),
    104: ("SINC0", "SINC0Ch1Request"),
    105: ("SINC0", "SINC0Ch2Request"),
    106: ("SINC0", "SINC0Ch3Request"),
    107: ("SINC0", "SINC0Ch4Request"),
    108: ("GPIO0", "GPIO0PinEvent0"),  # (A)
    109: ("GPIO0", "GPIO0PinEvent1"),
    110: ("GPIO1", "GPIO1PinEvent0"),
    111: ("GPIO1", "GPIO1PinEvent1"),
    112: ("GPIO2", "GPIO2PinEvent0"),
    113: ("GPIO2", "GPIO2PinEvent1"),
    114: ("GPIO3", "GPIO3PinEvent0"),
    115: ("GPIO3", "GPIO3PinEvent1"),
    116: ("GPIO4", "GPIO4PinEvent0"),
    117: ("GPIO4", "GPIO4PinEvent1"),
    118: ("GPIO5", "GPIO5PinEvent0"),
    119: ("GPIO5", "GPIO5PinEvent1"),
    120: ("TSI0", "TSI0EndOfScan"),
    121: ("TSI0", "TSI0OutOfRange"),
}

# The (alias, description) the RM is expected to give for each request. Pinning
# this means a different manual revision, or a pdftotext version that lays the
# columns out differently, fails loudly instead of silently renaming a source.
EXPECTED = {
    0: ("—", "Disabled"),
    1: ("FlexSPI0", "Receive event"),
    2: ("FlexSPI0", "Transmit event"),
    17: ("WUU0", "Wake up event"),
    18: ("MICFIL0", "FIFO_request"),
    19: ("SCT0", "DMA0"),
    20: ("SCT0", "DMA1"),
    21: ("ADC0", "FIFO A request"),
    22: ("ADC0", "FIFO B request"),
    55: ("Reserved", "—"),
    56: ("Reserved", "—"),
    59: ("CAN0", "DMA request"),
    61: ("FlexIO0", "Shifter0 Status DMA request OR Timer0 Status DMA request"),
    69: ("LP_FLEXCOMM0", "Receive request"),
    70: ("LP_FLEXCOMM0", "Transmit request"),
    88: ("LP_FLEXCOMM9", "Transmit request"),
    89: ("Reserved", "—"),
    90: ("Reserved", "—"),
    103: ("SINC0", "ipd_req_sinc[0] or ipd_req_alt [0]"),
    108: ("GPIO0", "Pin event request 0"),
    120: ("TSI0", "End of Scan"),
    121: ("TSI0", "Out of Range"),
}


def parse_table(lines, first, last):
    """Extract (request, alias, description) rows from one DMAMUX table.

    Every data row is a single line; the number column is not at a fixed offset,
    so split on runs of two or more spaces rather than slicing. Page furniture
    and the one-off `Modules` section label do not match the leading-number
    pattern. Keep the `\\S` in the pattern: loosening it to `^\\s*(\\d+)` starts
    ingesting page numbers ("733 / 3763") as request IDs.
    """
    rows = []
    for line in lines[first - 1 : last]:
        if re.match(r"^ +\d+ +\S", line):
            fields = re.split(r" {2,}", line.strip())
            assert len(fields) == 3, f"expected 3 columns, got {fields!r}"
            rows.append((int(fields[0]), fields[1], fields[2]))
    return rows


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    lines = Path(sys.argv[1]).read_text(encoding="utf-8").split("\n")

    mux0 = parse_table(lines, *DMAMUX0)
    mux1 = parse_table(lines, *DMAMUX1)

    assert len(mux0) == 122, f"DMAMUX0: expected 122 rows, got {len(mux0)}"
    assert [r[0] for r in mux0] == list(range(122)), "DMAMUX0 requests not 0..121"
    # The premise of a single shared request map. If this ever fails, the HAL's
    # one flat DmaRequest enum is wrong and needs per-controller handling.
    assert mux0 == mux1, "DMAMUX0 and DMAMUX1 differ; a single request map is no longer valid"

    by_request = {r[0]: (r[1], r[2]) for r in mux0}
    for request, expected in EXPECTED.items():
        assert by_request[request] == expected, (
            f"request {request}: RM says {by_request[request]!r}, expected {expected!r}"
        )

    assert set(NAMES) == set(range(122)) - OMITTED, "NAMES does not cover exactly the non-omitted requests"
    assert len(NAMES) == 117, f"expected 117 entries, got {len(NAMES)}"

    signals = [signal for _, signal in NAMES.values()]
    assert len(set(signals)) == len(signals), "duplicate signal strings would silently collapse in build.rs"
    # Two distinct signals can still collide once pascal-cased, which would also
    # collapse silently. Approximate the casing conservatively.
    folded = [s.replace("_", "").lower() for s in signals]
    assert len(set(folded)) == len(folded), "signals collide after case folding"
    # CH_MUX[SRC] is 7 bits and set_src masks silently, so an out-of-range
    # request would be truncated rather than rejected.
    assert max(NAMES) <= 127, "request number exceeds CH_MUX[SRC]"

    # Each entry must hang off the peripheral the RM's alias refers to.
    for request, (peripheral, _) in NAMES.items():
        alias = by_request[request][0]
        assert ALIAS_TO_PERIPHERAL.get(alias, alias) == peripheral, (
            f"request {request}: alias {alias!r} does not correspond to peripheral {peripheral!r}"
        )

    original = OUT.read_text(encoding="utf-8")
    meta = json.loads(original)
    # Check the formatting round-trips before mutating, so a drift in the
    # committed file is reported rather than absorbed into this diff.
    assert json.dumps(meta, indent=2) + "\n" == original, (
        f"{OUT.name} is not in json.dumps(indent=2) form; reformat it separately first"
    )

    by_peripheral = {}
    for request in sorted(NAMES):
        peripheral, signal = NAMES[request]
        by_peripheral.setdefault(peripheral, []).append(
            {"signal": signal, "mux": MUX, "request": request}
        )

    known = {p["name"] for p in meta["peripherals"]}
    missing = sorted(set(by_peripheral) - known)
    assert not missing, f"no such peripherals in {OUT.name}: {missing}"

    # Rebuild each touched peripheral so `dma_muxing` lands before `signals`,
    # matching MCXA5xx's key order, rather than being appended at the end.
    patched = 0
    for i, peripheral in enumerate(meta["peripherals"]):
        entries = by_peripheral.get(peripheral["name"])
        if entries is None:
            continue
        rebuilt = {}
        for key, value in peripheral.items():
            if key == "signals":
                rebuilt["dma_muxing"] = entries
            if key != "dma_muxing":
                rebuilt[key] = value
        rebuilt.setdefault("dma_muxing", entries)
        meta["peripherals"][i] = rebuilt
        patched += 1

    assert patched == len(by_peripheral), f"patched {patched} of {len(by_peripheral)} peripherals"
    total = sum(len(v) for v in by_peripheral.values())
    assert total == 117, f"wrote {total} entries, expected 117"

    OUT.write_text(json.dumps(meta, indent=2) + "\n", encoding="utf-8")
    print(f"patched {total} dma_muxing entries across {patched} peripherals in {OUT.name}")


if __name__ == "__main__":
    main()
