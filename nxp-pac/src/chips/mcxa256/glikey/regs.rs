#[doc = "Control Register 0 SFR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "Write Index"]
    #[must_use]
    #[inline(always)]
    pub const fn write_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Index"]
    #[inline(always)]
    pub const fn set_write_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Write Enable 0"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_en_0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Write Enable 0"]
    #[inline(always)]
    pub const fn set_wr_en_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Soft reset for the core reset (SFR configuration will be preseved).This register reads as 0"]
    #[must_use]
    #[inline(always)]
    pub const fn sft_rst(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Soft reset for the core reset (SFR configuration will be preseved).This register reads as 0"]
    #[inline(always)]
    pub const fn set_sft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved31(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved31(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("write_index", &self.write_index())
            .field("reserved15", &self.reserved15())
            .field("wr_en_0", &self.wr_en_0())
            .field("sft_rst", &self.sft_rst())
            .field("reserved31", &self.reserved31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl0 {{ write_index: {=u8:?}, reserved15: {=u8:?}, wr_en_0: {=u8:?}, sft_rst: {=bool:?}, reserved31: {=u16:?} }}",
            self.write_index(),
            self.reserved15(),
            self.wr_en_0(),
            self.sft_rst(),
            self.reserved31()
        )
    }
}
#[doc = "Control Register 1 SFR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Index status, Writing an index value to this register will request the block to return the lock status of this index."]
    #[must_use]
    #[inline(always)]
    pub const fn read_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Index status, Writing an index value to this register will request the block to return the lock status of this index."]
    #[inline(always)]
    pub const fn set_read_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Write Enable One"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_en_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Write Enable One"]
    #[inline(always)]
    pub const fn set_wr_en_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "LOCK register for GLIKEY"]
    #[must_use]
    #[inline(always)]
    pub const fn sfr_lock(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "LOCK register for GLIKEY"]
    #[inline(always)]
    pub const fn set_sfr_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved31(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved31(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("read_index", &self.read_index())
            .field("reserved15", &self.reserved15())
            .field("wr_en_1", &self.wr_en_1())
            .field("sfr_lock", &self.sfr_lock())
            .field("reserved31", &self.reserved31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ read_index: {=u8:?}, reserved15: {=u8:?}, wr_en_1: {=u8:?}, sfr_lock: {=u8:?}, reserved31: {=u16:?} }}",
            self.read_index(),
            self.reserved15(),
            self.wr_en_1(),
            self.sfr_lock(),
            self.reserved31()
        )
    }
}
#[doc = "Interrupt Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCtrl(pub u32);
impl IntrCtrl {
    #[doc = "Interrupt Enable. Writing a 1, Interrupt asserts on Interrupt output port"]
    #[must_use]
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable. Writing a 1, Interrupt asserts on Interrupt output port"]
    #[inline(always)]
    pub const fn set_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Clear. Writing a 1 to this register creates a single interrupt clear pulse. This register reads as 0"]
    #[must_use]
    #[inline(always)]
    pub const fn int_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Clear. Writing a 1 to this register creates a single interrupt clear pulse. This register reads as 0"]
    #[inline(always)]
    pub const fn set_int_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Set. Writing a 1 to this register asserts the interrupt. This register reads as 0"]
    #[must_use]
    #[inline(always)]
    pub const fn int_set(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Set. Writing a 1 to this register asserts the interrupt. This register reads as 0"]
    #[inline(always)]
    pub const fn set_int_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved31(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved31(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for IntrCtrl {
    #[inline(always)]
    fn default() -> IntrCtrl {
        IntrCtrl(0)
    }
}
impl core::fmt::Debug for IntrCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntrCtrl")
            .field("int_en", &self.int_en())
            .field("int_clr", &self.int_clr())
            .field("int_set", &self.int_set())
            .field("reserved31", &self.reserved31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntrCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntrCtrl {{ int_en: {=bool:?}, int_clr: {=bool:?}, int_set: {=bool:?}, reserved31: {=u32:?} }}",
            self.int_en(),
            self.int_clr(),
            self.int_set(),
            self.reserved31()
        )
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn int_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn set_int_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Provides the current lock status of indexes."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_status(&self) -> super::vals::LockStatus {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LockStatus::from_bits(val as u8)
    }
    #[doc = "Provides the current lock status of indexes."]
    #[inline(always)]
    pub const fn set_lock_status(&mut self, val: super::vals::LockStatus) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Status of the Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error_status(&self) -> super::vals::ErrorStatus {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::ErrorStatus::from_bits(val as u8)
    }
    #[doc = "Status of the Error"]
    #[inline(always)]
    pub const fn set_error_status(&mut self, val: super::vals::ErrorStatus) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved18(&self) -> u16 {
        let val = (self.0 >> 5usize) & 0x3fff;
        val as u16
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved18(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 5usize)) | (((val as u32) & 0x3fff) << 5usize);
    }
    #[doc = "Status of FSM"]
    #[must_use]
    #[inline(always)]
    pub const fn fsm_state(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "Status of FSM"]
    #[inline(always)]
    pub const fn set_fsm_state(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("int_status", &self.int_status())
            .field("lock_status", &self.lock_status())
            .field("error_status", &self.error_status())
            .field("reserved18", &self.reserved18())
            .field("fsm_state", &self.fsm_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ int_status: {=bool:?}, lock_status: {:?}, error_status: {:?}, reserved18: {=u16:?}, fsm_state: {=u16:?} }}",
            self.int_status(),
            self.lock_status(),
            self.error_status(),
            self.reserved18(),
            self.fsm_state()
        )
    }
}
#[doc = "IP Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved7(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved11(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Release milestone. 00-PREL, 01-BR, 10-SI, 11-GO."]
    #[must_use]
    #[inline(always)]
    pub const fn milestone(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Release milestone. 00-PREL, 01-BR, 10-SI, 11-GO."]
    #[inline(always)]
    pub const fn set_milestone(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "0:4 step, 1:8 step"]
    #[must_use]
    #[inline(always)]
    pub const fn fsm_config(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0:4 step, 1:8 step"]
    #[inline(always)]
    pub const fn set_fsm_config(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Configured number of addressable indexes"]
    #[must_use]
    #[inline(always)]
    pub const fn index_config(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0xff;
        val as u8
    }
    #[doc = "Configured number of addressable indexes"]
    #[inline(always)]
    pub const fn set_index_config(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 19usize)) | (((val as u32) & 0xff) << 19usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved31(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_reserved31(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("reserved3", &self.reserved3())
            .field("reserved7", &self.reserved7())
            .field("reserved11", &self.reserved11())
            .field("reserved15", &self.reserved15())
            .field("milestone", &self.milestone())
            .field("fsm_config", &self.fsm_config())
            .field("index_config", &self.index_config())
            .field("reserved31", &self.reserved31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ reserved3: {=u8:?}, reserved7: {=u8:?}, reserved11: {=u8:?}, reserved15: {=u8:?}, milestone: {=u8:?}, fsm_config: {=bool:?}, index_config: {=u8:?}, reserved31: {=u8:?} }}",
            self.reserved3(),
            self.reserved7(),
            self.reserved11(),
            self.reserved15(),
            self.milestone(),
            self.fsm_config(),
            self.index_config(),
            self.reserved31()
        )
    }
}
