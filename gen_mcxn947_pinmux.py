#!/usr/bin/env python3
"""Add MCXN947 pin-mux metadata from the reference manual.

Usage:  ./gen_mcxn947_pinmux.py <rm.txt>

where rm.txt is `pdftotext -layout MCXNP184M150F70RM.pdf rm.txt` output.

Table 119 "pinmux" lists, for each of the 124 pins, which peripheral signal
each ALT setting selects. ALT0 is GPIO on every pin and is already present in
the metadata, so this script contributes the 788 non-GPIO pairs, which is what
the `signals`-driven pin traits in embassy-mcxn are generated from.

The script is additive and idempotent: it only writes `signals` entries for
peripherals it resolves, and rewrites the whole set each run, so re-running
against the same RM is a no-op.
"""

import json
import re
import sys
from collections import defaultdict

METADATA = "data/metadata/MCXN947.json"

# Line range of Table 119 in the pdftotext extraction. The table is long enough
# to cross many page breaks, and the surrounding text contains `ALTn` tokens of
# its own, so the parse is bounded rather than searching the whole document.
TABLE_START, TABLE_END = 18486, 21469

# The Pinmux Assignment column occupies roughly columns 80..104. Truncating each
# line here is what keeps three neighbouring columns out of the parse:
#
#   * `Default - ALT1` in the pad-settings column (6 rows) would otherwise
#     create a phantom ALT1 that then steals the following row's signal.
#   * `LPTMR1_ALT3` in the analog column would create a phantom ALT3 for P1_8,
#     which already has a real one. Note this cannot be filtered by blacklisting
#     the substring, because `LPTMR0_ALT2`/`LPTMR1_ALT2` are *legitimate* ALT2
#     signal values on P5_0/P5_1/P5_5/P5_6.
#   * Longest signal in the table (`USB1_VBUSVALID_EXT`, 18 chars from col 83)
#     ends at column 101, so 105 clears every real value.
BAND_END = 105

# Minimum column for an `ALTn` token to count as a pinmux entry. Signal values
# in other columns never appear this far right within the truncated band.
ALT_MIN_COL = 70

# Inventory pinning the RM text this script was written against. A parse that
# drifts - a changed layout, a different pdftotext version, an edited line
# range - fails here rather than silently writing a subset.
EXPECTED_PINS = 124
EXPECTED_PAIRS = 912
EXPECTED_NON_GPIO = 788

# Exhaustive (ALT, signal family) -> count census of the 788 non-GPIO entries.
# This is the cheapest broad guard against a systematic parse error: nothing
# cross-checks a pad against the signal it is meant to carry, so a shifted
# column or a dropped page shows up here as a count that no longer matches.
#
# Each ALT setting selects one peripheral class, which is why the table is this
# regular - a family spread across unrelated ALTs would itself be a parse bug.
EXPECTED_CENSUS = {
    (1, "CLKOUT"): 2,
    (1, "EWM"): 6,
    (1, "FREQME"): 4,
    (1, "ISPMODE"): 1,
    (1, "TCLK"): 1,
    (1, "TDI"): 1,
    (1, "TDO"): 1,
    (1, "TMS"): 1,
    (1, "TRACE"): 10,
    (1, "TRIG"): 40,
    (1, "USB"): 1,
    (1, "VBAT"): 1,
    (2, "FC"): 91,
    (2, "LPTMR"): 4,
    (2, "RTC"): 1,
    (2, "SPC"): 2,
    (3, "FC"): 49,
    (3, "TAMPER"): 8,
    (3, "USB"): 4,
    (3, "USDHC"): 10,
    (4, "CT"): 102,
    (4, "SCT"): 12,
    (5, "PWM"): 36,
    (5, "SCT"): 26,
    (5, "UTICK"): 8,
    (6, "FLEXIO"): 88,
    (7, "SMARTDMA"): 64,
    (8, "FLEXSPI"): 24,
    (8, "HSCMP"): 6,
    (8, "PLU"): 30,
    (9, "ENET"): 20,
    (9, "PDM"): 6,
    (9, "SIM"): 15,
    (9, "SINC"): 24,
    (10, "I"): 12,
    (10, "SAI"): 40,
    (11, "CAN"): 16,
    (11, "ESPI"): 8,
    (11, "PF"): 13,
}

# Signals recorded as-is rather than split on a peripheral-name prefix.
#
# Flexcomm pads are deliberately kept raw. RM 66.2.3 maps `FCn_Pm` to per-
# function signals, but the mapping is not the intuitive one (for UART, `P0` is
# RXD and `P1` is TXD), so deriving LPUART/LPSPI/LPI2C signal names is left for
# when a loopback test can confirm the direction.
ALIASES = [
    # (signal pattern, peripheral template, signal template)
    (r"^CT(\d)_(MAT\d)$", r"CTIMER\1", r"\2"),
    (r"^FC(\d)_(P\d)$", r"LP_FLEXCOMM\1", r"\2"),
    (r"^SMARTDMA_PIO(\d+)$", r"SMARTDMA0", r"PIO\1"),
    (r"^SIM(\d)_(.+)$", r"EMVSIM\1", r"\2"),
    (r"^HSCMP(\d)_(OUT)$", r"CMP\1", r"\2"),
    (r"^PDM0_(.+)$", r"PDM", r"\1"),  # the peripheral is named PDM, not PDM0
    (r"^UTICK_(CAP\d)$", r"UTICK0", r"\1"),
    (r"^FREQME_(CLK_IN\d)$", r"FREQME0", r"\1"),
    (r"^PLU_(.+)$", r"PLU0", r"\1"),
    (r"^SPC_(LPREQ)$", r"SPC0", r"\1"),
    (r"^(TAMPER\d)$", r"ITRC0", r"\1"),
    (r"^USB0_(VBUS_DET)$", r"USBFS0", r"\1"),
    # Trigger I/O pads feed the INPUTMUX trigger network.
    (r"^TRIG_(IN\d+|OUT\d+)$", r"INPUTMUX0", r"\1"),
]

# `CT_INPn` carries no timer index: the 20 capture inputs are shared and routed
# to any timer through INPUTMUX. Recorded on all five CTIMERs with identical pin
# lists, which is how the MCXA metadata models the same hardware.
SHARED = [(r"^CT_(INP\d+)$", [f"CTIMER{i}" for i in range(5)], r"\1")]

# Signals with no peripheral in the metadata to attach them to. Recording these
# would mean inventing an owner, so they are skipped and reported.
SKIP = [
    (r"^ESPI0_", "no eSPI peripheral in the MCXN947 metadata"),
    (r"^PF_(QSPI|SPI)_", "unidentified 'PF' block"),
    (r"^USB1_", "ambiguous between USBHS1__USBC, USBHS1__USBNC and USBPHY"),
    (r"^TRACE_(DATA|CLK)", "trace port, not a metadata peripheral"),
    (r"^(TMS/SWDIO|TCLK/SWCLK|TDO/SWO|TDI)$", "debug port"),
    (r"^(CLKOUT|RTC_CLKOUT)$", "clock output pad with no owning block"),
    (r"^(ISPMODE_N|VBAT_WAKEUP_b)$", "boot/wake system pad"),
]


def parse_table(text):
    """Return [(pin, alt, signal)] for every ALT entry in Table 119."""
    lines = text.split("\n")
    pairs = []
    pin = None
    # An `ALTn` whose signal is on a later line. 107 entries wrap this way, and
    # one (P4_15 ALT3) wraps with the leading dash absent as well.
    pending = None

    for raw in lines[TABLE_START - 1 : TABLE_END]:
        band = raw[:BAND_END]

        # A row starting with a pin name opens that pin's block. Everything up
        # to the next such row belongs to it, including across page breaks.
        m = re.match(r"^ (P\d+_\d+)\s", raw)
        if m:
            pin, pending = m.group(1), None
        if pin is None:
            continue

        alt = re.search(r"\bALT(\d+)\b", band)
        if alt and alt.start() >= ALT_MIN_COL:
            sig = re.match(r"\s*-\s*([A-Za-z0-9_/]+)", band[alt.end() :])
            if sig:
                pairs.append((pin, int(alt.group(1)), sig.group(1)))
                pending = None
            else:
                pending = (pin, int(alt.group(1)))
        elif pending:
            sig = re.match(r"^ {%d,}-?\s*([A-Za-z0-9_/]+)\s*$" % ALT_MIN_COL, band)
            if sig:
                pairs.append((pending[0], pending[1], sig.group(1)))
                pending = None

    assert pending is None, f"ALT entry with no signal: {pending}"
    return pairs


def check(pairs, pins):
    """Assert the parse matches the RM inventory."""
    parsed_pins = {p for p, _, _ in pairs}
    assert len(parsed_pins) == EXPECTED_PINS, f"{len(parsed_pins)} pins, want {EXPECTED_PINS}"
    assert len(pairs) == EXPECTED_PAIRS, f"{len(pairs)} pairs, want {EXPECTED_PAIRS}"

    missing = parsed_pins - pins
    assert not missing, f"pins absent from the metadata pin list: {sorted(missing)}"

    # A pad-settings value leaking through the column window shows up here.
    leaked = {s for _, _, s in pairs if s in ("Default", "DIS", "RESET")}
    assert not leaked, f"pad-setting values parsed as signals: {sorted(leaked)}"

    seen = set()
    for pin, alt, _ in pairs:
        assert (pin, alt) not in seen, f"duplicate {pin} ALT{alt}"
        seen.add((pin, alt))
        # `Mux` in embassy-mcxn covers Mux0..Mux13.
        assert alt <= 13, f"{pin} ALT{alt} exceeds the Mux enum"
        # PORT5 pins have a 2-bit MUX field.
        assert not (pin.startswith("P5_") and alt > 3), f"{pin} ALT{alt} exceeds PORT5's 2-bit field"

    # ALT0 is GPIO on every pin, and its signal value is the pin name itself,
    # which is what makes embassy-mcxn's hardcoded Mux0 for GPIO correct.
    alt0 = {p: s for p, a, s in pairs if a == 0}
    assert len(alt0) == EXPECTED_PINS, "ALT0 missing on some pins"
    wrong = {p: s for p, s in alt0.items() if p != s}
    assert not wrong, f"ALT0 signal is not the pin name: {wrong}"

    non_gpio = [x for x in pairs if x[1] != 0]
    assert len(non_gpio) == EXPECTED_NON_GPIO, f"{len(non_gpio)} non-GPIO, want {EXPECTED_NON_GPIO}"

    census = defaultdict(int)
    for _, alt, sig in non_gpio:
        census[(alt, re.match(r"^([A-Za-z]+)", sig).group(1).upper())] += 1
    assert sum(EXPECTED_CENSUS.values()) == EXPECTED_NON_GPIO, "census does not cover every entry"
    assert dict(census) == EXPECTED_CENSUS, (
        "census mismatch: "
        + str(
            {
                k: (census.get(k), EXPECTED_CENSUS.get(k))
                for k in set(census) | set(EXPECTED_CENSUS)
                if census.get(k) != EXPECTED_CENSUS.get(k)
            }
        )
    )

    return non_gpio


def resolve(signal):
    """Map an RM signal to [(peripheral, signal)], or [] if skipped."""
    for pattern, _reason in SKIP:
        if re.search(pattern, signal):
            return []

    for pattern, peripherals, template in SHARED:
        m = re.match(pattern, signal)
        if m:
            return [(p, m.expand(template)) for p in peripherals]

    for pattern, peripheral, template in ALIASES:
        # Case-insensitive only to absorb `SmartDMA_PIO3`, which the RM spells
        # inconsistently with the other 63 SMARTDMA entries. Every alias
        # template is literal uppercase or captures already-uppercase RM text,
        # so no case folding is applied to the result - that would corrupt the
        # active-low `_b` suffix that reaches the prefix split below.
        m = re.match(pattern, signal, re.IGNORECASE)
        if m:
            return [(m.expand(peripheral), m.expand(template))]

    return None  # fall through to the peripheral-name prefix split


def signal_sort_key(signal):
    """Group signals by family, ordering the index numerically (INP9 < INP10)."""
    m = re.match(r"^(.*?)(\d+)$", signal)
    return (m.group(1), int(m.group(2))) if m else (signal, -1)


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)

    with open(METADATA, encoding="utf-8") as f:
        original = f.read()
    data = json.loads(original)

    # Guarantee that writing back an unmodified structure is byte-identical, so
    # the diff this script produces contains only the metadata it adds.
    assert json.dumps(data, indent=2) + "\n" == original, "metadata is not round-trip stable"

    peripherals = {p["name"]: p for p in data["peripherals"]}
    pin_names = {
        s["pins"][0]["pin"]
        for p in data["peripherals"]
        if p["name"].startswith("GPIO")
        for s in p.get("signals", [])
    }

    with open(sys.argv[1], encoding="utf-8") as f:
        pairs = parse_table(f.read())
    non_gpio = check(pairs, pin_names)

    # Longest peripheral-name prefix, so `LPTMR0_ALT2` splits on LPTMR0 rather
    # than a shorter name that happens to be a prefix of it.
    by_length = sorted(peripherals, key=len, reverse=True)

    signals = defaultdict(lambda: defaultdict(list))
    skipped = defaultdict(int)
    for pin, alt, sig in non_gpio:
        targets = resolve(sig)

        if targets == []:
            reason = next(r for pat, r in SKIP if re.search(pat, sig))
            skipped[reason] += 1
            continue

        if targets is None:
            name = next((n for n in by_length if sig.upper().startswith(n.upper())), None)
            assert name, f"no peripheral owns signal {sig!r}"
            targets = [(name, sig[len(name) :].lstrip("_"))]

        for peripheral, signal in targets:
            assert peripheral in peripherals, f"{sig!r} resolved to unknown peripheral {peripheral}"
            assert signal, f"{sig!r} resolved to an empty signal name on {peripheral}"
            signals[peripheral][signal].append({"pin": pin, "alt": alt})

    added = 0
    for name, sigs in signals.items():
        peripheral = peripherals[name]
        # GPIO is the only peripheral with curated signals - its ALT0 entries -
        # and no signal here resolves to GPIO, so rewriting the whole set is
        # safe and is what makes a re-run against the same RM a no-op.
        assert not name.startswith("GPIO"), f"refusing to overwrite curated GPIO signals on {name}"
        peripheral["signals"] = [{"name": s, "pins": sigs[s]} for s in sorted(sigs, key=signal_sort_key)]
        added += sum(len(v) for v in sigs.values())

        # `signals` must follow `block` to match the ordering elsewhere in the file.
        order = ["name", "address", "block", "signals", "gate", "dma_muxing"]
        for key in order:
            if key in peripheral:
                peripheral[key] = peripheral.pop(key)

    with open(METADATA, "w", encoding="utf-8") as f:
        f.write(json.dumps(data, indent=2) + "\n")

    print(f"{len(pairs)} pin/ALT pairs parsed ({EXPECTED_NON_GPIO} non-GPIO)")
    print(f"{added} entries written across {len(signals)} peripherals")
    for reason, count in sorted(skipped.items(), key=lambda kv: -kv[1]):
        print(f"  skipped {count:3}: {reason}")


if __name__ == "__main__":
    main()
