#!/usr/bin/env python3
"""One-off script: generate data/metadata/MCXN947.json from the core0 SVD.

Minimal pass — only names/addresses/interrupts, no pin/signal muxing data
(not present in the SVD; needs NXP Pins Tool / reference manual data later).

DMA0/DMA1 are deliberately excluded: embassy-mcxn's build.rs (copied from
embassy-mcxa) expects DMA peripherals to have a `block` string encoding a
channel count (e.g. "mcxa/DMA::DMA8") and `.unwrap()`s a regex match against
it. Without real transform data assigning that block/channel-count for
MCXN947, including DMA0/DMA1 here would make build.rs panic.
"""

import json
import re
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).parent
SVD = ROOT / "data/mcux-soc-svd/MCXN947/MCXN947_cm33_core0.xml"
OUT = ROOT / "data/metadata/MCXN947.json"

DMA_RE = re.compile(r"^DMA\d+$", re.IGNORECASE)

tree = ET.parse(SVD)
root = tree.getroot()

nvic_prio_bits = int(root.find("cpu/nvicPrioBits").text)

interrupts = {}
peripherals = []

for periph in root.find("peripherals").findall("peripheral"):
    name = periph.find("name").text

    base_address_el = periph.find("baseAddress")
    if base_address_el is not None:
        address = int(base_address_el.text, 16)
        if not DMA_RE.match(name):
            peripherals.append(
                {
                    "name": name,
                    "address": f"0x{address:08X}",
                    "signals": [],
                }
            )

    for interrupt in periph.findall("interrupt"):
        # The SVD has inconsistent casing for some interrupt names (e.g.
        # "Freqme" instead of "FREQME"), but nxp-pac's chiptool-generated
        # `Interrupt` enum uppercases them all. Match that here so
        # `METADATA.interrupts` names line up with `pac::Interrupt` variants.
        iname = interrupt.find("name").text.upper()
        ivalue = int(interrupt.find("value").text)
        if iname in interrupts and interrupts[iname] != ivalue:
            raise ValueError(f"conflicting values for interrupt {iname}: {interrupts[iname]} vs {ivalue}")
        interrupts[iname] = ivalue

metadata = {
    "$schema": "./schema.json",
    "$comment": "MCXN947",
    "chips": ["MCXN947"],
    "nvic_prio_bits": nvic_prio_bits,
    "interrupts": interrupts,
    "pins": [],
    "peripherals": peripherals,
}

OUT.write_text(json.dumps(metadata, indent=2) + "\n")
print(f"wrote {OUT} ({len(peripherals)} peripherals, {len(interrupts)} interrupts, nvic_prio_bits={nvic_prio_bits})")
