#[doc = "Memory base address where ATL PTD0 is stored"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atlptd(pub u32);
impl Atlptd {
    #[doc = "This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_cur(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub const fn set_atl_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Base address to be used by the hardware to find the start of the ATL list."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_base(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub const fn set_atl_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for Atlptd {
    #[inline(always)]
    fn default() -> Atlptd {
        Atlptd(0)
    }
}
impl core::fmt::Debug for Atlptd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atlptd")
            .field("atl_cur", &self.atl_cur())
            .field("atl_base", &self.atl_base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atlptd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Atlptd {{ atl_cur: {=u8:?}, atl_base: {=u32:?} }}",
            self.atl_cur(),
            self.atl_base()
        )
    }
}
#[doc = "Done map for each ATL PTD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atlptdd(pub u32);
impl Atlptdd {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_done(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_atl_done(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Atlptdd {
    #[inline(always)]
    fn default() -> Atlptdd {
        Atlptdd(0)
    }
}
impl core::fmt::Debug for Atlptdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atlptdd")
            .field("atl_done", &self.atl_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atlptdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atlptdd {{ atl_done: {=u32:?} }}", self.atl_done())
    }
}
#[doc = "Skip map for each ATL PTD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atlptds(pub u32);
impl Atlptds {
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_skip(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub const fn set_atl_skip(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Atlptds {
    #[inline(always)]
    fn default() -> Atlptds {
        Atlptds(0)
    }
}
impl core::fmt::Debug for Atlptds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atlptds")
            .field("atl_skip", &self.atl_skip())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atlptds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atlptds {{ atl_skip: {=u32:?} }}", self.atl_skip())
    }
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaplengthChipid(pub u32);
impl CaplengthChipid {
    #[doc = "Capability Length: This is used as an offset."]
    #[must_use]
    #[inline(always)]
    pub const fn caplength(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Capability Length: This is used as an offset."]
    #[inline(always)]
    pub const fn set_caplength(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Chip identification: indicates major and minor revision of the IP: \\[31:24\\] = Major revision \\[23:16\\] = Minor revision Major revisions used: 0x01: USB2."]
    #[must_use]
    #[inline(always)]
    pub const fn chipid(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Chip identification: indicates major and minor revision of the IP: \\[31:24\\] = Major revision \\[23:16\\] = Minor revision Major revisions used: 0x01: USB2."]
    #[inline(always)]
    pub const fn set_chipid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CaplengthChipid {
    #[inline(always)]
    fn default() -> CaplengthChipid {
        CaplengthChipid(0)
    }
}
impl core::fmt::Debug for CaplengthChipid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CaplengthChipid")
            .field("caplength", &self.caplength())
            .field("chipid", &self.chipid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CaplengthChipid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CaplengthChipid {{ caplength: {=u8:?}, chipid: {=u16:?} }}",
            self.caplength(),
            self.chipid()
        )
    }
}
#[doc = "Memory base address that indicates the start of the data payload buffers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datapayload(pub u32);
impl Datapayload {
    #[doc = "Base address to be used by the hardware to find the start of the data payload section."]
    #[must_use]
    #[inline(always)]
    pub const fn dat_base(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub const fn set_dat_base(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Datapayload {
    #[inline(always)]
    fn default() -> Datapayload {
        Datapayload(0)
    }
}
impl core::fmt::Debug for Datapayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Datapayload")
            .field("dat_base", &self.dat_base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Datapayload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Datapayload {{ dat_base: {=u16:?} }}", self.dat_base())
    }
}
#[doc = "Frame Length Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FladjFrindex(pub u32);
impl FladjFrindex {
    #[doc = "Frame Length Timing Value."]
    #[must_use]
    #[inline(always)]
    pub const fn fladj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Frame Length Timing Value."]
    #[inline(always)]
    pub const fn set_fladj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[must_use]
    #[inline(always)]
    pub const fn frindex(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub const fn set_frindex(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for FladjFrindex {
    #[inline(always)]
    fn default() -> FladjFrindex {
        FladjFrindex(0)
    }
}
impl core::fmt::Debug for FladjFrindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FladjFrindex")
            .field("fladj", &self.fladj())
            .field("frindex", &self.frindex())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FladjFrindex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FladjFrindex {{ fladj: {=u8:?}, frindex: {=u16:?} }}",
            self.fladj(),
            self.frindex()
        )
    }
}
#[doc = "Host Controller Structural Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcsparams(pub u32);
impl Hcsparams {
    #[doc = "This register specifies the number of physical downstream ports implemented on this host controller."]
    #[must_use]
    #[inline(always)]
    pub const fn n_ports(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "This register specifies the number of physical downstream ports implemented on this host controller."]
    #[inline(always)]
    pub const fn set_n_ports(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This field indicates whether the host controller implementation includes port power control."]
    #[must_use]
    #[inline(always)]
    pub const fn ppc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub const fn set_ppc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates whether the ports support port indicator control."]
    #[must_use]
    #[inline(always)]
    pub const fn p_indicator(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates whether the ports support port indicator control."]
    #[inline(always)]
    pub const fn set_p_indicator(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Hcsparams {
    #[inline(always)]
    fn default() -> Hcsparams {
        Hcsparams(0)
    }
}
impl core::fmt::Debug for Hcsparams {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcsparams")
            .field("n_ports", &self.n_ports())
            .field("ppc", &self.ppc())
            .field("p_indicator", &self.p_indicator())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcsparams {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcsparams {{ n_ports: {=u8:?}, ppc: {=bool:?}, p_indicator: {=bool:?} }}",
            self.n_ports(),
            self.ppc(),
            self.p_indicator()
        )
    }
}
#[doc = "Memory base address where INT PTD0 is stored"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intptd(pub u32);
impl Intptd {
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[must_use]
    #[inline(always)]
    pub const fn int_first(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub const fn set_int_first(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Base address to be used by the hardware to find the start of the INT list."]
    #[must_use]
    #[inline(always)]
    pub const fn int_base(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub const fn set_int_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Intptd {
    #[inline(always)]
    fn default() -> Intptd {
        Intptd(0)
    }
}
impl core::fmt::Debug for Intptd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intptd")
            .field("int_first", &self.int_first())
            .field("int_base", &self.int_base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intptd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intptd {{ int_first: {=u8:?}, int_base: {=u32:?} }}",
            self.int_first(),
            self.int_base()
        )
    }
}
#[doc = "Done map for each INT PTD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intptdd(pub u32);
impl Intptdd {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn int_done(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_int_done(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intptdd {
    #[inline(always)]
    fn default() -> Intptdd {
        Intptdd(0)
    }
}
impl core::fmt::Debug for Intptdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intptdd")
            .field("int_done", &self.int_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intptdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intptdd {{ int_done: {=u32:?} }}", self.int_done())
    }
}
#[doc = "Skip map for each INT PTD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intptds(pub u32);
impl Intptds {
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[must_use]
    #[inline(always)]
    pub const fn int_skip(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub const fn set_int_skip(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intptds {
    #[inline(always)]
    fn default() -> Intptds {
        Intptds(0)
    }
}
impl core::fmt::Debug for Intptds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intptds")
            .field("int_skip", &self.int_skip())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intptds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intptds {{ int_skip: {=u32:?} }}", self.int_skip())
    }
}
#[doc = "Memory base address where ISO PTD0 is stored"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoptd(pub u32);
impl Isoptd {
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_first(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub const fn set_iso_first(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Base address to be used by the hardware to find the start of the ISO list."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_base(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub const fn set_iso_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Isoptd {
    #[inline(always)]
    fn default() -> Isoptd {
        Isoptd(0)
    }
}
impl core::fmt::Debug for Isoptd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isoptd")
            .field("iso_first", &self.iso_first())
            .field("iso_base", &self.iso_base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isoptd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isoptd {{ iso_first: {=u8:?}, iso_base: {=u32:?} }}",
            self.iso_first(),
            self.iso_base()
        )
    }
}
#[doc = "Done map for each ISO PTD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoptdd(pub u32);
impl Isoptdd {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_done(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_iso_done(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isoptdd {
    #[inline(always)]
    fn default() -> Isoptdd {
        Isoptdd(0)
    }
}
impl core::fmt::Debug for Isoptdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isoptdd")
            .field("iso_done", &self.iso_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isoptdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isoptdd {{ iso_done: {=u32:?} }}", self.iso_done())
    }
}
#[doc = "Skip map for each ISO PTD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoptds(pub u32);
impl Isoptds {
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_skip(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub const fn set_iso_skip(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isoptds {
    #[inline(always)]
    fn default() -> Isoptds {
        Isoptds(0)
    }
}
impl core::fmt::Debug for Isoptds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isoptds")
            .field("iso_skip", &self.iso_skip())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isoptds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isoptds {{ iso_skip: {=u32:?} }}", self.iso_skip())
    }
}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lastptd(pub u32);
impl Lastptd {
    #[doc = "If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_last(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub const fn set_atl_last(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "This indicates the last PTD in the ISO list."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_last(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub const fn set_iso_last(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "This indicates the last PTD in the INT list."]
    #[must_use]
    #[inline(always)]
    pub const fn int_last(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub const fn set_int_last(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Lastptd {
    #[inline(always)]
    fn default() -> Lastptd {
        Lastptd(0)
    }
}
impl core::fmt::Debug for Lastptd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lastptd")
            .field("atl_last", &self.atl_last())
            .field("iso_last", &self.iso_last())
            .field("int_last", &self.int_last())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lastptd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lastptd {{ atl_last: {=u8:?}, iso_last: {=u8:?}, int_last: {=u8:?} }}",
            self.atl_last(),
            self.iso_last(),
            self.int_last()
        )
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portmode(pub u32);
impl Portmode {
    #[doc = "If this bit is set to one, one of the ports will behave as a USB device."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_enable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub const fn set_dev_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_ctrl_pdcom(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub const fn set_sw_ctrl_pdcom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_pdcom(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub const fn set_sw_pdcom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Portmode {
    #[inline(always)]
    fn default() -> Portmode {
        Portmode(0)
    }
}
impl core::fmt::Debug for Portmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Portmode")
            .field("dev_enable", &self.dev_enable())
            .field("sw_ctrl_pdcom", &self.sw_ctrl_pdcom())
            .field("sw_pdcom", &self.sw_pdcom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Portmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Portmode {{ dev_enable: {=bool:?}, sw_ctrl_pdcom: {=bool:?}, sw_pdcom: {=bool:?} }}",
            self.dev_enable(),
            self.sw_ctrl_pdcom(),
            self.sw_pdcom()
        )
    }
}
#[doc = "Port Status and Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portsc1(pub u32);
impl Portsc1 {
    #[doc = "Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub const fn set_ccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn csc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub const fn set_csc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enabled/Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ped(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled."]
    #[inline(always)]
    pub const fn set_ped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn pedc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub const fn set_pedc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Over-current active: Logic 1 means that this port has an over-current condition."]
    #[must_use]
    #[inline(always)]
    pub const fn oca(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub const fn set_oca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Over-current change: Logic 1 means that the value of OCA has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn occ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub const fn set_occ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[must_use]
    #[inline(always)]
    pub const fn fpr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub const fn set_fpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend: Logic 1 means port is in the suspend state."]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset: Logic 1 means the port is in the reset state."]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[must_use]
    #[inline(always)]
    pub const fn ls(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[inline(always)]
    pub const fn set_ls(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[must_use]
    #[inline(always)]
    pub const fn pp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub const fn set_pp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[must_use]
    #[inline(always)]
    pub const fn pic(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub const fn set_pic(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[must_use]
    #[inline(always)]
    pub const fn ptc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub const fn set_ptc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn pspd(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub const fn set_pspd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn woo(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub const fn set_woo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Portsc1 {
    #[inline(always)]
    fn default() -> Portsc1 {
        Portsc1(0)
    }
}
impl core::fmt::Debug for Portsc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Portsc1")
            .field("ccs", &self.ccs())
            .field("csc", &self.csc())
            .field("ped", &self.ped())
            .field("pedc", &self.pedc())
            .field("oca", &self.oca())
            .field("occ", &self.occ())
            .field("fpr", &self.fpr())
            .field("susp", &self.susp())
            .field("pr", &self.pr())
            .field("ls", &self.ls())
            .field("pp", &self.pp())
            .field("pic", &self.pic())
            .field("ptc", &self.ptc())
            .field("pspd", &self.pspd())
            .field("woo", &self.woo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Portsc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Portsc1 {{ ccs: {=bool:?}, csc: {=bool:?}, ped: {=bool:?}, pedc: {=bool:?}, oca: {=bool:?}, occ: {=bool:?}, fpr: {=bool:?}, susp: {=bool:?}, pr: {=bool:?}, ls: {=u8:?}, pp: {=bool:?}, pic: {=u8:?}, ptc: {=u8:?}, pspd: {=u8:?}, woo: {=bool:?} }}",
            self.ccs(),
            self.csc(),
            self.ped(),
            self.pedc(),
            self.oca(),
            self.occ(),
            self.fpr(),
            self.susp(),
            self.pr(),
            self.ls(),
            self.pp(),
            self.pic(),
            self.ptc(),
            self.pspd(),
            self.woo()
        )
    }
}
#[doc = "USB Command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcmd(pub u32);
impl Usbcmd {
    #[doc = "Run/Stop: 1b = Run."]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Run/Stop: 1b = Run."]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[must_use]
    #[inline(always)]
    pub const fn hcreset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub const fn set_hcreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame List Size: This field specifies the size of the frame list."]
    #[must_use]
    #[inline(always)]
    pub const fn fls(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub const fn set_fls(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[must_use]
    #[inline(always)]
    pub const fn lhcr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub const fn set_lhcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ATL List enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ATL List enabled."]
    #[inline(always)]
    pub const fn set_atl_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ISO List enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ISO List enabled."]
    #[inline(always)]
    pub const fn set_iso_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "INT List enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "INT List enabled."]
    #[inline(always)]
    pub const fn set_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Usbcmd {
    #[inline(always)]
    fn default() -> Usbcmd {
        Usbcmd(0)
    }
}
impl core::fmt::Debug for Usbcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbcmd")
            .field("rs", &self.rs())
            .field("hcreset", &self.hcreset())
            .field("fls", &self.fls())
            .field("lhcr", &self.lhcr())
            .field("atl_en", &self.atl_en())
            .field("iso_en", &self.iso_en())
            .field("int_en", &self.int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbcmd {{ rs: {=bool:?}, hcreset: {=bool:?}, fls: {=u8:?}, lhcr: {=bool:?}, atl_en: {=bool:?}, iso_en: {=bool:?}, int_en: {=bool:?} }}",
            self.rs(),
            self.hcreset(),
            self.fls(),
            self.lhcr(),
            self.atl_en(),
            self.iso_en(),
            self.int_en()
        )
    }
}
#[doc = "USB Interrupt Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbintr(pub u32);
impl Usbintr {
    #[doc = "Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pcde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_pcde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn flre(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_flre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ATL IRQ Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_irq_e(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_atl_irq_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ISO IRQ Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_irq_e(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_iso_irq_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "INT IRQ Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn int_irq_e(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_int_irq_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[must_use]
    #[inline(always)]
    pub const fn sof_e(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub const fn set_sof_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Usbintr {
    #[inline(always)]
    fn default() -> Usbintr {
        Usbintr(0)
    }
}
impl core::fmt::Debug for Usbintr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbintr")
            .field("pcde", &self.pcde())
            .field("flre", &self.flre())
            .field("atl_irq_e", &self.atl_irq_e())
            .field("iso_irq_e", &self.iso_irq_e())
            .field("int_irq_e", &self.int_irq_e())
            .field("sof_e", &self.sof_e())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbintr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbintr {{ pcde: {=bool:?}, flre: {=bool:?}, atl_irq_e: {=bool:?}, iso_irq_e: {=bool:?}, int_irq_e: {=bool:?}, sof_e: {=bool:?} }}",
            self.pcde(),
            self.flre(),
            self.atl_irq_e(),
            self.iso_irq_e(),
            self.int_irq_e(),
            self.sof_e()
        )
    }
}
#[doc = "USB Interrupt Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbsts(pub u32);
impl Usbsts {
    #[doc = "Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[must_use]
    #[inline(always)]
    pub const fn pcd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub const fn set_pcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn flr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub const fn set_flr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[must_use]
    #[inline(always)]
    pub const fn atl_irq(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub const fn set_atl_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub const fn set_iso_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[must_use]
    #[inline(always)]
    pub const fn int_irq(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub const fn set_int_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn sof_irq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub const fn set_sof_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Usbsts {
    #[inline(always)]
    fn default() -> Usbsts {
        Usbsts(0)
    }
}
impl core::fmt::Debug for Usbsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbsts")
            .field("pcd", &self.pcd())
            .field("flr", &self.flr())
            .field("atl_irq", &self.atl_irq())
            .field("iso_irq", &self.iso_irq())
            .field("int_irq", &self.int_irq())
            .field("sof_irq", &self.sof_irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbsts {{ pcd: {=bool:?}, flr: {=bool:?}, atl_irq: {=bool:?}, iso_irq: {=bool:?}, int_irq: {=bool:?}, sof_irq: {=bool:?} }}",
            self.pcd(),
            self.flr(),
            self.atl_irq(),
            self.iso_irq(),
            self.int_irq(),
            self.sof_irq()
        )
    }
}
