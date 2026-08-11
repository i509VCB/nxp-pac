#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Enhanced Serial Peripheral Interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espi {
    ptr: *mut u8,
}
unsafe impl Send for Espi {}
unsafe impl Sync for Espi {}
impl Espi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Control."]
    #[inline(always)]
    pub const fn MCTRL(self) -> crate::pac::common::Reg<MCTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Master Status."]
    #[inline(always)]
    pub const fn MSTAT(self) -> crate::pac::common::Reg<MSTAT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Enable Set."]
    #[inline(always)]
    pub const fn INTENSET(self) -> crate::pac::common::Reg<INTENSET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Clear."]
    #[inline(always)]
    pub const fn INTENCLR(self) -> crate::pac::common::Reg<INTENCLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Masked Interrupt Status."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::pac::common::Reg<INTSTAT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMA Control."]
    #[inline(always)]
    pub const fn DMACTRL(self) -> crate::pac::common::Reg<DMACTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RAM Base."]
    #[inline(always)]
    pub const fn RAMBASE(self) -> crate::pac::common::Reg<RAMBASE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Mapped Base."]
    #[inline(always)]
    pub const fn MAPBASE(self) -> crate::pac::common::Reg<MAPBASE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "IRQ Push."]
    #[inline(always)]
    pub const fn IRQPUSH(self) -> crate::pac::common::Reg<IRQPUSH, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Virtual Wire MCU-to-host."]
    #[inline(always)]
    pub const fn WIREWO(self) -> crate::pac::common::Reg<WIREWO, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Virtual Wire Host-to-MCU."]
    #[inline(always)]
    pub const fn WIRERO(self) -> crate::pac::common::Reg<WIRERO, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Port 80 Status."]
    #[inline(always)]
    pub const fn P80STAT(self) -> crate::pac::common::Reg<P80STAT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Status Block Address."]
    #[inline(always)]
    pub const fn STATADDR(self) -> crate::pac::common::Reg<STATADDR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "eSPI Capabilities."]
    #[inline(always)]
    pub const fn ESPICAP(self) -> crate::pac::common::Reg<ESPICAP, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "eSPI Configuration."]
    #[inline(always)]
    pub const fn ESPICFG(self) -> crate::pac::common::Reg<ESPICFG, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "eSPI Miscellaneous."]
    #[inline(always)]
    pub const fn ESPIMISC(self) -> crate::pac::common::Reg<ESPIMISC, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "RPMC Support 1."]
    #[inline(always)]
    pub const fn RPMC_SUPPORT1(
        self,
    ) -> crate::pac::common::Reg<RPMC_SUPPORT1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RPMC Support 2."]
    #[inline(always)]
    pub const fn RPMC_SUPPORT2(
        self,
    ) -> crate::pac::common::Reg<RPMC_SUPPORT2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "WIREIN_GPIO."]
    #[inline(always)]
    pub const fn WIREIN_GPIO(self) -> crate::pac::common::Reg<WIREIN_GPIO, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "WIREOUT_GPIO."]
    #[inline(always)]
    pub const fn WIREOUT_GPIO(
        self,
    ) -> crate::pac::common::Reg<WIREOUT_GPIO, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Port registers."]
    #[inline(always)]
    pub const fn PORT(self, n: usize) -> Port {
        assert!(n < 5usize);
        unsafe { Port::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 32usize) as _) }
    }
}
#[doc = "Port registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port {
    ptr: *mut u8,
}
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
impl Port {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port Configuration."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::pac::common::Reg<CFG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Port Status."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::pac::common::Reg<STAT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Set Interrupt Rules and User Status."]
    #[inline(always)]
    pub const fn IRULESTAT(self) -> crate::pac::common::Reg<IRULESTAT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Port Address."]
    #[inline(always)]
    pub const fn ADDR(self) -> crate::pac::common::Reg<ADDR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Port OOB, Mastering, and Flash Length."]
    #[inline(always)]
    pub const fn OMFLEN(self) -> crate::pac::common::Reg<OMFLEN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn DATAIN(self) -> crate::pac::common::Reg<DATAIN, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Port Data Out."]
    #[inline(always)]
    pub const fn DATAOUT(self) -> crate::pac::common::Reg<DATAOUT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Port RAM Use."]
    #[inline(always)]
    pub const fn RAMUSE(self) -> crate::pac::common::Reg<RAMUSE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
#[doc = "Port Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADDR(pub u32);
impl ADDR {
    #[doc = "Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn OFF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset."]
    #[inline(always)]
    pub const fn set_OFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE_ASZ(&self) -> ADDR_BASE_ASZ {
        let val = (self.0 >> 16usize) & 0x03;
        ADDR_BASE_ASZ::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:."]
    #[inline(always)]
    pub const fn set_BASE_ASZ(&mut self, val: ADDR_BASE_ASZ) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Index Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn IDXOFF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Index Offset."]
    #[inline(always)]
    pub const fn set_IDXOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index-and-data register only:."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX1ST(&self) -> ADDR_IDX1ST {
        let val = (self.0 >> 28usize) & 0x01;
        ADDR_IDX1ST::from_bits(val as u8)
    }
    #[doc = "For index-and-data register only:."]
    #[inline(always)]
    pub const fn set_IDX1ST(&mut self, val: ADDR_IDX1ST) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for ADDR {
    #[inline(always)]
    fn default() -> ADDR {
        ADDR(0)
    }
}
impl core::fmt::Debug for ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR")
            .field("OFF", &self.OFF())
            .field("BASE_ASZ", &self.BASE_ASZ())
            .field("IDXOFF", &self.IDXOFF())
            .field("IDX1ST", &self.IDX1ST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADDR {{ OFF: {=u16:?}, BASE_ASZ: {:?}, IDXOFF: {=u8:?}, IDX1ST: {:?} }}",
            self.OFF(),
            self.BASE_ASZ(),
            self.IDXOFF(),
            self.IDX1ST()
        )
    }
}
#[doc = "Port Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "Interaction Type between Port and Host."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> CFG_TYPE {
        let val = (self.0 >> 0usize) & 0x0f;
        CFG_TYPE::from_bits(val as u8)
    }
    #[doc = "Interaction Type between Port and Host."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: CFG_TYPE) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn DIRECTION(&self) -> CFG_DIRECTION {
        let val = (self.0 >> 5usize) & 0x03;
        CFG_DIRECTION::from_bits(val as u8)
    }
    #[doc = "Port Direction."]
    #[inline(always)]
    pub const fn set_DIRECTION(&mut self, val: CFG_DIRECTION) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: map interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn MBINTALL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: map interrupt."]
    #[inline(always)]
    pub const fn set_MBINTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any read."]
    #[must_use]
    #[inline(always)]
    pub const fn STALLRD(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any read."]
    #[inline(always)]
    pub const fn set_STALLRD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on write."]
    #[must_use]
    #[inline(always)]
    pub const fn STALLWR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on write."]
    #[inline(always)]
    pub const fn set_STALLWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Origin."]
    #[must_use]
    #[inline(always)]
    pub const fn ERRORIGN(&self) -> CFG_ERRORIGN {
        let val = (self.0 >> 10usize) & 0x01;
        CFG_ERRORIGN::from_bits(val as u8)
    }
    #[doc = "Error Origin."]
    #[inline(always)]
    pub const fn set_ERRORIGN(&mut self, val: CFG_ERRORIGN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
            .field("TYPE", &self.TYPE())
            .field("DIRECTION", &self.DIRECTION())
            .field("MBINTALL", &self.MBINTALL())
            .field("STALLRD", &self.STALLRD())
            .field("STALLWR", &self.STALLWR())
            .field("ERRORIGN", &self.ERRORIGN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ TYPE: {:?}, DIRECTION: {:?}, MBINTALL: {=bool:?}, STALLRD: {=bool:?}, STALLWR: {=bool:?}, ERRORIGN: {:?} }}",
            self.TYPE(),
            self.DIRECTION(),
            self.MBINTALL(),
            self.STALLRD(),
            self.STALLWR(),
            self.ERRORIGN()
        )
    }
}
#[doc = "Port Data Input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATAIN(pub u32);
impl DATAIN {
    #[doc = "Data Length."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_LEN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Length."]
    #[inline(always)]
    pub const fn set_DATA_LEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access."]
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> DATAIN_DIR {
        let val = (self.0 >> 8usize) & 0x01;
        DATAIN_DIR::from_bits(val as u8)
    }
    #[doc = "Direction of last access."]
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: DATAIN_DIR) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of Last Access."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of Last Access."]
    #[inline(always)]
    pub const fn set_IDX(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "SAF transaction tag (undocumented in the SVD; used by the SDK flash path)."]
    #[must_use]
    #[inline(always)]
    pub const fn TAG(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x0f;
        val as u8
    }
    #[doc = "SAF transaction tag (undocumented in the SVD; used by the SDK flash path)."]
    #[inline(always)]
    pub const fn set_TAG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
    }
}
impl Default for DATAIN {
    #[inline(always)]
    fn default() -> DATAIN {
        DATAIN(0)
    }
}
impl core::fmt::Debug for DATAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAIN")
            .field("DATA_LEN", &self.DATA_LEN())
            .field("DIR", &self.DIR())
            .field("IDX", &self.IDX())
            .field("TAG", &self.TAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATAIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DATAIN {{ DATA_LEN: {=u8:?}, DIR: {:?}, IDX: {=u16:?}, TAG: {=u8:?} }}",
            self.DATA_LEN(),
            self.DIR(),
            self.IDX(),
            self.TAG()
        )
    }
}
#[doc = "Port Data Out."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATAOUT(pub u32);
impl DATAOUT {
    #[doc = "Data to Send to Host."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to Send to Host."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DATAOUT {
    #[inline(always)]
    fn default() -> DATAOUT {
        DATAOUT(0)
    }
}
impl core::fmt::Debug for DATAOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAOUT")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATAOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DATAOUT {{ DATA: {=u8:?} }}", self.DATA())
    }
}
#[doc = "DMA Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACTRL(pub u32);
impl DMACTRL {
    #[doc = "DMA 0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA0EN(&self) -> DMA0EN {
        let val = (self.0 >> 0usize) & 0x03;
        DMA0EN::from_bits(val as u8)
    }
    #[doc = "DMA 0 Enable."]
    #[inline(always)]
    pub const fn set_DMA0EN(&mut self, val: DMA0EN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA 1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA1EN(&self) -> DMA1EN {
        let val = (self.0 >> 2usize) & 0x03;
        DMA1EN::from_bits(val as u8)
    }
    #[doc = "DMA 1 Enable."]
    #[inline(always)]
    pub const fn set_DMA1EN(&mut self, val: DMA1EN) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DMA 0 Port."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA0PORT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DMA 0 Port."]
    #[inline(always)]
    pub const fn set_DMA0PORT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Selects port operating DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA1PORT(&self) -> DMA1PORT {
        let val = (self.0 >> 12usize) & 0x0f;
        DMA1PORT::from_bits(val as u8)
    }
    #[doc = "Selects port operating DMA."]
    #[inline(always)]
    pub const fn set_DMA1PORT(&mut self, val: DMA1PORT) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Reload Count 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Reload Count 0."]
    #[inline(always)]
    pub const fn set_CNT0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Reload Count 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Reload Count 1."]
    #[inline(always)]
    pub const fn set_CNT1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for DMACTRL {
    #[inline(always)]
    fn default() -> DMACTRL {
        DMACTRL(0)
    }
}
impl core::fmt::Debug for DMACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTRL")
            .field("DMA0EN", &self.DMA0EN())
            .field("DMA1EN", &self.DMA1EN())
            .field("DMA0PORT", &self.DMA0PORT())
            .field("DMA1PORT", &self.DMA1PORT())
            .field("CNT0", &self.CNT0())
            .field("CNT1", &self.CNT1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACTRL {{ DMA0EN: {:?}, DMA1EN: {:?}, DMA0PORT: {=u8:?}, DMA1PORT: {:?}, CNT0: {=u8:?}, CNT1: {=u8:?} }}",
            self.DMA0EN(),
            self.DMA1EN(),
            self.DMA0PORT(),
            self.DMA1PORT(),
            self.CNT0(),
            self.CNT1()
        )
    }
}
#[doc = "eSPI Capabilities."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ESPICAP(pub u32);
impl ESPICAP {
    #[doc = "SPI mode allowed (host still has to select):."]
    #[must_use]
    #[inline(always)]
    pub const fn SPICAP(&self) -> SPICAP {
        let val = (self.0 >> 0usize) & 0x03;
        SPICAP::from_bits(val as u8)
    }
    #[doc = "SPI mode allowed (host still has to select):."]
    #[inline(always)]
    pub const fn set_SPICAP(&mut self, val: SPICAP) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Maximum SPI Clock Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn MAXSPD(&self) -> MAXSPD {
        let val = (self.0 >> 4usize) & 0x07;
        MAXSPD::from_bits(val as u8)
    }
    #[doc = "Maximum SPI Clock Speed."]
    #[inline(always)]
    pub const fn set_MAXSPD(&mut self, val: MAXSPD) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Alert Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn ALPIN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Pin."]
    #[inline(always)]
    pub const fn set_ALPIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OOB Allow."]
    #[must_use]
    #[inline(always)]
    pub const fn OOBOK(&self) -> OOBOK {
        let val = (self.0 >> 8usize) & 0x01;
        OOBOK::from_bits(val as u8)
    }
    #[doc = "OOB Allow."]
    #[inline(always)]
    pub const fn set_OOBOK(&mut self, val: OOBOK) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Maximum Flash Payload Size."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHMX(&self) -> FLASHMX {
        let val = (self.0 >> 10usize) & 0x03;
        FLASHMX::from_bits(val as u8)
    }
    #[doc = "Maximum Flash Payload Size."]
    #[inline(always)]
    pub const fn set_FLASHMX(&mut self, val: FLASHMX) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Slave-Attached Flash."]
    #[must_use]
    #[inline(always)]
    pub const fn SAF(&self) -> ESPICAP_SAF {
        let val = (self.0 >> 12usize) & 0x01;
        ESPICAP_SAF::from_bits(val as u8)
    }
    #[doc = "Slave-Attached Flash."]
    #[inline(always)]
    pub const fn set_SAF(&mut self, val: ESPICAP_SAF) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "SAF Erase Sector."]
    #[must_use]
    #[inline(always)]
    pub const fn SAFERA(&self) -> SAFERA {
        let val = (self.0 >> 13usize) & 0x0f;
        SAFERA::from_bits(val as u8)
    }
    #[doc = "SAF Erase Sector."]
    #[inline(always)]
    pub const fn set_SAFERA(&mut self, val: SAFERA) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val.to_bits() as u32) & 0x0f) << 13usize);
    }
    #[doc = "Master-Attached Flash."]
    #[must_use]
    #[inline(always)]
    pub const fn MAF(&self) -> MAF {
        let val = (self.0 >> 17usize) & 0x01;
        MAF::from_bits(val as u8)
    }
    #[doc = "Master-Attached Flash."]
    #[inline(always)]
    pub const fn set_MAF(&mut self, val: MAF) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Target Maximum Read Request Size Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn TRGT_REQ_SIZE_SUPP(&self) -> TRGT_REQ_SIZE_SUPP {
        let val = (self.0 >> 18usize) & 0x07;
        TRGT_REQ_SIZE_SUPP::from_bits(val as u8)
    }
    #[doc = "Target Maximum Read Request Size Supported."]
    #[inline(always)]
    pub const fn set_TRGT_REQ_SIZE_SUPP(&mut self, val: TRGT_REQ_SIZE_SUPP) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Peripheral Channel Maximum Payload Size Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMMX(&self) -> MEMMX {
        let val = (self.0 >> 21usize) & 0x03;
        MEMMX::from_bits(val as u8)
    }
    #[doc = "Peripheral Channel Maximum Payload Size Supported."]
    #[inline(always)]
    pub const fn set_MEMMX(&mut self, val: MEMMX) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "OOB Message Channel Maximum Payload Size Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn OOBMX(&self) -> OOBMX {
        let val = (self.0 >> 23usize) & 0x03;
        OOBMX::from_bits(val as u8)
    }
    #[doc = "OOB Message Channel Maximum Payload Size Supported."]
    #[inline(always)]
    pub const fn set_OOBMX(&mut self, val: OOBMX) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val.to_bits() as u32) & 0x03) << 23usize);
    }
}
impl Default for ESPICAP {
    #[inline(always)]
    fn default() -> ESPICAP {
        ESPICAP(0)
    }
}
impl core::fmt::Debug for ESPICAP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPICAP")
            .field("SPICAP", &self.SPICAP())
            .field("MAXSPD", &self.MAXSPD())
            .field("ALPIN", &self.ALPIN())
            .field("OOBOK", &self.OOBOK())
            .field("FLASHMX", &self.FLASHMX())
            .field("SAF", &self.SAF())
            .field("SAFERA", &self.SAFERA())
            .field("MAF", &self.MAF())
            .field("TRGT_REQ_SIZE_SUPP", &self.TRGT_REQ_SIZE_SUPP())
            .field("MEMMX", &self.MEMMX())
            .field("OOBMX", &self.OOBMX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ESPICAP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ESPICAP {{ SPICAP: {:?}, MAXSPD: {:?}, ALPIN: {=bool:?}, OOBOK: {:?}, FLASHMX: {:?}, SAF: {:?}, SAFERA: {:?}, MAF: {:?}, TRGT_REQ_SIZE_SUPP: {:?}, MEMMX: {:?}, OOBMX: {:?} }}",
            self.SPICAP(),
            self.MAXSPD(),
            self.ALPIN(),
            self.OOBOK(),
            self.FLASHMX(),
            self.SAF(),
            self.SAFERA(),
            self.MAF(),
            self.TRGT_REQ_SIZE_SUPP(),
            self.MEMMX(),
            self.OOBMX()
        )
    }
}
#[doc = "eSPI Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ESPICFG(pub u32);
impl ESPICFG {
    #[doc = "Slave-Attached Flash Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn SAF(&self) -> ESPICFG_SAF {
        let val = (self.0 >> 0usize) & 0x01;
        ESPICFG_SAF::from_bits(val as u8)
    }
    #[doc = "Slave-Attached Flash Supported."]
    #[inline(always)]
    pub const fn set_SAF(&mut self, val: ESPICFG_SAF) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash Size."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHSZ(&self) -> FLASHSZ {
        let val = (self.0 >> 1usize) & 0x03;
        FLASHSZ::from_bits(val as u8)
    }
    #[doc = "Flash Size."]
    #[inline(always)]
    pub const fn set_FLASHSZ(&mut self, val: FLASHSZ) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SPI Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SPIMOD(&self) -> SPIMOD {
        let val = (self.0 >> 3usize) & 0x03;
        SPIMOD::from_bits(val as u8)
    }
    #[doc = "SPI Mode."]
    #[inline(always)]
    pub const fn set_SPIMOD(&mut self, val: SPIMOD) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Alert Is Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn ALERT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Is Pin."]
    #[inline(always)]
    pub const fn set_ALERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Alert Is Open Drain as Pin."]
    #[must_use]
    #[inline(always)]
    pub const fn ALERTOD(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Is Open Drain as Pin."]
    #[inline(always)]
    pub const fn set_ALERTOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SPI Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn SPISPD(&self) -> SPISPD {
        let val = (self.0 >> 7usize) & 0x07;
        SPISPD::from_bits(val as u8)
    }
    #[doc = "SPI Speed."]
    #[inline(always)]
    pub const fn set_SPISPD(&mut self, val: SPISPD) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "CRC Checking Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CRC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Checking Enabled."]
    #[inline(always)]
    pub const fn set_CRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus Master OK."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSMOK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Master OK."]
    #[inline(always)]
    pub const fn set_BUSMOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 0 (Memory) Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMENA(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 (Memory) Enabled."]
    #[inline(always)]
    pub const fn set_MEMENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 1 (VWire) Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn VWOK(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 (VWire) Enabled."]
    #[inline(always)]
    pub const fn set_VWOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 2 (OOB) Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn OOBOK(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 (OOB) Enabled."]
    #[inline(always)]
    pub const fn set_OOBOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Flash Erase Size and Whether Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn FLSHERA(&self) -> FLSHERA {
        let val = (self.0 >> 15usize) & 0x07;
        FLSHERA::from_bits(val as u8)
    }
    #[doc = "Flash Erase Size and Whether Enabled."]
    #[inline(always)]
    pub const fn set_FLSHERA(&mut self, val: FLSHERA) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Channel 3 (Flash) Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn FLSHOK(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 (Flash) Enabled."]
    #[inline(always)]
    pub const fn set_FLSHOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Peripheral Channel Maximum Payload Size Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMSZ(&self) -> MEMSZ {
        let val = (self.0 >> 19usize) & 0x03;
        MEMSZ::from_bits(val as u8)
    }
    #[doc = "Peripheral Channel Maximum Payload Size Selected."]
    #[inline(always)]
    pub const fn set_MEMSZ(&mut self, val: MEMSZ) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "OOB Message Channel Maximum Payload Size Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn OOBSZ(&self) -> OOBSZ {
        let val = (self.0 >> 21usize) & 0x03;
        OOBSZ::from_bits(val as u8)
    }
    #[doc = "OOB Message Channel Maximum Payload Size Selected."]
    #[inline(always)]
    pub const fn set_OOBSZ(&mut self, val: OOBSZ) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for ESPICFG {
    #[inline(always)]
    fn default() -> ESPICFG {
        ESPICFG(0)
    }
}
impl core::fmt::Debug for ESPICFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPICFG")
            .field("SAF", &self.SAF())
            .field("FLASHSZ", &self.FLASHSZ())
            .field("SPIMOD", &self.SPIMOD())
            .field("ALERT", &self.ALERT())
            .field("ALERTOD", &self.ALERTOD())
            .field("SPISPD", &self.SPISPD())
            .field("CRC", &self.CRC())
            .field("BUSMOK", &self.BUSMOK())
            .field("MEMENA", &self.MEMENA())
            .field("VWOK", &self.VWOK())
            .field("OOBOK", &self.OOBOK())
            .field("FLSHERA", &self.FLSHERA())
            .field("FLSHOK", &self.FLSHOK())
            .field("MEMSZ", &self.MEMSZ())
            .field("OOBSZ", &self.OOBSZ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ESPICFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ESPICFG {{ SAF: {:?}, FLASHSZ: {:?}, SPIMOD: {:?}, ALERT: {=bool:?}, ALERTOD: {=bool:?}, SPISPD: {:?}, CRC: {=bool:?}, BUSMOK: {=bool:?}, MEMENA: {=bool:?}, VWOK: {=bool:?}, OOBOK: {=bool:?}, FLSHERA: {:?}, FLSHOK: {=bool:?}, MEMSZ: {:?}, OOBSZ: {:?} }}",
            self.SAF(),
            self.FLASHSZ(),
            self.SPIMOD(),
            self.ALERT(),
            self.ALERTOD(),
            self.SPISPD(),
            self.CRC(),
            self.BUSMOK(),
            self.MEMENA(),
            self.VWOK(),
            self.OOBOK(),
            self.FLSHERA(),
            self.FLSHOK(),
            self.MEMSZ(),
            self.OOBSZ()
        )
    }
}
#[doc = "eSPI Miscellaneous."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ESPIMISC(pub u32);
impl ESPIMISC {
    #[doc = "GPIO Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_OE(&self) -> GPIO_OE {
        let val = (self.0 >> 0usize) & 0x01;
        GPIO_OE::from_bits(val as u8)
    }
    #[doc = "GPIO Output Enable."]
    #[inline(always)]
    pub const fn set_GPIO_OE(&mut self, val: GPIO_OE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO Open Drain."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_OD(&self) -> GPIO_OD {
        let val = (self.0 >> 1usize) & 0x01;
        GPIO_OD::from_bits(val as u8)
    }
    #[doc = "GPIO Open Drain."]
    #[inline(always)]
    pub const fn set_GPIO_OD(&mut self, val: GPIO_OD) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Not used in eSPI if ESPICFG\\[ALERT\\] is 1."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_OUT(&self) -> GPIO_OUT {
        let val = (self.0 >> 2usize) & 0x01;
        GPIO_OUT::from_bits(val as u8)
    }
    #[doc = "Not used in eSPI if ESPICFG\\[ALERT\\] is 1."]
    #[inline(always)]
    pub const fn set_GPIO_OUT(&mut self, val: GPIO_OUT) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO Input."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_IN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Input."]
    #[inline(always)]
    pub const fn set_GPIO_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RSTN Pin Is GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn RISGP(&self) -> RISGP {
        let val = (self.0 >> 4usize) & 0x01;
        RISGP::from_bits(val as u8)
    }
    #[doc = "RSTN Pin Is GPIO."]
    #[inline(always)]
    pub const fn set_RISGP(&mut self, val: RISGP) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Power Save."]
    #[must_use]
    #[inline(always)]
    pub const fn PWRSAV(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Power Save."]
    #[inline(always)]
    pub const fn set_PWRSAV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ESPIMISC {
    #[inline(always)]
    fn default() -> ESPIMISC {
        ESPIMISC(0)
    }
}
impl core::fmt::Debug for ESPIMISC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPIMISC")
            .field("GPIO_OE", &self.GPIO_OE())
            .field("GPIO_OD", &self.GPIO_OD())
            .field("GPIO_OUT", &self.GPIO_OUT())
            .field("GPIO_IN", &self.GPIO_IN())
            .field("RISGP", &self.RISGP())
            .field("PWRSAV", &self.PWRSAV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ESPIMISC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ESPIMISC {{ GPIO_OE: {:?}, GPIO_OD: {:?}, GPIO_OUT: {:?}, GPIO_IN: {=bool:?}, RISGP: {:?}, PWRSAV: {=bool:?} }}",
            self.GPIO_OE(),
            self.GPIO_OD(),
            self.GPIO_OUT(),
            self.GPIO_IN(),
            self.RISGP(),
            self.PWRSAV()
        )
    }
}
#[doc = "Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR(pub u32);
impl INTENCLR {
    #[doc = "Port Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PORTINT(&self) -> INTENCLR_PORTINT {
        let val = (self.0 >> 0usize) & 0x1f;
        INTENCLR_PORTINT::from_bits(val as u8)
    }
    #[doc = "Port Interrupt."]
    #[inline(always)]
    pub const fn set_PORTINT(&mut self, val: INTENCLR_PORTINT) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn P80INT(&self) -> INTENCLR_P80INT {
        let val = (self.0 >> 8usize) & 0x01;
        INTENCLR_P80INT::from_bits(val as u8)
    }
    #[doc = "Port80 Interrupt."]
    #[inline(always)]
    pub const fn set_P80INT(&mut self, val: INTENCLR_P80INT) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSRST(&self) -> INTENCLR_BUSRST {
        let val = (self.0 >> 9usize) & 0x01;
        INTENCLR_BUSRST::from_bits(val as u8)
    }
    #[doc = "Bus Reset."]
    #[inline(always)]
    pub const fn set_BUSRST(&mut self, val: INTENCLR_BUSRST) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ Update."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQUPD(&self) -> INTENCLR_IRQUPD {
        let val = (self.0 >> 10usize) & 0x01;
        INTENCLR_IRQUPD::from_bits(val as u8)
    }
    #[doc = "IRQ Update."]
    #[inline(always)]
    pub const fn set_IRQUPD(&mut self, val: INTENCLR_IRQUPD) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn WIRECHG(&self) -> INTENCLR_WIRECHG {
        let val = (self.0 >> 11usize) & 0x01;
        INTENCLR_WIRECHG::from_bits(val as u8)
    }
    #[doc = "Wire Change."]
    #[inline(always)]
    pub const fn set_WIRECHG(&mut self, val: INTENCLR_WIRECHG) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTALL(&self) -> INTENCLR_HSTALL {
        let val = (self.0 >> 12usize) & 0x01;
        INTENCLR_HSTALL::from_bits(val as u8)
    }
    #[doc = "Host Stall."]
    #[inline(always)]
    pub const fn set_HSTALL(&mut self, val: INTENCLR_HSTALL) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCERR(&self) -> INTENCLR_CRCERR {
        let val = (self.0 >> 13usize) & 0x01;
        INTENCLR_CRCERR::from_bits(val as u8)
    }
    #[doc = "CRC Error."]
    #[inline(always)]
    pub const fn set_CRCERR(&mut self, val: INTENCLR_CRCERR) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO(&self) -> INTENCLR_GPIO {
        let val = (self.0 >> 14usize) & 0x01;
        INTENCLR_GPIO::from_bits(val as u8)
    }
    #[doc = "GPIO."]
    #[inline(always)]
    pub const fn set_GPIO(&mut self, val: INTENCLR_GPIO) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "CS_INT."]
    #[must_use]
    #[inline(always)]
    pub const fn CS_INT(&self) -> INTENCLR_CS_INT {
        let val = (self.0 >> 15usize) & 0x01;
        INTENCLR_CS_INT::from_bits(val as u8)
    }
    #[doc = "CS_INT."]
    #[inline(always)]
    pub const fn set_CS_INT(&mut self, val: INTENCLR_CS_INT) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Coprocessor Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CPORTINT(&self) -> INTENCLR_CPORTINT {
        let val = (self.0 >> 16usize) & 0x1f;
        INTENCLR_CPORTINT::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt."]
    #[inline(always)]
    pub const fn set_CPORTINT(&mut self, val: INTENCLR_CPORTINT) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Coprocesssor Port80 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CP80INT(&self) -> INTENCLR_CP80INT {
        let val = (self.0 >> 24usize) & 0x01;
        INTENCLR_CP80INT::from_bits(val as u8)
    }
    #[doc = "Coprocesssor Port80 Interrupt."]
    #[inline(always)]
    pub const fn set_CP80INT(&mut self, val: INTENCLR_CP80INT) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Coprocessor Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn CBUSRST(&self) -> INTENCLR_CBUSRST {
        let val = (self.0 >> 25usize) & 0x01;
        INTENCLR_CBUSRST::from_bits(val as u8)
    }
    #[doc = "Coprocessor Burst."]
    #[inline(always)]
    pub const fn set_CBUSRST(&mut self, val: INTENCLR_CBUSRST) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Coprocessor IRQ Update."]
    #[must_use]
    #[inline(always)]
    pub const fn CIRQUPD(&self) -> INTENCLR_CIRQUPD {
        let val = (self.0 >> 26usize) & 0x01;
        INTENCLR_CIRQUPD::from_bits(val as u8)
    }
    #[doc = "Coprocessor IRQ Update."]
    #[inline(always)]
    pub const fn set_CIRQUPD(&mut self, val: INTENCLR_CIRQUPD) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn CWIRECHG(&self) -> INTENCLR_CWIRECHG {
        let val = (self.0 >> 27usize) & 0x01;
        INTENCLR_CWIRECHG::from_bits(val as u8)
    }
    #[doc = "Coprocessor Wire Change."]
    #[inline(always)]
    pub const fn set_CWIRECHG(&mut self, val: INTENCLR_CWIRECHG) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Coprocessor Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn CHSTALL(&self) -> INTENCLR_CHSTALL {
        let val = (self.0 >> 28usize) & 0x01;
        INTENCLR_CHSTALL::from_bits(val as u8)
    }
    #[doc = "Coprocessor Host Stall."]
    #[inline(always)]
    pub const fn set_CHSTALL(&mut self, val: INTENCLR_CHSTALL) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Coprocessor CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CCRCERR(&self) -> INTENCLR_CCRCERR {
        let val = (self.0 >> 29usize) & 0x01;
        INTENCLR_CCRCERR::from_bits(val as u8)
    }
    #[doc = "Coprocessor CRC Error."]
    #[inline(always)]
    pub const fn set_CCRCERR(&mut self, val: INTENCLR_CCRCERR) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Coprocessor GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn CGPIO(&self) -> INTENCLR_CGPIO {
        let val = (self.0 >> 30usize) & 0x01;
        INTENCLR_CGPIO::from_bits(val as u8)
    }
    #[doc = "Coprocessor GPIO."]
    #[inline(always)]
    pub const fn set_CGPIO(&mut self, val: INTENCLR_CGPIO) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for INTENCLR {
    #[inline(always)]
    fn default() -> INTENCLR {
        INTENCLR(0)
    }
}
impl core::fmt::Debug for INTENCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("PORTINT", &self.PORTINT())
            .field("P80INT", &self.P80INT())
            .field("BUSRST", &self.BUSRST())
            .field("IRQUPD", &self.IRQUPD())
            .field("WIRECHG", &self.WIRECHG())
            .field("HSTALL", &self.HSTALL())
            .field("CRCERR", &self.CRCERR())
            .field("GPIO", &self.GPIO())
            .field("CS_INT", &self.CS_INT())
            .field("CPORTINT", &self.CPORTINT())
            .field("CP80INT", &self.CP80INT())
            .field("CBUSRST", &self.CBUSRST())
            .field("CIRQUPD", &self.CIRQUPD())
            .field("CWIRECHG", &self.CWIRECHG())
            .field("CHSTALL", &self.CHSTALL())
            .field("CCRCERR", &self.CCRCERR())
            .field("CGPIO", &self.CGPIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENCLR {{ PORTINT: {:?}, P80INT: {:?}, BUSRST: {:?}, IRQUPD: {:?}, WIRECHG: {:?}, HSTALL: {:?}, CRCERR: {:?}, GPIO: {:?}, CS_INT: {:?}, CPORTINT: {:?}, CP80INT: {:?}, CBUSRST: {:?}, CIRQUPD: {:?}, CWIRECHG: {:?}, CHSTALL: {:?}, CCRCERR: {:?}, CGPIO: {:?} }}",
            self.PORTINT(),
            self.P80INT(),
            self.BUSRST(),
            self.IRQUPD(),
            self.WIRECHG(),
            self.HSTALL(),
            self.CRCERR(),
            self.GPIO(),
            self.CS_INT(),
            self.CPORTINT(),
            self.CP80INT(),
            self.CBUSRST(),
            self.CIRQUPD(),
            self.CWIRECHG(),
            self.CHSTALL(),
            self.CCRCERR(),
            self.CGPIO()
        )
    }
}
#[doc = "Interrupt Enable Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET(pub u32);
impl INTENSET {
    #[doc = "Port interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PORTINT(&self) -> INTENSET_PORTINT {
        let val = (self.0 >> 0usize) & 0x1f;
        INTENSET_PORTINT::from_bits(val as u8)
    }
    #[doc = "Port interrupt."]
    #[inline(always)]
    pub const fn set_PORTINT(&mut self, val: INTENSET_PORTINT) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn P80INT(&self) -> INTENSET_P80INT {
        let val = (self.0 >> 8usize) & 0x01;
        INTENSET_P80INT::from_bits(val as u8)
    }
    #[doc = "Port80 Interrupt."]
    #[inline(always)]
    pub const fn set_P80INT(&mut self, val: INTENSET_P80INT) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSRST(&self) -> INTENSET_BUSRST {
        let val = (self.0 >> 9usize) & 0x01;
        INTENSET_BUSRST::from_bits(val as u8)
    }
    #[doc = "Bus Reset."]
    #[inline(always)]
    pub const fn set_BUSRST(&mut self, val: INTENSET_BUSRST) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ Update."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQUPD(&self) -> INTENSET_IRQUPD {
        let val = (self.0 >> 10usize) & 0x01;
        INTENSET_IRQUPD::from_bits(val as u8)
    }
    #[doc = "IRQ Update."]
    #[inline(always)]
    pub const fn set_IRQUPD(&mut self, val: INTENSET_IRQUPD) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn WIRECHG(&self) -> INTENSET_WIRECHG {
        let val = (self.0 >> 11usize) & 0x01;
        INTENSET_WIRECHG::from_bits(val as u8)
    }
    #[doc = "Wire Change."]
    #[inline(always)]
    pub const fn set_WIRECHG(&mut self, val: INTENSET_WIRECHG) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTALL(&self) -> INTENSET_HSTALL {
        let val = (self.0 >> 12usize) & 0x01;
        INTENSET_HSTALL::from_bits(val as u8)
    }
    #[doc = "Host Stall."]
    #[inline(always)]
    pub const fn set_HSTALL(&mut self, val: INTENSET_HSTALL) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCERR(&self) -> INTENSET_CRCERR {
        let val = (self.0 >> 13usize) & 0x01;
        INTENSET_CRCERR::from_bits(val as u8)
    }
    #[doc = "CRC Error."]
    #[inline(always)]
    pub const fn set_CRCERR(&mut self, val: INTENSET_CRCERR) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO(&self) -> INTENSET_GPIO {
        let val = (self.0 >> 14usize) & 0x01;
        INTENSET_GPIO::from_bits(val as u8)
    }
    #[doc = "GPIO."]
    #[inline(always)]
    pub const fn set_GPIO(&mut self, val: INTENSET_GPIO) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "CS_INT."]
    #[must_use]
    #[inline(always)]
    pub const fn CS_INT(&self) -> INTENSET_CS_INT {
        let val = (self.0 >> 15usize) & 0x01;
        INTENSET_CS_INT::from_bits(val as u8)
    }
    #[doc = "CS_INT."]
    #[inline(always)]
    pub const fn set_CS_INT(&mut self, val: INTENSET_CS_INT) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Coprocessor Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CPORTINT(&self) -> INTENSET_CPORTINT {
        let val = (self.0 >> 16usize) & 0x1f;
        INTENSET_CPORTINT::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt."]
    #[inline(always)]
    pub const fn set_CPORTINT(&mut self, val: INTENSET_CPORTINT) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Coprocessor Port80 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CP80INT(&self) -> INTENSET_CP80INT {
        let val = (self.0 >> 24usize) & 0x01;
        INTENSET_CP80INT::from_bits(val as u8)
    }
    #[doc = "Coprocessor Port80 Interrupt."]
    #[inline(always)]
    pub const fn set_CP80INT(&mut self, val: INTENSET_CP80INT) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Coprocessor Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn CBUSRST(&self) -> INTENSET_CBUSRST {
        let val = (self.0 >> 25usize) & 0x01;
        INTENSET_CBUSRST::from_bits(val as u8)
    }
    #[doc = "Coprocessor Burst."]
    #[inline(always)]
    pub const fn set_CBUSRST(&mut self, val: INTENSET_CBUSRST) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Coprocessor Interrupt Update."]
    #[must_use]
    #[inline(always)]
    pub const fn CIRQUPD(&self) -> INTENSET_CIRQUPD {
        let val = (self.0 >> 26usize) & 0x01;
        INTENSET_CIRQUPD::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt Update."]
    #[inline(always)]
    pub const fn set_CIRQUPD(&mut self, val: INTENSET_CIRQUPD) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn CWIRECHG(&self) -> INTENSET_CWIRECHG {
        let val = (self.0 >> 27usize) & 0x01;
        INTENSET_CWIRECHG::from_bits(val as u8)
    }
    #[doc = "Coprocessor Wire Change."]
    #[inline(always)]
    pub const fn set_CWIRECHG(&mut self, val: INTENSET_CWIRECHG) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Coprocessor Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn CHSTALL(&self) -> INTENSET_CHSTALL {
        let val = (self.0 >> 28usize) & 0x01;
        INTENSET_CHSTALL::from_bits(val as u8)
    }
    #[doc = "Coprocessor Host Stall."]
    #[inline(always)]
    pub const fn set_CHSTALL(&mut self, val: INTENSET_CHSTALL) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Coprocessor Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CCRCERR(&self) -> INTENSET_CCRCERR {
        let val = (self.0 >> 29usize) & 0x01;
        INTENSET_CCRCERR::from_bits(val as u8)
    }
    #[doc = "Coprocessor Error."]
    #[inline(always)]
    pub const fn set_CCRCERR(&mut self, val: INTENSET_CCRCERR) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Coprocessor GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn CGPIO(&self) -> INTENSET_CGPIO {
        let val = (self.0 >> 30usize) & 0x01;
        INTENSET_CGPIO::from_bits(val as u8)
    }
    #[doc = "Coprocessor GPIO."]
    #[inline(always)]
    pub const fn set_CGPIO(&mut self, val: INTENSET_CGPIO) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for INTENSET {
    #[inline(always)]
    fn default() -> INTENSET {
        INTENSET(0)
    }
}
impl core::fmt::Debug for INTENSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("PORTINT", &self.PORTINT())
            .field("P80INT", &self.P80INT())
            .field("BUSRST", &self.BUSRST())
            .field("IRQUPD", &self.IRQUPD())
            .field("WIRECHG", &self.WIRECHG())
            .field("HSTALL", &self.HSTALL())
            .field("CRCERR", &self.CRCERR())
            .field("GPIO", &self.GPIO())
            .field("CS_INT", &self.CS_INT())
            .field("CPORTINT", &self.CPORTINT())
            .field("CP80INT", &self.CP80INT())
            .field("CBUSRST", &self.CBUSRST())
            .field("CIRQUPD", &self.CIRQUPD())
            .field("CWIRECHG", &self.CWIRECHG())
            .field("CHSTALL", &self.CHSTALL())
            .field("CCRCERR", &self.CCRCERR())
            .field("CGPIO", &self.CGPIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENSET {{ PORTINT: {:?}, P80INT: {:?}, BUSRST: {:?}, IRQUPD: {:?}, WIRECHG: {:?}, HSTALL: {:?}, CRCERR: {:?}, GPIO: {:?}, CS_INT: {:?}, CPORTINT: {:?}, CP80INT: {:?}, CBUSRST: {:?}, CIRQUPD: {:?}, CWIRECHG: {:?}, CHSTALL: {:?}, CCRCERR: {:?}, CGPIO: {:?} }}",
            self.PORTINT(),
            self.P80INT(),
            self.BUSRST(),
            self.IRQUPD(),
            self.WIRECHG(),
            self.HSTALL(),
            self.CRCERR(),
            self.GPIO(),
            self.CS_INT(),
            self.CPORTINT(),
            self.CP80INT(),
            self.CBUSRST(),
            self.CIRQUPD(),
            self.CWIRECHG(),
            self.CHSTALL(),
            self.CCRCERR(),
            self.CGPIO()
        )
    }
}
#[doc = "Masked Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Port Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PORTINT(&self) -> INTSTAT_PORTINT {
        let val = (self.0 >> 0usize) & 0x1f;
        INTSTAT_PORTINT::from_bits(val as u8)
    }
    #[doc = "Port Interrupt."]
    #[inline(always)]
    pub const fn set_PORTINT(&mut self, val: INTSTAT_PORTINT) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn P80INT(&self) -> INTSTAT_P80INT {
        let val = (self.0 >> 8usize) & 0x01;
        INTSTAT_P80INT::from_bits(val as u8)
    }
    #[doc = "Port80 Interrupt."]
    #[inline(always)]
    pub const fn set_P80INT(&mut self, val: INTSTAT_P80INT) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSRST(&self) -> INTSTAT_BUSRST {
        let val = (self.0 >> 9usize) & 0x01;
        INTSTAT_BUSRST::from_bits(val as u8)
    }
    #[doc = "Bus Reset."]
    #[inline(always)]
    pub const fn set_BUSRST(&mut self, val: INTSTAT_BUSRST) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ Update."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQUPD(&self) -> INTSTAT_IRQUPD {
        let val = (self.0 >> 10usize) & 0x01;
        INTSTAT_IRQUPD::from_bits(val as u8)
    }
    #[doc = "IRQ Update."]
    #[inline(always)]
    pub const fn set_IRQUPD(&mut self, val: INTSTAT_IRQUPD) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn WIRECHG(&self) -> INTSTAT_WIRECHG {
        let val = (self.0 >> 11usize) & 0x01;
        INTSTAT_WIRECHG::from_bits(val as u8)
    }
    #[doc = "Wire Change."]
    #[inline(always)]
    pub const fn set_WIRECHG(&mut self, val: INTSTAT_WIRECHG) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTALL(&self) -> INTSTAT_HSTALL {
        let val = (self.0 >> 12usize) & 0x01;
        INTSTAT_HSTALL::from_bits(val as u8)
    }
    #[doc = "Host Stall."]
    #[inline(always)]
    pub const fn set_HSTALL(&mut self, val: INTSTAT_HSTALL) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCERR(&self) -> INTSTAT_CRCERR {
        let val = (self.0 >> 13usize) & 0x01;
        INTSTAT_CRCERR::from_bits(val as u8)
    }
    #[doc = "CRC Error."]
    #[inline(always)]
    pub const fn set_CRCERR(&mut self, val: INTSTAT_CRCERR) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO(&self) -> INTSTAT_GPIO {
        let val = (self.0 >> 14usize) & 0x01;
        INTSTAT_GPIO::from_bits(val as u8)
    }
    #[doc = "GPIO."]
    #[inline(always)]
    pub const fn set_GPIO(&mut self, val: INTSTAT_GPIO) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Coprocessor Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CPORTINT(&self) -> INTSTAT_CPORTINT {
        let val = (self.0 >> 16usize) & 0x1f;
        INTSTAT_CPORTINT::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt."]
    #[inline(always)]
    pub const fn set_CPORTINT(&mut self, val: INTSTAT_CPORTINT) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Coprocessor Port80 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CP80INT(&self) -> INTSTAT_CP80INT {
        let val = (self.0 >> 24usize) & 0x01;
        INTSTAT_CP80INT::from_bits(val as u8)
    }
    #[doc = "Coprocessor Port80 Interrupt."]
    #[inline(always)]
    pub const fn set_CP80INT(&mut self, val: INTSTAT_CP80INT) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Coprocessor Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn CBUSRST(&self) -> INTSTAT_CBUSRST {
        let val = (self.0 >> 25usize) & 0x01;
        INTSTAT_CBUSRST::from_bits(val as u8)
    }
    #[doc = "Coprocessor Burst."]
    #[inline(always)]
    pub const fn set_CBUSRST(&mut self, val: INTSTAT_CBUSRST) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Coprocessor IRQ Update."]
    #[must_use]
    #[inline(always)]
    pub const fn CIRQUPD(&self) -> INTSTAT_CIRQUPD {
        let val = (self.0 >> 26usize) & 0x01;
        INTSTAT_CIRQUPD::from_bits(val as u8)
    }
    #[doc = "Coprocessor IRQ Update."]
    #[inline(always)]
    pub const fn set_CIRQUPD(&mut self, val: INTSTAT_CIRQUPD) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn CWIRECHG(&self) -> INTSTAT_CWIRECHG {
        let val = (self.0 >> 27usize) & 0x01;
        INTSTAT_CWIRECHG::from_bits(val as u8)
    }
    #[doc = "Coprocessor Wire Change."]
    #[inline(always)]
    pub const fn set_CWIRECHG(&mut self, val: INTSTAT_CWIRECHG) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Coprocessor Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn CHSTALL(&self) -> INTSTAT_CHSTALL {
        let val = (self.0 >> 28usize) & 0x01;
        INTSTAT_CHSTALL::from_bits(val as u8)
    }
    #[doc = "Coprocessor Host Stall."]
    #[inline(always)]
    pub const fn set_CHSTALL(&mut self, val: INTSTAT_CHSTALL) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Coprocessor CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CCRCERR(&self) -> INTSTAT_CCRCERR {
        let val = (self.0 >> 29usize) & 0x01;
        INTSTAT_CCRCERR::from_bits(val as u8)
    }
    #[doc = "Coprocessor CRC Error."]
    #[inline(always)]
    pub const fn set_CCRCERR(&mut self, val: INTSTAT_CCRCERR) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Coprocessor GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn CGPIO(&self) -> INTSTAT_CGPIO {
        let val = (self.0 >> 30usize) & 0x01;
        INTSTAT_CGPIO::from_bits(val as u8)
    }
    #[doc = "Coprocessor GPIO."]
    #[inline(always)]
    pub const fn set_CGPIO(&mut self, val: INTSTAT_CGPIO) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("PORTINT", &self.PORTINT())
            .field("P80INT", &self.P80INT())
            .field("BUSRST", &self.BUSRST())
            .field("IRQUPD", &self.IRQUPD())
            .field("WIRECHG", &self.WIRECHG())
            .field("HSTALL", &self.HSTALL())
            .field("CRCERR", &self.CRCERR())
            .field("GPIO", &self.GPIO())
            .field("CPORTINT", &self.CPORTINT())
            .field("CP80INT", &self.CP80INT())
            .field("CBUSRST", &self.CBUSRST())
            .field("CIRQUPD", &self.CIRQUPD())
            .field("CWIRECHG", &self.CWIRECHG())
            .field("CHSTALL", &self.CHSTALL())
            .field("CCRCERR", &self.CCRCERR())
            .field("CGPIO", &self.CGPIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ PORTINT: {:?}, P80INT: {:?}, BUSRST: {:?}, IRQUPD: {:?}, WIRECHG: {:?}, HSTALL: {:?}, CRCERR: {:?}, GPIO: {:?}, CPORTINT: {:?}, CP80INT: {:?}, CBUSRST: {:?}, CIRQUPD: {:?}, CWIRECHG: {:?}, CHSTALL: {:?}, CCRCERR: {:?}, CGPIO: {:?} }}",
            self.PORTINT(),
            self.P80INT(),
            self.BUSRST(),
            self.IRQUPD(),
            self.WIRECHG(),
            self.HSTALL(),
            self.CRCERR(),
            self.GPIO(),
            self.CPORTINT(),
            self.CP80INT(),
            self.CBUSRST(),
            self.CIRQUPD(),
            self.CWIRECHG(),
            self.CHSTALL(),
            self.CCRCERR(),
            self.CGPIO()
        )
    }
}
#[doc = "IRQ Push."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQPUSH(pub u32);
impl IRQPUSH {
    #[doc = "Interrupt Request Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt Request Queue."]
    #[inline(always)]
    pub const fn set_IRQ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "IRQ Update Done."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ Update Done."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IRQPUSH {
    #[inline(always)]
    fn default() -> IRQPUSH {
        IRQPUSH(0)
    }
}
impl core::fmt::Debug for IRQPUSH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQPUSH")
            .field("IRQ", &self.IRQ())
            .field("DONE", &self.DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQPUSH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQPUSH {{ IRQ: {=u8:?}, DONE: {=bool:?} }}",
            self.IRQ(),
            self.DONE()
        )
    }
}
#[doc = "Set Interrupt Rules and User Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRULESTAT(pub u32);
impl IRULESTAT {
    #[doc = "User-Defined Status Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn USTAT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User-Defined Status Bits."]
    #[inline(always)]
    pub const fn set_USTAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt If Error Detected."]
    #[must_use]
    #[inline(always)]
    pub const fn INTERR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Error Detected."]
    #[inline(always)]
    pub const fn set_INTERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn INTRD(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started."]
    #[inline(always)]
    pub const fn set_INTRD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn INTWR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished."]
    #[inline(always)]
    pub const fn set_INTWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn INTSPC(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]."]
    #[inline(always)]
    pub const fn set_INTSPC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status Set and Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn SSTCL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status Set and Clear."]
    #[inline(always)]
    pub const fn set_SSTCL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn SRST(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]."]
    #[inline(always)]
    pub const fn set_SRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flash Completion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_COMPLETION_TYPE(&self) -> IRULESTAT_FLASH_COMPLETION_TYPE {
        let val = (self.0 >> 21usize) & 0x03;
        IRULESTAT_FLASH_COMPLETION_TYPE::from_bits(val as u8)
    }
    #[doc = "Flash Completion Type."]
    #[inline(always)]
    pub const fn set_FLASH_COMPLETION_TYPE(&mut self, val: IRULESTAT_FLASH_COMPLETION_TYPE) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "CPU Tag."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU_TAG(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU Tag."]
    #[inline(always)]
    pub const fn set_CPU_TAG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for IRULESTAT {
    #[inline(always)]
    fn default() -> IRULESTAT {
        IRULESTAT(0)
    }
}
impl core::fmt::Debug for IRULESTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRULESTAT")
            .field("USTAT", &self.USTAT())
            .field("INTERR", &self.INTERR())
            .field("INTRD", &self.INTRD())
            .field("INTWR", &self.INTWR())
            .field("INTSPC", &self.INTSPC())
            .field("SSTCL", &self.SSTCL())
            .field("SRST", &self.SRST())
            .field("FLASH_COMPLETION_TYPE", &self.FLASH_COMPLETION_TYPE())
            .field("CPU_TAG", &self.CPU_TAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRULESTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRULESTAT {{ USTAT: {=u8:?}, INTERR: {=bool:?}, INTRD: {=bool:?}, INTWR: {=bool:?}, INTSPC: {=u8:?}, SSTCL: {=u8:?}, SRST: {=bool:?}, FLASH_COMPLETION_TYPE: {:?}, CPU_TAG: {=u8:?} }}",
            self.USTAT(),
            self.INTERR(),
            self.INTRD(),
            self.INTWR(),
            self.INTSPC(),
            self.SSTCL(),
            self.SRST(),
            self.FLASH_COMPLETION_TYPE(),
            self.CPU_TAG()
        )
    }
}
#[doc = "Mapped Base."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAPBASE(pub u32);
impl MAPBASE {
    #[doc = "Base 0."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Base 0."]
    #[inline(always)]
    pub const fn set_BASE0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Base 1."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base 1."]
    #[inline(always)]
    pub const fn set_BASE1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MAPBASE {
    #[inline(always)]
    fn default() -> MAPBASE {
        MAPBASE(0)
    }
}
impl core::fmt::Debug for MAPBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAPBASE")
            .field("BASE0", &self.BASE0())
            .field("BASE1", &self.BASE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAPBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MAPBASE {{ BASE0: {=u16:?}, BASE1: {=u16:?} }}",
            self.BASE0(),
            self.BASE1()
        )
    }
}
#[doc = "Master Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCTRL(pub u32);
impl MCTRL {
    #[doc = "Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> ENABLE {
        let val = (self.0 >> 0usize) & 0x03;
        ENABLE::from_bits(val as u8)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: ENABLE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "RTC-Integrated-BMC."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_INT_BMC(&self) -> RTC_INT_BMC {
        let val = (self.0 >> 2usize) & 0x01;
        RTC_INT_BMC::from_bits(val as u8)
    }
    #[doc = "RTC-Integrated-BMC."]
    #[inline(always)]
    pub const fn set_RTC_INT_BMC(&mut self, val: RTC_INT_BMC) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PENA(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Enable."]
    #[inline(always)]
    pub const fn set_PENA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Port 80 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn P80ENA(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port 80 Enable."]
    #[inline(always)]
    pub const fn set_P80ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status Block Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SBLKENA(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status Block Enable."]
    #[inline(always)]
    pub const fn set_SBLKENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Clock Division Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_DIV_DISABLE(&self) -> CLK_DIV_DISABLE {
        let val = (self.0 >> 18usize) & 0x01;
        CLK_DIV_DISABLE::from_bits(val as u8)
    }
    #[doc = "Clock Division Disable."]
    #[inline(always)]
    pub const fn set_CLK_DIV_DISABLE(&mut self, val: CLK_DIV_DISABLE) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Early Sample."]
    #[must_use]
    #[inline(always)]
    pub const fn EARLY_SAMPLE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Early Sample."]
    #[inline(always)]
    pub const fn set_EARLY_SAMPLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for MCTRL {
    #[inline(always)]
    fn default() -> MCTRL {
        MCTRL(0)
    }
}
impl core::fmt::Debug for MCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("ENABLE", &self.ENABLE())
            .field("RTC_INT_BMC", &self.RTC_INT_BMC())
            .field("PENA", &self.PENA())
            .field("P80ENA", &self.P80ENA())
            .field("SBLKENA", &self.SBLKENA())
            .field("CLK_DIV_DISABLE", &self.CLK_DIV_DISABLE())
            .field("EARLY_SAMPLE", &self.EARLY_SAMPLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCTRL {{ ENABLE: {:?}, RTC_INT_BMC: {:?}, PENA: {=u8:?}, P80ENA: {=bool:?}, SBLKENA: {=bool:?}, CLK_DIV_DISABLE: {:?}, EARLY_SAMPLE: {=bool:?} }}",
            self.ENABLE(),
            self.RTC_INT_BMC(),
            self.PENA(),
            self.P80ENA(),
            self.SBLKENA(),
            self.CLK_DIV_DISABLE(),
            self.EARLY_SAMPLE()
        )
    }
}
#[doc = "Master Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSTAT(pub u32);
impl MSTAT {
    #[doc = "Port Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn PORTINT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Interrupt."]
    #[inline(always)]
    pub const fn set_PORTINT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn P80INT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port80 Interrupt Request."]
    #[inline(always)]
    pub const fn set_P80INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSRST(&self) -> MSTAT_BUSRST {
        let val = (self.0 >> 9usize) & 0x01;
        MSTAT_BUSRST::from_bits(val as u8)
    }
    #[doc = "Bus Reset."]
    #[inline(always)]
    pub const fn set_BUSRST(&mut self, val: MSTAT_BUSRST) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Request Update Completion."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQUPD(&self) -> MSTAT_IRQUPD {
        let val = (self.0 >> 10usize) & 0x01;
        MSTAT_IRQUPD::from_bits(val as u8)
    }
    #[doc = "Interrupt Request Update Completion."]
    #[inline(always)]
    pub const fn set_IRQUPD(&mut self, val: MSTAT_IRQUPD) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change."]
    #[must_use]
    #[inline(always)]
    pub const fn WIRECHG(&self) -> MSTAT_WIRECHG {
        let val = (self.0 >> 11usize) & 0x01;
        MSTAT_WIRECHG::from_bits(val as u8)
    }
    #[doc = "Wire Change."]
    #[inline(always)]
    pub const fn set_WIRECHG(&mut self, val: MSTAT_WIRECHG) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTALL(&self) -> MSTAT_HSTALL {
        let val = (self.0 >> 12usize) & 0x01;
        MSTAT_HSTALL::from_bits(val as u8)
    }
    #[doc = "Host Stall."]
    #[inline(always)]
    pub const fn set_HSTALL(&mut self, val: MSTAT_HSTALL) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Cyclic Redundancy Check (CRC) Error."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCERR(&self) -> MSTAT_CRCERR {
        let val = (self.0 >> 13usize) & 0x01;
        MSTAT_CRCERR::from_bits(val as u8)
    }
    #[doc = "Cyclic Redundancy Check (CRC) Error."]
    #[inline(always)]
    pub const fn set_CRCERR(&mut self, val: MSTAT_CRCERR) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "General Purpose Input/Output (GPIO)."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO(&self) -> MSTAT_GPIO {
        let val = (self.0 >> 14usize) & 0x01;
        MSTAT_GPIO::from_bits(val as u8)
    }
    #[doc = "General Purpose Input/Output (GPIO)."]
    #[inline(always)]
    pub const fn set_GPIO(&mut self, val: MSTAT_GPIO) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bus Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> BUSY {
        let val = (self.0 >> 16usize) & 0x01;
        BUSY::from_bits(val as u8)
    }
    #[doc = "Bus Busy."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: BUSY) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn INRST(&self) -> INRST {
        let val = (self.0 >> 17usize) & 0x01;
        INRST::from_bits(val as u8)
    }
    #[doc = "Bus Reset."]
    #[inline(always)]
    pub const fn set_INRST(&mut self, val: INRST) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Completion Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPPEND(&self) -> COMPPEND {
        let val = (self.0 >> 18usize) & 0x01;
        COMPPEND::from_bits(val as u8)
    }
    #[doc = "Completion Pending."]
    #[inline(always)]
    pub const fn set_COMPPEND(&mut self, val: COMPPEND) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Master Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTPEND(&self) -> MASTPEND {
        let val = (self.0 >> 19usize) & 0x01;
        MASTPEND::from_bits(val as u8)
    }
    #[doc = "Master Pending."]
    #[inline(always)]
    pub const fn set_MASTPEND(&mut self, val: MASTPEND) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Alert Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn ALERTPEND(&self) -> ALERTPEND {
        let val = (self.0 >> 20usize) & 0x01;
        ALERTPEND::from_bits(val as u8)
    }
    #[doc = "Alert Pending."]
    #[inline(always)]
    pub const fn set_ALERTPEND(&mut self, val: ALERTPEND) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for MSTAT {
    #[inline(always)]
    fn default() -> MSTAT {
        MSTAT(0)
    }
}
impl core::fmt::Debug for MSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTAT")
            .field("PORTINT", &self.PORTINT())
            .field("P80INT", &self.P80INT())
            .field("BUSRST", &self.BUSRST())
            .field("IRQUPD", &self.IRQUPD())
            .field("WIRECHG", &self.WIRECHG())
            .field("HSTALL", &self.HSTALL())
            .field("CRCERR", &self.CRCERR())
            .field("GPIO", &self.GPIO())
            .field("BUSY", &self.BUSY())
            .field("INRST", &self.INRST())
            .field("COMPPEND", &self.COMPPEND())
            .field("MASTPEND", &self.MASTPEND())
            .field("ALERTPEND", &self.ALERTPEND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MSTAT {{ PORTINT: {=u8:?}, P80INT: {=bool:?}, BUSRST: {:?}, IRQUPD: {:?}, WIRECHG: {:?}, HSTALL: {:?}, CRCERR: {:?}, GPIO: {:?}, BUSY: {:?}, INRST: {:?}, COMPPEND: {:?}, MASTPEND: {:?}, ALERTPEND: {:?} }}",
            self.PORTINT(),
            self.P80INT(),
            self.BUSRST(),
            self.IRQUPD(),
            self.WIRECHG(),
            self.HSTALL(),
            self.CRCERR(),
            self.GPIO(),
            self.BUSY(),
            self.INRST(),
            self.COMPPEND(),
            self.MASTPEND(),
            self.ALERTPEND()
        )
    }
}
#[doc = "Port OOB, Mastering, and Flash Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OMFLEN(pub u32);
impl OMFLEN {
    #[doc = "Length in Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in Bytes."]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer Request."]
    #[must_use]
    #[inline(always)]
    pub const fn TRANS(&self) -> OMFLEN_TRANS {
        let val = (self.0 >> 12usize) & 0x03;
        OMFLEN_TRANS::from_bits(val as u8)
    }
    #[doc = "Transfer Request."]
    #[inline(always)]
    pub const fn set_TRANS(&mut self, val: OMFLEN_TRANS) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for OMFLEN {
    #[inline(always)]
    fn default() -> OMFLEN {
        OMFLEN(0)
    }
}
impl core::fmt::Debug for OMFLEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OMFLEN")
            .field("LEN", &self.LEN())
            .field("TRANS", &self.TRANS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OMFLEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OMFLEN {{ LEN: {=u8:?}, TRANS: {:?} }}",
            self.LEN(),
            self.TRANS()
        )
    }
}
#[doc = "Port 80 Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80STAT(pub u32);
impl P80STAT {
    #[doc = "Current Port80 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn CURR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Current Port80 Value."]
    #[inline(always)]
    pub const fn set_CURR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Previous Port80 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn PREV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Previous Port80 Value."]
    #[inline(always)]
    pub const fn set_PREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Counter Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RST(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Reset."]
    #[inline(always)]
    pub const fn set_RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P80STAT {
    #[inline(always)]
    fn default() -> P80STAT {
        P80STAT(0)
    }
}
impl core::fmt::Debug for P80STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P80STAT")
            .field("CURR", &self.CURR())
            .field("PREV", &self.PREV())
            .field("CNT", &self.CNT())
            .field("RST", &self.RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P80STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P80STAT {{ CURR: {=u8:?}, PREV: {=u8:?}, CNT: {=u8:?}, RST: {=bool:?} }}",
            self.CURR(),
            self.PREV(),
            self.CNT(),
            self.RST()
        )
    }
}
#[doc = "RAM Base."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMBASE(pub u32);
impl RAMBASE {
    #[doc = "Always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ZERO(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Always 0."]
    #[inline(always)]
    pub const fn set_ZERO(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "RAM Location."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "RAM Location."]
    #[inline(always)]
    pub const fn set_RAM(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for RAMBASE {
    #[inline(always)]
    fn default() -> RAMBASE {
        RAMBASE(0)
    }
}
impl core::fmt::Debug for RAMBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMBASE")
            .field("ZERO", &self.ZERO())
            .field("RAM", &self.RAM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAMBASE {{ ZERO: {=u16:?}, RAM: {=u32:?} }}",
            self.ZERO(),
            self.RAM()
        )
    }
}
#[doc = "Port RAM Use."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMUSE(pub u32);
impl RAMUSE {
    #[doc = "Offset into RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn OFF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset into RAM."]
    #[inline(always)]
    pub const fn set_OFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Length."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Length."]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for RAMUSE {
    #[inline(always)]
    fn default() -> RAMUSE {
        RAMUSE(0)
    }
}
impl core::fmt::Debug for RAMUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMUSE")
            .field("OFF", &self.OFF())
            .field("LEN", &self.LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMUSE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAMUSE {{ OFF: {=u16:?}, LEN: {=u8:?} }}",
            self.OFF(),
            self.LEN()
        )
    }
}
#[doc = "RPMC Support 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RPMC_SUPPORT1(pub u32);
impl RPMC_SUPPORT1 {
    #[doc = "Target RPMC Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn TARGET_RPMC_SUPPORTED(&self) -> TARGET_RPMC_SUPPORTED {
        let val = (self.0 >> 0usize) & 0x3f;
        TARGET_RPMC_SUPPORTED::from_bits(val as u8)
    }
    #[doc = "Target RPMC Supported."]
    #[inline(always)]
    pub const fn set_TARGET_RPMC_SUPPORTED(&mut self, val: TARGET_RPMC_SUPPORTED) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Number of Target Attached Flash RPMC flash devices."]
    #[must_use]
    #[inline(always)]
    pub const fn NUM_OF_TARGET(&self) -> NUM_OF_TARGET {
        let val = (self.0 >> 6usize) & 0x03;
        NUM_OF_TARGET::from_bits(val as u8)
    }
    #[doc = "Number of Target Attached Flash RPMC flash devices."]
    #[inline(always)]
    pub const fn set_NUM_OF_TARGET(&mut self, val: NUM_OF_TARGET) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "RPMC OP1 Opcode on the 1st RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_OP1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 1st RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_OP1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "RPMC Counter on the 1st RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_COUNTER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 1st RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_COUNTER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "RPMC OP1 Opcode on the 2nd RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_OP1_2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 2nd RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_OP1_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "RPMC Counter on the 2nd RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_COUNTER_2(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 2nd RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_COUNTER_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for RPMC_SUPPORT1 {
    #[inline(always)]
    fn default() -> RPMC_SUPPORT1 {
        RPMC_SUPPORT1(0)
    }
}
impl core::fmt::Debug for RPMC_SUPPORT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPMC_SUPPORT1")
            .field("TARGET_RPMC_SUPPORTED", &self.TARGET_RPMC_SUPPORTED())
            .field("NUM_OF_TARGET", &self.NUM_OF_TARGET())
            .field("RPMC_OP1", &self.RPMC_OP1())
            .field("RPMC_COUNTER", &self.RPMC_COUNTER())
            .field("RPMC_OP1_2", &self.RPMC_OP1_2())
            .field("RPMC_COUNTER_2", &self.RPMC_COUNTER_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RPMC_SUPPORT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RPMC_SUPPORT1 {{ TARGET_RPMC_SUPPORTED: {:?}, NUM_OF_TARGET: {:?}, RPMC_OP1: {=u8:?}, RPMC_COUNTER: {=u8:?}, RPMC_OP1_2: {=u8:?}, RPMC_COUNTER_2: {=u8:?} }}",
            self.TARGET_RPMC_SUPPORTED(),
            self.NUM_OF_TARGET(),
            self.RPMC_OP1(),
            self.RPMC_COUNTER(),
            self.RPMC_OP1_2(),
            self.RPMC_COUNTER_2()
        )
    }
}
#[doc = "RPMC Support 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RPMC_SUPPORT2(pub u32);
impl RPMC_SUPPORT2 {
    #[doc = "RPMC Counter on the 3rd RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_COUNTER_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 3rd RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_COUNTER_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "RPMC OP1 Opcode on the 3rd RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_OP1_3(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 3rd RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_OP1_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "RPMC Counter on the 4th RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_COUNTER_4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 4th RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_COUNTER_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "RPMC OP1 Opcode on the 4th RPMC Flash device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_OP1_4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 4th RPMC Flash device."]
    #[inline(always)]
    pub const fn set_RPMC_OP1_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RPMC_SUPPORT2 {
    #[inline(always)]
    fn default() -> RPMC_SUPPORT2 {
        RPMC_SUPPORT2(0)
    }
}
impl core::fmt::Debug for RPMC_SUPPORT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPMC_SUPPORT2")
            .field("RPMC_COUNTER_3", &self.RPMC_COUNTER_3())
            .field("RPMC_OP1_3", &self.RPMC_OP1_3())
            .field("RPMC_COUNTER_4", &self.RPMC_COUNTER_4())
            .field("RPMC_OP1_4", &self.RPMC_OP1_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RPMC_SUPPORT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RPMC_SUPPORT2 {{ RPMC_COUNTER_3: {=u8:?}, RPMC_OP1_3: {=u8:?}, RPMC_COUNTER_4: {=u8:?}, RPMC_OP1_4: {=u8:?} }}",
            self.RPMC_COUNTER_3(),
            self.RPMC_OP1_3(),
            self.RPMC_COUNTER_4(),
            self.RPMC_OP1_4()
        )
    }
}
#[doc = "Port Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Host Read Data Status."]
    #[must_use]
    #[inline(always)]
    pub const fn RDSTAT(&self) -> STAT_RDSTAT {
        let val = (self.0 >> 0usize) & 0x03;
        STAT_RDSTAT::from_bits(val as u8)
    }
    #[doc = "Host Read Data Status."]
    #[inline(always)]
    pub const fn set_RDSTAT(&mut self, val: STAT_RDSTAT) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes."]
    #[must_use]
    #[inline(always)]
    pub const fn WRSTAT(&self) -> STAT_WRSTAT {
        let val = (self.0 >> 2usize) & 0x03;
        STAT_WRSTAT::from_bits(val as u8)
    }
    #[doc = "Status of Host Writes."]
    #[inline(always)]
    pub const fn set_WRSTAT(&mut self, val: STAT_WRSTAT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt Caused by Error."]
    #[must_use]
    #[inline(always)]
    pub const fn INTERR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Error."]
    #[inline(always)]
    pub const fn set_INTERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Caused by Read."]
    #[must_use]
    #[inline(always)]
    pub const fn INTRD(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Read."]
    #[inline(always)]
    pub const fn set_INTRD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Caused by Write."]
    #[must_use]
    #[inline(always)]
    pub const fn INTWR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Write."]
    #[inline(always)]
    pub const fn set_INTWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPC0 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn INTSPC0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPC0 Interrupt."]
    #[inline(always)]
    pub const fn set_INTSPC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPC1 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn INTSPC1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPC1 Interrupt."]
    #[inline(always)]
    pub const fn set_INTSPC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPC2 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn INTSPC2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPC2 Interrupt."]
    #[inline(always)]
    pub const fn set_INTSPC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SPC3 Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn INSTSPC3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPC3 Interrupt."]
    #[inline(always)]
    pub const fn set_INSTSPC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error 0."]
    #[inline(always)]
    pub const fn set_ERR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error 1."]
    #[inline(always)]
    pub const fn set_ERR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error 2."]
    #[inline(always)]
    pub const fn set_ERR2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error 3."]
    #[inline(always)]
    pub const fn set_ERR3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RPMC 1 or 2."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_1_OR_2(&self) -> STAT_RPMC_1_OR_2 {
        let val = (self.0 >> 20usize) & 0x01;
        STAT_RPMC_1_OR_2::from_bits(val as u8)
    }
    #[doc = "RPMC 1 or 2."]
    #[inline(always)]
    pub const fn set_RPMC_1_OR_2(&mut self, val: STAT_RPMC_1_OR_2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RPMC Flash Device."]
    #[must_use]
    #[inline(always)]
    pub const fn RPMC_FLASH_DEVICE(&self) -> STAT_RPMC_FLASH_DEVICE {
        let val = (self.0 >> 21usize) & 0x03;
        STAT_RPMC_FLASH_DEVICE::from_bits(val as u8)
    }
    #[doc = "RPMC Flash Device."]
    #[inline(always)]
    pub const fn set_RPMC_FLASH_DEVICE(&mut self, val: STAT_RPMC_FLASH_DEVICE) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("RDSTAT", &self.RDSTAT())
            .field("WRSTAT", &self.WRSTAT())
            .field("INTERR", &self.INTERR())
            .field("INTRD", &self.INTRD())
            .field("INTWR", &self.INTWR())
            .field("INTSPC0", &self.INTSPC0())
            .field("INTSPC1", &self.INTSPC1())
            .field("INTSPC2", &self.INTSPC2())
            .field("INSTSPC3", &self.INSTSPC3())
            .field("ERR0", &self.ERR0())
            .field("ERR1", &self.ERR1())
            .field("ERR2", &self.ERR2())
            .field("ERR3", &self.ERR3())
            .field("RPMC_1_OR_2", &self.RPMC_1_OR_2())
            .field("RPMC_FLASH_DEVICE", &self.RPMC_FLASH_DEVICE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ RDSTAT: {:?}, WRSTAT: {:?}, INTERR: {=bool:?}, INTRD: {=bool:?}, INTWR: {=bool:?}, INTSPC0: {=bool:?}, INTSPC1: {=bool:?}, INTSPC2: {=bool:?}, INSTSPC3: {=bool:?}, ERR0: {=bool:?}, ERR1: {=bool:?}, ERR2: {=bool:?}, ERR3: {=bool:?}, RPMC_1_OR_2: {:?}, RPMC_FLASH_DEVICE: {:?} }}",
            self.RDSTAT(),
            self.WRSTAT(),
            self.INTERR(),
            self.INTRD(),
            self.INTWR(),
            self.INTSPC0(),
            self.INTSPC1(),
            self.INTSPC2(),
            self.INSTSPC3(),
            self.ERR0(),
            self.ERR1(),
            self.ERR2(),
            self.ERR3(),
            self.RPMC_1_OR_2(),
            self.RPMC_FLASH_DEVICE()
        )
    }
}
#[doc = "Status Block Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATADDR(pub u32);
impl STATADDR {
    #[doc = "Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn OFF(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "Offset."]
    #[inline(always)]
    pub const fn set_OFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
    #[doc = "Offset Base."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> BASE {
        let val = (self.0 >> 16usize) & 0x03;
        BASE::from_bits(val as u8)
    }
    #[doc = "Offset Base."]
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: BASE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for STATADDR {
    #[inline(always)]
    fn default() -> STATADDR {
        STATADDR(0)
    }
}
impl core::fmt::Debug for STATADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATADDR")
            .field("OFF", &self.OFF())
            .field("BASE", &self.BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATADDR {{ OFF: {=u16:?}, BASE: {:?} }}",
            self.OFF(),
            self.BASE()
        )
    }
}
#[doc = "WIREIN_GPIO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WIREIN_GPIO(pub u32);
impl WIREIN_GPIO {
    #[doc = "Level."]
    #[must_use]
    #[inline(always)]
    pub const fn LEVEL(&self) -> WIREIN_GPIO_LEVEL {
        let val = (self.0 >> 0usize) & 0x0f;
        WIREIN_GPIO_LEVEL::from_bits(val as u8)
    }
    #[doc = "Level."]
    #[inline(always)]
    pub const fn set_LEVEL(&mut self, val: WIREIN_GPIO_LEVEL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> WIREIN_GPIO_VALID {
        let val = (self.0 >> 4usize) & 0x0f;
        WIREIN_GPIO_VALID::from_bits(val as u8)
    }
    #[doc = "Valid."]
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: WIREIN_GPIO_VALID) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Index."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Index."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WIREIN_GPIO {
    #[inline(always)]
    fn default() -> WIREIN_GPIO {
        WIREIN_GPIO(0)
    }
}
impl core::fmt::Debug for WIREIN_GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIREIN_GPIO")
            .field("LEVEL", &self.LEVEL())
            .field("VALID", &self.VALID())
            .field("INDEX", &self.INDEX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WIREIN_GPIO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WIREIN_GPIO {{ LEVEL: {:?}, VALID: {:?}, INDEX: {=u8:?} }}",
            self.LEVEL(),
            self.VALID(),
            self.INDEX()
        )
    }
}
#[doc = "WIREOUT_GPIO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WIREOUT_GPIO(pub u32);
impl WIREOUT_GPIO {
    #[doc = "Level."]
    #[must_use]
    #[inline(always)]
    pub const fn LEVEL(&self) -> WIREOUT_GPIO_LEVEL {
        let val = (self.0 >> 0usize) & 0x0f;
        WIREOUT_GPIO_LEVEL::from_bits(val as u8)
    }
    #[doc = "Level."]
    #[inline(always)]
    pub const fn set_LEVEL(&mut self, val: WIREOUT_GPIO_LEVEL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> WIREOUT_GPIO_VALID {
        let val = (self.0 >> 4usize) & 0x0f;
        WIREOUT_GPIO_VALID::from_bits(val as u8)
    }
    #[doc = "Valid."]
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: WIREOUT_GPIO_VALID) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Index."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Index."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WIREOUT_GPIO {
    #[inline(always)]
    fn default() -> WIREOUT_GPIO {
        WIREOUT_GPIO(0)
    }
}
impl core::fmt::Debug for WIREOUT_GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIREOUT_GPIO")
            .field("LEVEL", &self.LEVEL())
            .field("VALID", &self.VALID())
            .field("INDEX", &self.INDEX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WIREOUT_GPIO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WIREOUT_GPIO {{ LEVEL: {:?}, VALID: {:?}, INDEX: {=u8:?} }}",
            self.LEVEL(),
            self.VALID(),
            self.INDEX()
        )
    }
}
#[doc = "Virtual Wire Host-to-MCU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WIRERO(pub u32);
impl WIRERO {
    #[doc = "Sleep State 3."]
    #[must_use]
    #[inline(always)]
    pub const fn SLP_S3N(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep State 3."]
    #[inline(always)]
    pub const fn set_SLP_S3N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sleep State 4."]
    #[must_use]
    #[inline(always)]
    pub const fn SLP_S4N(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep State 4."]
    #[inline(always)]
    pub const fn set_SLP_S4N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sleep State 5."]
    #[must_use]
    #[inline(always)]
    pub const fn SLP_S5N(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep State 5."]
    #[inline(always)]
    pub const fn set_SLP_S5N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Suspend Status."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS_STAT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Status."]
    #[inline(always)]
    pub const fn set_SUS_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Request."]
    #[must_use]
    #[inline(always)]
    pub const fn PLTRSTN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Request."]
    #[inline(always)]
    pub const fn set_PLTRSTN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Out-Of-Band Reset Warning."]
    #[must_use]
    #[inline(always)]
    pub const fn OOB_RST_WARN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Out-Of-Band Reset Warning."]
    #[inline(always)]
    pub const fn set_OOB_RST_WARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Host Reset Warning."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_RST_WARN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Host Reset Warning."]
    #[inline(always)]
    pub const fn set_HOST_RST_WARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend Warning."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS_WARN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Warning."]
    #[inline(always)]
    pub const fn set_SUS_WARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Suspend Power Well Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS_PWRDN_ACKN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Power Well Acknowledge."]
    #[inline(always)]
    pub const fn set_SUS_PWRDN_ACKN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Sleep AN."]
    #[must_use]
    #[inline(always)]
    pub const fn SLP_AN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep AN."]
    #[inline(always)]
    pub const fn set_SLP_AN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Wired LAN Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn SLP_LAN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Wired LAN Sleep."]
    #[inline(always)]
    pub const fn set_SLP_LAN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wireless LAN Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn SLP_WLAN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wireless LAN Sleep."]
    #[inline(always)]
    pub const fn set_SLP_WLAN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PCIe to EC."]
    #[must_use]
    #[inline(always)]
    pub const fn P2E(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0xff;
        val as u8
    }
    #[doc = "PCIe to EC."]
    #[inline(always)]
    pub const fn set_P2E(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
    }
    #[doc = "Host Entering Deep Power Down C10 State."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_C10N(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Host Entering Deep Power Down C10 State."]
    #[inline(always)]
    pub const fn set_HOST_C10N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for WIRERO {
    #[inline(always)]
    fn default() -> WIRERO {
        WIRERO(0)
    }
}
impl core::fmt::Debug for WIRERO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIRERO")
            .field("SLP_S3N", &self.SLP_S3N())
            .field("SLP_S4N", &self.SLP_S4N())
            .field("SLP_S5N", &self.SLP_S5N())
            .field("SUS_STAT", &self.SUS_STAT())
            .field("PLTRSTN", &self.PLTRSTN())
            .field("OOB_RST_WARN", &self.OOB_RST_WARN())
            .field("HOST_RST_WARN", &self.HOST_RST_WARN())
            .field("SUS_WARN", &self.SUS_WARN())
            .field("SUS_PWRDN_ACKN", &self.SUS_PWRDN_ACKN())
            .field("SLP_AN", &self.SLP_AN())
            .field("SLP_LAN", &self.SLP_LAN())
            .field("SLP_WLAN", &self.SLP_WLAN())
            .field("P2E", &self.P2E())
            .field("HOST_C10N", &self.HOST_C10N())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WIRERO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WIRERO {{ SLP_S3N: {=bool:?}, SLP_S4N: {=bool:?}, SLP_S5N: {=bool:?}, SUS_STAT: {=bool:?}, PLTRSTN: {=bool:?}, OOB_RST_WARN: {=bool:?}, HOST_RST_WARN: {=bool:?}, SUS_WARN: {=bool:?}, SUS_PWRDN_ACKN: {=bool:?}, SLP_AN: {=bool:?}, SLP_LAN: {=bool:?}, SLP_WLAN: {=bool:?}, P2E: {=u8:?}, HOST_C10N: {=bool:?} }}",
            self.SLP_S3N(),
            self.SLP_S4N(),
            self.SLP_S5N(),
            self.SUS_STAT(),
            self.PLTRSTN(),
            self.OOB_RST_WARN(),
            self.HOST_RST_WARN(),
            self.SUS_WARN(),
            self.SUS_PWRDN_ACKN(),
            self.SLP_AN(),
            self.SLP_LAN(),
            self.SLP_WLAN(),
            self.P2E(),
            self.HOST_C10N()
        )
    }
}
#[doc = "Virtual Wire MCU-to-host."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WIREWO(pub u32);
impl WIREWO {
    #[doc = "Out-Of-Band Reset Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn OOB_RST_ACK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Out-Of-Band Reset Acknowledge."]
    #[inline(always)]
    pub const fn set_OOB_RST_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IRQ1."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEN_SCIN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ1."]
    #[inline(always)]
    pub const fn set_WAKEN_SCIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SMIN."]
    #[must_use]
    #[inline(always)]
    pub const fn PMEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SMIN."]
    #[inline(always)]
    pub const fn set_PMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IRQ2."]
    #[must_use]
    #[inline(always)]
    pub const fn SCIN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ2."]
    #[inline(always)]
    pub const fn set_SCIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IRQ3."]
    #[must_use]
    #[inline(always)]
    pub const fn SMIN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ3."]
    #[inline(always)]
    pub const fn set_SMIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IRQ4."]
    #[must_use]
    #[inline(always)]
    pub const fn RCINN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ4."]
    #[inline(always)]
    pub const fn set_RCINN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IRQ5."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_RST_ACK(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ5."]
    #[inline(always)]
    pub const fn set_HOST_RST_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IRQ6."]
    #[must_use]
    #[inline(always)]
    pub const fn SUSACKN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ6."]
    #[inline(always)]
    pub const fn set_SUSACKN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IRQ7-IRQ15."]
    #[must_use]
    #[inline(always)]
    pub const fn E2P(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "IRQ7-IRQ15."]
    #[inline(always)]
    pub const fn set_E2P(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Boot Load Done."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DONE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Load Done."]
    #[inline(always)]
    pub const fn set_BOOT_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Boot Load Error."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_ERRN(&self) -> BOOT_ERRN {
        let val = (self.0 >> 17usize) & 0x01;
        BOOT_ERRN::from_bits(val as u8)
    }
    #[doc = "Boot Load Error."]
    #[inline(always)]
    pub const fn set_BOOT_ERRN(&mut self, val: BOOT_ERRN) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Force."]
    #[must_use]
    #[inline(always)]
    pub const fn DSW_PWROK_RST(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force."]
    #[inline(always)]
    pub const fn set_DSW_PWROK_RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write Done."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> DONE {
        let val = (self.0 >> 31usize) & 0x01;
        DONE::from_bits(val as u8)
    }
    #[doc = "Write Done."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: DONE) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for WIREWO {
    #[inline(always)]
    fn default() -> WIREWO {
        WIREWO(0)
    }
}
impl core::fmt::Debug for WIREWO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIREWO")
            .field("OOB_RST_ACK", &self.OOB_RST_ACK())
            .field("WAKEN_SCIN", &self.WAKEN_SCIN())
            .field("PMEN", &self.PMEN())
            .field("SCIN", &self.SCIN())
            .field("SMIN", &self.SMIN())
            .field("RCINN", &self.RCINN())
            .field("HOST_RST_ACK", &self.HOST_RST_ACK())
            .field("SUSACKN", &self.SUSACKN())
            .field("E2P", &self.E2P())
            .field("BOOT_DONE", &self.BOOT_DONE())
            .field("BOOT_ERRN", &self.BOOT_ERRN())
            .field("DSW_PWROK_RST", &self.DSW_PWROK_RST())
            .field("DONE", &self.DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WIREWO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WIREWO {{ OOB_RST_ACK: {=bool:?}, WAKEN_SCIN: {=bool:?}, PMEN: {=bool:?}, SCIN: {=bool:?}, SMIN: {=bool:?}, RCINN: {=bool:?}, HOST_RST_ACK: {=bool:?}, SUSACKN: {=bool:?}, E2P: {=u8:?}, BOOT_DONE: {=bool:?}, BOOT_ERRN: {:?}, DSW_PWROK_RST: {=bool:?}, DONE: {:?} }}",
            self.OOB_RST_ACK(),
            self.WAKEN_SCIN(),
            self.PMEN(),
            self.SCIN(),
            self.SMIN(),
            self.RCINN(),
            self.HOST_RST_ACK(),
            self.SUSACKN(),
            self.E2P(),
            self.BOOT_DONE(),
            self.BOOT_ERRN(),
            self.DSW_PWROK_RST(),
            self.DONE()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADDR_BASE_ASZ {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET_FROM_0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory."]
    USE_BASE0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory."]
    USE_BASE1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ADDR_BASE_ASZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADDR_BASE_ASZ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADDR_BASE_ASZ {
    #[inline(always)]
    fn from(val: u8) -> ADDR_BASE_ASZ {
        ADDR_BASE_ASZ::from_bits(val)
    }
}
impl From<ADDR_BASE_ASZ> for u8 {
    #[inline(always)]
    fn from(val: ADDR_BASE_ASZ) -> u8 {
        ADDR_BASE_ASZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADDR_IDX1ST {
    #[doc = "Index is higher address than data (for example, data at OFF, index at OFF+IDXOFF)."]
    dataoff = 0x0,
    #[doc = "Index is lower address than data (for example, index at OFF, data at OFF+IDXOFF)."]
    idxoff = 0x01,
}
impl ADDR_IDX1ST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADDR_IDX1ST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADDR_IDX1ST {
    #[inline(always)]
    fn from(val: u8) -> ADDR_IDX1ST {
        ADDR_IDX1ST::from_bits(val)
    }
}
impl From<ADDR_IDX1ST> for u8 {
    #[inline(always)]
    fn from(val: ADDR_IDX1ST) -> u8 {
        ADDR_IDX1ST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ALERTPEND {
    #[doc = "No alert request pending."]
    DISABLE = 0x0,
    #[doc = "Alert request pin is pending, whether a separate pin or master-in slave-out (MISO). For LPC, indicates that SERIRQ is in process."]
    ENABLE = 0x01,
}
impl ALERTPEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ALERTPEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ALERTPEND {
    #[inline(always)]
    fn from(val: u8) -> ALERTPEND {
        ALERTPEND::from_bits(val)
    }
}
impl From<ALERTPEND> for u8 {
    #[inline(always)]
    fn from(val: ALERTPEND) -> u8 {
        ALERTPEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BASE {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET = 0x0,
    #[doc = "Base1 offset in host memory."]
    BASE1 = 0x01,
    #[doc = "Base2 offset in host memory."]
    BASE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl BASE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BASE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BASE {
    #[inline(always)]
    fn from(val: u8) -> BASE {
        BASE::from_bits(val)
    }
}
impl From<BASE> for u8 {
    #[inline(always)]
    fn from(val: BASE) -> u8 {
        BASE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOT_ERRN {
    #[doc = "Boot load error."]
    ERROR = 0x0,
    #[doc = "Boot load successful."]
    SUCCESS = 0x01,
}
impl BOOT_ERRN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOT_ERRN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOT_ERRN {
    #[inline(always)]
    fn from(val: u8) -> BOOT_ERRN {
        BOOT_ERRN::from_bits(val)
    }
}
impl From<BOOT_ERRN> for u8 {
    #[inline(always)]
    fn from(val: BOOT_ERRN) -> u8 {
        BOOT_ERRN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUSY {
    #[doc = "Idle."]
    DISABLE = 0x0,
    #[doc = "Busy."]
    ENABLE = 0x01,
}
impl BUSY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUSY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUSY {
    #[inline(always)]
    fn from(val: u8) -> BUSY {
        BUSY::from_bits(val)
    }
}
impl From<BUSY> for u8 {
    #[inline(always)]
    fn from(val: BUSY) -> u8 {
        BUSY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG_DIRECTION {
    #[doc = "Endpoint or index-and-data: bidirectional (normal). Mailbox Single: unenforced. Mailbox Split or Mailbox Shared: bidirectional."]
    dir0 = 0x0,
    #[doc = "Endpoint or index-and-data: ignore read. Mailbox Single: write only. Mailbox Split or Mailbox Shared: ignore read."]
    dir1 = 0x01,
    #[doc = "Endpoint or index-and-data: ignore write. Mailbox Single: read only. Mailbox Split or Mailbox Shared: ignore write."]
    dir2 = 0x02,
    #[doc = "Endpoint or index-and-data: Ignore both. Mailbox Single: ignore both. Mailbox Split or Mailbox Shared: ignore both."]
    dir3 = 0x03,
}
impl CFG_DIRECTION {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG_DIRECTION {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG_DIRECTION {
    #[inline(always)]
    fn from(val: u8) -> CFG_DIRECTION {
        CFG_DIRECTION::from_bits(val)
    }
}
impl From<CFG_DIRECTION> for u8 {
    #[inline(always)]
    fn from(val: CFG_DIRECTION) -> u8 {
        CFG_DIRECTION::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG_ERRORIGN {
    #[doc = "The host receives an error when trying to perform a read or write that is blocked by PnCFG\\[DIRECTION\\]."]
    errorigin0 = 0x0,
    #[doc = "Ignored silently. Reads return FFh for each byte if ignored (host property). In either case, PnSTAT\\[INTERR\\] bit becomes 1, causing an interrupt if masked for it."]
    errorigin1 = 0x01,
}
impl CFG_ERRORIGN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG_ERRORIGN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG_ERRORIGN {
    #[inline(always)]
    fn from(val: u8) -> CFG_ERRORIGN {
        CFG_ERRORIGN::from_bits(val)
    }
}
impl From<CFG_ERRORIGN> for u8 {
    #[inline(always)]
    fn from(val: CFG_ERRORIGN) -> u8 {
        CFG_ERRORIGN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG_TYPE {
    #[doc = "Unconfigured (reset condition)."]
    UNCONFIGURED = 0x0,
    #[doc = "ACPI style Endpoint."]
    ACPI_END = 0x01,
    #[doc = "ACPI style index-and-data. Index and data byte locations. Index gives offset into implied space. Uses registers for data and index."]
    ACPI_INDEX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Memory Single."]
    BUS_M_MEM_S = 0x04,
    #[doc = "Bus Master Flash Single."]
    BUS_M_FLASH_S = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared."]
    MAILBOX_SHARED = 0x08,
    #[doc = "Mailbox Single."]
    MAILBOX_SINGLE = 0x09,
    #[doc = "Mailbox Split."]
    MAILBOX_SPLIT = 0x0a,
    #[doc = "Mailbox OOB Split."]
    MAILBOX_OOB_SPLIT = 0x0b,
    #[doc = "Mailbox OEM."]
    MAILBOX_OEM = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CFG_TYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG_TYPE {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG_TYPE {
    #[inline(always)]
    fn from(val: u8) -> CFG_TYPE {
        CFG_TYPE::from_bits(val)
    }
}
impl From<CFG_TYPE> for u8 {
    #[inline(always)]
    fn from(val: CFG_TYPE) -> u8 {
        CFG_TYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLK_DIV_DISABLE {
    #[doc = "Clock division is enabled."]
    ENABLE = 0x0,
    #[doc = "Clock division is disabled."]
    DISABLE = 0x01,
}
impl CLK_DIV_DISABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLK_DIV_DISABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLK_DIV_DISABLE {
    #[inline(always)]
    fn from(val: u8) -> CLK_DIV_DISABLE {
        CLK_DIV_DISABLE::from_bits(val)
    }
}
impl From<CLK_DIV_DISABLE> for u8 {
    #[inline(always)]
    fn from(val: CLK_DIV_DISABLE) -> u8 {
        CLK_DIV_DISABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMPPEND {
    #[doc = "No completions pending."]
    DISABLE = 0x0,
    #[doc = "Completions are pending for eSPI. Indicates quiet mode for LPC."]
    ENABLE = 0x01,
}
impl COMPPEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMPPEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMPPEND {
    #[inline(always)]
    fn from(val: u8) -> COMPPEND {
        COMPPEND::from_bits(val)
    }
}
impl From<COMPPEND> for u8 {
    #[inline(always)]
    fn from(val: COMPPEND) -> u8 {
        COMPPEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DATAIN_DIR {
    #[doc = "Read by host."]
    ENABLE = 0x0,
    #[doc = "Write by host."]
    DISABLE = 0x01,
}
impl DATAIN_DIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DATAIN_DIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DATAIN_DIR {
    #[inline(always)]
    fn from(val: u8) -> DATAIN_DIR {
        DATAIN_DIR::from_bits(val)
    }
}
impl From<DATAIN_DIR> for u8 {
    #[inline(always)]
    fn from(val: DATAIN_DIR) -> u8 {
        DATAIN_DIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA0EN {
    #[doc = "Disabled. The DMA channel is not used."]
    DISABLED = 0x0,
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory."]
    TRIGGERD_ON_HOST_READ = 0x01,
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte, or mailbox and many bytes)."]
    TRIGGERS_ON_HOST_WRITE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DMA0EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA0EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA0EN {
    #[inline(always)]
    fn from(val: u8) -> DMA0EN {
        DMA0EN::from_bits(val)
    }
}
impl From<DMA0EN> for u8 {
    #[inline(always)]
    fn from(val: DMA0EN) -> u8 {
        DMA0EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA1EN {
    #[doc = "Disabled. The DMA channel is not used."]
    DISABLED = 0x0,
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory."]
    TRIGGERD_ON_HOST_READ = 0x01,
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    TRIGGERS_ON_HOST_WRITE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DMA1EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA1EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA1EN {
    #[inline(always)]
    fn from(val: u8) -> DMA1EN {
        DMA1EN::from_bits(val)
    }
}
impl From<DMA1EN> for u8 {
    #[inline(always)]
    fn from(val: DMA1EN) -> u8 {
        DMA1EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA1PORT {
    #[doc = "Port 0."]
    port0 = 0x0,
    #[doc = "Port 1."]
    port1 = 0x01,
    #[doc = "Port 2."]
    port2 = 0x02,
    #[doc = "Port 3."]
    port3 = 0x03,
    #[doc = "Port 4."]
    port4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Port80. Used to offload the Port80 bytes (only host writes apply to Port80)."]
    port80 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DMA1PORT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA1PORT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA1PORT {
    #[inline(always)]
    fn from(val: u8) -> DMA1PORT {
        DMA1PORT::from_bits(val)
    }
}
impl From<DMA1PORT> for u8 {
    #[inline(always)]
    fn from(val: DMA1PORT) -> u8 {
        DMA1PORT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DONE {
    #[doc = "Write not done."]
    NOTDONE = 0x0,
    #[doc = "Write done."]
    DONE = 0x01,
}
impl DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DONE {
    #[inline(always)]
    fn from(val: u8) -> DONE {
        DONE::from_bits(val)
    }
}
impl From<DONE> for u8 {
    #[inline(always)]
    fn from(val: DONE) -> u8 {
        DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENABLE {
    #[doc = "Disabled. Block is not operational."]
    DISABLED = 0x0,
    #[doc = "eSPI (Enhanced Serial Peripheral Interface)."]
    ESPI = 0x01,
    #[doc = "LPC (Low Pin Count)."]
    LPC = 0x02,
    _RESERVED_3 = 0x03,
}
impl ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENABLE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENABLE {
    #[inline(always)]
    fn from(val: u8) -> ENABLE {
        ENABLE::from_bits(val)
    }
}
impl From<ENABLE> for u8 {
    #[inline(always)]
    fn from(val: ENABLE) -> u8 {
        ENABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ESPICAP_SAF {
    #[doc = "Host cannot support SAF."]
    DISABLE = 0x0,
    #[doc = "SAF is possible with the firmware."]
    ENABLE = 0x01,
}
impl ESPICAP_SAF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ESPICAP_SAF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ESPICAP_SAF {
    #[inline(always)]
    fn from(val: u8) -> ESPICAP_SAF {
        ESPICAP_SAF::from_bits(val)
    }
}
impl From<ESPICAP_SAF> for u8 {
    #[inline(always)]
    fn from(val: ESPICAP_SAF) -> u8 {
        ESPICAP_SAF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ESPICFG_SAF {
    #[doc = "Host cannot support SAF."]
    DISABLE = 0x0,
    #[doc = "Slave Attached Flash is possible with the firmware."]
    ENABLE = 0x01,
}
impl ESPICFG_SAF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ESPICFG_SAF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ESPICFG_SAF {
    #[inline(always)]
    fn from(val: u8) -> ESPICFG_SAF {
        ESPICFG_SAF::from_bits(val)
    }
}
impl From<ESPICFG_SAF> for u8 {
    #[inline(always)]
    fn from(val: ESPICFG_SAF) -> u8 {
        ESPICFG_SAF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASHMX {
    #[doc = "64 bytes."]
    BYTE64 = 0x0,
    #[doc = "128 bytes."]
    BYTE128 = 0x01,
    #[doc = "256 bytes."]
    BYTE256 = 0x02,
    #[doc = "512 bytes."]
    BYTE512 = 0x03,
}
impl FLASHMX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASHMX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASHMX {
    #[inline(always)]
    fn from(val: u8) -> FLASHMX {
        FLASHMX::from_bits(val)
    }
}
impl From<FLASHMX> for u8 {
    #[inline(always)]
    fn from(val: FLASHMX) -> u8 {
        FLASHMX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASHSZ {
    #[doc = "64 bytes."]
    BYTE64 = 0x0,
    #[doc = "128 bytes."]
    BYTE128 = 0x01,
    #[doc = "256 bytes."]
    BYTE256 = 0x02,
    #[doc = "512 bytes."]
    BYTE512 = 0x03,
}
impl FLASHSZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASHSZ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASHSZ {
    #[inline(always)]
    fn from(val: u8) -> FLASHSZ {
        FLASHSZ::from_bits(val)
    }
}
impl From<FLASHSZ> for u8 {
    #[inline(always)]
    fn from(val: FLASHSZ) -> u8 {
        FLASHSZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLSHERA {
    #[doc = "Flash not enabled."]
    DISABLED = 0x0,
    #[doc = "Erase is 4 kB."]
    ERASE_4K = 0x01,
    #[doc = "Erase is 64 kB."]
    ERASE_64K = 0x02,
    #[doc = "Erase allows 4 kB and 64 kB."]
    ERASE_4K_64K = 0x03,
    #[doc = "Erase is 128 kB."]
    ERASE_128K = 0x04,
    #[doc = "Erase is 256 kB."]
    ERASE_256K = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FLSHERA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLSHERA {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLSHERA {
    #[inline(always)]
    fn from(val: u8) -> FLSHERA {
        FLSHERA::from_bits(val)
    }
}
impl From<FLSHERA> for u8 {
    #[inline(always)]
    fn from(val: FLSHERA) -> u8 {
        FLSHERA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO_OD {
    _RESERVED_0 = 0x0,
    #[doc = "Alert/Reset pin acts as open drain when ESPIMISC\\[GPIO_OE\\] = 1. This means ESPIMISC\\[GPIO_OUT\\] = 0 is low, and ESPIMISC\\[GPIO_OUT\\] = 1 is high-Z."]
    od1 = 0x01,
}
impl GPIO_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO_OD {
    #[inline(always)]
    fn from(val: u8) -> GPIO_OD {
        GPIO_OD::from_bits(val)
    }
}
impl From<GPIO_OD> for u8 {
    #[inline(always)]
    fn from(val: GPIO_OD) -> u8 {
        GPIO_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO_OE {
    #[doc = "Input (High-Z)."]
    oe0 = 0x0,
    #[doc = "Alert or reset pin is set as an output GPIO."]
    oe1 = 0x01,
}
impl GPIO_OE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO_OE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO_OE {
    #[inline(always)]
    fn from(val: u8) -> GPIO_OE {
        GPIO_OE::from_bits(val)
    }
}
impl From<GPIO_OE> for u8 {
    #[inline(always)]
    fn from(val: GPIO_OE) -> u8 {
        GPIO_OE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO_OUT {
    #[doc = "Low."]
    oe0 = 0x0,
    #[doc = "High, high-Z if ESPIMISC\\[GPIO_OD\\] = 1."]
    oe1 = 0x01,
}
impl GPIO_OUT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO_OUT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO_OUT {
    #[inline(always)]
    fn from(val: u8) -> GPIO_OUT {
        GPIO_OUT::from_bits(val)
    }
}
impl From<GPIO_OUT> for u8 {
    #[inline(always)]
    fn from(val: GPIO_OUT) -> u8 {
        GPIO_OUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INRST {
    #[doc = "Bus not in reset."]
    DISABLE = 0x0,
    #[doc = "Bus is in reset."]
    ENABLE = 0x01,
}
impl INRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INRST {
    #[inline(always)]
    fn from(val: u8) -> INRST {
        INRST::from_bits(val)
    }
}
impl From<INRST> for u8 {
    #[inline(always)]
    fn from(val: INRST) -> u8 {
        INRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_BUSRST {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to reset change interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_BUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_BUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_BUSRST {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_BUSRST {
        INTENCLR_BUSRST::from_bits(val)
    }
}
impl From<INTENCLR_BUSRST> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_BUSRST) -> u8 {
        INTENCLR_BUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CBUSRST {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor Reset change interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CBUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CBUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CBUSRST {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CBUSRST {
        INTENCLR_CBUSRST::from_bits(val)
    }
}
impl From<INTENCLR_CBUSRST> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CBUSRST) -> u8 {
        INTENCLR_CBUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CCRCERR {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor CRCERR interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CCRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CCRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CCRCERR {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CCRCERR {
        INTENCLR_CCRCERR::from_bits(val)
    }
}
impl From<INTENCLR_CCRCERR> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CCRCERR) -> u8 {
        INTENCLR_CCRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CGPIO {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor GPIO interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CGPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CGPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CGPIO {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CGPIO {
        INTENCLR_CGPIO::from_bits(val)
    }
}
impl From<INTENCLR_CGPIO> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CGPIO) -> u8 {
        INTENCLR_CGPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CHSTALL {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor HSTALL interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CHSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CHSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CHSTALL {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CHSTALL {
        INTENCLR_CHSTALL::from_bits(val)
    }
}
impl From<INTENCLR_CHSTALL> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CHSTALL) -> u8 {
        INTENCLR_CHSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CIRQUPD {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor IRQ completion interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CIRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CIRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CIRQUPD {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CIRQUPD {
        INTENCLR_CIRQUPD::from_bits(val)
    }
}
impl From<INTENCLR_CIRQUPD> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CIRQUPD) -> u8 {
        INTENCLR_CIRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CP80INT {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor Port80 interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CP80INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CP80INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CP80INT {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CP80INT {
        INTENCLR_CP80INT::from_bits(val)
    }
}
impl From<INTENCLR_CP80INT> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CP80INT) -> u8 {
        INTENCLR_CP80INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CPORTINT {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to corresponding coprocessor port interrupt enable."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl INTENCLR_CPORTINT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CPORTINT {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CPORTINT {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CPORTINT {
        INTENCLR_CPORTINT::from_bits(val)
    }
}
impl From<INTENCLR_CPORTINT> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CPORTINT) -> u8 {
        INTENCLR_CPORTINT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CRCERR {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to CRCERR interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CRCERR {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CRCERR {
        INTENCLR_CRCERR::from_bits(val)
    }
}
impl From<INTENCLR_CRCERR> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CRCERR) -> u8 {
        INTENCLR_CRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CS_INT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Clear CS falls enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CS_INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CS_INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CS_INT {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CS_INT {
        INTENCLR_CS_INT::from_bits(val)
    }
}
impl From<INTENCLR_CS_INT> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CS_INT) -> u8 {
        INTENCLR_CS_INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_CWIRECHG {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to coprocessor Wire Change interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_CWIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_CWIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_CWIRECHG {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_CWIRECHG {
        INTENCLR_CWIRECHG::from_bits(val)
    }
}
impl From<INTENCLR_CWIRECHG> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_CWIRECHG) -> u8 {
        INTENCLR_CWIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_GPIO {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to GPIO interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_GPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_GPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_GPIO {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_GPIO {
        INTENCLR_GPIO::from_bits(val)
    }
}
impl From<INTENCLR_GPIO> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_GPIO) -> u8 {
        INTENCLR_GPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_HSTALL {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to HSTALL interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_HSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_HSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_HSTALL {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_HSTALL {
        INTENCLR_HSTALL::from_bits(val)
    }
}
impl From<INTENCLR_HSTALL> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_HSTALL) -> u8 {
        INTENCLR_HSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_IRQUPD {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to IRQ completion interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_IRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_IRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_IRQUPD {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_IRQUPD {
        INTENCLR_IRQUPD::from_bits(val)
    }
}
impl From<INTENCLR_IRQUPD> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_IRQUPD) -> u8 {
        INTENCLR_IRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_P80INT {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to Port80 interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_P80INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_P80INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_P80INT {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_P80INT {
        INTENCLR_P80INT::from_bits(val)
    }
}
impl From<INTENCLR_P80INT> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_P80INT) -> u8 {
        INTENCLR_P80INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_PORTINT {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to corresponding port interrupt enable."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl INTENCLR_PORTINT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_PORTINT {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_PORTINT {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_PORTINT {
        INTENCLR_PORTINT::from_bits(val)
    }
}
impl From<INTENCLR_PORTINT> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_PORTINT) -> u8 {
        INTENCLR_PORTINT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_WIRECHG {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to Wire Change interrupt enable."]
    ENABLE = 0x01,
}
impl INTENCLR_WIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_WIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_WIRECHG {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_WIRECHG {
        INTENCLR_WIRECHG::from_bits(val)
    }
}
impl From<INTENCLR_WIRECHG> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_WIRECHG) -> u8 {
        INTENCLR_WIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_BUSRST {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "A change in bus reset status interrupts CPU."]
    ENABLE = 0x01,
}
impl INTENSET_BUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_BUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_BUSRST {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_BUSRST {
        INTENSET_BUSRST::from_bits(val)
    }
}
impl From<INTENSET_BUSRST> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_BUSRST) -> u8 {
        INTENSET_BUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CBUSRST {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Change in bus reset status interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTENSET_CBUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CBUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CBUSRST {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CBUSRST {
        INTENSET_CBUSRST::from_bits(val)
    }
}
impl From<INTENSET_CBUSRST> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CBUSRST) -> u8 {
        INTENSET_CBUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CCRCERR {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When CRC error detected, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTENSET_CCRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CCRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CCRCERR {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CCRCERR {
        INTENSET_CCRCERR::from_bits(val)
    }
}
impl From<INTENSET_CCRCERR> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CCRCERR) -> u8 {
        INTENSET_CCRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CGPIO {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When ESPICFG GPIO changes input value, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTENSET_CGPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CGPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CGPIO {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CGPIO {
        INTENSET_CGPIO::from_bits(val)
    }
}
impl From<INTENSET_CGPIO> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CGPIO) -> u8 {
        INTENSET_CGPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CHSTALL {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When MSTAT\\[HSTALL\\] is 1, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTENSET_CHSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CHSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CHSTALL {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CHSTALL {
        INTENSET_CHSTALL::from_bits(val)
    }
}
impl From<INTENSET_CHSTALL> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CHSTALL) -> u8 {
        INTENSET_CHSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CIRQUPD {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Completion of an IRQ update interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTENSET_CIRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CIRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CIRQUPD {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CIRQUPD {
        INTENSET_CIRQUPD::from_bits(val)
    }
}
impl From<INTENSET_CIRQUPD> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CIRQUPD) -> u8 {
        INTENSET_CIRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CP80INT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Port80 interrupts coprocessor on update from host."]
    ENABLE = 0x01,
}
impl INTENSET_CP80INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CP80INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CP80INT {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CP80INT {
        INTENSET_CP80INT::from_bits(val)
    }
}
impl From<INTENSET_CP80INT> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CP80INT) -> u8 {
        INTENSET_CP80INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CPORTINT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "no description available."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl INTENSET_CPORTINT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CPORTINT {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CPORTINT {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CPORTINT {
        INTENSET_CPORTINT::from_bits(val)
    }
}
impl From<INTENSET_CPORTINT> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CPORTINT) -> u8 {
        INTENSET_CPORTINT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CRCERR {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When CRC error detected, interrupts CPU."]
    ENABLE = 0x01,
}
impl INTENSET_CRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CRCERR {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CRCERR {
        INTENSET_CRCERR::from_bits(val)
    }
}
impl From<INTENSET_CRCERR> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CRCERR) -> u8 {
        INTENSET_CRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CS_INT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When CS falls, interrupts the main processor (wake_async is asserted)."]
    ENABLE = 0x01,
}
impl INTENSET_CS_INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CS_INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CS_INT {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CS_INT {
        INTENSET_CS_INT::from_bits(val)
    }
}
impl From<INTENSET_CS_INT> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CS_INT) -> u8 {
        INTENSET_CS_INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_CWIRECHG {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When one one or more Vwire inputs have changed, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTENSET_CWIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_CWIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_CWIRECHG {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_CWIRECHG {
        INTENSET_CWIRECHG::from_bits(val)
    }
}
impl From<INTENSET_CWIRECHG> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_CWIRECHG) -> u8 {
        INTENSET_CWIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_GPIO {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When ESPICFG GPIO changes input value, interrupts CPU."]
    ENABLE = 0x01,
}
impl INTENSET_GPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_GPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_GPIO {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_GPIO {
        INTENSET_GPIO::from_bits(val)
    }
}
impl From<INTENSET_GPIO> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_GPIO) -> u8 {
        INTENSET_GPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_HSTALL {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When the MSTAT\\[HSTALL\\] is 1, interrupt main processor."]
    ENABLE = 0x01,
}
impl INTENSET_HSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_HSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_HSTALL {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_HSTALL {
        INTENSET_HSTALL::from_bits(val)
    }
}
impl From<INTENSET_HSTALL> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_HSTALL) -> u8 {
        INTENSET_HSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_IRQUPD {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Completion of an IRQ update interrupts CPU."]
    ENABLE = 0x01,
}
impl INTENSET_IRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_IRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_IRQUPD {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_IRQUPD {
        INTENSET_IRQUPD::from_bits(val)
    }
}
impl From<INTENSET_IRQUPD> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_IRQUPD) -> u8 {
        INTENSET_IRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_P80INT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Port80 interrupts main processor on update from host."]
    ENABLE = 0x01,
}
impl INTENSET_P80INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_P80INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_P80INT {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_P80INT {
        INTENSET_P80INT::from_bits(val)
    }
}
impl From<INTENSET_P80INT> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_P80INT) -> u8 {
        INTENSET_P80INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_PORTINT {
    _RESERVED_0 = 0x0,
    #[doc = "Corresponding port interrupts main processor, if it matches IRule."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl INTENSET_PORTINT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_PORTINT {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_PORTINT {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_PORTINT {
        INTENSET_PORTINT::from_bits(val)
    }
}
impl From<INTENSET_PORTINT> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_PORTINT) -> u8 {
        INTENSET_PORTINT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENSET_WIRECHG {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When one or more VWire inputs have changed, interrupts CPU."]
    ENABLE = 0x01,
}
impl INTENSET_WIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENSET_WIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENSET_WIRECHG {
    #[inline(always)]
    fn from(val: u8) -> INTENSET_WIRECHG {
        INTENSET_WIRECHG::from_bits(val)
    }
}
impl From<INTENSET_WIRECHG> for u8 {
    #[inline(always)]
    fn from(val: INTENSET_WIRECHG) -> u8 {
        INTENSET_WIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_BUSRST {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "A change in Bus Reset status interrupts CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_BUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_BUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_BUSRST {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_BUSRST {
        INTSTAT_BUSRST::from_bits(val)
    }
}
impl From<INTSTAT_BUSRST> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_BUSRST) -> u8 {
        INTSTAT_BUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CBUSRST {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Bus reset status change interrupt is enabled and pending for the CPU. A change in bus reset status interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTSTAT_CBUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CBUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CBUSRST {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CBUSRST {
        INTSTAT_CBUSRST::from_bits(val)
    }
}
impl From<INTSTAT_CBUSRST> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CBUSRST) -> u8 {
        INTSTAT_CBUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CCRCERR {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When a CRC error detected, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTSTAT_CCRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CCRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CCRCERR {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CCRCERR {
        INTSTAT_CCRCERR::from_bits(val)
    }
}
impl From<INTSTAT_CCRCERR> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CCRCERR) -> u8 {
        INTSTAT_CCRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CGPIO {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When ESPICFG GPIO changes input value, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTSTAT_CGPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CGPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CGPIO {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CGPIO {
        INTSTAT_CGPIO::from_bits(val)
    }
}
impl From<INTSTAT_CGPIO> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CGPIO) -> u8 {
        INTSTAT_CGPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CHSTALL {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When MSTAT\\[HSTALL\\] is 1, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTSTAT_CHSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CHSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CHSTALL {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CHSTALL {
        INTSTAT_CHSTALL::from_bits(val)
    }
}
impl From<INTSTAT_CHSTALL> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CHSTALL) -> u8 {
        INTSTAT_CHSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CIRQUPD {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "IRQ update interrupt is enabled and pending for the CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_CIRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CIRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CIRQUPD {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CIRQUPD {
        INTSTAT_CIRQUPD::from_bits(val)
    }
}
impl From<INTSTAT_CIRQUPD> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CIRQUPD) -> u8 {
        INTSTAT_CIRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CP80INT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Port80 interrupt is enabled and pending for the CPU. Port80 interrupts coprocessor on update from host."]
    ENABLE = 0x01,
}
impl INTSTAT_CP80INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CP80INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CP80INT {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CP80INT {
        INTSTAT_CP80INT::from_bits(val)
    }
}
impl From<INTSTAT_CP80INT> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CP80INT) -> u8 {
        INTSTAT_CP80INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CPORTINT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt is enabled and pending for the coprocessor."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl INTSTAT_CPORTINT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CPORTINT {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CPORTINT {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CPORTINT {
        INTSTAT_CPORTINT::from_bits(val)
    }
}
impl From<INTSTAT_CPORTINT> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CPORTINT) -> u8 {
        INTSTAT_CPORTINT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CRCERR {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "The CRC error interrupt is enabled and pending for the CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_CRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CRCERR {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CRCERR {
        INTSTAT_CRCERR::from_bits(val)
    }
}
impl From<INTSTAT_CRCERR> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CRCERR) -> u8 {
        INTSTAT_CRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_CWIRECHG {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "When one one or more Vwire input has changed, interrupts coprocessor."]
    ENABLE = 0x01,
}
impl INTSTAT_CWIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_CWIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_CWIRECHG {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_CWIRECHG {
        INTSTAT_CWIRECHG::from_bits(val)
    }
}
impl From<INTSTAT_CWIRECHG> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_CWIRECHG) -> u8 {
        INTSTAT_CWIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_GPIO {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "The GPIO input change interrupt is enabled and pending for the CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_GPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_GPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_GPIO {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_GPIO {
        INTSTAT_GPIO::from_bits(val)
    }
}
impl From<INTSTAT_GPIO> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_GPIO) -> u8 {
        INTSTAT_GPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_HSTALL {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "The HSTALL interrupt is enabled and pending for the CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_HSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_HSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_HSTALL {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_HSTALL {
        INTSTAT_HSTALL::from_bits(val)
    }
}
impl From<INTSTAT_HSTALL> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_HSTALL) -> u8 {
        INTSTAT_HSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_IRQUPD {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Completion of an IRQ update interrupts CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_IRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_IRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_IRQUPD {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_IRQUPD {
        INTSTAT_IRQUPD::from_bits(val)
    }
}
impl From<INTSTAT_IRQUPD> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_IRQUPD) -> u8 {
        INTSTAT_IRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_P80INT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Port80 interrupts main processor on update from host."]
    ENABLE = 0x01,
}
impl INTSTAT_P80INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_P80INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_P80INT {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_P80INT {
        INTSTAT_P80INT::from_bits(val)
    }
}
impl From<INTSTAT_P80INT> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_P80INT) -> u8 {
        INTSTAT_P80INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_PORTINT {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Corresponding port interrupts main processor if it matches IRule."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl INTSTAT_PORTINT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_PORTINT {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_PORTINT {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_PORTINT {
        INTSTAT_PORTINT::from_bits(val)
    }
}
impl From<INTSTAT_PORTINT> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_PORTINT) -> u8 {
        INTSTAT_PORTINT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTSTAT_WIRECHG {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "The VWire input change interrupt is enabled and pending for the CPU."]
    ENABLE = 0x01,
}
impl INTSTAT_WIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTSTAT_WIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTSTAT_WIRECHG {
    #[inline(always)]
    fn from(val: u8) -> INTSTAT_WIRECHG {
        INTSTAT_WIRECHG::from_bits(val)
    }
}
impl From<INTSTAT_WIRECHG> for u8 {
    #[inline(always)]
    fn from(val: INTSTAT_WIRECHG) -> u8 {
        INTSTAT_WIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRULESTAT_FLASH_COMPLETION_TYPE {
    #[doc = "Indicates the middle completion of a split completion sequence."]
    MIDDLE = 0x0,
    #[doc = "Indicates the first completion of a split completion sequence."]
    FIRST = 0x01,
    #[doc = "Indicates the last completion of a split completion sequence."]
    LAST = 0x02,
    #[doc = "Indicates the only completion for a split transaction."]
    ONLY = 0x03,
}
impl IRULESTAT_FLASH_COMPLETION_TYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRULESTAT_FLASH_COMPLETION_TYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRULESTAT_FLASH_COMPLETION_TYPE {
    #[inline(always)]
    fn from(val: u8) -> IRULESTAT_FLASH_COMPLETION_TYPE {
        IRULESTAT_FLASH_COMPLETION_TYPE::from_bits(val)
    }
}
impl From<IRULESTAT_FLASH_COMPLETION_TYPE> for u8 {
    #[inline(always)]
    fn from(val: IRULESTAT_FLASH_COMPLETION_TYPE) -> u8 {
        IRULESTAT_FLASH_COMPLETION_TYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAF {
    #[doc = "Host cannot support MAF."]
    DISABLE = 0x0,
    #[doc = "If 1, then Master Attached Flash is possible with this firmware."]
    ENABLE = 0x01,
}
impl MAF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAF {
    #[inline(always)]
    fn from(val: u8) -> MAF {
        MAF::from_bits(val)
    }
}
impl From<MAF> for u8 {
    #[inline(always)]
    fn from(val: MAF) -> u8 {
        MAF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTPEND {
    #[doc = "No mastering pending."]
    DISABLE = 0x0,
    #[doc = "Mastering is pending (flash or memory)."]
    ENABLE = 0x01,
}
impl MASTPEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTPEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTPEND {
    #[inline(always)]
    fn from(val: u8) -> MASTPEND {
        MASTPEND::from_bits(val)
    }
}
impl From<MASTPEND> for u8 {
    #[inline(always)]
    fn from(val: MASTPEND) -> u8 {
        MASTPEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAXSPD {
    #[doc = "<=20 MHz."]
    SMALL_THAN_20M = 0x0,
    #[doc = "<=25 MHz (24 Mhz)."]
    SMALL_THAN_25M = 0x01,
    #[doc = "<=33 MHz (30 MHz)."]
    SMALL_THAN_33M = 0x02,
    #[doc = "<=50 MHz (48 MHz)."]
    SMALL_THAN_50M = 0x03,
    #[doc = "<=66 MHz (60 MHz)."]
    SMALL_THAN_66M = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MAXSPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAXSPD {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAXSPD {
    #[inline(always)]
    fn from(val: u8) -> MAXSPD {
        MAXSPD::from_bits(val)
    }
}
impl From<MAXSPD> for u8 {
    #[inline(always)]
    fn from(val: MAXSPD) -> u8 {
        MAXSPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MEMMX {
    _RESERVED_0 = 0x0,
    #[doc = "64 bytes address aligned max payload size."]
    MIN_4KB = 0x01,
    #[doc = "128 bytes address aligned max payload size."]
    MIN_8KB = 0x02,
    #[doc = "256 bytes address aligned max payload size."]
    MIN_16KB = 0x03,
}
impl MEMMX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MEMMX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MEMMX {
    #[inline(always)]
    fn from(val: u8) -> MEMMX {
        MEMMX::from_bits(val)
    }
}
impl From<MEMMX> for u8 {
    #[inline(always)]
    fn from(val: MEMMX) -> u8 {
        MEMMX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MEMSZ {
    _RESERVED_0 = 0x0,
    #[doc = "64-byte payload for memory."]
    MEMSZ0 = 0x01,
    #[doc = "128-byte payload for memory and OOB access."]
    MEMSZ1 = 0x02,
    #[doc = "256-byte payload for memory and OOB access."]
    MEMSZ11 = 0x03,
}
impl MEMSZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MEMSZ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MEMSZ {
    #[inline(always)]
    fn from(val: u8) -> MEMSZ {
        MEMSZ::from_bits(val)
    }
}
impl From<MEMSZ> for u8 {
    #[inline(always)]
    fn from(val: MEMSZ) -> u8 {
        MEMSZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTAT_BUSRST {
    _RESERVED_0 = 0x0,
    #[doc = "The bit entered or exited reset. Sticky, must clear."]
    ENABLE = 0x01,
}
impl MSTAT_BUSRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTAT_BUSRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTAT_BUSRST {
    #[inline(always)]
    fn from(val: u8) -> MSTAT_BUSRST {
        MSTAT_BUSRST::from_bits(val)
    }
}
impl From<MSTAT_BUSRST> for u8 {
    #[inline(always)]
    fn from(val: MSTAT_BUSRST) -> u8 {
        MSTAT_BUSRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTAT_CRCERR {
    _RESERVED_0 = 0x0,
    #[doc = "CRC from the master did not match the computed CRC. The state of the app may be in error (due to bad data). Write 1 to clear."]
    ENABLE = 0x01,
}
impl MSTAT_CRCERR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTAT_CRCERR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTAT_CRCERR {
    #[inline(always)]
    fn from(val: u8) -> MSTAT_CRCERR {
        MSTAT_CRCERR::from_bits(val)
    }
}
impl From<MSTAT_CRCERR> for u8 {
    #[inline(always)]
    fn from(val: MSTAT_CRCERR) -> u8 {
        MSTAT_CRCERR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTAT_GPIO {
    _RESERVED_0 = 0x0,
    #[doc = "ESPIMISC\\[GPIO\\] has had an input change. Only detected if INTENSET has configured this bit to interrupt. Write 1 to clear."]
    ENABLE = 0x01,
}
impl MSTAT_GPIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTAT_GPIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTAT_GPIO {
    #[inline(always)]
    fn from(val: u8) -> MSTAT_GPIO {
        MSTAT_GPIO::from_bits(val)
    }
}
impl From<MSTAT_GPIO> for u8 {
    #[inline(always)]
    fn from(val: MSTAT_GPIO) -> u8 {
        MSTAT_GPIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTAT_HSTALL {
    _RESERVED_0 = 0x0,
    #[doc = "Host is stalled on a read from or write to a port that has the STALLRD or STALLWR bit set in the PnCFG register. The application must write 1 to this bit to clear and release the host. The stall should not allowed to persist."]
    ENABLE = 0x01,
}
impl MSTAT_HSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTAT_HSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTAT_HSTALL {
    #[inline(always)]
    fn from(val: u8) -> MSTAT_HSTALL {
        MSTAT_HSTALL::from_bits(val)
    }
}
impl From<MSTAT_HSTALL> for u8 {
    #[inline(always)]
    fn from(val: MSTAT_HSTALL) -> u8 {
        MSTAT_HSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTAT_IRQUPD {
    _RESERVED_0 = 0x0,
    #[doc = "The bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done). Sticky, must clear."]
    ENABLE = 0x01,
}
impl MSTAT_IRQUPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTAT_IRQUPD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTAT_IRQUPD {
    #[inline(always)]
    fn from(val: u8) -> MSTAT_IRQUPD {
        MSTAT_IRQUPD::from_bits(val)
    }
}
impl From<MSTAT_IRQUPD> for u8 {
    #[inline(always)]
    fn from(val: MSTAT_IRQUPD) -> u8 {
        MSTAT_IRQUPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTAT_WIRECHG {
    _RESERVED_0 = 0x0,
    #[doc = "For eSPI, one or more input VWires have changed since last cleared for eSPI. For LPC, SERIRQ started. Sticky, write to clear."]
    ENABLE = 0x01,
}
impl MSTAT_WIRECHG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTAT_WIRECHG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTAT_WIRECHG {
    #[inline(always)]
    fn from(val: u8) -> MSTAT_WIRECHG {
        MSTAT_WIRECHG::from_bits(val)
    }
}
impl From<MSTAT_WIRECHG> for u8 {
    #[inline(always)]
    fn from(val: MSTAT_WIRECHG) -> u8 {
        MSTAT_WIRECHG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NUM_OF_TARGET {
    #[doc = "1 RPMC flash device is supported."]
    MIXXN_2KB = 0x0,
    #[doc = "2 RPMC flash device is supported."]
    MIYYN_4KB = 0x01,
    #[doc = "3 RPMC flash device is supported."]
    MXXXIN_8KB = 0x02,
    #[doc = "4 RPMC flash device is supported."]
    MINTTT_16KB = 0x03,
}
impl NUM_OF_TARGET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NUM_OF_TARGET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NUM_OF_TARGET {
    #[inline(always)]
    fn from(val: u8) -> NUM_OF_TARGET {
        NUM_OF_TARGET::from_bits(val)
    }
}
impl From<NUM_OF_TARGET> for u8 {
    #[inline(always)]
    fn from(val: NUM_OF_TARGET) -> u8 {
        NUM_OF_TARGET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OMFLEN_TRANS {
    #[doc = "OOB: to host; Master: to host 32 (host reads with 32-bit address); SAF: Completion fail."]
    oob = 0x0,
    #[doc = "Master: to host 64 (host reads w/64-bit address); MAF: read flash (location in RAM); SAF: completion with data."]
    read = 0x01,
    #[doc = "Master: from host 32 (host writes w/32-bit address); MAF: write flash (location in RAM); SAF: completion with no data."]
    write = 0x02,
    #[doc = "Master: from host 64 (host writes w/64-bit address); MAF: erase flash (sector in RAM)."]
    erase = 0x03,
}
impl OMFLEN_TRANS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OMFLEN_TRANS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OMFLEN_TRANS {
    #[inline(always)]
    fn from(val: u8) -> OMFLEN_TRANS {
        OMFLEN_TRANS::from_bits(val)
    }
}
impl From<OMFLEN_TRANS> for u8 {
    #[inline(always)]
    fn from(val: OMFLEN_TRANS) -> u8 {
        OMFLEN_TRANS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OOBMX {
    _RESERVED_0 = 0x0,
    #[doc = "64 bytes address aligned max payload size."]
    MIN_4KB = 0x01,
    #[doc = "128 bytes address aligned max payload size."]
    MIN_8KB = 0x02,
    #[doc = "256 bytes address aligned max payload size."]
    MIN_16KB = 0x03,
}
impl OOBMX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OOBMX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OOBMX {
    #[inline(always)]
    fn from(val: u8) -> OOBMX {
        OOBMX::from_bits(val)
    }
}
impl From<OOBMX> for u8 {
    #[inline(always)]
    fn from(val: OOBMX) -> u8 {
        OOBMX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OOBOK {
    #[doc = "Do not allow OOB."]
    DISABLE = 0x0,
    #[doc = "Allow OOB."]
    ENABLE = 0x01,
}
impl OOBOK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OOBOK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OOBOK {
    #[inline(always)]
    fn from(val: u8) -> OOBOK {
        OOBOK::from_bits(val)
    }
}
impl From<OOBOK> for u8 {
    #[inline(always)]
    fn from(val: OOBOK) -> u8 {
        OOBOK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OOBSZ {
    _RESERVED_0 = 0x0,
    #[doc = "64-byte payload for memory."]
    MEMSZ0 = 0x01,
    #[doc = "128-byte payload for memory and OOB access."]
    MEMSZ1 = 0x02,
    #[doc = "256-byte payload for memory and OOB access."]
    MEMSZ11 = 0x03,
}
impl OOBSZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OOBSZ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OOBSZ {
    #[inline(always)]
    fn from(val: u8) -> OOBSZ {
        OOBSZ::from_bits(val)
    }
}
impl From<OOBSZ> for u8 {
    #[inline(always)]
    fn from(val: OOBSZ) -> u8 {
        OOBSZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RISGP {
    _RESERVED_0 = 0x0,
    #[doc = "In eSPI mode, if this bit is 1, RSTN pin is not used as Reset (affects behavior). It can be used as GPIO if Alert is dedicated to Alert (instead of MISO). This option does not apply in LPC mode."]
    riscp1 = 0x01,
}
impl RISGP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RISGP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RISGP {
    #[inline(always)]
    fn from(val: u8) -> RISGP {
        RISGP::from_bits(val)
    }
}
impl From<RISGP> for u8 {
    #[inline(always)]
    fn from(val: RISGP) -> u8 {
        RISGP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_INT_BMC {
    #[doc = "BMC does not support an integrated RTC."]
    BMC_NOT_SUPPORT = 0x0,
    #[doc = "BMC supports an integrated RTC to which eSPI controller can forward RTC targeting IO cycles. (**ESPI slave regs: 0x8, bit 29**)."]
    BMC_SUPPORT = 0x01,
}
impl RTC_INT_BMC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_INT_BMC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_INT_BMC {
    #[inline(always)]
    fn from(val: u8) -> RTC_INT_BMC {
        RTC_INT_BMC::from_bits(val)
    }
}
impl From<RTC_INT_BMC> for u8 {
    #[inline(always)]
    fn from(val: RTC_INT_BMC) -> u8 {
        RTC_INT_BMC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SAFERA {
    #[doc = "2 kB."]
    MIN_2KB = 0x0,
    #[doc = "4 kB."]
    MIN_4KB = 0x01,
    #[doc = "8 kB."]
    MIN_8KB = 0x02,
    #[doc = "16 kB."]
    MIN_16KB = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SAFERA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SAFERA {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SAFERA {
    #[inline(always)]
    fn from(val: u8) -> SAFERA {
        SAFERA::from_bits(val)
    }
}
impl From<SAFERA> for u8 {
    #[inline(always)]
    fn from(val: SAFERA) -> u8 {
        SAFERA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPICAP {
    #[doc = "SPI only."]
    SPI = 0x0,
    #[doc = "BiSPI and SPI."]
    BSPI_SPI = 0x01,
    #[doc = "FLEXSPI and SPI."]
    FLEXSPI_SPI = 0x02,
    #[doc = "Any."]
    ANY = 0x03,
}
impl SPICAP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPICAP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPICAP {
    #[inline(always)]
    fn from(val: u8) -> SPICAP {
        SPICAP::from_bits(val)
    }
}
impl From<SPICAP> for u8 {
    #[inline(always)]
    fn from(val: SPICAP) -> u8 {
        SPICAP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPIMOD {
    #[doc = "SPI."]
    SPI = 0x0,
    #[doc = "BiSPI."]
    BSPI = 0x01,
    #[doc = "FLEXSPI."]
    FLEXSPI = 0x02,
    _RESERVED_3 = 0x03,
}
impl SPIMOD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPIMOD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPIMOD {
    #[inline(always)]
    fn from(val: u8) -> SPIMOD {
        SPIMOD::from_bits(val)
    }
}
impl From<SPIMOD> for u8 {
    #[inline(always)]
    fn from(val: SPIMOD) -> u8 {
        SPIMOD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPISPD {
    #[doc = "20 MHz or less."]
    LESS_AND_20M = 0x0,
    #[doc = "25 MHz or 24 MHz."]
    FREQ_25M_24M = 0x01,
    #[doc = "33 MHz or 30 MHz."]
    FREQ_33M_30M = 0x02,
    #[doc = "50 MHz or 48 MHz."]
    FREQ_50M_48M = 0x03,
    #[doc = "66 MHz or 60 MHz."]
    FREQ_66M_60M = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SPISPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPISPD {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPISPD {
    #[inline(always)]
    fn from(val: u8) -> SPISPD {
        SPISPD::from_bits(val)
    }
}
impl From<SPISPD> for u8 {
    #[inline(always)]
    fn from(val: SPISPD) -> u8 {
        SPISPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STAT_RDSTAT {
    #[doc = "Endpoint or index-and-data: Empty. Mailbox: Empty (host read to end). Master/Flash: No requests."]
    rdstat0 = 0x0,
    #[doc = "Endpoint or index-and-data: Data waiting from MCU. Mailbox: Started (by MCU). Master/Flash: Started. From-host: goes to Pending next. To-host: goes to Complete next. SAF: host made request: Started."]
    rdstat1 = 0x01,
    #[doc = "Mailbox: Complete (by MCU). Master/Flash: Complete. SAF: Complete."]
    rdstat2 = 0x02,
    #[doc = "Mailbox: Partially read (by host). Master/Flash: From-host only, Pending (request made); goes to Complete next. SAF: MCU has setup completion."]
    rdstat3 = 0x03,
}
impl STAT_RDSTAT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STAT_RDSTAT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STAT_RDSTAT {
    #[inline(always)]
    fn from(val: u8) -> STAT_RDSTAT {
        STAT_RDSTAT::from_bits(val)
    }
}
impl From<STAT_RDSTAT> for u8 {
    #[inline(always)]
    fn from(val: STAT_RDSTAT) -> u8 {
        STAT_RDSTAT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STAT_RPMC_1_OR_2 {
    #[doc = "RPMC operation 1 has been chosen."]
    OPERATION_0 = 0x0,
    #[doc = "RPMC operation 2 has been chosen."]
    OPERATION_1 = 0x01,
}
impl STAT_RPMC_1_OR_2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STAT_RPMC_1_OR_2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STAT_RPMC_1_OR_2 {
    #[inline(always)]
    fn from(val: u8) -> STAT_RPMC_1_OR_2 {
        STAT_RPMC_1_OR_2::from_bits(val)
    }
}
impl From<STAT_RPMC_1_OR_2> for u8 {
    #[inline(always)]
    fn from(val: STAT_RPMC_1_OR_2) -> u8 {
        STAT_RPMC_1_OR_2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STAT_RPMC_FLASH_DEVICE {
    #[doc = "First device."]
    FIRST = 0x0,
    #[doc = "Second device."]
    SECOND = 0x01,
    #[doc = "Third device."]
    THIRD = 0x02,
    _RESERVED_3 = 0x03,
}
impl STAT_RPMC_FLASH_DEVICE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STAT_RPMC_FLASH_DEVICE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STAT_RPMC_FLASH_DEVICE {
    #[inline(always)]
    fn from(val: u8) -> STAT_RPMC_FLASH_DEVICE {
        STAT_RPMC_FLASH_DEVICE::from_bits(val)
    }
}
impl From<STAT_RPMC_FLASH_DEVICE> for u8 {
    #[inline(always)]
    fn from(val: STAT_RPMC_FLASH_DEVICE) -> u8 {
        STAT_RPMC_FLASH_DEVICE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STAT_WRSTAT {
    #[doc = "Endpoint: Empty. Index-and-data: Empty. Mailbox: Empty. Master/flash: Not last request."]
    wrstat0 = 0x0,
    #[doc = "Endpoint: data waiting from host. Index-and-data: data waiting from host (index may have been written before). Mailbox: Started (by host). Master/flash: last request from-host/read-flash (writes to MCU). SAF: Request from master was or is Flash Read."]
    wrstat1 = 0x01,
    #[doc = "Endpoint: Empty, but last was CMD. Index-and-data: wrote index. Mailbox: complete/last (host to end). Master/flash: last request to-host/write-flash (read from MCU). SAF: Request from Master was or is Flash Write."]
    wrstat2 = 0x02,
    #[doc = "Endpoint: CMD waiting from host. Index-and-data: Wrote Data then Index. Mailbox: Partially read (by MCU). Flash: Last Requested Erase. SAF: Request from Master was or is Flash Erase."]
    wrstat3 = 0x03,
}
impl STAT_WRSTAT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STAT_WRSTAT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STAT_WRSTAT {
    #[inline(always)]
    fn from(val: u8) -> STAT_WRSTAT {
        STAT_WRSTAT::from_bits(val)
    }
}
impl From<STAT_WRSTAT> for u8 {
    #[inline(always)]
    fn from(val: STAT_WRSTAT) -> u8 {
        STAT_WRSTAT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TARGET_RPMC_SUPPORTED {
    #[doc = "Target does not support Replay Protected Monotonic counter."]
    MIN_2KB = 0x0,
    #[doc = "Target supports up to 1 Replay Protected Monotonic counter."]
    MIN_4KB = 0x01,
    #[doc = "Target supports up to 2 Replay Protected Monotonic counters."]
    MIN_8KB = 0x02,
    #[doc = "Target supports up to 63 Replay Protected Monotonic counters. The value of this field is the total sum of Replay Protected Monotonic counters supported by all RPMC flash devices behind the target. If RPMC is not supported by the target, this field must indicate a value of 0h."]
    MIN_16KB = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl TARGET_RPMC_SUPPORTED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TARGET_RPMC_SUPPORTED {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TARGET_RPMC_SUPPORTED {
    #[inline(always)]
    fn from(val: u8) -> TARGET_RPMC_SUPPORTED {
        TARGET_RPMC_SUPPORTED::from_bits(val)
    }
}
impl From<TARGET_RPMC_SUPPORTED> for u8 {
    #[inline(always)]
    fn from(val: TARGET_RPMC_SUPPORTED) -> u8 {
        TARGET_RPMC_SUPPORTED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRGT_REQ_SIZE_SUPP {
    _RESERVED_0 = 0x0,
    #[doc = "64 bytes max read request size."]
    SIXTY_FOUR = 0x01,
    #[doc = "128 bytes max read request size."]
    BYTESDDDD = 0x02,
    #[doc = "256 bytes max read request size."]
    BYTESSSSSS = 0x03,
    #[doc = "512 bytes max read request size."]
    BYTES = 0x04,
    #[doc = "1024 bytes max read request size."]
    BYTESS = 0x05,
    #[doc = "2048 bytes max read request size."]
    BYTESSSS = 0x06,
    #[doc = "4096 bytes max read request size."]
    BYTESSSSSTTTS = 0x07,
}
impl TRGT_REQ_SIZE_SUPP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRGT_REQ_SIZE_SUPP {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRGT_REQ_SIZE_SUPP {
    #[inline(always)]
    fn from(val: u8) -> TRGT_REQ_SIZE_SUPP {
        TRGT_REQ_SIZE_SUPP::from_bits(val)
    }
}
impl From<TRGT_REQ_SIZE_SUPP> for u8 {
    #[inline(always)]
    fn from(val: TRGT_REQ_SIZE_SUPP) -> u8 {
        TRGT_REQ_SIZE_SUPP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIREIN_GPIO_LEVEL {
    #[doc = "Low."]
    DISABLE = 0x0,
    #[doc = "High."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WIREIN_GPIO_LEVEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIREIN_GPIO_LEVEL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIREIN_GPIO_LEVEL {
    #[inline(always)]
    fn from(val: u8) -> WIREIN_GPIO_LEVEL {
        WIREIN_GPIO_LEVEL::from_bits(val)
    }
}
impl From<WIREIN_GPIO_LEVEL> for u8 {
    #[inline(always)]
    fn from(val: WIREIN_GPIO_LEVEL) -> u8 {
        WIREIN_GPIO_LEVEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIREIN_GPIO_VALID {
    #[doc = "Not valid."]
    NOT = 0x0,
    #[doc = "Valid."]
    VALID1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WIREIN_GPIO_VALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIREIN_GPIO_VALID {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIREIN_GPIO_VALID {
    #[inline(always)]
    fn from(val: u8) -> WIREIN_GPIO_VALID {
        WIREIN_GPIO_VALID::from_bits(val)
    }
}
impl From<WIREIN_GPIO_VALID> for u8 {
    #[inline(always)]
    fn from(val: WIREIN_GPIO_VALID) -> u8 {
        WIREIN_GPIO_VALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIREOUT_GPIO_LEVEL {
    #[doc = "Low."]
    DISABLE = 0x0,
    #[doc = "High."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WIREOUT_GPIO_LEVEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIREOUT_GPIO_LEVEL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIREOUT_GPIO_LEVEL {
    #[inline(always)]
    fn from(val: u8) -> WIREOUT_GPIO_LEVEL {
        WIREOUT_GPIO_LEVEL::from_bits(val)
    }
}
impl From<WIREOUT_GPIO_LEVEL> for u8 {
    #[inline(always)]
    fn from(val: WIREOUT_GPIO_LEVEL) -> u8 {
        WIREOUT_GPIO_LEVEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIREOUT_GPIO_VALID {
    #[doc = "Not valid."]
    NOT = 0x0,
    #[doc = "Valid."]
    VALID1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WIREOUT_GPIO_VALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIREOUT_GPIO_VALID {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIREOUT_GPIO_VALID {
    #[inline(always)]
    fn from(val: u8) -> WIREOUT_GPIO_VALID {
        WIREOUT_GPIO_VALID::from_bits(val)
    }
}
impl From<WIREOUT_GPIO_VALID> for u8 {
    #[inline(always)]
    fn from(val: WIREOUT_GPIO_VALID) -> u8 {
        WIREOUT_GPIO_VALID::to_bits(val)
    }
}
