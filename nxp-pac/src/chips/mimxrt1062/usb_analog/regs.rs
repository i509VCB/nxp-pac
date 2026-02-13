#[doc = "Chip Silicon Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Digprog(pub u32);
impl Digprog {
    #[doc = "Chip silicon revision"]
    #[must_use]
    #[inline(always)]
    pub const fn silicon_revision(&self) -> super::vals::SiliconRevision {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SiliconRevision::from_bits(val as u32)
    }
    #[doc = "Chip silicon revision"]
    #[inline(always)]
    pub const fn set_silicon_revision(&mut self, val: super::vals::SiliconRevision) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Digprog {
    #[inline(always)]
    fn default() -> Digprog {
        Digprog(0)
    }
}
impl core::fmt::Debug for Digprog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Digprog")
            .field("silicon_revision", &self.silicon_revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Digprog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Digprog {{ silicon_revision: {:?} }}",
            self.silicon_revision()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetect(pub u32);
impl Usb1ChrgDetect {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb1ChrgDetectChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb1ChrgDetectChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb1ChrgDetectChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb1ChrgDetectEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1ChrgDetectEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb1ChrgDetectEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1ChrgDetect {
    #[inline(always)]
    fn default() -> Usb1ChrgDetect {
        Usb1ChrgDetect(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetect")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetect {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectClr(pub u32);
impl Usb1ChrgDetectClr {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb1ChrgDetectClrChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb1ChrgDetectClrChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb1ChrgDetectClrChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb1ChrgDetectClrEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1ChrgDetectClrEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb1ChrgDetectClrEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1ChrgDetectClr {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectClr {
        Usb1ChrgDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectClr")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectClr {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectSet(pub u32);
impl Usb1ChrgDetectSet {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb1ChrgDetectSetChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb1ChrgDetectSetChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb1ChrgDetectSetChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb1ChrgDetectSetEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1ChrgDetectSetEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb1ChrgDetectSetEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1ChrgDetectSet {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectSet {
        Usb1ChrgDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectSet")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectSet {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectStat(pub u32);
impl Usb1ChrgDetectStat {
    #[doc = "State of the USB plug contact detector."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> super::vals::Usb1ChrgDetectStatPlugContact {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usb1ChrgDetectStatPlugContact::from_bits(val as u8)
    }
    #[doc = "State of the USB plug contact detector."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: super::vals::Usb1ChrgDetectStatPlugContact) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> super::vals::Usb1ChrgDetectStatChrgDetected {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Usb1ChrgDetectStatChrgDetected::from_bits(val as u8)
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: super::vals::Usb1ChrgDetectStatChrgDetected) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DM line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb1ChrgDetectStat {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectStat {
        Usb1ChrgDetectStat(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectStat")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectStat {{ plug_contact: {:?}, chrg_detected: {:?}, dm_state: {=bool:?}, dp_state: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectTog(pub u32);
impl Usb1ChrgDetectTog {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb1ChrgDetectTogChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb1ChrgDetectTogChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb1ChrgDetectTogChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb1ChrgDetectTogEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1ChrgDetectTogEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb1ChrgDetectTogEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1ChrgDetectTog {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectTog {
        Usb1ChrgDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectTog")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectTog {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1Loopback(pub u32);
impl Usb1Loopback {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1Loopback {
    #[inline(always)]
    fn default() -> Usb1Loopback {
        Usb1Loopback(0)
    }
}
impl core::fmt::Debug for Usb1Loopback {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1Loopback")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1Loopback {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1Loopback {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackClr(pub u32);
impl Usb1LoopbackClr {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1LoopbackClr {
    #[inline(always)]
    fn default() -> Usb1LoopbackClr {
        Usb1LoopbackClr(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackClr")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackClr {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackSet(pub u32);
impl Usb1LoopbackSet {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1LoopbackSet {
    #[inline(always)]
    fn default() -> Usb1LoopbackSet {
        Usb1LoopbackSet(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackSet")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackSet {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackTog(pub u32);
impl Usb1LoopbackTog {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1LoopbackTog {
    #[inline(always)]
    fn default() -> Usb1LoopbackTog {
        Usb1LoopbackTog(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackTog")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackTog {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1Misc(pub u32);
impl Usb1Misc {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1Misc {
    #[inline(always)]
    fn default() -> Usb1Misc {
        Usb1Misc(0)
    }
}
impl core::fmt::Debug for Usb1Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1Misc")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1Misc {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1MiscClr(pub u32);
impl Usb1MiscClr {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1MiscClr {
    #[inline(always)]
    fn default() -> Usb1MiscClr {
        Usb1MiscClr(0)
    }
}
impl core::fmt::Debug for Usb1MiscClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1MiscClr")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1MiscClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1MiscClr {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1MiscSet(pub u32);
impl Usb1MiscSet {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1MiscSet {
    #[inline(always)]
    fn default() -> Usb1MiscSet {
        Usb1MiscSet(0)
    }
}
impl core::fmt::Debug for Usb1MiscSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1MiscSet")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1MiscSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1MiscSet {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1MiscTog(pub u32);
impl Usb1MiscTog {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1MiscTog {
    #[inline(always)]
    fn default() -> Usb1MiscTog {
        Usb1MiscTog(0)
    }
}
impl core::fmt::Debug for Usb1MiscTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1MiscTog")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1MiscTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1MiscTog {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetect(pub u32);
impl Usb1VbusDetect {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: super::vals::Usb1VbusDetectVbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1VbusDetect {
    #[inline(always)]
    fn default() -> Usb1VbusDetect {
        Usb1VbusDetect(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetect")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetect {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectClr(pub u32);
impl Usb1VbusDetectClr {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectClrVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectClrVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1VbusDetectClrVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1VbusDetectClr {
    #[inline(always)]
    fn default() -> Usb1VbusDetectClr {
        Usb1VbusDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectClr")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectClr {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectSet(pub u32);
impl Usb1VbusDetectSet {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectSetVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectSetVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1VbusDetectSetVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1VbusDetectSet {
    #[inline(always)]
    fn default() -> Usb1VbusDetectSet {
        Usb1VbusDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectSet")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectSet {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB VBUS Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectStat(pub u32);
impl Usb1VbusDetectStat {
    #[doc = "Session End for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End for USB OTG"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBus valid for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBus valid for USB OTG"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb1VbusDetectStat {
    #[inline(always)]
    fn default() -> Usb1VbusDetectStat {
        Usb1VbusDetectStat(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectStat")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectStat {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectTog(pub u32);
impl Usb1VbusDetectTog {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectTogVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectTogVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1VbusDetectTogVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1VbusDetectTog {
    #[inline(always)]
    fn default() -> Usb1VbusDetectTog {
        Usb1VbusDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectTog")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectTog {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2ChrgDetect(pub u32);
impl Usb2ChrgDetect {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb2ChrgDetectChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb2ChrgDetectChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb2ChrgDetectChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb2ChrgDetectEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb2ChrgDetectEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb2ChrgDetectEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2ChrgDetect {
    #[inline(always)]
    fn default() -> Usb2ChrgDetect {
        Usb2ChrgDetect(0)
    }
}
impl core::fmt::Debug for Usb2ChrgDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2ChrgDetect")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2ChrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2ChrgDetect {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2ChrgDetectClr(pub u32);
impl Usb2ChrgDetectClr {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb2ChrgDetectClrChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb2ChrgDetectClrChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb2ChrgDetectClrChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb2ChrgDetectClrEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb2ChrgDetectClrEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb2ChrgDetectClrEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2ChrgDetectClr {
    #[inline(always)]
    fn default() -> Usb2ChrgDetectClr {
        Usb2ChrgDetectClr(0)
    }
}
impl core::fmt::Debug for Usb2ChrgDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2ChrgDetectClr")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2ChrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2ChrgDetectClr {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2ChrgDetectSet(pub u32);
impl Usb2ChrgDetectSet {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb2ChrgDetectSetChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb2ChrgDetectSetChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb2ChrgDetectSetChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb2ChrgDetectSetEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb2ChrgDetectSetEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb2ChrgDetectSetEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2ChrgDetectSet {
    #[inline(always)]
    fn default() -> Usb2ChrgDetectSet {
        Usb2ChrgDetectSet(0)
    }
}
impl core::fmt::Debug for Usb2ChrgDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2ChrgDetectSet")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2ChrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2ChrgDetectSet {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2ChrgDetectStat(pub u32);
impl Usb2ChrgDetectStat {
    #[doc = "State of the USB plug contact detector."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> super::vals::Usb2ChrgDetectStatPlugContact {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usb2ChrgDetectStatPlugContact::from_bits(val as u8)
    }
    #[doc = "State of the USB plug contact detector."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: super::vals::Usb2ChrgDetectStatPlugContact) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> super::vals::Usb2ChrgDetectStatChrgDetected {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Usb2ChrgDetectStatChrgDetected::from_bits(val as u8)
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: super::vals::Usb2ChrgDetectStatChrgDetected) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DM line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb2ChrgDetectStat {
    #[inline(always)]
    fn default() -> Usb2ChrgDetectStat {
        Usb2ChrgDetectStat(0)
    }
}
impl core::fmt::Debug for Usb2ChrgDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2ChrgDetectStat")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2ChrgDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2ChrgDetectStat {{ plug_contact: {:?}, chrg_detected: {:?}, dm_state: {=bool:?}, dp_state: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2ChrgDetectTog(pub u32);
impl Usb2ChrgDetectTog {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::Usb2ChrgDetectTogChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Usb2ChrgDetectTogChkChrgB::from_bits(val as u8)
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::Usb2ChrgDetectTogChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::Usb2ChrgDetectTogEnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb2ChrgDetectTogEnB::from_bits(val as u8)
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::Usb2ChrgDetectTogEnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2ChrgDetectTog {
    #[inline(always)]
    fn default() -> Usb2ChrgDetectTog {
        Usb2ChrgDetectTog(0)
    }
}
impl core::fmt::Debug for Usb2ChrgDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2ChrgDetectTog")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2ChrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2ChrgDetectTog {{ chk_contact: {=bool:?}, chk_chrg_b: {:?}, en_b: {:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2Loopback(pub u32);
impl Usb2Loopback {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2Loopback {
    #[inline(always)]
    fn default() -> Usb2Loopback {
        Usb2Loopback(0)
    }
}
impl core::fmt::Debug for Usb2Loopback {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2Loopback")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2Loopback {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2Loopback {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2LoopbackClr(pub u32);
impl Usb2LoopbackClr {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2LoopbackClr {
    #[inline(always)]
    fn default() -> Usb2LoopbackClr {
        Usb2LoopbackClr(0)
    }
}
impl core::fmt::Debug for Usb2LoopbackClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2LoopbackClr")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2LoopbackClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2LoopbackClr {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2LoopbackSet(pub u32);
impl Usb2LoopbackSet {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2LoopbackSet {
    #[inline(always)]
    fn default() -> Usb2LoopbackSet {
        Usb2LoopbackSet(0)
    }
}
impl core::fmt::Debug for Usb2LoopbackSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2LoopbackSet")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2LoopbackSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2LoopbackSet {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2LoopbackTog(pub u32);
impl Usb2LoopbackTog {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2LoopbackTog {
    #[inline(always)]
    fn default() -> Usb2LoopbackTog {
        Usb2LoopbackTog(0)
    }
}
impl core::fmt::Debug for Usb2LoopbackTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2LoopbackTog")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2LoopbackTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2LoopbackTog {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2Misc(pub u32);
impl Usb2Misc {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2Misc {
    #[inline(always)]
    fn default() -> Usb2Misc {
        Usb2Misc(0)
    }
}
impl core::fmt::Debug for Usb2Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2Misc")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2Misc {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2MiscClr(pub u32);
impl Usb2MiscClr {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2MiscClr {
    #[inline(always)]
    fn default() -> Usb2MiscClr {
        Usb2MiscClr(0)
    }
}
impl core::fmt::Debug for Usb2MiscClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2MiscClr")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2MiscClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2MiscClr {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2MiscSet(pub u32);
impl Usb2MiscSet {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2MiscSet {
    #[inline(always)]
    fn default() -> Usb2MiscSet {
        Usb2MiscSet(0)
    }
}
impl core::fmt::Debug for Usb2MiscSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2MiscSet")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2MiscSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2MiscSet {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2MiscTog(pub u32);
impl Usb2MiscTog {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2MiscTog {
    #[inline(always)]
    fn default() -> Usb2MiscTog {
        Usb2MiscTog(0)
    }
}
impl core::fmt::Debug for Usb2MiscTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2MiscTog")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2MiscTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2MiscTog {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}",
            self.hs_use_external_r(),
            self.en_deglitch(),
            self.en_clk_utmi()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2VbusDetect(pub u32);
impl Usb2VbusDetect {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2VbusDetectVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2VbusDetectVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: super::vals::Usb2VbusDetectVbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2VbusDetect {
    #[inline(always)]
    fn default() -> Usb2VbusDetect {
        Usb2VbusDetect(0)
    }
}
impl core::fmt::Debug for Usb2VbusDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2VbusDetect")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2VbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2VbusDetect {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2VbusDetectClr(pub u32);
impl Usb2VbusDetectClr {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2VbusDetectClrVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2VbusDetectClrVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb2VbusDetectClrVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2VbusDetectClr {
    #[inline(always)]
    fn default() -> Usb2VbusDetectClr {
        Usb2VbusDetectClr(0)
    }
}
impl core::fmt::Debug for Usb2VbusDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2VbusDetectClr")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2VbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2VbusDetectClr {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2VbusDetectSet(pub u32);
impl Usb2VbusDetectSet {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2VbusDetectSetVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2VbusDetectSetVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb2VbusDetectSetVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2VbusDetectSet {
    #[inline(always)]
    fn default() -> Usb2VbusDetectSet {
        Usb2VbusDetectSet(0)
    }
}
impl core::fmt::Debug for Usb2VbusDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2VbusDetectSet")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2VbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2VbusDetectSet {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
#[doc = "USB VBUS Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2VbusDetectStat(pub u32);
impl Usb2VbusDetectStat {
    #[doc = "Session End for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End for USB OTG"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBus valid for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBus valid for USB OTG"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb2VbusDetectStat {
    #[inline(always)]
    fn default() -> Usb2VbusDetectStat {
        Usb2VbusDetectStat(0)
    }
}
impl core::fmt::Debug for Usb2VbusDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2VbusDetectStat")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2VbusDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2VbusDetectStat {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid()
        )
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2VbusDetectTog(pub u32);
impl Usb2VbusDetectTog {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2VbusDetectTogVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2VbusDetectTogVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb2VbusDetectTogVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2VbusDetectTog {
    #[inline(always)]
    fn default() -> Usb2VbusDetectTog {
        Usb2VbusDetectTog(0)
    }
}
impl core::fmt::Debug for Usb2VbusDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2VbusDetectTog")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2VbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2VbusDetectTog {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus(),
            self.charge_vbus()
        )
    }
}
