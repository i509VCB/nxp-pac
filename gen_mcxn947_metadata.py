#!/usr/bin/env python3
"""One-off script: generate data/metadata/MCXN947.json from the core0 SVD.

Names/addresses/interrupts, plus GPIOn/PORTn pin data. No *other* pin/signal
muxing data (e.g. which package pin carries LPUART2_TX) - that needs real NXP
Pins Tool / reference manual data we don't have. GPIOn<->PORTn pin naming
(P<port>_<pin>) doesn't need that: it's a fixed 1:1 relationship (PORTn pin N
is always GPIOn pin N), and which bit positions actually exist per port
*is* in the SVD (as the set of PCR<N> registers each PORTn peripheral has -
ports aren't uniformly 32 pins wide, e.g. PORT1 has 0-23 then jumps to 30-31).

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
PORT_RE = re.compile(r"^PORT(\d+)$")
GPIO_RE = re.compile(r"^GPIO(\d+)$")
PCR_RE = re.compile(r"^PCR(\d+)$")

tree = ET.parse(SVD)
root = tree.getroot()

nvic_prio_bits = int(root.find("cpu/nvicPrioBits").text)

interrupts = {}
peripherals = []

# port number -> sorted list of pin bit indices that PORTn actually has a
# PCR register for (i.e. actually exist on this part, gaps and all).
port_pins = {}

for periph in root.find("peripherals").findall("peripheral"):
    name = periph.find("name").text

    port_match = PORT_RE.match(name)
    if port_match is not None:
        registers = periph.find("registers")
        if registers is not None:
            pins = sorted(
                int(m.group(1))
                for reg in registers.findall("register")
                if (m := PCR_RE.match(reg.find("name").text))
            )
            port_pins[int(port_match.group(1))] = pins

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

# Attach GPIOn <-> PORTn pin signals (skips GPIOn_ALIAS1, which isn't matched
# by GPIO_RE's `$`-anchored pattern - the alias shouldn't also claim the same
# P<port>_<pin> singletons as the canonical GPIOn).
pins = []
for peripheral in peripherals:
    gpio_match = GPIO_RE.match(peripheral["name"])
    if gpio_match is None:
        continue
    port_num = int(gpio_match.group(1))
    for pin_num in port_pins.get(port_num, []):
        pin_name = f"P{port_num}_{pin_num}"
        pins.append({"name": pin_name})
        peripheral["signals"].append({"name": str(pin_num), "pins": [{"pin": pin_name, "alt": 0}]})

metadata = {
    "$schema": "./schema.json",
    "$comment": "MCXN947",
    "chips": ["MCXN947"],
    "nvic_prio_bits": nvic_prio_bits,
    "interrupts": interrupts,
    "pins": pins,
    "peripherals": peripherals,
}

OUT.write_text(json.dumps(metadata, indent=2) + "\n")
print(f"wrote {OUT} ({len(peripherals)} peripherals, {len(interrupts)} interrupts, nvic_prio_bits={nvic_prio_bits})")
