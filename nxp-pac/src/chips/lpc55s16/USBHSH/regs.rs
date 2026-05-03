#[doc = "Memory base address where ATL PTD0 is stored."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATLPTD(pub u32);
impl ATLPTD {
    #[doc = "This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_CUR(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub const fn set_ATL_CUR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Base address to be used by the hardware to find the start of the ATL list."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_BASE(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub const fn set_ATL_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for ATLPTD {
    #[inline(always)]
    fn default() -> ATLPTD {
        ATLPTD(0)
    }
}
impl core::fmt::Debug for ATLPTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATLPTD")
            .field("ATL_CUR", &self.ATL_CUR())
            .field("ATL_BASE", &self.ATL_BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ATLPTD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ATLPTD {{ ATL_CUR: {=u8:?}, ATL_BASE: {=u32:?} }}",
            self.ATL_CUR(),
            self.ATL_BASE()
        )
    }
}
#[doc = "Done map for each ATL PTD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATLPTDD(pub u32);
impl ATLPTDD {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_DONE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_ATL_DONE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ATLPTDD {
    #[inline(always)]
    fn default() -> ATLPTDD {
        ATLPTDD(0)
    }
}
impl core::fmt::Debug for ATLPTDD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATLPTDD")
            .field("ATL_DONE", &self.ATL_DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ATLPTDD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ATLPTDD {{ ATL_DONE: {=u32:?} }}", self.ATL_DONE())
    }
}
#[doc = "Skip map for each ATL PTD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATLPTDS(pub u32);
impl ATLPTDS {
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_SKIP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub const fn set_ATL_SKIP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ATLPTDS {
    #[inline(always)]
    fn default() -> ATLPTDS {
        ATLPTDS(0)
    }
}
impl core::fmt::Debug for ATLPTDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATLPTDS")
            .field("ATL_SKIP", &self.ATL_SKIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ATLPTDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ATLPTDS {{ ATL_SKIP: {=u32:?} }}", self.ATL_SKIP())
    }
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPLENGTH_CHIPID(pub u32);
impl CAPLENGTH_CHIPID {
    #[doc = "Capability Length: This is used as an offset."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPLENGTH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Capability Length: This is used as an offset."]
    #[inline(always)]
    pub const fn set_CAPLENGTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Chip identification: indicates major and minor revision of the IP: \\[31:24\\] = Major revision \\[23:16\\] = Minor revision Major revisions used: 0x01: USB2."]
    #[must_use]
    #[inline(always)]
    pub const fn CHIPID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Chip identification: indicates major and minor revision of the IP: \\[31:24\\] = Major revision \\[23:16\\] = Minor revision Major revisions used: 0x01: USB2."]
    #[inline(always)]
    pub const fn set_CHIPID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPLENGTH_CHIPID {
    #[inline(always)]
    fn default() -> CAPLENGTH_CHIPID {
        CAPLENGTH_CHIPID(0)
    }
}
impl core::fmt::Debug for CAPLENGTH_CHIPID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPLENGTH_CHIPID")
            .field("CAPLENGTH", &self.CAPLENGTH())
            .field("CHIPID", &self.CHIPID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPLENGTH_CHIPID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPLENGTH_CHIPID {{ CAPLENGTH: {=u8:?}, CHIPID: {=u16:?} }}",
            self.CAPLENGTH(),
            self.CHIPID()
        )
    }
}
#[doc = "Memory base address that indicates the start of the data payload buffers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATAPAYLOAD(pub u32);
impl DATAPAYLOAD {
    #[doc = "Base address to be used by the hardware to find the start of the data payload section."]
    #[must_use]
    #[inline(always)]
    pub const fn DAT_BASE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub const fn set_DAT_BASE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DATAPAYLOAD {
    #[inline(always)]
    fn default() -> DATAPAYLOAD {
        DATAPAYLOAD(0)
    }
}
impl core::fmt::Debug for DATAPAYLOAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAPAYLOAD")
            .field("DAT_BASE", &self.DAT_BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATAPAYLOAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DATAPAYLOAD {{ DAT_BASE: {=u16:?} }}", self.DAT_BASE())
    }
}
#[doc = "Frame Length Adjustment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLADJ_FRINDEX(pub u32);
impl FLADJ_FRINDEX {
    #[doc = "Frame Length Timing Value."]
    #[must_use]
    #[inline(always)]
    pub const fn FLADJ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Frame Length Timing Value."]
    #[inline(always)]
    pub const fn set_FLADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[must_use]
    #[inline(always)]
    pub const fn FRINDEX(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub const fn set_FRINDEX(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for FLADJ_FRINDEX {
    #[inline(always)]
    fn default() -> FLADJ_FRINDEX {
        FLADJ_FRINDEX(0)
    }
}
impl core::fmt::Debug for FLADJ_FRINDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLADJ_FRINDEX")
            .field("FLADJ", &self.FLADJ())
            .field("FRINDEX", &self.FRINDEX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLADJ_FRINDEX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLADJ_FRINDEX {{ FLADJ: {=u8:?}, FRINDEX: {=u16:?} }}",
            self.FLADJ(),
            self.FRINDEX()
        )
    }
}
#[doc = "Host Controller Capability Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCCPARAMS(pub u32);
impl HCCPARAMS {
    #[doc = "Link Power Management Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn LPMC(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Link Power Management Capability."]
    #[inline(always)]
    pub const fn set_LPMC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for HCCPARAMS {
    #[inline(always)]
    fn default() -> HCCPARAMS {
        HCCPARAMS(0)
    }
}
impl core::fmt::Debug for HCCPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCPARAMS")
            .field("LPMC", &self.LPMC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCCPARAMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCCPARAMS {{ LPMC: {=bool:?} }}", self.LPMC())
    }
}
#[doc = "Host Controller Structural Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCSPARAMS(pub u32);
impl HCSPARAMS {
    #[doc = "This register specifies the number of physical downstream ports implemented on this host controller."]
    #[must_use]
    #[inline(always)]
    pub const fn N_PORTS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "This register specifies the number of physical downstream ports implemented on this host controller."]
    #[inline(always)]
    pub const fn set_N_PORTS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This field indicates whether the host controller implementation includes port power control."]
    #[must_use]
    #[inline(always)]
    pub const fn PPC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub const fn set_PPC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates whether the ports support port indicator control."]
    #[must_use]
    #[inline(always)]
    pub const fn P_INDICATOR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates whether the ports support port indicator control."]
    #[inline(always)]
    pub const fn set_P_INDICATOR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for HCSPARAMS {
    #[inline(always)]
    fn default() -> HCSPARAMS {
        HCSPARAMS(0)
    }
}
impl core::fmt::Debug for HCSPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPARAMS")
            .field("N_PORTS", &self.N_PORTS())
            .field("PPC", &self.PPC())
            .field("P_INDICATOR", &self.P_INDICATOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCSPARAMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCSPARAMS {{ N_PORTS: {=u8:?}, PPC: {=bool:?}, P_INDICATOR: {=bool:?} }}",
            self.N_PORTS(),
            self.PPC(),
            self.P_INDICATOR()
        )
    }
}
#[doc = "Memory base address where INT PTD0 is stored."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTPTD(pub u32);
impl INTPTD {
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_FIRST(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub const fn set_INT_FIRST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Base address to be used by the hardware to find the start of the INT list."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_BASE(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub const fn set_INT_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for INTPTD {
    #[inline(always)]
    fn default() -> INTPTD {
        INTPTD(0)
    }
}
impl core::fmt::Debug for INTPTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPTD")
            .field("INT_FIRST", &self.INT_FIRST())
            .field("INT_BASE", &self.INT_BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTPTD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTPTD {{ INT_FIRST: {=u8:?}, INT_BASE: {=u32:?} }}",
            self.INT_FIRST(),
            self.INT_BASE()
        )
    }
}
#[doc = "Done map for each INT PTD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTPTDD(pub u32);
impl INTPTDD {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_DONE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_INT_DONE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INTPTDD {
    #[inline(always)]
    fn default() -> INTPTDD {
        INTPTDD(0)
    }
}
impl core::fmt::Debug for INTPTDD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPTDD")
            .field("INT_DONE", &self.INT_DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTPTDD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTPTDD {{ INT_DONE: {=u32:?} }}", self.INT_DONE())
    }
}
#[doc = "Skip map for each INT PTD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTPTDS(pub u32);
impl INTPTDS {
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_SKIP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub const fn set_INT_SKIP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INTPTDS {
    #[inline(always)]
    fn default() -> INTPTDS {
        INTPTDS(0)
    }
}
impl core::fmt::Debug for INTPTDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPTDS")
            .field("INT_SKIP", &self.INT_SKIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTPTDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTPTDS {{ INT_SKIP: {=u32:?} }}", self.INT_SKIP())
    }
}
#[doc = "Memory base address where ISO PTD0 is stored."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISOPTD(pub u32);
impl ISOPTD {
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_FIRST(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub const fn set_ISO_FIRST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Base address to be used by the hardware to find the start of the ISO list."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_BASE(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub const fn set_ISO_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for ISOPTD {
    #[inline(always)]
    fn default() -> ISOPTD {
        ISOPTD(0)
    }
}
impl core::fmt::Debug for ISOPTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISOPTD")
            .field("ISO_FIRST", &self.ISO_FIRST())
            .field("ISO_BASE", &self.ISO_BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISOPTD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ISOPTD {{ ISO_FIRST: {=u8:?}, ISO_BASE: {=u32:?} }}",
            self.ISO_FIRST(),
            self.ISO_BASE()
        )
    }
}
#[doc = "Done map for each ISO PTD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISOPTDD(pub u32);
impl ISOPTDD {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_DONE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_ISO_DONE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ISOPTDD {
    #[inline(always)]
    fn default() -> ISOPTDD {
        ISOPTDD(0)
    }
}
impl core::fmt::Debug for ISOPTDD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISOPTDD")
            .field("ISO_DONE", &self.ISO_DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISOPTDD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ISOPTDD {{ ISO_DONE: {=u32:?} }}", self.ISO_DONE())
    }
}
#[doc = "Skip map for each ISO PTD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISOPTDS(pub u32);
impl ISOPTDS {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_SKIP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_ISO_SKIP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ISOPTDS {
    #[inline(always)]
    fn default() -> ISOPTDS {
        ISOPTDS(0)
    }
}
impl core::fmt::Debug for ISOPTDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISOPTDS")
            .field("ISO_SKIP", &self.ISO_SKIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISOPTDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ISOPTDS {{ ISO_SKIP: {=u32:?} }}", self.ISO_SKIP())
    }
}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LASTPTD(pub u32);
impl LASTPTD {
    #[doc = "If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_LAST(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub const fn set_ATL_LAST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "This indicates the last PTD in the ISO list."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_LAST(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub const fn set_ISO_LAST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "This indicates the last PTD in the INT list."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_LAST(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub const fn set_INT_LAST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for LASTPTD {
    #[inline(always)]
    fn default() -> LASTPTD {
        LASTPTD(0)
    }
}
impl core::fmt::Debug for LASTPTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LASTPTD")
            .field("ATL_LAST", &self.ATL_LAST())
            .field("ISO_LAST", &self.ISO_LAST())
            .field("INT_LAST", &self.INT_LAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LASTPTD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LASTPTD {{ ATL_LAST: {=u8:?}, ISO_LAST: {=u8:?}, INT_LAST: {=u8:?} }}",
            self.ATL_LAST(),
            self.ISO_LAST(),
            self.INT_LAST()
        )
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORTMODE(pub u32);
impl PORTMODE {
    #[doc = "If this bit is set to one, one of the ports will behave as a USB device."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_ENABLE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub const fn set_DEV_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[must_use]
    #[inline(always)]
    pub const fn SW_CTRL_PDCOM(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub const fn set_SW_CTRL_PDCOM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[must_use]
    #[inline(always)]
    pub const fn SW_PDCOM(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub const fn set_SW_PDCOM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for PORTMODE {
    #[inline(always)]
    fn default() -> PORTMODE {
        PORTMODE(0)
    }
}
impl core::fmt::Debug for PORTMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTMODE")
            .field("DEV_ENABLE", &self.DEV_ENABLE())
            .field("SW_CTRL_PDCOM", &self.SW_CTRL_PDCOM())
            .field("SW_PDCOM", &self.SW_PDCOM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PORTMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PORTMODE {{ DEV_ENABLE: {=bool:?}, SW_CTRL_PDCOM: {=bool:?}, SW_PDCOM: {=bool:?} }}",
            self.DEV_ENABLE(),
            self.SW_CTRL_PDCOM(),
            self.SW_PDCOM()
        )
    }
}
#[doc = "Port Status and Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORTSC1(pub u32);
impl PORTSC1 {
    #[doc = "Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[must_use]
    #[inline(always)]
    pub const fn CCS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub const fn set_CCS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn CSC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub const fn set_CSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enabled/Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PED(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled."]
    #[inline(always)]
    pub const fn set_PED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn PEDC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub const fn set_PEDC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Over-current active: Logic 1 means that this port has an over-current condition."]
    #[must_use]
    #[inline(always)]
    pub const fn OCA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub const fn set_OCA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Over-current change: Logic 1 means that the value of OCA has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn OCC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub const fn set_OCC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[must_use]
    #[inline(always)]
    pub const fn FPR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub const fn set_FPR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend: Logic 1 means port is in the suspend state."]
    #[must_use]
    #[inline(always)]
    pub const fn SUSP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub const fn set_SUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset: Logic 1 means the port is in the reset state."]
    #[must_use]
    #[inline(always)]
    pub const fn PR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub const fn set_PR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS_L1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
    #[inline(always)]
    pub const fn set_SUS_L1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[must_use]
    #[inline(always)]
    pub const fn LS(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[inline(always)]
    pub const fn set_LS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[must_use]
    #[inline(always)]
    pub const fn PP(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub const fn set_PP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[must_use]
    #[inline(always)]
    pub const fn PIC(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub const fn set_PIC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[must_use]
    #[inline(always)]
    pub const fn PTC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub const fn set_PTC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn PSPD(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub const fn set_PSPD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn WOO(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub const fn set_WOO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS_STAT(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
    #[inline(always)]
    pub const fn set_SUS_STAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[doc = "Device Address for LPM tokens."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_ADD(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address for LPM tokens."]
    #[inline(always)]
    pub const fn set_DEV_ADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for PORTSC1 {
    #[inline(always)]
    fn default() -> PORTSC1 {
        PORTSC1(0)
    }
}
impl core::fmt::Debug for PORTSC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTSC1")
            .field("CCS", &self.CCS())
            .field("CSC", &self.CSC())
            .field("PED", &self.PED())
            .field("PEDC", &self.PEDC())
            .field("OCA", &self.OCA())
            .field("OCC", &self.OCC())
            .field("FPR", &self.FPR())
            .field("SUSP", &self.SUSP())
            .field("PR", &self.PR())
            .field("SUS_L1", &self.SUS_L1())
            .field("LS", &self.LS())
            .field("PP", &self.PP())
            .field("PIC", &self.PIC())
            .field("PTC", &self.PTC())
            .field("PSPD", &self.PSPD())
            .field("WOO", &self.WOO())
            .field("SUS_STAT", &self.SUS_STAT())
            .field("DEV_ADD", &self.DEV_ADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PORTSC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PORTSC1 {{ CCS: {=bool:?}, CSC: {=bool:?}, PED: {=bool:?}, PEDC: {=bool:?}, OCA: {=bool:?}, OCC: {=bool:?}, FPR: {=bool:?}, SUSP: {=bool:?}, PR: {=bool:?}, SUS_L1: {=bool:?}, LS: {=u8:?}, PP: {=bool:?}, PIC: {=u8:?}, PTC: {=u8:?}, PSPD: {=u8:?}, WOO: {=bool:?}, SUS_STAT: {=u8:?}, DEV_ADD: {=u8:?} }}",
            self.CCS(),
            self.CSC(),
            self.PED(),
            self.PEDC(),
            self.OCA(),
            self.OCC(),
            self.FPR(),
            self.SUSP(),
            self.PR(),
            self.SUS_L1(),
            self.LS(),
            self.PP(),
            self.PIC(),
            self.PTC(),
            self.PSPD(),
            self.WOO(),
            self.SUS_STAT(),
            self.DEV_ADD()
        )
    }
}
#[doc = "USB Command register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBCMD(pub u32);
impl USBCMD {
    #[doc = "Run/Stop: 1b = Run."]
    #[must_use]
    #[inline(always)]
    pub const fn RS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Run/Stop: 1b = Run."]
    #[inline(always)]
    pub const fn set_RS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[must_use]
    #[inline(always)]
    pub const fn HCRESET(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub const fn set_HCRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame List Size: This field specifies the size of the frame list."]
    #[must_use]
    #[inline(always)]
    pub const fn FLS(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub const fn set_FLS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[must_use]
    #[inline(always)]
    pub const fn LHCR(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub const fn set_LHCR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ATL List enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ATL List enabled."]
    #[inline(always)]
    pub const fn set_ATL_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ISO List enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_EN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ISO List enabled."]
    #[inline(always)]
    pub const fn set_ISO_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "INT List enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_EN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "INT List enabled."]
    #[inline(always)]
    pub const fn set_INT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Host-Initiated Resume Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn HIRD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Host-Initiated Resume Duration."]
    #[inline(always)]
    pub const fn set_HIRD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "bRemoteWake field."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_RWU(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "bRemoteWake field."]
    #[inline(always)]
    pub const fn set_LPM_RWU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for USBCMD {
    #[inline(always)]
    fn default() -> USBCMD {
        USBCMD(0)
    }
}
impl core::fmt::Debug for USBCMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCMD")
            .field("RS", &self.RS())
            .field("HCRESET", &self.HCRESET())
            .field("FLS", &self.FLS())
            .field("LHCR", &self.LHCR())
            .field("ATL_EN", &self.ATL_EN())
            .field("ISO_EN", &self.ISO_EN())
            .field("INT_EN", &self.INT_EN())
            .field("HIRD", &self.HIRD())
            .field("LPM_RWU", &self.LPM_RWU())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBCMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBCMD {{ RS: {=bool:?}, HCRESET: {=bool:?}, FLS: {=u8:?}, LHCR: {=bool:?}, ATL_EN: {=bool:?}, ISO_EN: {=bool:?}, INT_EN: {=bool:?}, HIRD: {=u8:?}, LPM_RWU: {=bool:?} }}",
            self.RS(),
            self.HCRESET(),
            self.FLS(),
            self.LHCR(),
            self.ATL_EN(),
            self.ISO_EN(),
            self.INT_EN(),
            self.HIRD(),
            self.LPM_RWU()
        )
    }
}
#[doc = "USB Interrupt Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBINTR(pub u32);
impl USBINTR {
    #[doc = "Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn PCDE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_PCDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn FLRE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_FLRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ATL IRQ Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_IRQ_E(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_ATL_IRQ_E(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ISO IRQ Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_IRQ_E(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_ISO_IRQ_E(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "INT IRQ Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_IRQ_E(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_INT_IRQ_E(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn SOF_E(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_SOF_E(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for USBINTR {
    #[inline(always)]
    fn default() -> USBINTR {
        USBINTR(0)
    }
}
impl core::fmt::Debug for USBINTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBINTR")
            .field("PCDE", &self.PCDE())
            .field("FLRE", &self.FLRE())
            .field("ATL_IRQ_E", &self.ATL_IRQ_E())
            .field("ISO_IRQ_E", &self.ISO_IRQ_E())
            .field("INT_IRQ_E", &self.INT_IRQ_E())
            .field("SOF_E", &self.SOF_E())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBINTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBINTR {{ PCDE: {=bool:?}, FLRE: {=bool:?}, ATL_IRQ_E: {=bool:?}, ISO_IRQ_E: {=bool:?}, INT_IRQ_E: {=bool:?}, SOF_E: {=bool:?} }}",
            self.PCDE(),
            self.FLRE(),
            self.ATL_IRQ_E(),
            self.ISO_IRQ_E(),
            self.INT_IRQ_E(),
            self.SOF_E()
        )
    }
}
#[doc = "USB Interrupt Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBSTS(pub u32);
impl USBSTS {
    #[doc = "Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[must_use]
    #[inline(always)]
    pub const fn PCD(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub const fn set_PCD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn FLR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub const fn set_FLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[must_use]
    #[inline(always)]
    pub const fn ATL_IRQ(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub const fn set_ATL_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[must_use]
    #[inline(always)]
    pub const fn ISO_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub const fn set_ISO_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_IRQ(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub const fn set_INT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn SOF_IRQ(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub const fn set_SOF_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for USBSTS {
    #[inline(always)]
    fn default() -> USBSTS {
        USBSTS(0)
    }
}
impl core::fmt::Debug for USBSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBSTS")
            .field("PCD", &self.PCD())
            .field("FLR", &self.FLR())
            .field("ATL_IRQ", &self.ATL_IRQ())
            .field("ISO_IRQ", &self.ISO_IRQ())
            .field("INT_IRQ", &self.INT_IRQ())
            .field("SOF_IRQ", &self.SOF_IRQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBSTS {{ PCD: {=bool:?}, FLR: {=bool:?}, ATL_IRQ: {=bool:?}, ISO_IRQ: {=bool:?}, INT_IRQ: {=bool:?}, SOF_IRQ: {=bool:?} }}",
            self.PCD(),
            self.FLR(),
            self.ATL_IRQ(),
            self.ISO_IRQ(),
            self.INT_IRQ(),
            self.SOF_IRQ()
        )
    }
}
