#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Els {
    ptr: *mut u8,
}
unsafe impl Send for Els {}
unsafe impl Sync for Els {}
impl Els {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::pac::common::Reg<STATUS, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::pac::common::Reg<CTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command Configuration."]
    #[inline(always)]
    pub const fn CMDCFG0(self) -> crate::pac::common::Reg<CMDCFG0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::pac::common::Reg<CFG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Keystore Index 0."]
    #[inline(always)]
    pub const fn KIDX0(self) -> crate::pac::common::Reg<KIDX0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Keystore Index 1."]
    #[inline(always)]
    pub const fn KIDX1(self) -> crate::pac::common::Reg<KIDX1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Key Properties Request."]
    #[inline(always)]
    pub const fn KPROPIN(self) -> crate::pac::common::Reg<KPROPIN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Source."]
    #[inline(always)]
    pub const fn DMA_SRC(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<DMA_SRC, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 8usize) as _)
        }
    }
    #[doc = "DMA Source 0 Length."]
    #[inline(always)]
    pub const fn DMA_SRC0_LEN(
        self,
    ) -> crate::pac::common::Reg<DMA_SRC0_LEN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA Source 2 Length."]
    #[inline(always)]
    pub const fn DMA_SRC2_LEN(
        self,
    ) -> crate::pac::common::Reg<DMA_SRC2_LEN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "DMA Result 0."]
    #[inline(always)]
    pub const fn DMA_RES0(self) -> crate::pac::common::Reg<DMA_RES0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "DMA Result 0 Length."]
    #[inline(always)]
    pub const fn DMA_RES0_LEN(
        self,
    ) -> crate::pac::common::Reg<DMA_RES0_LEN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn INT_ENABLE(self) -> crate::pac::common::Reg<INT_ENABLE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Status Clear."]
    #[inline(always)]
    pub const fn INT_STATUS_CLR(
        self,
    ) -> crate::pac::common::Reg<INT_STATUS_CLR, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Interrupt Status Set."]
    #[inline(always)]
    pub const fn INT_STATUS_SET(
        self,
    ) -> crate::pac::common::Reg<INT_STATUS_SET, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Error Status."]
    #[inline(always)]
    pub const fn ERR_STATUS(self) -> crate::pac::common::Reg<ERR_STATUS, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Error Status Clear."]
    #[inline(always)]
    pub const fn ERR_STATUS_CLR(
        self,
    ) -> crate::pac::common::Reg<ERR_STATUS_CLR, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Version Register."]
    #[inline(always)]
    pub const fn VERSION(self) -> crate::pac::common::Reg<VERSION, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "PRNG SW Read Out."]
    #[inline(always)]
    pub const fn PRNG_DATOUT(self) -> crate::pac::common::Reg<PRNG_DATOUT, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "CRC Configuration."]
    #[inline(always)]
    pub const fn CMDCRC_CTRL(self) -> crate::pac::common::Reg<CMDCRC_CTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Command CRC Value."]
    #[inline(always)]
    pub const fn CMDCRC(self) -> crate::pac::common::Reg<CMDCRC, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Session ID."]
    #[inline(always)]
    pub const fn SESSION_ID(self) -> crate::pac::common::Reg<SESSION_ID, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Final DMA Address."]
    #[inline(always)]
    pub const fn DMA_FIN_ADDR(
        self,
    ) -> crate::pac::common::Reg<DMA_FIN_ADDR, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Master ID."]
    #[inline(always)]
    pub const fn MASTER_ID(self) -> crate::pac::common::Reg<MASTER_ID, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Keystore Index 2."]
    #[inline(always)]
    pub const fn KIDX2(self) -> crate::pac::common::Reg<KIDX2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn ELS_KS(self, n: usize) -> crate::pac::common::Reg<ELS_KS, crate::pac::common::R> {
        assert!(n < 20usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize + n * 4usize) as _)
        }
    }
}
#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "Controls the maximum value of a variable delay that will be applied before any ELS AES operation is started."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCTRL(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Controls the maximum value of a variable delay that will be applied before any ELS AES operation is started."]
    #[inline(always)]
    pub const fn set_ADCTRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("ADCTRL", &self.ADCTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CFG {{ ADCTRL: {=u16:?} }}", self.ADCTRL())
    }
}
#[doc = "Command Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDCFG0(pub u32);
impl CMDCFG0 {
    #[doc = "See."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDCFG0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "See."]
    #[inline(always)]
    pub const fn set_CMDCFG0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CMDCFG0 {
    #[inline(always)]
    fn default() -> CMDCFG0 {
        CMDCFG0(0)
    }
}
impl core::fmt::Debug for CMDCFG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDCFG0")
            .field("CMDCFG0", &self.CMDCFG0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDCFG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMDCFG0 {{ CMDCFG0: {=u32:?} }}", self.CMDCFG0())
    }
}
#[doc = "Command CRC Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDCRC(pub u32);
impl CMDCRC {
    #[doc = "Indicates the current CRC value."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDCRC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the current CRC value."]
    #[inline(always)]
    pub const fn set_CMDCRC(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CMDCRC {
    #[inline(always)]
    fn default() -> CMDCRC {
        CMDCRC(0)
    }
}
impl core::fmt::Debug for CMDCRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDCRC")
            .field("CMDCRC", &self.CMDCRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDCRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMDCRC {{ CMDCRC: {=u32:?} }}", self.CMDCRC())
    }
}
#[doc = "CRC Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDCRC_CTRL(pub u32);
impl CMDCRC_CTRL {
    #[doc = "CRC reset to initial valueCMDCRC_EN and CMDCRC_RST fields act independently."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDCRC_RST(&self) -> CMDCRC_RST {
        let val = (self.0 >> 0usize) & 0x01;
        CMDCRC_RST::from_bits(val as u8)
    }
    #[doc = "CRC reset to initial valueCMDCRC_EN and CMDCRC_RST fields act independently."]
    #[inline(always)]
    pub const fn set_CMDCRC_RST(&mut self, val: CMDCRC_RST) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CRC enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDCRC_EN(&self) -> CMDCRC_EN {
        let val = (self.0 >> 1usize) & 0x01;
        CMDCRC_EN::from_bits(val as u8)
    }
    #[doc = "CRC enable bit."]
    #[inline(always)]
    pub const fn set_CMDCRC_EN(&mut self, val: CMDCRC_EN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for CMDCRC_CTRL {
    #[inline(always)]
    fn default() -> CMDCRC_CTRL {
        CMDCRC_CTRL(0)
    }
}
impl core::fmt::Debug for CMDCRC_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDCRC_CTRL")
            .field("CMDCRC_RST", &self.CMDCRC_RST())
            .field("CMDCRC_EN", &self.CMDCRC_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDCRC_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDCRC_CTRL {{ CMDCRC_RST: {:?}, CMDCRC_EN: {:?} }}",
            self.CMDCRC_RST(),
            self.CMDCRC_EN()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "ELS enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_EN(&self) -> ELS_EN {
        let val = (self.0 >> 0usize) & 0x01;
        ELS_EN::from_bits(val as u8)
    }
    #[doc = "ELS enable."]
    #[inline(always)]
    pub const fn set_ELS_EN(&mut self, val: ELS_EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Write to 1 to start an ELS operation. Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_START(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write to 1 to start an ELS operation. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_ELS_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write to 1 to perform an ELS synchronous reset. Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_RESET(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write to 1 to perform an ELS synchronous reset. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_ELS_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ELS Command ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_CMD(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "ELS Command ID."]
    #[inline(always)]
    pub const fn set_ELS_CMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "Defines endianness."]
    #[must_use]
    #[inline(always)]
    pub const fn BYTE_ORDER(&self) -> BYTE_ORDER {
        let val = (self.0 >> 8usize) & 0x01;
        BYTE_ORDER::from_bits(val as u8)
    }
    #[doc = "Defines endianness."]
    #[inline(always)]
    pub const fn set_BYTE_ORDER(&mut self, val: BYTE_ORDER) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ELS_EN", &self.ELS_EN())
            .field("ELS_START", &self.ELS_START())
            .field("ELS_RESET", &self.ELS_RESET())
            .field("ELS_CMD", &self.ELS_CMD())
            .field("BYTE_ORDER", &self.BYTE_ORDER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ELS_EN: {:?}, ELS_START: {=bool:?}, ELS_RESET: {=bool:?}, ELS_CMD: {=u8:?}, BYTE_ORDER: {:?} }}",
            self.ELS_EN(),
            self.ELS_START(),
            self.ELS_RESET(),
            self.ELS_CMD(),
            self.BYTE_ORDER()
        )
    }
}
#[doc = "Final DMA Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_FIN_ADDR(pub u32);
impl DMA_FIN_ADDR {
    #[doc = "Indicates the final address of system memory that was accessed by ELS during the last command."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_FIN_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the final address of system memory that was accessed by ELS during the last command."]
    #[inline(always)]
    pub const fn set_DMA_FIN_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA_FIN_ADDR {
    #[inline(always)]
    fn default() -> DMA_FIN_ADDR {
        DMA_FIN_ADDR(0)
    }
}
impl core::fmt::Debug for DMA_FIN_ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_FIN_ADDR")
            .field("DMA_FIN_ADDR", &self.DMA_FIN_ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_FIN_ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_FIN_ADDR {{ DMA_FIN_ADDR: {=u32:?} }}",
            self.DMA_FIN_ADDR()
        )
    }
}
#[doc = "DMA Result 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_RES0(pub u32);
impl DMA_RES0 {
    #[doc = "Defines the system start address where the result of the ELS operation is transferred via DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_RES0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Defines the system start address where the result of the ELS operation is transferred via DMA."]
    #[inline(always)]
    pub const fn set_ADDR_RES0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA_RES0 {
    #[inline(always)]
    fn default() -> DMA_RES0 {
        DMA_RES0(0)
    }
}
impl core::fmt::Debug for DMA_RES0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RES0")
            .field("ADDR_RES0", &self.ADDR_RES0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_RES0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_RES0 {{ ADDR_RES0: {=u32:?} }}", self.ADDR_RES0())
    }
}
#[doc = "DMA Result 0 Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_RES0_LEN(pub u32);
impl DMA_RES0_LEN {
    #[doc = "Size in bytes of the data to be transferred."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE_RES0_LEN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size in bytes of the data to be transferred."]
    #[inline(always)]
    pub const fn set_SIZE_RES0_LEN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA_RES0_LEN {
    #[inline(always)]
    fn default() -> DMA_RES0_LEN {
        DMA_RES0_LEN(0)
    }
}
impl core::fmt::Debug for DMA_RES0_LEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RES0_LEN")
            .field("SIZE_RES0_LEN", &self.SIZE_RES0_LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_RES0_LEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_RES0_LEN {{ SIZE_RES0_LEN: {=u32:?} }}",
            self.SIZE_RES0_LEN()
        )
    }
}
#[doc = "DMA Source."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_SRC(pub u32);
impl DMA_SRC {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_SRC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[inline(always)]
    pub const fn set_ADDR_SRC(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA_SRC {
    #[inline(always)]
    fn default() -> DMA_SRC {
        DMA_SRC(0)
    }
}
impl core::fmt::Debug for DMA_SRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_SRC")
            .field("ADDR_SRC", &self.ADDR_SRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_SRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_SRC {{ ADDR_SRC: {=u32:?} }}", self.ADDR_SRC())
    }
}
#[doc = "DMA Source 0 Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_SRC0_LEN(pub u32);
impl DMA_SRC0_LEN {
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE_SRC0_LEN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC0."]
    #[inline(always)]
    pub const fn set_SIZE_SRC0_LEN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA_SRC0_LEN {
    #[inline(always)]
    fn default() -> DMA_SRC0_LEN {
        DMA_SRC0_LEN(0)
    }
}
impl core::fmt::Debug for DMA_SRC0_LEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_SRC0_LEN")
            .field("SIZE_SRC0_LEN", &self.SIZE_SRC0_LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_SRC0_LEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_SRC0_LEN {{ SIZE_SRC0_LEN: {=u32:?} }}",
            self.SIZE_SRC0_LEN()
        )
    }
}
#[doc = "DMA Source 2 Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_SRC2_LEN(pub u32);
impl DMA_SRC2_LEN {
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC2."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE_SRC2_LEN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC2."]
    #[inline(always)]
    pub const fn set_SIZE_SRC2_LEN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA_SRC2_LEN {
    #[inline(always)]
    fn default() -> DMA_SRC2_LEN {
        DMA_SRC2_LEN(0)
    }
}
impl core::fmt::Debug for DMA_SRC2_LEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_SRC2_LEN")
            .field("SIZE_SRC2_LEN", &self.SIZE_SRC2_LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_SRC2_LEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_SRC2_LEN {{ SIZE_SRC2_LEN: {=u32:?} }}",
            self.SIZE_SRC2_LEN()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ELS_KS(pub u32);
impl ELS_KS {
    #[doc = "Key size."]
    #[must_use]
    #[inline(always)]
    pub const fn KSIZE(&self) -> KSIZE {
        let val = (self.0 >> 0usize) & 0x03;
        KSIZE::from_bits(val as u8)
    }
    #[doc = "Key size."]
    #[inline(always)]
    pub const fn set_KSIZE(&mut self, val: KSIZE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active."]
    #[must_use]
    #[inline(always)]
    pub const fn KACT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active."]
    #[inline(always)]
    pub const fn set_KACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key."]
    #[must_use]
    #[inline(always)]
    pub const fn KBASE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key."]
    #[inline(always)]
    pub const fn set_KBASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose."]
    #[must_use]
    #[inline(always)]
    pub const fn FGP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose."]
    #[inline(always)]
    pub const fn set_FGP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention."]
    #[must_use]
    #[inline(always)]
    pub const fn FRTN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention."]
    #[inline(always)]
    pub const fn set_FRTN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output."]
    #[must_use]
    #[inline(always)]
    pub const fn FHWO(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output."]
    #[inline(always)]
    pub const fn set_FHWO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key."]
    #[must_use]
    #[inline(always)]
    pub const fn UKPUK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key."]
    #[inline(always)]
    pub const fn set_UKPUK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key."]
    #[must_use]
    #[inline(always)]
    pub const fn UTECDH(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key."]
    #[inline(always)]
    pub const fn set_UTECDH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key."]
    #[must_use]
    #[inline(always)]
    pub const fn UCMAC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key."]
    #[inline(always)]
    pub const fn set_UCMAC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key."]
    #[must_use]
    #[inline(always)]
    pub const fn UKSK(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key."]
    #[inline(always)]
    pub const fn set_UKSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key."]
    #[must_use]
    #[inline(always)]
    pub const fn URTF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key."]
    #[inline(always)]
    pub const fn set_URTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command."]
    #[must_use]
    #[inline(always)]
    pub const fn UCKDF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command."]
    #[inline(always)]
    pub const fn set_UCKDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command."]
    #[must_use]
    #[inline(always)]
    pub const fn UHKDF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command."]
    #[inline(always)]
    pub const fn set_UHKDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key."]
    #[must_use]
    #[inline(always)]
    pub const fn UECSG(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key."]
    #[inline(always)]
    pub const fn set_UECSG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key."]
    #[must_use]
    #[inline(always)]
    pub const fn UECDH(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key."]
    #[inline(always)]
    pub const fn set_UECDH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key."]
    #[must_use]
    #[inline(always)]
    pub const fn UAES(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key."]
    #[inline(always)]
    pub const fn set_UAES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key."]
    #[must_use]
    #[inline(always)]
    pub const fn UHMAC(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key."]
    #[inline(always)]
    pub const fn set_UHMAC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key."]
    #[must_use]
    #[inline(always)]
    pub const fn UKWK(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key."]
    #[inline(always)]
    pub const fn set_UKWK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key."]
    #[must_use]
    #[inline(always)]
    pub const fn UKUOK(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key."]
    #[inline(always)]
    pub const fn set_UKUOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret."]
    #[must_use]
    #[inline(always)]
    pub const fn UTLSPMS(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret."]
    #[inline(always)]
    pub const fn set_UTLSPMS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret."]
    #[must_use]
    #[inline(always)]
    pub const fn UTLSMS(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret."]
    #[inline(always)]
    pub const fn set_UTLSMS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source."]
    #[must_use]
    #[inline(always)]
    pub const fn UKGSRC(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source."]
    #[inline(always)]
    pub const fn set_UKGSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key."]
    #[must_use]
    #[inline(always)]
    pub const fn UHWO(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key."]
    #[inline(always)]
    pub const fn set_UHWO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Wrap key."]
    #[must_use]
    #[inline(always)]
    pub const fn UWRPOK(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Wrap key."]
    #[inline(always)]
    pub const fn set_UWRPOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key."]
    #[must_use]
    #[inline(always)]
    pub const fn UDUK(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key."]
    #[inline(always)]
    pub const fn set_UDUK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level."]
    #[must_use]
    #[inline(always)]
    pub const fn UPPROT(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level."]
    #[inline(always)]
    pub const fn set_UPPROT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ELS_KS {
    #[inline(always)]
    fn default() -> ELS_KS {
        ELS_KS(0)
    }
}
impl core::fmt::Debug for ELS_KS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELS_KS")
            .field("KSIZE", &self.KSIZE())
            .field("KACT", &self.KACT())
            .field("KBASE", &self.KBASE())
            .field("FGP", &self.FGP())
            .field("FRTN", &self.FRTN())
            .field("FHWO", &self.FHWO())
            .field("UKPUK", &self.UKPUK())
            .field("UTECDH", &self.UTECDH())
            .field("UCMAC", &self.UCMAC())
            .field("UKSK", &self.UKSK())
            .field("URTF", &self.URTF())
            .field("UCKDF", &self.UCKDF())
            .field("UHKDF", &self.UHKDF())
            .field("UECSG", &self.UECSG())
            .field("UECDH", &self.UECDH())
            .field("UAES", &self.UAES())
            .field("UHMAC", &self.UHMAC())
            .field("UKWK", &self.UKWK())
            .field("UKUOK", &self.UKUOK())
            .field("UTLSPMS", &self.UTLSPMS())
            .field("UTLSMS", &self.UTLSMS())
            .field("UKGSRC", &self.UKGSRC())
            .field("UHWO", &self.UHWO())
            .field("UWRPOK", &self.UWRPOK())
            .field("UDUK", &self.UDUK())
            .field("UPPROT", &self.UPPROT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ELS_KS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ELS_KS {{ KSIZE: {:?}, KACT: {=bool:?}, KBASE: {=bool:?}, FGP: {=bool:?}, FRTN: {=bool:?}, FHWO: {=bool:?}, UKPUK: {=bool:?}, UTECDH: {=bool:?}, UCMAC: {=bool:?}, UKSK: {=bool:?}, URTF: {=bool:?}, UCKDF: {=bool:?}, UHKDF: {=bool:?}, UECSG: {=bool:?}, UECDH: {=bool:?}, UAES: {=bool:?}, UHMAC: {=bool:?}, UKWK: {=bool:?}, UKUOK: {=bool:?}, UTLSPMS: {=bool:?}, UTLSMS: {=bool:?}, UKGSRC: {=bool:?}, UHWO: {=bool:?}, UWRPOK: {=bool:?}, UDUK: {=bool:?}, UPPROT: {=u8:?} }}",
            self.KSIZE(),
            self.KACT(),
            self.KBASE(),
            self.FGP(),
            self.FRTN(),
            self.FHWO(),
            self.UKPUK(),
            self.UTECDH(),
            self.UCMAC(),
            self.UKSK(),
            self.URTF(),
            self.UCKDF(),
            self.UHKDF(),
            self.UECSG(),
            self.UECDH(),
            self.UAES(),
            self.UHMAC(),
            self.UKWK(),
            self.UKUOK(),
            self.UTLSPMS(),
            self.UTLSMS(),
            self.UKGSRC(),
            self.UHWO(),
            self.UWRPOK(),
            self.UDUK(),
            self.UPPROT()
        )
    }
}
#[doc = "Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERR_STATUS(pub u32);
impl ERR_STATUS {
    #[doc = "Indicates public or private bus access error."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ERR(&self) -> BUS_ERR {
        let val = (self.0 >> 0usize) & 0x01;
        BUS_ERR::from_bits(val as u8)
    }
    #[doc = "Indicates public or private bus access error."]
    #[inline(always)]
    pub const fn set_BUS_ERR(&mut self, val: BUS_ERR) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates operational error, that is, ELS has been incorrectly operated."]
    #[must_use]
    #[inline(always)]
    pub const fn OPN_ERR(&self) -> OPN_ERR {
        let val = (self.0 >> 1usize) & 0x01;
        OPN_ERR::from_bits(val as u8)
    }
    #[doc = "Indicates operational error, that is, ELS has been incorrectly operated."]
    #[inline(always)]
    pub const fn set_OPN_ERR(&mut self, val: OPN_ERR) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates algorithm error; an internal algorithm has produced an unexpected result."]
    #[must_use]
    #[inline(always)]
    pub const fn ALG_ERR(&self) -> ALG_ERR {
        let val = (self.0 >> 2usize) & 0x01;
        ALG_ERR::from_bits(val as u8)
    }
    #[doc = "Indicates algorithm error; an internal algorithm has produced an unexpected result."]
    #[inline(always)]
    pub const fn set_ALG_ERR(&mut self, val: ALG_ERR) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates data integrity error, that is, internal data integrity check has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn ITG_ERR(&self) -> ITG_ERR {
        let val = (self.0 >> 3usize) & 0x01;
        ITG_ERR::from_bits(val as u8)
    }
    #[doc = "Indicates data integrity error, that is, internal data integrity check has failed."]
    #[inline(always)]
    pub const fn set_ITG_ERR(&mut self, val: ITG_ERR) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates hardware fault error; an attempt to change the value of an internal register."]
    #[must_use]
    #[inline(always)]
    pub const fn FLT_ERR(&self) -> FLT_ERR {
        let val = (self.0 >> 4usize) & 0x01;
        FLT_ERR::from_bits(val as u8)
    }
    #[doc = "Indicates hardware fault error; an attempt to change the value of an internal register."]
    #[inline(always)]
    pub const fn set_FLT_ERR(&mut self, val: FLT_ERR) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates user read of PRNG_DATOUT when STATUS\\[PRNG_RDY\\] is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn PRNG_ERR(&self) -> PRNG_ERR {
        let val = (self.0 >> 5usize) & 0x01;
        PRNG_ERR::from_bits(val as u8)
    }
    #[doc = "Indicates user read of PRNG_DATOUT when STATUS\\[PRNG_RDY\\] is 0."]
    #[inline(always)]
    pub const fn set_PRNG_ERR(&mut self, val: PRNG_ERR) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates the triggered error level: 0, 1 ,2."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR_LVL(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates the triggered error level: 0, 1 ,2."]
    #[inline(always)]
    pub const fn set_ERR_LVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "TRNG unable to gather entropy with the current configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn DTRNG_ERR(&self) -> DTRNG_ERR {
        let val = (self.0 >> 8usize) & 0x01;
        DTRNG_ERR::from_bits(val as u8)
    }
    #[doc = "TRNG unable to gather entropy with the current configuration."]
    #[inline(always)]
    pub const fn set_DTRNG_ERR(&mut self, val: DTRNG_ERR) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for ERR_STATUS {
    #[inline(always)]
    fn default() -> ERR_STATUS {
        ERR_STATUS(0)
    }
}
impl core::fmt::Debug for ERR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_STATUS")
            .field("BUS_ERR", &self.BUS_ERR())
            .field("OPN_ERR", &self.OPN_ERR())
            .field("ALG_ERR", &self.ALG_ERR())
            .field("ITG_ERR", &self.ITG_ERR())
            .field("FLT_ERR", &self.FLT_ERR())
            .field("PRNG_ERR", &self.PRNG_ERR())
            .field("ERR_LVL", &self.ERR_LVL())
            .field("DTRNG_ERR", &self.DTRNG_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERR_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ERR_STATUS {{ BUS_ERR: {:?}, OPN_ERR: {:?}, ALG_ERR: {:?}, ITG_ERR: {:?}, FLT_ERR: {:?}, PRNG_ERR: {:?}, ERR_LVL: {=u8:?}, DTRNG_ERR: {:?} }}",
            self.BUS_ERR(),
            self.OPN_ERR(),
            self.ALG_ERR(),
            self.ITG_ERR(),
            self.FLT_ERR(),
            self.PRNG_ERR(),
            self.ERR_LVL(),
            self.DTRNG_ERR()
        )
    }
}
#[doc = "Error Status Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERR_STATUS_CLR(pub u32);
impl ERR_STATUS_CLR {
    #[doc = "ELS error status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR_CLR(&self) -> ERR_CLR {
        let val = (self.0 >> 0usize) & 0x01;
        ERR_CLR::from_bits(val as u8)
    }
    #[doc = "ELS error status bit."]
    #[inline(always)]
    pub const fn set_ERR_CLR(&mut self, val: ERR_CLR) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for ERR_STATUS_CLR {
    #[inline(always)]
    fn default() -> ERR_STATUS_CLR {
        ERR_STATUS_CLR(0)
    }
}
impl core::fmt::Debug for ERR_STATUS_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_STATUS_CLR")
            .field("ERR_CLR", &self.ERR_CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERR_STATUS_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ERR_STATUS_CLR {{ ERR_CLR: {:?} }}", self.ERR_CLR())
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_ENABLE(pub u32);
impl INT_ENABLE {
    #[doc = "Enables or disables the operation of the ELS interrupt output port."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_EN(&self) -> INT_EN {
        let val = (self.0 >> 0usize) & 0x01;
        INT_EN::from_bits(val as u8)
    }
    #[doc = "Enables or disables the operation of the ELS interrupt output port."]
    #[inline(always)]
    pub const fn set_INT_EN(&mut self, val: INT_EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for INT_ENABLE {
    #[inline(always)]
    fn default() -> INT_ENABLE {
        INT_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENABLE")
            .field("INT_EN", &self.INT_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INT_ENABLE {{ INT_EN: {:?} }}", self.INT_EN())
    }
}
#[doc = "Interrupt Status Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_STATUS_CLR(pub u32);
impl INT_STATUS_CLR {
    #[doc = "Interrupt status clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_CLR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status clear bit."]
    #[inline(always)]
    pub const fn set_INT_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for INT_STATUS_CLR {
    #[inline(always)]
    fn default() -> INT_STATUS_CLR {
        INT_STATUS_CLR(0)
    }
}
impl core::fmt::Debug for INT_STATUS_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_CLR")
            .field("INT_CLR", &self.INT_CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_STATUS_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INT_STATUS_CLR {{ INT_CLR: {=bool:?} }}", self.INT_CLR())
    }
}
#[doc = "Interrupt Status Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_STATUS_SET(pub u32);
impl INT_STATUS_SET {
    #[doc = "Software triggered interrupt bit."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_SET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software triggered interrupt bit."]
    #[inline(always)]
    pub const fn set_INT_SET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for INT_STATUS_SET {
    #[inline(always)]
    fn default() -> INT_STATUS_SET {
        INT_STATUS_SET(0)
    }
}
impl core::fmt::Debug for INT_STATUS_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_SET")
            .field("INT_SET", &self.INT_SET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_STATUS_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INT_STATUS_SET {{ INT_SET: {=bool:?} }}", self.INT_SET())
    }
}
#[doc = "Keystore Index 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KIDX0(pub u32);
impl KIDX0 {
    #[doc = "Selects the base 128-bit section of a key in ELS keystore."]
    #[must_use]
    #[inline(always)]
    pub const fn KIDX0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the base 128-bit section of a key in ELS keystore."]
    #[inline(always)]
    pub const fn set_KIDX0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for KIDX0 {
    #[inline(always)]
    fn default() -> KIDX0 {
        KIDX0(0)
    }
}
impl core::fmt::Debug for KIDX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KIDX0")
            .field("KIDX0", &self.KIDX0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KIDX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KIDX0 {{ KIDX0: {=u8:?} }}", self.KIDX0())
    }
}
#[doc = "Keystore Index 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KIDX1(pub u32);
impl KIDX1 {
    #[doc = "Selects the base 128-bit section of a key in ELS keystore."]
    #[must_use]
    #[inline(always)]
    pub const fn KIDX1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the base 128-bit section of a key in ELS keystore."]
    #[inline(always)]
    pub const fn set_KIDX1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for KIDX1 {
    #[inline(always)]
    fn default() -> KIDX1 {
        KIDX1(0)
    }
}
impl core::fmt::Debug for KIDX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KIDX1")
            .field("KIDX1", &self.KIDX1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KIDX1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KIDX1 {{ KIDX1: {=u8:?} }}", self.KIDX1())
    }
}
#[doc = "Keystore Index 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KIDX2(pub u32);
impl KIDX2 {
    #[doc = "Selects the base 128-bit section of a key in ELS keystore."]
    #[must_use]
    #[inline(always)]
    pub const fn KIDX2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the base 128-bit section of a key in ELS keystore."]
    #[inline(always)]
    pub const fn set_KIDX2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for KIDX2 {
    #[inline(always)]
    fn default() -> KIDX2 {
        KIDX2(0)
    }
}
impl core::fmt::Debug for KIDX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KIDX2")
            .field("KIDX2", &self.KIDX2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KIDX2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KIDX2 {{ KIDX2: {=u8:?} }}", self.KIDX2())
    }
}
#[doc = "Key Properties Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KPROPIN(pub u32);
impl KPROPIN {
    #[doc = "Specifies requested properties of the key created by ELS command."]
    #[must_use]
    #[inline(always)]
    pub const fn KPROPIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies requested properties of the key created by ELS command."]
    #[inline(always)]
    pub const fn set_KPROPIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KPROPIN {
    #[inline(always)]
    fn default() -> KPROPIN {
        KPROPIN(0)
    }
}
impl core::fmt::Debug for KPROPIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KPROPIN")
            .field("KPROPIN", &self.KPROPIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KPROPIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KPROPIN {{ KPROPIN: {=u32:?} }}", self.KPROPIN())
    }
}
#[doc = "Master ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASTER_ID(pub u32);
impl MASTER_ID {
    #[doc = "Sets the privileged master ID."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTER_ID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the privileged master ID."]
    #[inline(always)]
    pub const fn set_MASTER_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for MASTER_ID {
    #[inline(always)]
    fn default() -> MASTER_ID {
        MASTER_ID(0)
    }
}
impl core::fmt::Debug for MASTER_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASTER_ID")
            .field("MASTER_ID", &self.MASTER_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASTER_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASTER_ID {{ MASTER_ID: {=u8:?} }}", self.MASTER_ID())
    }
}
#[doc = "PRNG SW Read Out."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRNG_DATOUT(pub u32);
impl PRNG_DATOUT {
    #[doc = "32-bit wide pseudo-random number."]
    #[must_use]
    #[inline(always)]
    pub const fn PRNG_DATOUT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit wide pseudo-random number."]
    #[inline(always)]
    pub const fn set_PRNG_DATOUT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRNG_DATOUT {
    #[inline(always)]
    fn default() -> PRNG_DATOUT {
        PRNG_DATOUT(0)
    }
}
impl core::fmt::Debug for PRNG_DATOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRNG_DATOUT")
            .field("PRNG_DATOUT", &self.PRNG_DATOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRNG_DATOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRNG_DATOUT {{ PRNG_DATOUT: {=u32:?} }}",
            self.PRNG_DATOUT()
        )
    }
}
#[doc = "Session ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SESSION_ID(pub u32);
impl SESSION_ID {
    #[doc = "Indicates the current value of the session ID."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSION_ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the current value of the session ID."]
    #[inline(always)]
    pub const fn set_SESSION_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SESSION_ID {
    #[inline(always)]
    fn default() -> SESSION_ID {
        SESSION_ID(0)
    }
}
impl core::fmt::Debug for SESSION_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SESSION_ID")
            .field("SESSION_ID", &self.SESSION_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SESSION_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SESSION_ID {{ SESSION_ID: {=u32:?} }}",
            self.SESSION_ID()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "When set, indicates the ELS is executing a crypto sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_BUSY(&self) -> ELS_BUSY {
        let val = (self.0 >> 0usize) & 0x01;
        ELS_BUSY::from_bits(val as u8)
    }
    #[doc = "When set, indicates the ELS is executing a crypto sequence."]
    #[inline(always)]
    pub const fn set_ELS_BUSY(&mut self, val: ELS_BUSY) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "When set, indicates the ELS has an active interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_IRQ(&self) -> ELS_IRQ {
        let val = (self.0 >> 1usize) & 0x01;
        ELS_IRQ::from_bits(val as u8)
    }
    #[doc = "When set, indicates the ELS has an active interrupt."]
    #[inline(always)]
    pub const fn set_ELS_IRQ(&mut self, val: ELS_IRQ) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "When set, indicates the ELS has detected an internal error."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_ERR(&self) -> ELS_ERR {
        let val = (self.0 >> 2usize) & 0x01;
        ELS_ERR::from_bits(val as u8)
    }
    #[doc = "When set, indicates the ELS has detected an internal error."]
    #[inline(always)]
    pub const fn set_ELS_ERR(&mut self, val: ELS_ERR) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "When set, indicates the internal PRNG is ready."]
    #[must_use]
    #[inline(always)]
    pub const fn PRNG_RDY(&self) -> PRNG_RDY {
        let val = (self.0 >> 3usize) & 0x01;
        PRNG_RDY::from_bits(val as u8)
    }
    #[doc = "When set, indicates the internal PRNG is ready."]
    #[inline(always)]
    pub const fn set_PRNG_RDY(&mut self, val: PRNG_RDY) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Signature verify result status."]
    #[must_use]
    #[inline(always)]
    pub const fn ECDSA_VFY_STATUS(&self) -> ECDSA_VFY_STATUS {
        let val = (self.0 >> 4usize) & 0x03;
        ECDSA_VFY_STATUS::from_bits(val as u8)
    }
    #[doc = "Signature verify result status."]
    #[inline(always)]
    pub const fn set_ECDSA_VFY_STATUS(&mut self, val: ECDSA_VFY_STATUS) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Current command privilege level."]
    #[must_use]
    #[inline(always)]
    pub const fn PPROT(&self) -> PPROT {
        let val = (self.0 >> 6usize) & 0x03;
        PPROT::from_bits(val as u8)
    }
    #[doc = "Current command privilege level."]
    #[inline(always)]
    pub const fn set_PPROT(&mut self, val: PPROT) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Entropy quality of the current DRBG instance."]
    #[must_use]
    #[inline(always)]
    pub const fn DRBG_ENT_LVL(&self) -> DRBG_ENT_LVL {
        let val = (self.0 >> 8usize) & 0x03;
        DRBG_ENT_LVL::from_bits(val as u8)
    }
    #[doc = "Entropy quality of the current DRBG instance."]
    #[inline(always)]
    pub const fn set_DRBG_ENT_LVL(&mut self, val: DRBG_ENT_LVL) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "When set, it indicates TRNG is gathering entropy."]
    #[must_use]
    #[inline(always)]
    pub const fn DTRNG_BUSY(&self) -> DTRNG_BUSY {
        let val = (self.0 >> 10usize) & 0x01;
        DTRNG_BUSY::from_bits(val as u8)
    }
    #[doc = "When set, it indicates TRNG is gathering entropy."]
    #[inline(always)]
    pub const fn set_DTRNG_BUSY(&mut self, val: DTRNG_BUSY) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "When set, indicates that ELS is locked by a master."]
    #[must_use]
    #[inline(always)]
    pub const fn ELS_LOCKED(&self) -> ELS_LOCKED {
        let val = (self.0 >> 16usize) & 0x01;
        ELS_LOCKED::from_bits(val as u8)
    }
    #[doc = "When set, indicates that ELS is locked by a master."]
    #[inline(always)]
    pub const fn set_ELS_LOCKED(&mut self, val: ELS_LOCKED) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("ELS_BUSY", &self.ELS_BUSY())
            .field("ELS_IRQ", &self.ELS_IRQ())
            .field("ELS_ERR", &self.ELS_ERR())
            .field("PRNG_RDY", &self.PRNG_RDY())
            .field("ECDSA_VFY_STATUS", &self.ECDSA_VFY_STATUS())
            .field("PPROT", &self.PPROT())
            .field("DRBG_ENT_LVL", &self.DRBG_ENT_LVL())
            .field("DTRNG_BUSY", &self.DTRNG_BUSY())
            .field("ELS_LOCKED", &self.ELS_LOCKED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ ELS_BUSY: {:?}, ELS_IRQ: {:?}, ELS_ERR: {:?}, PRNG_RDY: {:?}, ECDSA_VFY_STATUS: {:?}, PPROT: {:?}, DRBG_ENT_LVL: {:?}, DTRNG_BUSY: {:?}, ELS_LOCKED: {:?} }}",
            self.ELS_BUSY(),
            self.ELS_IRQ(),
            self.ELS_ERR(),
            self.PRNG_RDY(),
            self.ECDSA_VFY_STATUS(),
            self.PPROT(),
            self.DRBG_ENT_LVL(),
            self.DTRNG_BUSY(),
            self.ELS_LOCKED()
        )
    }
}
#[doc = "Version Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VERSION(pub u32);
impl VERSION {
    #[doc = "Specifies the extended release version digit1; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn Z(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the extended release version digit1; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_Z(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Specifies the minor release version digit0; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn Y2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the minor release version digit0; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_Y2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Specifies the minor release version digit1; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn Y1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the minor release version digit1; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_Y1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Specifies the major release version; possible values are from 1-9."]
    #[must_use]
    #[inline(always)]
    pub const fn X(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the major release version; possible values are from 1-9."]
    #[inline(always)]
    pub const fn set_X(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Specifies the software extended revision version; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn SW_Z(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software extended revision version; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_SW_Z(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Specifies the software minor release version digit0; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn SW_Y2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software minor release version digit0; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_SW_Y2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Specifies the software minor release version digit1; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn SW_Y1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software minor release version digit1; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_SW_Y1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Specifies the software major release version; possible values are from 1-9."]
    #[must_use]
    #[inline(always)]
    pub const fn SW_X(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software major release version; possible values are from 1-9."]
    #[inline(always)]
    pub const fn set_SW_X(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for VERSION {
    #[inline(always)]
    fn default() -> VERSION {
        VERSION(0)
    }
}
impl core::fmt::Debug for VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("Z", &self.Z())
            .field("Y2", &self.Y2())
            .field("Y1", &self.Y1())
            .field("X", &self.X())
            .field("SW_Z", &self.SW_Z())
            .field("SW_Y2", &self.SW_Y2())
            .field("SW_Y1", &self.SW_Y1())
            .field("SW_X", &self.SW_X())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VERSION {{ Z: {=u8:?}, Y2: {=u8:?}, Y1: {=u8:?}, X: {=u8:?}, SW_Z: {=u8:?}, SW_Y2: {=u8:?}, SW_Y1: {=u8:?}, SW_X: {=u8:?} }}",
            self.Z(),
            self.Y2(),
            self.Y1(),
            self.X(),
            self.SW_Z(),
            self.SW_Y2(),
            self.SW_Y1(),
            self.SW_X()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ALG_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "Error occurred."]
    en = 0x01,
}
impl ALG_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ALG_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ALG_ERR {
    #[inline(always)]
    fn from(val: u8) -> ALG_ERR {
        ALG_ERR::from_bits(val)
    }
}
impl From<ALG_ERR> for u8 {
    #[inline(always)]
    fn from(val: ALG_ERR) -> u8 {
        ALG_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUS_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "Error occurred."]
    en = 0x01,
}
impl BUS_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUS_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUS_ERR {
    #[inline(always)]
    fn from(val: u8) -> BUS_ERR {
        BUS_ERR::from_bits(val)
    }
}
impl From<BUS_ERR> for u8 {
    #[inline(always)]
    fn from(val: BUS_ERR) -> u8 {
        BUS_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BYTE_ORDER {
    #[doc = "Little endian."]
    lit = 0x0,
    #[doc = "Big endian."]
    big = 0x01,
}
impl BYTE_ORDER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BYTE_ORDER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BYTE_ORDER {
    #[inline(always)]
    fn from(val: u8) -> BYTE_ORDER {
        BYTE_ORDER::from_bits(val)
    }
}
impl From<BYTE_ORDER> for u8 {
    #[inline(always)]
    fn from(val: BYTE_ORDER) -> u8 {
        BYTE_ORDER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDCRC_EN {
    #[doc = "Disables the CRC command CRC. The CRC command will not be updated on completion of each ELS command."]
    exit = 0x0,
    #[doc = "Enables the CRC command. The CRC command will be updated on completion of each ELS command."]
    clr = 0x01,
}
impl CMDCRC_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDCRC_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDCRC_EN {
    #[inline(always)]
    fn from(val: u8) -> CMDCRC_EN {
        CMDCRC_EN::from_bits(val)
    }
}
impl From<CMDCRC_EN> for u8 {
    #[inline(always)]
    fn from(val: CMDCRC_EN) -> u8 {
        CMDCRC_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDCRC_RST {
    #[doc = "No effect."]
    exit = 0x0,
    #[doc = "Resets the CRC command to its default value."]
    clr = 0x01,
}
impl CMDCRC_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDCRC_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDCRC_RST {
    #[inline(always)]
    fn from(val: u8) -> CMDCRC_RST {
        CMDCRC_RST::from_bits(val)
    }
}
impl From<CMDCRC_RST> for u8 {
    #[inline(always)]
    fn from(val: CMDCRC_RST) -> u8 {
        CMDCRC_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DRBG_ENT_LVL {
    #[doc = "NONE."]
    none = 0x0,
    #[doc = "LOW, DRBG generates random numbers of low quality entropy."]
    low = 0x01,
    #[doc = "HIGH, DRBG generates random numbers of high quality entropy."]
    high = 0x02,
    #[doc = "RFU, Reserved for Future Use."]
    rfu = 0x03,
}
impl DRBG_ENT_LVL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DRBG_ENT_LVL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DRBG_ENT_LVL {
    #[inline(always)]
    fn from(val: u8) -> DRBG_ENT_LVL {
        DRBG_ENT_LVL::from_bits(val)
    }
}
impl From<DRBG_ENT_LVL> for u8 {
    #[inline(always)]
    fn from(val: DRBG_ENT_LVL) -> u8 {
        DRBG_ENT_LVL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DTRNG_BUSY {
    #[doc = "Not gathering entropy."]
    notent = 0x0,
    #[doc = "Gathering entropy."]
    ent = 0x01,
}
impl DTRNG_BUSY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DTRNG_BUSY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DTRNG_BUSY {
    #[inline(always)]
    fn from(val: u8) -> DTRNG_BUSY {
        DTRNG_BUSY::from_bits(val)
    }
}
impl From<DTRNG_BUSY> for u8 {
    #[inline(always)]
    fn from(val: DTRNG_BUSY) -> u8 {
        DTRNG_BUSY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DTRNG_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "TRNG error occurred."]
    en = 0x01,
}
impl DTRNG_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DTRNG_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DTRNG_ERR {
    #[inline(always)]
    fn from(val: u8) -> DTRNG_ERR {
        DTRNG_ERR::from_bits(val)
    }
}
impl From<DTRNG_ERR> for u8 {
    #[inline(always)]
    fn from(val: DTRNG_ERR) -> u8 {
        DTRNG_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ECDSA_VFY_STATUS {
    #[doc = "No verify run."]
    no_v_run = 0x0,
    #[doc = "Signature verify failed."]
    sig_fail = 0x01,
    #[doc = "Signature verify passed."]
    sig_pass = 0x02,
    #[doc = "Invalid, Error."]
    err = 0x03,
}
impl ECDSA_VFY_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ECDSA_VFY_STATUS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ECDSA_VFY_STATUS {
    #[inline(always)]
    fn from(val: u8) -> ECDSA_VFY_STATUS {
        ECDSA_VFY_STATUS::from_bits(val)
    }
}
impl From<ECDSA_VFY_STATUS> for u8 {
    #[inline(always)]
    fn from(val: ECDSA_VFY_STATUS) -> u8 {
        ECDSA_VFY_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ELS_BUSY {
    #[doc = "Crypto sequence not executing."]
    ntcry = 0x0,
    #[doc = "Crypto sequence executing."]
    cryp = 0x01,
}
impl ELS_BUSY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ELS_BUSY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ELS_BUSY {
    #[inline(always)]
    fn from(val: u8) -> ELS_BUSY {
        ELS_BUSY::from_bits(val)
    }
}
impl From<ELS_BUSY> for u8 {
    #[inline(always)]
    fn from(val: ELS_BUSY) -> u8 {
        ELS_BUSY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ELS_EN {
    #[doc = "Disabled."]
    dis = 0x0,
    #[doc = "Enabled."]
    en = 0x01,
}
impl ELS_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ELS_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ELS_EN {
    #[inline(always)]
    fn from(val: u8) -> ELS_EN {
        ELS_EN::from_bits(val)
    }
}
impl From<ELS_EN> for u8 {
    #[inline(always)]
    fn from(val: ELS_EN) -> u8 {
        ELS_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ELS_ERR {
    #[doc = "Internal error not detected."]
    nterr = 0x0,
    #[doc = "Internal error detected."]
    err = 0x01,
}
impl ELS_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ELS_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ELS_ERR {
    #[inline(always)]
    fn from(val: u8) -> ELS_ERR {
        ELS_ERR::from_bits(val)
    }
}
impl From<ELS_ERR> for u8 {
    #[inline(always)]
    fn from(val: ELS_ERR) -> u8 {
        ELS_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ELS_IRQ {
    #[doc = "No active interrupt."]
    ntint = 0x0,
    #[doc = "Active interrupt."]
    int = 0x01,
}
impl ELS_IRQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ELS_IRQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ELS_IRQ {
    #[inline(always)]
    fn from(val: u8) -> ELS_IRQ {
        ELS_IRQ::from_bits(val)
    }
}
impl From<ELS_IRQ> for u8 {
    #[inline(always)]
    fn from(val: ELS_IRQ) -> u8 {
        ELS_IRQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ELS_LOCKED {
    #[doc = "Not locked by master."]
    notl = 0x0,
    #[doc = "Locked by master."]
    lock = 0x01,
}
impl ELS_LOCKED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ELS_LOCKED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ELS_LOCKED {
    #[inline(always)]
    fn from(val: u8) -> ELS_LOCKED {
        ELS_LOCKED::from_bits(val)
    }
}
impl From<ELS_LOCKED> for u8 {
    #[inline(always)]
    fn from(val: ELS_LOCKED) -> u8 {
        ELS_LOCKED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ERR_CLR {
    #[doc = "Exits ELS error state."]
    exit = 0x0,
    #[doc = "Clears ELS error state."]
    clr = 0x01,
}
impl ERR_CLR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ERR_CLR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ERR_CLR {
    #[inline(always)]
    fn from(val: u8) -> ERR_CLR {
        ERR_CLR::from_bits(val)
    }
}
impl From<ERR_CLR> for u8 {
    #[inline(always)]
    fn from(val: ERR_CLR) -> u8 {
        ERR_CLR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLT_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "Error occurred."]
    en = 0x01,
}
impl FLT_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLT_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLT_ERR {
    #[inline(always)]
    fn from(val: u8) -> FLT_ERR {
        FLT_ERR::from_bits(val)
    }
}
impl From<FLT_ERR> for u8 {
    #[inline(always)]
    fn from(val: FLT_ERR) -> u8 {
        FLT_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT_EN {
    #[doc = "Disables."]
    dis = 0x0,
    #[doc = "Enables."]
    en = 0x01,
}
impl INT_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT_EN {
    #[inline(always)]
    fn from(val: u8) -> INT_EN {
        INT_EN::from_bits(val)
    }
}
impl From<INT_EN> for u8 {
    #[inline(always)]
    fn from(val: INT_EN) -> u8 {
        INT_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ITG_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "Error occurred."]
    en = 0x01,
}
impl ITG_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ITG_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ITG_ERR {
    #[inline(always)]
    fn from(val: u8) -> ITG_ERR {
        ITG_ERR::from_bits(val)
    }
}
impl From<ITG_ERR> for u8 {
    #[inline(always)]
    fn from(val: ITG_ERR) -> u8 {
        ITG_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KSIZE {
    #[doc = "128."]
    size128 = 0x0,
    #[doc = "256."]
    size256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KSIZE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KSIZE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KSIZE {
    #[inline(always)]
    fn from(val: u8) -> KSIZE {
        KSIZE::from_bits(val)
    }
}
impl From<KSIZE> for u8 {
    #[inline(always)]
    fn from(val: KSIZE) -> u8 {
        KSIZE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OPN_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "Error occurred."]
    en = 0x01,
}
impl OPN_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OPN_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OPN_ERR {
    #[inline(always)]
    fn from(val: u8) -> OPN_ERR {
        OPN_ERR::from_bits(val)
    }
}
impl From<OPN_ERR> for u8 {
    #[inline(always)]
    fn from(val: OPN_ERR) -> u8 {
        OPN_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PPROT {
    #[doc = "Secure, non-privileged."]
    secnp = 0x0,
    #[doc = "Secure, privileged."]
    secp = 0x01,
    #[doc = "Non-secure, non-privileged."]
    nsecnp = 0x02,
    #[doc = "Non-secure, privileged."]
    nsecp = 0x03,
}
impl PPROT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PPROT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PPROT {
    #[inline(always)]
    fn from(val: u8) -> PPROT {
        PPROT::from_bits(val)
    }
}
impl From<PPROT> for u8 {
    #[inline(always)]
    fn from(val: PPROT) -> u8 {
        PPROT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PRNG_ERR {
    #[doc = "No error."]
    dis = 0x0,
    #[doc = "Error occurred."]
    en = 0x01,
}
impl PRNG_ERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PRNG_ERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PRNG_ERR {
    #[inline(always)]
    fn from(val: u8) -> PRNG_ERR {
        PRNG_ERR::from_bits(val)
    }
}
impl From<PRNG_ERR> for u8 {
    #[inline(always)]
    fn from(val: PRNG_ERR) -> u8 {
        PRNG_ERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PRNG_RDY {
    #[doc = "Internal PRNG not ready."]
    ntready = 0x0,
    #[doc = "Internal PRNG ready."]
    ready = 0x01,
}
impl PRNG_RDY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PRNG_RDY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PRNG_RDY {
    #[inline(always)]
    fn from(val: u8) -> PRNG_RDY {
        PRNG_RDY::from_bits(val)
    }
}
impl From<PRNG_RDY> for u8 {
    #[inline(always)]
    fn from(val: PRNG_RDY) -> u8 {
        PRNG_RDY::to_bits(val)
    }
}
