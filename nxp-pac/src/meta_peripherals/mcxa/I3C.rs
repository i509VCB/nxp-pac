#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Improved Inter-Integrated Circuit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c {
    ptr: *mut u8,
}
unsafe impl Send for I3c {}
unsafe impl Sync for I3c {}
impl I3c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Controller Configuration."]
    #[inline(always)]
    pub const fn mconfig(self) -> crate::pac::common::Reg<Mconfig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Target Configuration."]
    #[inline(always)]
    pub const fn sconfig(self) -> crate::pac::common::Reg<Sconfig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Target Status."]
    #[inline(always)]
    pub const fn sstatus(self) -> crate::pac::common::Reg<Sstatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Target Control."]
    #[inline(always)]
    pub const fn sctrl(self) -> crate::pac::common::Reg<Sctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Target Interrupt Set."]
    #[inline(always)]
    pub const fn sintset(self) -> crate::pac::common::Reg<Sintset, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Target Interrupt Clear."]
    #[inline(always)]
    pub const fn sintclr(self) -> crate::pac::common::Reg<Sintclr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Target Interrupt Mask."]
    #[inline(always)]
    pub const fn sintmasked(self) -> crate::pac::common::Reg<Sintmasked, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Target Errors and Warnings."]
    #[inline(always)]
    pub const fn serrwarn(self) -> crate::pac::common::Reg<Serrwarn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Target DMA Control."]
    #[inline(always)]
    pub const fn sdmactrl(self) -> crate::pac::common::Reg<Sdmactrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Target HDR-BT Configuration."]
    #[inline(always)]
    pub const fn shdrbtcfg(self) -> crate::pac::common::Reg<Shdrbtcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Target HDR-Last."]
    #[inline(always)]
    pub const fn shdrbtlast(self) -> crate::pac::common::Reg<Shdrbtlast, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Target Data Control."]
    #[inline(always)]
    pub const fn sdatactrl(self) -> crate::pac::common::Reg<Sdatactrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Target Write Data Byte."]
    #[inline(always)]
    pub const fn swdatab(self) -> crate::pac::common::Reg<Swdatab, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Target Write Data Byte End."]
    #[inline(always)]
    pub const fn swdatabe(self) -> crate::pac::common::Reg<Swdatabe, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Target Write Data Halfword."]
    #[inline(always)]
    pub const fn swdatah(self) -> crate::pac::common::Reg<Swdatah, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Target Write Data Halfword End."]
    #[inline(always)]
    pub const fn swdatahe(self) -> crate::pac::common::Reg<Swdatahe, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Target Read Data Byte."]
    #[inline(always)]
    pub const fn srdatab(self) -> crate::pac::common::Reg<Srdatab, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Target Read Data Halfword."]
    #[inline(always)]
    pub const fn srdatah(self) -> crate::pac::common::Reg<Srdatah, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Target Write Data Byte."]
    #[inline(always)]
    pub const fn swdatab1(self) -> crate::pac::common::Reg<Swdatab1, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Target Capabilities 2."]
    #[inline(always)]
    pub const fn scapabilities2(
        self,
    ) -> crate::pac::common::Reg<Scapabilities2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Target Capabilities."]
    #[inline(always)]
    pub const fn scapabilities(
        self,
    ) -> crate::pac::common::Reg<Scapabilities, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Target Maximum Limits."]
    #[inline(always)]
    pub const fn smaxlimits(self) -> crate::pac::common::Reg<Smaxlimits, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Target ID Part Number."]
    #[inline(always)]
    pub const fn sidpartno(self) -> crate::pac::common::Reg<Sidpartno, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Target ID Extension."]
    #[inline(always)]
    pub const fn sidext(self) -> crate::pac::common::Reg<Sidext, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Target Vendor ID."]
    #[inline(always)]
    pub const fn svendorid(self) -> crate::pac::common::Reg<Svendorid, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Target Time Control Clock."]
    #[inline(always)]
    pub const fn stcclock(self) -> crate::pac::common::Reg<Stcclock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Target Message Map Address."]
    #[inline(always)]
    pub const fn smsgmapaddr(self) -> crate::pac::common::Reg<Smsgmapaddr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Controller Extended Configuration."]
    #[inline(always)]
    pub const fn mconfig_ext(self) -> crate::pac::common::Reg<MconfigExt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Controller Control."]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::pac::common::Reg<Mctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Controller Status."]
    #[inline(always)]
    pub const fn mstatus(self) -> crate::pac::common::Reg<Mstatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Controller In-band Interrupt Registry and Rules."]
    #[inline(always)]
    pub const fn mibirules(self) -> crate::pac::common::Reg<Mibirules, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Controller Interrupt Set."]
    #[inline(always)]
    pub const fn mintset(self) -> crate::pac::common::Reg<Mintset, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Controller Interrupt Clear."]
    #[inline(always)]
    pub const fn mintclr(self) -> crate::pac::common::Reg<Mintclr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Controller Interrupt Mask."]
    #[inline(always)]
    pub const fn mintmasked(self) -> crate::pac::common::Reg<Mintmasked, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Controller Errors and Warnings."]
    #[inline(always)]
    pub const fn merrwarn(self) -> crate::pac::common::Reg<Merrwarn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Controller DMA Control."]
    #[inline(always)]
    pub const fn mdmactrl(self) -> crate::pac::common::Reg<Mdmactrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Controller HDR-BT Configuration."]
    #[inline(always)]
    pub const fn mhdrbtcfg(self) -> crate::pac::common::Reg<Mhdrbtcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Controller HDR-Last."]
    #[inline(always)]
    pub const fn mhdrbtlast(self) -> crate::pac::common::Reg<Mhdrbtlast, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Controller Data Control."]
    #[inline(always)]
    pub const fn mdatactrl(self) -> crate::pac::common::Reg<Mdatactrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Controller Write Data Byte."]
    #[inline(always)]
    pub const fn mwdatab(self) -> crate::pac::common::Reg<Mwdatab, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Controller Write Data Byte End."]
    #[inline(always)]
    pub const fn mwdatabe(self) -> crate::pac::common::Reg<Mwdatabe, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Controller Write Data Halfword."]
    #[inline(always)]
    pub const fn mwdatah(self) -> crate::pac::common::Reg<Mwdatah, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Controller Write Data Halfword End."]
    #[inline(always)]
    pub const fn mwdatahe(self) -> crate::pac::common::Reg<Mwdatahe, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Controller Read Data Byte."]
    #[inline(always)]
    pub const fn mrdatab(self) -> crate::pac::common::Reg<Mrdatab, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Controller Read Data Halfword."]
    #[inline(always)]
    pub const fn mrdatah(self) -> crate::pac::common::Reg<Mrdatah, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Controller Write Byte Data 1 (to Bus)."]
    #[inline(always)]
    pub const fn mwdatab1(self) -> crate::pac::common::Reg<Mwdatab1, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Controller Write Message Control in SDR mode."]
    #[inline(always)]
    pub const fn mwmsg_sdr_control(
        self,
    ) -> crate::pac::common::Reg<MwmsgSdrControl, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Controller Write Message Data in SDR mode."]
    #[inline(always)]
    pub const fn mwmsg_sdr_data(
        self,
    ) -> crate::pac::common::Reg<MwmsgSdrData, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Controller Read Message in SDR mode."]
    #[inline(always)]
    pub const fn mrmsg_sdr(self) -> crate::pac::common::Reg<MrmsgSdr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Controller Write Message in DDR mode: First Control Word."]
    #[inline(always)]
    pub const fn mwmsg_ddr_control(
        self,
    ) -> crate::pac::common::Reg<MwmsgDdrControl, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Controller Write Message in DDR Mode Control 2."]
    #[inline(always)]
    pub const fn mwmsg_ddr_control2(
        self,
    ) -> crate::pac::common::Reg<MwmsgDdrControl2, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Controller Write Message Data in DDR mode."]
    #[inline(always)]
    pub const fn mwmsg_ddr_data(
        self,
    ) -> crate::pac::common::Reg<MwmsgDdrData, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Controller Read Message in DDR mode."]
    #[inline(always)]
    pub const fn mrmsg_ddr(self) -> crate::pac::common::Reg<MrmsgDdr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Controller Dynamic Address."]
    #[inline(always)]
    pub const fn mdynaddr(self) -> crate::pac::common::Reg<Mdynaddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Write Word Data (to Bus)."]
    #[inline(always)]
    pub const fn mwdataw(self) -> crate::pac::common::Reg<Mwdataw, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Read Word Data (from Bus)."]
    #[inline(always)]
    pub const fn mrdataw(self) -> crate::pac::common::Reg<Mrdataw, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Map Feature Control 0."]
    #[inline(always)]
    pub const fn smapctrl0(self) -> crate::pac::common::Reg<Smapctrl0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Map Feature Control 1."]
    #[inline(always)]
    pub const fn smapctrl1(self) -> crate::pac::common::Reg<Smapctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Extended IBI Data 1."]
    #[inline(always)]
    pub const fn ibiext1(self) -> crate::pac::common::Reg<Ibiext1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Extended IBI Data 2."]
    #[inline(always)]
    pub const fn ibiext2(self) -> crate::pac::common::Reg<Ibiext2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Target Write Word Data (to Bus)."]
    #[inline(always)]
    pub const fn swdataw(self) -> crate::pac::common::Reg<Swdataw, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Target Read Word Data (from Bus)."]
    #[inline(always)]
    pub const fn srdataw(self) -> crate::pac::common::Reg<Srdataw, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Target Module ID."]
    #[inline(always)]
    pub const fn sid(self) -> crate::pac::common::Reg<Sid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "Extended IBI Data 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibiext1(pub u32);
impl Ibiext1 {
    #[doc = "Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Count."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Maximum."]
    #[must_use]
    #[inline(always)]
    pub const fn max(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Maximum."]
    #[inline(always)]
    pub const fn set_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Extra Byte 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ext1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 1."]
    #[inline(always)]
    pub const fn set_ext1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Extra Byte 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ext2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 2."]
    #[inline(always)]
    pub const fn set_ext2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extra Byte 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ext3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 3."]
    #[inline(always)]
    pub const fn set_ext3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ibiext1 {
    #[inline(always)]
    fn default() -> Ibiext1 {
        Ibiext1(0)
    }
}
impl core::fmt::Debug for Ibiext1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ibiext1")
            .field("cnt", &self.cnt())
            .field("max", &self.max())
            .field("ext1", &self.ext1())
            .field("ext2", &self.ext2())
            .field("ext3", &self.ext3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ibiext1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ibiext1 {{ cnt: {=u8:?}, max: {=u8:?}, ext1: {=u8:?}, ext2: {=u8:?}, ext3: {=u8:?} }}",
            self.cnt(),
            self.max(),
            self.ext1(),
            self.ext2(),
            self.ext3()
        )
    }
}
#[doc = "Extended IBI Data 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibiext2(pub u32);
impl Ibiext2 {
    #[doc = "Extra Byte 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ext4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 4."]
    #[inline(always)]
    pub const fn set_ext4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Extra Byte 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ext5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 5."]
    #[inline(always)]
    pub const fn set_ext5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Extra Byte 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ext6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 6."]
    #[inline(always)]
    pub const fn set_ext6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extra Byte 7."]
    #[must_use]
    #[inline(always)]
    pub const fn ext7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 7."]
    #[inline(always)]
    pub const fn set_ext7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ibiext2 {
    #[inline(always)]
    fn default() -> Ibiext2 {
        Ibiext2(0)
    }
}
impl core::fmt::Debug for Ibiext2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ibiext2")
            .field("ext4", &self.ext4())
            .field("ext5", &self.ext5())
            .field("ext6", &self.ext6())
            .field("ext7", &self.ext7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ibiext2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ibiext2 {{ ext4: {=u8:?}, ext5: {=u8:?}, ext6: {=u8:?}, ext7: {=u8:?} }}",
            self.ext4(),
            self.ext5(),
            self.ext6(),
            self.ext7()
        )
    }
}
#[doc = "Controller Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mconfig(pub u32);
impl Mconfig {
    #[doc = "Controller Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mstena(&self) -> Mstena {
        let val = (self.0 >> 0usize) & 0x03;
        Mstena::from_bits(val as u8)
    }
    #[doc = "Controller Enable."]
    #[inline(always)]
    pub const fn set_mstena(&mut self, val: Mstena) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Disable Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn disto(&self) -> Disto {
        let val = (self.0 >> 3usize) & 0x01;
        Disto::from_bits(val as u8)
    }
    #[doc = "Disable Timeout."]
    #[inline(always)]
    pub const fn set_disto(&mut self, val: Disto) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "High-Keeper."]
    #[must_use]
    #[inline(always)]
    pub const fn hkeep(&self) -> Hkeep {
        let val = (self.0 >> 4usize) & 0x03;
        Hkeep::from_bits(val as u8)
    }
    #[doc = "High-Keeper."]
    #[inline(always)]
    pub const fn set_hkeep(&mut self, val: Hkeep) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Open-drain Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn odstop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Open-drain Stop."]
    #[inline(always)]
    pub const fn set_odstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Push-Pull Baud Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn ppbaud(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull Baud Rate."]
    #[inline(always)]
    pub const fn set_ppbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Push-Pull Low."]
    #[must_use]
    #[inline(always)]
    pub const fn pplow(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull Low."]
    #[inline(always)]
    pub const fn set_pplow(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Open-drain Baud Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn odbaud(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Open-drain Baud Rate."]
    #[inline(always)]
    pub const fn set_odbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Open-drain High Push-Pull."]
    #[must_use]
    #[inline(always)]
    pub const fn odhpp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Open-drain High Push-Pull."]
    #[inline(always)]
    pub const fn set_odhpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Skew."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Skew."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "I2C Baud Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cbaud(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "I2C Baud Rate."]
    #[inline(always)]
    pub const fn set_i2cbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Mconfig {
    #[inline(always)]
    fn default() -> Mconfig {
        Mconfig(0)
    }
}
impl core::fmt::Debug for Mconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mconfig")
            .field("mstena", &self.mstena())
            .field("disto", &self.disto())
            .field("hkeep", &self.hkeep())
            .field("odstop", &self.odstop())
            .field("ppbaud", &self.ppbaud())
            .field("pplow", &self.pplow())
            .field("odbaud", &self.odbaud())
            .field("odhpp", &self.odhpp())
            .field("skew", &self.skew())
            .field("i2cbaud", &self.i2cbaud())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mconfig {{ mstena: {:?}, disto: {:?}, hkeep: {:?}, odstop: {=bool:?}, ppbaud: {=u8:?}, pplow: {=u8:?}, odbaud: {=u8:?}, odhpp: {=bool:?}, skew: {=u8:?}, i2cbaud: {=u8:?} }}",
            self.mstena(),
            self.disto(),
            self.hkeep(),
            self.odstop(),
            self.ppbaud(),
            self.pplow(),
            self.odbaud(),
            self.odhpp(),
            self.skew(),
            self.i2cbaud()
        )
    }
}
#[doc = "Controller Extended Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MconfigExt(pub u32);
impl MconfigExt {
    #[doc = "I3C CAS Delay After START."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c_cas_del(&self) -> I3cCasDel {
        let val = (self.0 >> 16usize) & 0x03;
        I3cCasDel::from_bits(val as u8)
    }
    #[doc = "I3C CAS Delay After START."]
    #[inline(always)]
    pub const fn set_i3c_cas_del(&mut self, val: I3cCasDel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "I3C CAS Delay After Repeated START."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c_casr_del(&self) -> I3cCasrDel {
        let val = (self.0 >> 18usize) & 0x03;
        I3cCasrDel::from_bits(val as u8)
    }
    #[doc = "I3C CAS Delay After Repeated START."]
    #[inline(always)]
    pub const fn set_i3c_casr_del(&mut self, val: I3cCasrDel) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
}
impl Default for MconfigExt {
    #[inline(always)]
    fn default() -> MconfigExt {
        MconfigExt(0)
    }
}
impl core::fmt::Debug for MconfigExt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MconfigExt")
            .field("i3c_cas_del", &self.i3c_cas_del())
            .field("i3c_casr_del", &self.i3c_casr_del())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MconfigExt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MconfigExt {{ i3c_cas_del: {:?}, i3c_casr_del: {:?} }}",
            self.i3c_cas_del(),
            self.i3c_casr_del()
        )
    }
}
#[doc = "Controller Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc = "Request."]
    #[must_use]
    #[inline(always)]
    pub const fn request(&self) -> Request {
        let val = (self.0 >> 0usize) & 0x07;
        Request::from_bits(val as u8)
    }
    #[doc = "Request."]
    #[inline(always)]
    pub const fn set_request(&mut self, val: Request) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bus Type with EmitStartAddr."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> Type {
        let val = (self.0 >> 4usize) & 0x03;
        Type::from_bits(val as u8)
    }
    #[doc = "Bus Type with EmitStartAddr."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: Type) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "In-Band Interrupt Response."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiresp(&self) -> Ibiresp {
        let val = (self.0 >> 6usize) & 0x03;
        Ibiresp::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupt Response."]
    #[inline(always)]
    pub const fn set_ibiresp(&mut self, val: Ibiresp) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> MctrlDir {
        let val = (self.0 >> 8usize) & 0x01;
        MctrlDir::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: MctrlDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Address."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "Read Terminate Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rdterm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Read Terminate Counter."]
    #[inline(always)]
    pub const fn set_rdterm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("request", &self.request())
            .field("type_", &self.type_())
            .field("ibiresp", &self.ibiresp())
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("rdterm", &self.rdterm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ request: {:?}, type_: {:?}, ibiresp: {:?}, dir: {:?}, addr: {=u8:?}, rdterm: {=u8:?} }}",
            self.request(),
            self.type_(),
            self.ibiresp(),
            self.dir(),
            self.addr(),
            self.rdterm()
        )
    }
}
#[doc = "Controller Data Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdatactrl(pub u32);
impl Mdatactrl {
    #[doc = "Flush To-Bus Buffer or FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn flushtb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush To-Bus Buffer or FIFO."]
    #[inline(always)]
    pub const fn set_flushtb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flush From-Bus Buffer or FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn flushfb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flush From-Bus Buffer or FIFO."]
    #[inline(always)]
    pub const fn set_flushfb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Unlock."]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock."]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit Trigger Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txtrig(&self) -> MdatactrlTxtrig {
        let val = (self.0 >> 4usize) & 0x03;
        MdatactrlTxtrig::from_bits(val as u8)
    }
    #[doc = "Transmit Trigger Level."]
    #[inline(always)]
    pub const fn set_txtrig(&mut self, val: MdatactrlTxtrig) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Receive Trigger Level."]
    #[must_use]
    #[inline(always)]
    pub const fn rxtrig(&self) -> MdatactrlRxtrig {
        let val = (self.0 >> 6usize) & 0x03;
        MdatactrlRxtrig::from_bits(val as u8)
    }
    #[doc = "Receive Trigger Level."]
    #[inline(always)]
    pub const fn set_rxtrig(&mut self, val: MdatactrlRxtrig) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Transmit Entry Count."]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Entry Count."]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Receive Entry Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive Entry Count."]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Transmit is Full."]
    #[must_use]
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit is Full."]
    #[inline(always)]
    pub const fn set_txfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive is Empty."]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mdatactrl {
    #[inline(always)]
    fn default() -> Mdatactrl {
        Mdatactrl(0)
    }
}
impl core::fmt::Debug for Mdatactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdatactrl")
            .field("flushtb", &self.flushtb())
            .field("flushfb", &self.flushfb())
            .field("unlock", &self.unlock())
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdatactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdatactrl {{ flushtb: {=bool:?}, flushfb: {=bool:?}, unlock: {=bool:?}, txtrig: {:?}, rxtrig: {:?}, txcount: {=u8:?}, rxcount: {=u8:?}, txfull: {=bool:?}, rxempty: {=bool:?} }}",
            self.flushtb(),
            self.flushfb(),
            self.unlock(),
            self.txtrig(),
            self.rxtrig(),
            self.txcount(),
            self.rxcount(),
            self.txfull(),
            self.rxempty()
        )
    }
}
#[doc = "Controller DMA Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdmactrl(pub u32);
impl Mdmactrl {
    #[doc = "DMA from Bus."]
    #[must_use]
    #[inline(always)]
    pub const fn dmafb(&self) -> MdmactrlDmafb {
        let val = (self.0 >> 0usize) & 0x03;
        MdmactrlDmafb::from_bits(val as u8)
    }
    #[doc = "DMA from Bus."]
    #[inline(always)]
    pub const fn set_dmafb(&mut self, val: MdmactrlDmafb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA to Bus."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatb(&self) -> MdmactrlDmatb {
        let val = (self.0 >> 2usize) & 0x03;
        MdmactrlDmatb::from_bits(val as u8)
    }
    #[doc = "DMA to Bus."]
    #[inline(always)]
    pub const fn set_dmatb(&mut self, val: MdmactrlDmatb) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DMA Width."]
    #[must_use]
    #[inline(always)]
    pub const fn dmawidth(&self) -> MdmactrlDmawidth {
        let val = (self.0 >> 4usize) & 0x03;
        MdmactrlDmawidth::from_bits(val as u8)
    }
    #[doc = "DMA Width."]
    #[inline(always)]
    pub const fn set_dmawidth(&mut self, val: MdmactrlDmawidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Mdmactrl {
    #[inline(always)]
    fn default() -> Mdmactrl {
        Mdmactrl(0)
    }
}
impl core::fmt::Debug for Mdmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdmactrl")
            .field("dmafb", &self.dmafb())
            .field("dmatb", &self.dmatb())
            .field("dmawidth", &self.dmawidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdmactrl {{ dmafb: {:?}, dmatb: {:?}, dmawidth: {:?} }}",
            self.dmafb(),
            self.dmatb(),
            self.dmawidth()
        )
    }
}
#[doc = "Controller Dynamic Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdynaddr(pub u32);
impl Mdynaddr {
    #[doc = "Dynamic Address Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn davalid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic Address Valid."]
    #[inline(always)]
    pub const fn set_davalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address."]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address."]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Mdynaddr {
    #[inline(always)]
    fn default() -> Mdynaddr {
        Mdynaddr(0)
    }
}
impl core::fmt::Debug for Mdynaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdynaddr")
            .field("davalid", &self.davalid())
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdynaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdynaddr {{ davalid: {=bool:?}, daddr: {=u8:?} }}",
            self.davalid(),
            self.daddr()
        )
    }
}
#[doc = "Controller Errors and Warnings."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Merrwarn(pub u32);
impl Merrwarn {
    #[doc = "Underrun Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn urun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun Error Flag."]
    #[inline(always)]
    pub const fn set_urun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Not Acknowledge Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledge Error Flag."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Abort Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wrabt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write Abort Error Flag."]
    #[inline(always)]
    pub const fn set_wrabt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Terminate Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn term(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Terminate Error Flag."]
    #[inline(always)]
    pub const fn set_term(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "High Data Rate Parity Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hpar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate Parity Flag."]
    #[inline(always)]
    pub const fn set_hpar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High Data Rate CRC Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hcrc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate CRC Error Flag."]
    #[inline(always)]
    pub const fn set_hcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Overread Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn oread(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Overread Error Flag."]
    #[inline(always)]
    pub const fn set_oread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Overwrite Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn owrite(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Overwrite Error Flag."]
    #[inline(always)]
    pub const fn set_owrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Message Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn msgerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Message Error Flag."]
    #[inline(always)]
    pub const fn set_msgerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Invalid Request Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn invreq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Request Error Flag."]
    #[inline(always)]
    pub const fn set_invreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Timeout Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Error Flag."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Merrwarn {
    #[inline(always)]
    fn default() -> Merrwarn {
        Merrwarn(0)
    }
}
impl core::fmt::Debug for Merrwarn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Merrwarn")
            .field("urun", &self.urun())
            .field("nack", &self.nack())
            .field("wrabt", &self.wrabt())
            .field("term", &self.term())
            .field("hpar", &self.hpar())
            .field("hcrc", &self.hcrc())
            .field("oread", &self.oread())
            .field("owrite", &self.owrite())
            .field("msgerr", &self.msgerr())
            .field("invreq", &self.invreq())
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Merrwarn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Merrwarn {{ urun: {=bool:?}, nack: {=bool:?}, wrabt: {=bool:?}, term: {=bool:?}, hpar: {=bool:?}, hcrc: {=bool:?}, oread: {=bool:?}, owrite: {=bool:?}, msgerr: {=bool:?}, invreq: {=bool:?}, timeout: {=bool:?} }}",
            self.urun(),
            self.nack(),
            self.wrabt(),
            self.term(),
            self.hpar(),
            self.hcrc(),
            self.oread(),
            self.owrite(),
            self.msgerr(),
            self.invreq(),
            self.timeout()
        )
    }
}
#[doc = "Controller HDR-BT Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhdrbtcfg(pub u32);
impl Mhdrbtcfg {
    #[doc = "Multi-Lane HDR."]
    #[must_use]
    #[inline(always)]
    pub const fn mlhdr(&self) -> Mlhdr {
        let val = (self.0 >> 0usize) & 0x03;
        Mlhdr::from_bits(val as u8)
    }
    #[doc = "Multi-Lane HDR."]
    #[inline(always)]
    pub const fn set_mlhdr(&mut self, val: Mlhdr) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Multi-Lane Data."]
    #[must_use]
    #[inline(always)]
    pub const fn mldat(&self) -> Mldat {
        let val = (self.0 >> 2usize) & 0x03;
        Mldat::from_bits(val as u8)
    }
    #[doc = "Multi-Lane Data."]
    #[inline(always)]
    pub const fn set_mldat(&mut self, val: Mldat) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CRC32."]
    #[must_use]
    #[inline(always)]
    pub const fn crc32(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CRC32."]
    #[inline(always)]
    pub const fn set_crc32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Data Length."]
    #[must_use]
    #[inline(always)]
    pub const fn datalen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Length."]
    #[inline(always)]
    pub const fn set_datalen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mhdrbtcfg {
    #[inline(always)]
    fn default() -> Mhdrbtcfg {
        Mhdrbtcfg(0)
    }
}
impl core::fmt::Debug for Mhdrbtcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mhdrbtcfg")
            .field("mlhdr", &self.mlhdr())
            .field("mldat", &self.mldat())
            .field("crc32", &self.crc32())
            .field("datalen", &self.datalen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mhdrbtcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mhdrbtcfg {{ mlhdr: {:?}, mldat: {:?}, crc32: {=bool:?}, datalen: {=u16:?} }}",
            self.mlhdr(),
            self.mldat(),
            self.crc32(),
            self.datalen()
        )
    }
}
#[doc = "Controller HDR-Last."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhdrbtlast(pub u32);
impl Mhdrbtlast {
    #[doc = "Data Length."]
    #[must_use]
    #[inline(always)]
    pub const fn datalen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Length."]
    #[inline(always)]
    pub const fn set_datalen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mhdrbtlast {
    #[inline(always)]
    fn default() -> Mhdrbtlast {
        Mhdrbtlast(0)
    }
}
impl core::fmt::Debug for Mhdrbtlast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mhdrbtlast")
            .field("datalen", &self.datalen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mhdrbtlast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mhdrbtlast {{ datalen: {=u16:?} }}", self.datalen())
    }
}
#[doc = "Controller In-band Interrupt Registry and Rules."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mibirules(pub u32);
impl Mibirules {
    #[doc = "ADDR0."]
    #[must_use]
    #[inline(always)]
    pub const fn addr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR0."]
    #[inline(always)]
    pub const fn set_addr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "ADDR1."]
    #[must_use]
    #[inline(always)]
    pub const fn addr1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR1."]
    #[inline(always)]
    pub const fn set_addr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "ADDR2."]
    #[must_use]
    #[inline(always)]
    pub const fn addr2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR2."]
    #[inline(always)]
    pub const fn set_addr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "ADDR3."]
    #[must_use]
    #[inline(always)]
    pub const fn addr3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR3."]
    #[inline(always)]
    pub const fn set_addr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "ADDR4."]
    #[must_use]
    #[inline(always)]
    pub const fn addr4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR4."]
    #[inline(always)]
    pub const fn set_addr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Most Significant Address Bit is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn msb0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Most Significant Address Bit is 0."]
    #[inline(always)]
    pub const fn set_msb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "No IBI byte."]
    #[must_use]
    #[inline(always)]
    pub const fn nobyte(&self) -> Nobyte {
        let val = (self.0 >> 31usize) & 0x01;
        Nobyte::from_bits(val as u8)
    }
    #[doc = "No IBI byte."]
    #[inline(always)]
    pub const fn set_nobyte(&mut self, val: Nobyte) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mibirules {
    #[inline(always)]
    fn default() -> Mibirules {
        Mibirules(0)
    }
}
impl core::fmt::Debug for Mibirules {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mibirules")
            .field("addr0", &self.addr0())
            .field("addr1", &self.addr1())
            .field("addr2", &self.addr2())
            .field("addr3", &self.addr3())
            .field("addr4", &self.addr4())
            .field("msb0", &self.msb0())
            .field("nobyte", &self.nobyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mibirules {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mibirules {{ addr0: {=u8:?}, addr1: {=u8:?}, addr2: {=u8:?}, addr3: {=u8:?}, addr4: {=u8:?}, msb0: {=bool:?}, nobyte: {:?} }}",
            self.addr0(),
            self.addr1(),
            self.addr2(),
            self.addr3(),
            self.addr4(),
            self.msb0(),
            self.nobyte()
        )
    }
}
#[doc = "Controller Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintclr(pub u32);
impl Mintclr {
    #[doc = "SLVSTART Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SLVSTART Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MCTRLDONE Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MCTRLDONE Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "COMPLETE Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXNOTFULL Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXNOTFULL Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBIWON Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBIWON Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NOWCONTROLLER Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NOWCONTROLLER Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintclr {
    #[inline(always)]
    fn default() -> Mintclr {
        Mintclr(0)
    }
}
impl core::fmt::Debug for Mintclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintclr")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintclr {{ slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {=bool:?}, txnotfull: {=bool:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?} }}",
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster()
        )
    }
}
#[doc = "Controller Interrupt Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintmasked(pub u32);
impl Mintmasked {
    #[doc = "SLVSTART Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SLVSTART Interrupt Mask."]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MCTRLDONE Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MCTRLDONE Interrupt Mask."]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "COMPLETE Interrupt Mask."]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXNOTFULL Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXNOTFULL Interrupt Mask."]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBIWON Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBIWON Interrupt Mask."]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERRWARN Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Mask."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NOWCONTROLLER Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NOWCONTROLLER Interrupt Mask."]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintmasked {
    #[inline(always)]
    fn default() -> Mintmasked {
        Mintmasked(0)
    }
}
impl core::fmt::Debug for Mintmasked {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintmasked")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintmasked {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintmasked {{ slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {=bool:?}, txnotfull: {=bool:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?} }}",
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster()
        )
    }
}
#[doc = "Controller Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintset(pub u32);
impl Mintset {
    #[doc = "Target Start Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Target Start Interrupt Enable."]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Controller Control Done Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Control Done Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Completed Message Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Completed Message Interrupt Enable."]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Pending Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Pending Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Buffer/FIFO Not Full Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer/FIFO Not Full Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBI Won Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBI Won Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error or Warning (ERRWARN) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or Warning (ERRWARN) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Now Controller Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Now Controller Interrupt Enable."]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintset {
    #[inline(always)]
    fn default() -> Mintset {
        Mintset(0)
    }
}
impl core::fmt::Debug for Mintset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintset")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintset {{ slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {=bool:?}, txnotfull: {=bool:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?} }}",
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster()
        )
    }
}
#[doc = "Controller Read Data Byte."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdatab(pub u32);
impl Mrdatab {
    #[doc = "Value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mrdatab {
    #[inline(always)]
    fn default() -> Mrdatab {
        Mrdatab(0)
    }
}
impl core::fmt::Debug for Mrdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdatab")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mrdatab {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Controller Read Data Halfword."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdatah(pub u32);
impl Mrdatah {
    #[doc = "Low Byte."]
    #[must_use]
    #[inline(always)]
    pub const fn lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte."]
    #[inline(always)]
    pub const fn set_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High Byte."]
    #[must_use]
    #[inline(always)]
    pub const fn msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High Byte."]
    #[inline(always)]
    pub const fn set_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Mrdatah {
    #[inline(always)]
    fn default() -> Mrdatah {
        Mrdatah(0)
    }
}
impl core::fmt::Debug for Mrdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdatah")
            .field("lsb", &self.lsb())
            .field("msb", &self.msb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mrdatah {{ lsb: {=u8:?}, msb: {=u8:?} }}",
            self.lsb(),
            self.msb()
        )
    }
}
#[doc = "Read Word Data (from Bus)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdataw(pub u32);
impl Mrdataw {
    #[doc = "Value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mrdataw {
    #[inline(always)]
    fn default() -> Mrdataw {
        Mrdataw(0)
    }
}
impl core::fmt::Debug for Mrdataw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdataw")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdataw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mrdataw {{ value: {=u32:?} }}", self.value())
    }
}
#[doc = "Controller Read Message in DDR mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrmsgDdr(pub u32);
impl MrmsgDdr {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MrmsgDdr {
    #[inline(always)]
    fn default() -> MrmsgDdr {
        MrmsgDdr(0)
    }
}
impl core::fmt::Debug for MrmsgDdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrmsgDdr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrmsgDdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrmsgDdr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Controller Read Message in SDR mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrmsgSdr(pub u32);
impl MrmsgSdr {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MrmsgSdr {
    #[inline(always)]
    fn default() -> MrmsgSdr {
        MrmsgSdr(0)
    }
}
impl core::fmt::Debug for MrmsgSdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrmsgSdr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrmsgSdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrmsgSdr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Controller Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstatus(pub u32);
impl Mstatus {
    #[doc = "State of the Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> State {
        let val = (self.0 >> 0usize) & 0x07;
        State::from_bits(val as u8)
    }
    #[doc = "State of the Controller."]
    #[inline(always)]
    pub const fn set_state(&mut self, val: State) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Between."]
    #[must_use]
    #[inline(always)]
    pub const fn between(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Between."]
    #[inline(always)]
    pub const fn set_between(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Not Acknowledged."]
    #[must_use]
    #[inline(always)]
    pub const fn nacked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledged."]
    #[inline(always)]
    pub const fn set_nacked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "In-Band Interrupt (IBI) Type."]
    #[must_use]
    #[inline(always)]
    pub const fn ibitype(&self) -> Ibitype {
        let val = (self.0 >> 6usize) & 0x03;
        Ibitype::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupt (IBI) Type."]
    #[inline(always)]
    pub const fn set_ibitype(&mut self, val: Ibitype) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Target Start Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Target Start Flag."]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Controller Control Done Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Control Done Flag."]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Complete Flag."]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TX Buffer or FIFO Not Full."]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TX Buffer or FIFO Not Full."]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "In-Band Interrupt (IBI) Won Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt (IBI) Won Flag."]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error or Warning."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or Warning."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Module is now Controller Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Module is now Controller Flag."]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "IBI Address."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiaddr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "IBI Address."]
    #[inline(always)]
    pub const fn set_ibiaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Mstatus {
    #[inline(always)]
    fn default() -> Mstatus {
        Mstatus(0)
    }
}
impl core::fmt::Debug for Mstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstatus")
            .field("state", &self.state())
            .field("between", &self.between())
            .field("nacked", &self.nacked())
            .field("ibitype", &self.ibitype())
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .field("ibiaddr", &self.ibiaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstatus {{ state: {:?}, between: {=bool:?}, nacked: {=bool:?}, ibitype: {:?}, slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {=bool:?}, txnotfull: {=bool:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?}, ibiaddr: {=u8:?} }}",
            self.state(),
            self.between(),
            self.nacked(),
            self.ibitype(),
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster(),
            self.ibiaddr()
        )
    }
}
#[doc = "Controller Write Data Byte."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatab(pub u32);
impl Mwdatab {
    #[doc = "Data Byte."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "End of Message."]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message."]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "End of Message ALSO."]
    #[must_use]
    #[inline(always)]
    pub const fn end_also(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message ALSO."]
    #[inline(always)]
    pub const fn set_end_also(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mwdatab {
    #[inline(always)]
    fn default() -> Mwdatab {
        Mwdatab(0)
    }
}
impl core::fmt::Debug for Mwdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatab")
            .field("value", &self.value())
            .field("end", &self.end())
            .field("end_also", &self.end_also())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mwdatab {{ value: {=u8:?}, end: {=bool:?}, end_also: {=bool:?} }}",
            self.value(),
            self.end(),
            self.end_also()
        )
    }
}
#[doc = "Controller Write Byte Data 1 (to Bus)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatab1(pub u32);
impl Mwdatab1 {
    #[doc = "Value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mwdatab1 {
    #[inline(always)]
    fn default() -> Mwdatab1 {
        Mwdatab1(0)
    }
}
impl core::fmt::Debug for Mwdatab1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatab1")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatab1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mwdatab1 {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Controller Write Data Byte End."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatabe(pub u32);
impl Mwdatabe {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mwdatabe {
    #[inline(always)]
    fn default() -> Mwdatabe {
        Mwdatabe(0)
    }
}
impl core::fmt::Debug for Mwdatabe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatabe")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatabe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mwdatabe {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Controller Write Data Halfword."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatah(pub u32);
impl Mwdatah {
    #[doc = "Data Byte 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "End of Message."]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message."]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mwdatah {
    #[inline(always)]
    fn default() -> Mwdatah {
        Mwdatah(0)
    }
}
impl core::fmt::Debug for Mwdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatah")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("end", &self.end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mwdatah {{ data0: {=u8:?}, data1: {=u8:?}, end: {=bool:?} }}",
            self.data0(),
            self.data1(),
            self.end()
        )
    }
}
#[doc = "Controller Write Data Halfword End."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatahe(pub u32);
impl Mwdatahe {
    #[doc = "Data Byte 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Mwdatahe {
    #[inline(always)]
    fn default() -> Mwdatahe {
        Mwdatahe(0)
    }
}
impl core::fmt::Debug for Mwdatahe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatahe")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatahe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mwdatahe {{ data0: {=u8:?}, data1: {=u8:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "Write Word Data (to Bus)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdataw(pub u32);
impl Mwdataw {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mwdataw {
    #[inline(always)]
    fn default() -> Mwdataw {
        Mwdataw(0)
    }
}
impl core::fmt::Debug for Mwdataw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdataw")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdataw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mwdataw {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Controller Write Message in DDR mode: First Control Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrControl(pub u32);
impl MwmsgDdrControl {
    #[doc = "Address Command."]
    #[must_use]
    #[inline(always)]
    pub const fn addrcmd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Address Command."]
    #[inline(always)]
    pub const fn set_addrcmd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MwmsgDdrControl {
    #[inline(always)]
    fn default() -> MwmsgDdrControl {
        MwmsgDdrControl(0)
    }
}
impl core::fmt::Debug for MwmsgDdrControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgDdrControl")
            .field("addrcmd", &self.addrcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgDdrControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MwmsgDdrControl {{ addrcmd: {=u16:?} }}", self.addrcmd())
    }
}
#[doc = "Controller Write Message in DDR Mode Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrControl2(pub u32);
impl MwmsgDdrControl2 {
    #[doc = "Length of Message."]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Length of Message."]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "End of Message."]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message."]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for MwmsgDdrControl2 {
    #[inline(always)]
    fn default() -> MwmsgDdrControl2 {
        MwmsgDdrControl2(0)
    }
}
impl core::fmt::Debug for MwmsgDdrControl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgDdrControl2")
            .field("len", &self.len())
            .field("end", &self.end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgDdrControl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MwmsgDdrControl2 {{ len: {=u16:?}, end: {=bool:?} }}",
            self.len(),
            self.end()
        )
    }
}
#[doc = "Controller Write Message Data in DDR mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrData(pub u32);
impl MwmsgDdrData {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data16b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data16b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MwmsgDdrData {
    #[inline(always)]
    fn default() -> MwmsgDdrData {
        MwmsgDdrData(0)
    }
}
impl core::fmt::Debug for MwmsgDdrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgDdrData")
            .field("data16b", &self.data16b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgDdrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MwmsgDdrData {{ data16b: {=u16:?} }}", self.data16b())
    }
}
#[doc = "Controller Write Message Control in SDR mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgSdrControl(pub u32);
impl MwmsgSdrControl {
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> MwmsgSdrControlDir {
        let val = (self.0 >> 0usize) & 0x01;
        MwmsgSdrControlDir::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: MwmsgSdrControlDir) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Address."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "End of SDR Message."]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End of SDR Message."]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "I2C."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c(&self) -> I2c {
        let val = (self.0 >> 10usize) & 0x01;
        I2c::from_bits(val as u8)
    }
    #[doc = "I2C."]
    #[inline(always)]
    pub const fn set_i2c(&mut self, val: I2c) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Length."]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Length."]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for MwmsgSdrControl {
    #[inline(always)]
    fn default() -> MwmsgSdrControl {
        MwmsgSdrControl(0)
    }
}
impl core::fmt::Debug for MwmsgSdrControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgSdrControl")
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("end", &self.end())
            .field("i2c", &self.i2c())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgSdrControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MwmsgSdrControl {{ dir: {:?}, addr: {=u8:?}, end: {=bool:?}, i2c: {:?}, len: {=u8:?} }}",
            self.dir(),
            self.addr(),
            self.end(),
            self.i2c(),
            self.len()
        )
    }
}
#[doc = "Controller Write Message Data in SDR mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgSdrData(pub u32);
impl MwmsgSdrData {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data16b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data16b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MwmsgSdrData {
    #[inline(always)]
    fn default() -> MwmsgSdrData {
        MwmsgSdrData(0)
    }
}
impl core::fmt::Debug for MwmsgSdrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgSdrData")
            .field("data16b", &self.data16b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgSdrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MwmsgSdrData {{ data16b: {=u16:?} }}", self.data16b())
    }
}
#[doc = "Target Capabilities."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapabilities(pub u32);
impl Scapabilities {
    #[doc = "ID 48b Handler."]
    #[must_use]
    #[inline(always)]
    pub const fn idena(&self) -> Idena {
        let val = (self.0 >> 0usize) & 0x03;
        Idena::from_bits(val as u8)
    }
    #[doc = "ID 48b Handler."]
    #[inline(always)]
    pub const fn set_idena(&mut self, val: Idena) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ID Register."]
    #[must_use]
    #[inline(always)]
    pub const fn idreg(&self) -> Idreg {
        let val = (self.0 >> 2usize) & 0x0f;
        Idreg::from_bits(val as u8)
    }
    #[doc = "ID Register."]
    #[inline(always)]
    pub const fn set_idreg(&mut self, val: Idreg) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "High Data Rate Support."]
    #[must_use]
    #[inline(always)]
    pub const fn hdrsupp(&self) -> Hdrsupp {
        let val = (self.0 >> 6usize) & 0x03;
        Hdrsupp::from_bits(val as u8)
    }
    #[doc = "High Data Rate Support."]
    #[inline(always)]
    pub const fn set_hdrsupp(&mut self, val: Hdrsupp) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> Master {
        let val = (self.0 >> 9usize) & 0x01;
        Master::from_bits(val as u8)
    }
    #[doc = "Controller."]
    #[inline(always)]
    pub const fn set_master(&mut self, val: Master) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Static Address."]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> Saddr {
        let val = (self.0 >> 10usize) & 0x03;
        Saddr::from_bits(val as u8)
    }
    #[doc = "Static Address."]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: Saddr) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Common Command Codes Handling."]
    #[must_use]
    #[inline(always)]
    pub const fn ccchandle(&self) -> Ccchandle {
        let val = (self.0 >> 12usize) & 0x0f;
        Ccchandle::from_bits(val as u8)
    }
    #[doc = "Common Command Codes Handling."]
    #[inline(always)]
    pub const fn set_ccchandle(&mut self, val: Ccchandle) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "In-Band Interrupts, Controller Requests, Hot-Join Events."]
    #[must_use]
    #[inline(always)]
    pub const fn ibi_mr_hj(&self) -> IbiMrHj {
        let val = (self.0 >> 16usize) & 0x1f;
        IbiMrHj::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupts, Controller Requests, Hot-Join Events."]
    #[inline(always)]
    pub const fn set_ibi_mr_hj(&mut self, val: IbiMrHj) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Time Control."]
    #[must_use]
    #[inline(always)]
    pub const fn timectrl(&self) -> ScapabilitiesTimectrl {
        let val = (self.0 >> 21usize) & 0x01;
        ScapabilitiesTimectrl::from_bits(val as u8)
    }
    #[doc = "Time Control."]
    #[inline(always)]
    pub const fn set_timectrl(&mut self, val: ScapabilitiesTimectrl) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "External FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn extfifo(&self) -> Extfifo {
        let val = (self.0 >> 23usize) & 0x07;
        Extfifo::from_bits(val as u8)
    }
    #[doc = "External FIFO."]
    #[inline(always)]
    pub const fn set_extfifo(&mut self, val: Extfifo) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "FIFO Transmit."]
    #[must_use]
    #[inline(always)]
    pub const fn fifotx(&self) -> Fifotx {
        let val = (self.0 >> 26usize) & 0x03;
        Fifotx::from_bits(val as u8)
    }
    #[doc = "FIFO Transmit."]
    #[inline(always)]
    pub const fn set_fifotx(&mut self, val: Fifotx) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Receive."]
    #[must_use]
    #[inline(always)]
    pub const fn fiforx(&self) -> Fiforx {
        let val = (self.0 >> 28usize) & 0x03;
        Fiforx::from_bits(val as u8)
    }
    #[doc = "FIFO Receive."]
    #[inline(always)]
    pub const fn set_fiforx(&mut self, val: Fiforx) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> Int {
        let val = (self.0 >> 30usize) & 0x01;
        Int::from_bits(val as u8)
    }
    #[doc = "Interrupts."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: Int) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Direct Memory Access."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> Dma {
        let val = (self.0 >> 31usize) & 0x01;
        Dma::from_bits(val as u8)
    }
    #[doc = "Direct Memory Access."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: Dma) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Scapabilities {
    #[inline(always)]
    fn default() -> Scapabilities {
        Scapabilities(0)
    }
}
impl core::fmt::Debug for Scapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scapabilities")
            .field("idena", &self.idena())
            .field("idreg", &self.idreg())
            .field("hdrsupp", &self.hdrsupp())
            .field("master", &self.master())
            .field("saddr", &self.saddr())
            .field("ccchandle", &self.ccchandle())
            .field("ibi_mr_hj", &self.ibi_mr_hj())
            .field("timectrl", &self.timectrl())
            .field("extfifo", &self.extfifo())
            .field("fifotx", &self.fifotx())
            .field("fiforx", &self.fiforx())
            .field("int", &self.int())
            .field("dma", &self.dma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scapabilities {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scapabilities {{ idena: {:?}, idreg: {:?}, hdrsupp: {:?}, master: {:?}, saddr: {:?}, ccchandle: {:?}, ibi_mr_hj: {:?}, timectrl: {:?}, extfifo: {:?}, fifotx: {:?}, fiforx: {:?}, int: {:?}, dma: {:?} }}",
            self.idena(),
            self.idreg(),
            self.hdrsupp(),
            self.master(),
            self.saddr(),
            self.ccchandle(),
            self.ibi_mr_hj(),
            self.timectrl(),
            self.extfifo(),
            self.fifotx(),
            self.fiforx(),
            self.int(),
            self.dma()
        )
    }
}
#[doc = "Target Capabilities 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapabilities2(pub u32);
impl Scapabilities2 {
    #[doc = "Map Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mapcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Map Count."]
    #[inline(always)]
    pub const fn set_mapcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "I2C 10-bit Address."]
    #[must_use]
    #[inline(always)]
    pub const fn i2c10b(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 10-bit Address."]
    #[inline(always)]
    pub const fn set_i2c10b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C Device ID."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cdevid(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Device ID."]
    #[inline(always)]
    pub const fn set_i2cdevid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "In-Band Interrupt EXTDATA."]
    #[must_use]
    #[inline(always)]
    pub const fn ibiext(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt EXTDATA."]
    #[inline(always)]
    pub const fn set_ibiext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "In-Band Interrupt Extended Register."]
    #[must_use]
    #[inline(always)]
    pub const fn ibixreg(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt Extended Register."]
    #[inline(always)]
    pub const fn set_ibixreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Target Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn slvrst(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Target Reset."]
    #[inline(always)]
    pub const fn set_slvrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Group."]
    #[must_use]
    #[inline(always)]
    pub const fn group(&self) -> Group {
        let val = (self.0 >> 18usize) & 0x03;
        Group::from_bits(val as u8)
    }
    #[doc = "Group."]
    #[inline(always)]
    pub const fn set_group(&mut self, val: Group) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "SETAASA."]
    #[must_use]
    #[inline(always)]
    pub const fn aasa(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SETAASA."]
    #[inline(always)]
    pub const fn set_aasa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Target-Target(s)-Tunnel Subscriber Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn sstsub(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Target-Target(s)-Tunnel Subscriber Capable."]
    #[inline(always)]
    pub const fn set_sstsub(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Target-Target(s)-Tunnel Write Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn sstwr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Target-Target(s)-Tunnel Write Capable."]
    #[inline(always)]
    pub const fn set_sstwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Scapabilities2 {
    #[inline(always)]
    fn default() -> Scapabilities2 {
        Scapabilities2(0)
    }
}
impl core::fmt::Debug for Scapabilities2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scapabilities2")
            .field("mapcnt", &self.mapcnt())
            .field("i2c10b", &self.i2c10b())
            .field("i2cdevid", &self.i2cdevid())
            .field("ibiext", &self.ibiext())
            .field("ibixreg", &self.ibixreg())
            .field("slvrst", &self.slvrst())
            .field("group", &self.group())
            .field("aasa", &self.aasa())
            .field("sstsub", &self.sstsub())
            .field("sstwr", &self.sstwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scapabilities2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scapabilities2 {{ mapcnt: {=u8:?}, i2c10b: {=bool:?}, i2cdevid: {=bool:?}, ibiext: {=bool:?}, ibixreg: {=bool:?}, slvrst: {=bool:?}, group: {:?}, aasa: {=bool:?}, sstsub: {=bool:?}, sstwr: {=bool:?} }}",
            self.mapcnt(),
            self.i2c10b(),
            self.i2cdevid(),
            self.ibiext(),
            self.ibixreg(),
            self.slvrst(),
            self.group(),
            self.aasa(),
            self.sstsub(),
            self.sstwr()
        )
    }
}
#[doc = "Target Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sconfig(pub u32);
impl Sconfig {
    #[doc = "Target Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn slvena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Target Enable."]
    #[inline(always)]
    pub const fn set_slvena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Not Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledge."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Match Start or Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn matchss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Match Start or Stop."]
    #[inline(always)]
    pub const fn set_matchss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ignore TE0 or TE1 Errors."]
    #[must_use]
    #[inline(always)]
    pub const fn s0ignore(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore TE0 or TE1 Errors."]
    #[inline(always)]
    pub const fn set_s0ignore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Double Data Rate OK."]
    #[must_use]
    #[inline(always)]
    pub const fn ddrok(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Double Data Rate OK."]
    #[inline(always)]
    pub const fn set_ddrok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Offline."]
    #[must_use]
    #[inline(always)]
    pub const fn offline(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Offline."]
    #[inline(always)]
    pub const fn set_offline(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Available Match."]
    #[must_use]
    #[inline(always)]
    pub const fn bamatch(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Bus Available Match."]
    #[inline(always)]
    pub const fn set_bamatch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Static Address."]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Static Address."]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Sconfig {
    #[inline(always)]
    fn default() -> Sconfig {
        Sconfig(0)
    }
}
impl core::fmt::Debug for Sconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sconfig")
            .field("slvena", &self.slvena())
            .field("nack", &self.nack())
            .field("matchss", &self.matchss())
            .field("s0ignore", &self.s0ignore())
            .field("ddrok", &self.ddrok())
            .field("offline", &self.offline())
            .field("bamatch", &self.bamatch())
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sconfig {{ slvena: {=bool:?}, nack: {=bool:?}, matchss: {=bool:?}, s0ignore: {=bool:?}, ddrok: {=bool:?}, offline: {=bool:?}, bamatch: {=u8:?}, saddr: {=u8:?} }}",
            self.slvena(),
            self.nack(),
            self.matchss(),
            self.s0ignore(),
            self.ddrok(),
            self.offline(),
            self.bamatch(),
            self.saddr()
        )
    }
}
#[doc = "Target Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl(pub u32);
impl Sctrl {
    #[doc = "Event."]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> SctrlEvent {
        let val = (self.0 >> 0usize) & 0x03;
        SctrlEvent::from_bits(val as u8)
    }
    #[doc = "Event."]
    #[inline(always)]
    pub const fn set_event(&mut self, val: SctrlEvent) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Extended Data."]
    #[must_use]
    #[inline(always)]
    pub const fn extdata(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data."]
    #[inline(always)]
    pub const fn set_extdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Map Index."]
    #[must_use]
    #[inline(always)]
    pub const fn mapidx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Map Index."]
    #[inline(always)]
    pub const fn set_mapidx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "In-Band Interrupt Data."]
    #[must_use]
    #[inline(always)]
    pub const fn ibidata(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "In-Band Interrupt Data."]
    #[inline(always)]
    pub const fn set_ibidata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pending Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pendint(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Pending Interrupt."]
    #[inline(always)]
    pub const fn set_pendint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Activity State of Target."]
    #[must_use]
    #[inline(always)]
    pub const fn actstate(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Activity State of Target."]
    #[inline(always)]
    pub const fn set_actstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Vendor Information."]
    #[must_use]
    #[inline(always)]
    pub const fn vendinfo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Vendor Information."]
    #[inline(always)]
    pub const fn set_vendinfo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Sctrl {
    #[inline(always)]
    fn default() -> Sctrl {
        Sctrl(0)
    }
}
impl core::fmt::Debug for Sctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl")
            .field("event", &self.event())
            .field("extdata", &self.extdata())
            .field("mapidx", &self.mapidx())
            .field("ibidata", &self.ibidata())
            .field("pendint", &self.pendint())
            .field("actstate", &self.actstate())
            .field("vendinfo", &self.vendinfo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctrl {{ event: {:?}, extdata: {=bool:?}, mapidx: {=bool:?}, ibidata: {=u8:?}, pendint: {=u8:?}, actstate: {=u8:?}, vendinfo: {=u8:?} }}",
            self.event(),
            self.extdata(),
            self.mapidx(),
            self.ibidata(),
            self.pendint(),
            self.actstate(),
            self.vendinfo()
        )
    }
}
#[doc = "Target Data Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatactrl(pub u32);
impl Sdatactrl {
    #[doc = "Flush To-Bus Buffer or FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn flushtb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush To-Bus Buffer or FIFO."]
    #[inline(always)]
    pub const fn set_flushtb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flush From-Bus Buffer or FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn flushfb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flush From-Bus Buffer or FIFO."]
    #[inline(always)]
    pub const fn set_flushfb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Unlock."]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock."]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit Trigger Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txtrig(&self) -> SdatactrlTxtrig {
        let val = (self.0 >> 4usize) & 0x03;
        SdatactrlTxtrig::from_bits(val as u8)
    }
    #[doc = "Transmit Trigger Level."]
    #[inline(always)]
    pub const fn set_txtrig(&mut self, val: SdatactrlTxtrig) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Receive Trigger Level."]
    #[must_use]
    #[inline(always)]
    pub const fn rxtrig(&self) -> SdatactrlRxtrig {
        let val = (self.0 >> 6usize) & 0x03;
        SdatactrlRxtrig::from_bits(val as u8)
    }
    #[doc = "Receive Trigger Level."]
    #[inline(always)]
    pub const fn set_rxtrig(&mut self, val: SdatactrlRxtrig) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Count of Entries in Transmit."]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of Entries in Transmit."]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Count of Entries in Receive."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of Entries in Receive."]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Transmit is Full."]
    #[must_use]
    #[inline(always)]
    pub const fn txfull(&self) -> SdatactrlTxfull {
        let val = (self.0 >> 30usize) & 0x01;
        SdatactrlTxfull::from_bits(val as u8)
    }
    #[doc = "Transmit is Full."]
    #[inline(always)]
    pub const fn set_txfull(&mut self, val: SdatactrlTxfull) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive is Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> SdatactrlRxempty {
        let val = (self.0 >> 31usize) & 0x01;
        SdatactrlRxempty::from_bits(val as u8)
    }
    #[doc = "Receive is Empty."]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: SdatactrlRxempty) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdatactrl {
    #[inline(always)]
    fn default() -> Sdatactrl {
        Sdatactrl(0)
    }
}
impl core::fmt::Debug for Sdatactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdatactrl")
            .field("flushtb", &self.flushtb())
            .field("flushfb", &self.flushfb())
            .field("unlock", &self.unlock())
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdatactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdatactrl {{ flushtb: {=bool:?}, flushfb: {=bool:?}, unlock: {=bool:?}, txtrig: {:?}, rxtrig: {:?}, txcount: {=u8:?}, rxcount: {=u8:?}, txfull: {:?}, rxempty: {:?} }}",
            self.flushtb(),
            self.flushfb(),
            self.unlock(),
            self.txtrig(),
            self.rxtrig(),
            self.txcount(),
            self.rxcount(),
            self.txfull(),
            self.rxempty()
        )
    }
}
#[doc = "Target DMA Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmactrl(pub u32);
impl Sdmactrl {
    #[doc = "DMA Read (From-Bus) Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn dmafb(&self) -> SdmactrlDmafb {
        let val = (self.0 >> 0usize) & 0x03;
        SdmactrlDmafb::from_bits(val as u8)
    }
    #[doc = "DMA Read (From-Bus) Trigger."]
    #[inline(always)]
    pub const fn set_dmafb(&mut self, val: SdmactrlDmafb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA Write (To-Bus) Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatb(&self) -> SdmactrlDmatb {
        let val = (self.0 >> 2usize) & 0x03;
        SdmactrlDmatb::from_bits(val as u8)
    }
    #[doc = "DMA Write (To-Bus) Trigger."]
    #[inline(always)]
    pub const fn set_dmatb(&mut self, val: SdmactrlDmatb) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Width of DMA Operations."]
    #[must_use]
    #[inline(always)]
    pub const fn dmawidth(&self) -> SdmactrlDmawidth {
        let val = (self.0 >> 4usize) & 0x03;
        SdmactrlDmawidth::from_bits(val as u8)
    }
    #[doc = "Width of DMA Operations."]
    #[inline(always)]
    pub const fn set_dmawidth(&mut self, val: SdmactrlDmawidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sdmactrl {
    #[inline(always)]
    fn default() -> Sdmactrl {
        Sdmactrl(0)
    }
}
impl core::fmt::Debug for Sdmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdmactrl")
            .field("dmafb", &self.dmafb())
            .field("dmatb", &self.dmatb())
            .field("dmawidth", &self.dmawidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdmactrl {{ dmafb: {:?}, dmatb: {:?}, dmawidth: {:?} }}",
            self.dmafb(),
            self.dmatb(),
            self.dmawidth()
        )
    }
}
#[doc = "Target Errors and Warnings."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serrwarn(pub u32);
impl Serrwarn {
    #[doc = "Overrun Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn orun(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Flag."]
    #[inline(always)]
    pub const fn set_orun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Underrun Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn urun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun Error Flag."]
    #[inline(always)]
    pub const fn set_urun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn urunnack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) Error Flag."]
    #[inline(always)]
    pub const fn set_urunnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Terminated Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn term(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Terminated Error Flag."]
    #[inline(always)]
    pub const fn set_term(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalid Start Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn invstart(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Start Error Flag."]
    #[inline(always)]
    pub const fn set_invstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SDR Parity Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn spar(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SDR Parity Error Flag."]
    #[inline(always)]
    pub const fn set_spar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HDR Parity Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hpar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HDR Parity Error Flag."]
    #[inline(always)]
    pub const fn set_hpar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HDR-DDR CRC Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hcrc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HDR-DDR CRC Error Flag."]
    #[inline(always)]
    pub const fn set_hcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TE0 or TE1 Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn s0s1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TE0 or TE1 Error Flag."]
    #[inline(always)]
    pub const fn set_s0s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "HDR-BT Invalid Request Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hinvreq(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "HDR-BT Invalid Request Flag."]
    #[inline(always)]
    pub const fn set_hinvreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Over-Read Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn oread(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Over-Read Error Flag."]
    #[inline(always)]
    pub const fn set_oread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Over-Write Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn owrite(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Over-Write Error Flag."]
    #[inline(always)]
    pub const fn set_owrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Serrwarn {
    #[inline(always)]
    fn default() -> Serrwarn {
        Serrwarn(0)
    }
}
impl core::fmt::Debug for Serrwarn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serrwarn")
            .field("orun", &self.orun())
            .field("urun", &self.urun())
            .field("urunnack", &self.urunnack())
            .field("term", &self.term())
            .field("invstart", &self.invstart())
            .field("spar", &self.spar())
            .field("hpar", &self.hpar())
            .field("hcrc", &self.hcrc())
            .field("s0s1", &self.s0s1())
            .field("hinvreq", &self.hinvreq())
            .field("oread", &self.oread())
            .field("owrite", &self.owrite())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serrwarn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Serrwarn {{ orun: {=bool:?}, urun: {=bool:?}, urunnack: {=bool:?}, term: {=bool:?}, invstart: {=bool:?}, spar: {=bool:?}, hpar: {=bool:?}, hcrc: {=bool:?}, s0s1: {=bool:?}, hinvreq: {=bool:?}, oread: {=bool:?}, owrite: {=bool:?} }}",
            self.orun(),
            self.urun(),
            self.urunnack(),
            self.term(),
            self.invstart(),
            self.spar(),
            self.hpar(),
            self.hcrc(),
            self.s0s1(),
            self.hinvreq(),
            self.oread(),
            self.owrite()
        )
    }
}
#[doc = "Target HDR-BT Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shdrbtcfg(pub u32);
impl Shdrbtcfg {
    #[doc = "CRC32 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn crc32(&self) -> ShdrbtcfgCrc32 {
        let val = (self.0 >> 2usize) & 0x01;
        ShdrbtcfgCrc32::from_bits(val as u8)
    }
    #[doc = "CRC32 Select."]
    #[inline(always)]
    pub const fn set_crc32(&mut self, val: ShdrbtcfgCrc32) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Maximum Data."]
    #[must_use]
    #[inline(always)]
    pub const fn wdatamax(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Data."]
    #[inline(always)]
    pub const fn set_wdatamax(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "Read Data Length."]
    #[must_use]
    #[inline(always)]
    pub const fn datalen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Data Length."]
    #[inline(always)]
    pub const fn set_datalen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Shdrbtcfg {
    #[inline(always)]
    fn default() -> Shdrbtcfg {
        Shdrbtcfg(0)
    }
}
impl core::fmt::Debug for Shdrbtcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shdrbtcfg")
            .field("crc32", &self.crc32())
            .field("wdatamax", &self.wdatamax())
            .field("datalen", &self.datalen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shdrbtcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shdrbtcfg {{ crc32: {:?}, wdatamax: {=u16:?}, datalen: {=u16:?} }}",
            self.crc32(),
            self.wdatamax(),
            self.datalen()
        )
    }
}
#[doc = "Target HDR-Last."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shdrbtlast(pub u32);
impl Shdrbtlast {
    #[doc = "Data Length."]
    #[must_use]
    #[inline(always)]
    pub const fn datalen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Length."]
    #[inline(always)]
    pub const fn set_datalen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Shdrbtlast {
    #[inline(always)]
    fn default() -> Shdrbtlast {
        Shdrbtlast(0)
    }
}
impl core::fmt::Debug for Shdrbtlast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shdrbtlast")
            .field("datalen", &self.datalen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shdrbtlast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shdrbtlast {{ datalen: {=u16:?} }}", self.datalen())
    }
}
#[doc = "Target Module ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sid(pub u32);
impl Sid {
    #[doc = "ID."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ID."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sid {
    #[inline(always)]
    fn default() -> Sid {
        Sid(0)
    }
}
impl core::fmt::Debug for Sid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sid").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sid {{ id: {=u32:?} }}", self.id())
    }
}
#[doc = "Target ID Extension."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidext(pub u32);
impl Sidext {
    #[doc = "Device Characteristic Register."]
    #[must_use]
    #[inline(always)]
    pub const fn dcr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Device Characteristic Register."]
    #[inline(always)]
    pub const fn set_dcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Bus Characteristics Register."]
    #[must_use]
    #[inline(always)]
    pub const fn bcr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bus Characteristics Register."]
    #[inline(always)]
    pub const fn set_bcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Sidext {
    #[inline(always)]
    fn default() -> Sidext {
        Sidext(0)
    }
}
impl core::fmt::Debug for Sidext {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sidext")
            .field("dcr", &self.dcr())
            .field("bcr", &self.bcr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sidext {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sidext {{ dcr: {=u8:?}, bcr: {=u8:?} }}",
            self.dcr(),
            self.bcr()
        )
    }
}
#[doc = "Target ID Part Number."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidpartno(pub u32);
impl Sidpartno {
    #[doc = "Part Number."]
    #[must_use]
    #[inline(always)]
    pub const fn partno(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Part Number."]
    #[inline(always)]
    pub const fn set_partno(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sidpartno {
    #[inline(always)]
    fn default() -> Sidpartno {
        Sidpartno(0)
    }
}
impl core::fmt::Debug for Sidpartno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sidpartno")
            .field("partno", &self.partno())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sidpartno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sidpartno {{ partno: {=u32:?} }}", self.partno())
    }
}
#[doc = "Target Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintclr(pub u32);
impl Sintclr {
    #[doc = "START Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Matched Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Matched Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "STOP Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXSEND Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CCC Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CCC Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DDRMATCHED Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CHANDLED Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EVENT Interrupt Enable Clear Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT Interrupt Enable Clear Flag."]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintclr {
    #[inline(always)]
    fn default() -> Sintclr {
        Sintclr(0)
    }
}
impl core::fmt::Debug for Sintclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sintclr")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sintclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sintclr {{ start: {=bool:?}, matched: {=bool:?}, stop: {=bool:?}, rxpend: {=bool:?}, txsend: {=bool:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, ddrmatched: {=bool:?}, chandled: {=bool:?}, event: {=bool:?} }}",
            self.start(),
            self.matched(),
            self.stop(),
            self.rxpend(),
            self.txsend(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.ddrmatched(),
            self.chandled(),
            self.event()
        )
    }
}
#[doc = "Target Interrupt Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintmasked(pub u32);
impl Sintmasked {
    #[doc = "START Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Mask."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MATCHED Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MATCHED Interrupt Mask."]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "STOP Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Interrupt Mask."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXSEND Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND Interrupt Mask."]
    #[inline(always)]
    pub const fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG Interrupt Mask."]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CCC Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CCC Interrupt Mask."]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERRWARN Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Mask."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DDRMATCHED Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED Interrupt Mask."]
    #[inline(always)]
    pub const fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CHANDLED Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED Interrupt Mask."]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EVENT Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT Interrupt Mask."]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintmasked {
    #[inline(always)]
    fn default() -> Sintmasked {
        Sintmasked(0)
    }
}
impl core::fmt::Debug for Sintmasked {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sintmasked")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sintmasked {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sintmasked {{ start: {=bool:?}, matched: {=bool:?}, stop: {=bool:?}, rxpend: {=bool:?}, txsend: {=bool:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, ddrmatched: {=bool:?}, chandled: {=bool:?}, event: {=bool:?} }}",
            self.start(),
            self.matched(),
            self.stop(),
            self.rxpend(),
            self.txsend(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.ddrmatched(),
            self.chandled(),
            self.event()
        )
    }
}
#[doc = "Target Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintset(pub u32);
impl Sintset {
    #[doc = "Start Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Start Interrupt Enable."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Match Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Match Interrupt Enable."]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Stop Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Interrupt Enable."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Dynamic Address Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic Address Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error or Warning Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or Warning Interrupt Enable."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Double Data Rate Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Double Data Rate Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable."]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintset {
    #[inline(always)]
    fn default() -> Sintset {
        Sintset(0)
    }
}
impl core::fmt::Debug for Sintset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sintset")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sintset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sintset {{ start: {=bool:?}, matched: {=bool:?}, stop: {=bool:?}, rxpend: {=bool:?}, txsend: {=bool:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, ddrmatched: {=bool:?}, chandled: {=bool:?}, event: {=bool:?} }}",
            self.start(),
            self.matched(),
            self.stop(),
            self.rxpend(),
            self.txsend(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.ddrmatched(),
            self.chandled(),
            self.event()
        )
    }
}
#[doc = "Map Feature Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smapctrl0(pub u32);
impl Smapctrl0 {
    #[doc = "Enable Primary Dynamic Address."]
    #[must_use]
    #[inline(always)]
    pub const fn ena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Primary Dynamic Address."]
    #[inline(always)]
    pub const fn set_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address."]
    #[must_use]
    #[inline(always)]
    pub const fn da(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address."]
    #[inline(always)]
    pub const fn set_da(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Cause."]
    #[must_use]
    #[inline(always)]
    pub const fn cause(&self) -> Cause {
        let val = (self.0 >> 8usize) & 0x07;
        Cause::from_bits(val as u8)
    }
    #[doc = "Cause."]
    #[inline(always)]
    pub const fn set_cause(&mut self, val: Cause) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Smapctrl0 {
    #[inline(always)]
    fn default() -> Smapctrl0 {
        Smapctrl0(0)
    }
}
impl core::fmt::Debug for Smapctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smapctrl0")
            .field("ena", &self.ena())
            .field("da", &self.da())
            .field("cause", &self.cause())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smapctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smapctrl0 {{ ena: {=bool:?}, da: {=u8:?}, cause: {:?} }}",
            self.ena(),
            self.da(),
            self.cause()
        )
    }
}
#[doc = "Map Feature Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smapctrl1(pub u32);
impl Smapctrl1 {
    #[doc = "Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn set_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Address."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "MAP Static Address."]
    #[must_use]
    #[inline(always)]
    pub const fn mapsa(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "MAP Static Address."]
    #[inline(always)]
    pub const fn set_mapsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Static Address 10-Bit Extension."]
    #[must_use]
    #[inline(always)]
    pub const fn sa10b(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Static Address 10-Bit Extension."]
    #[inline(always)]
    pub const fn set_sa10b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Not Acknowledged."]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledged."]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Smapctrl1 {
    #[inline(always)]
    fn default() -> Smapctrl1 {
        Smapctrl1(0)
    }
}
impl core::fmt::Debug for Smapctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smapctrl1")
            .field("ena", &self.ena())
            .field("addr", &self.addr())
            .field("mapsa", &self.mapsa())
            .field("sa10b", &self.sa10b())
            .field("nack", &self.nack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smapctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smapctrl1 {{ ena: {=bool:?}, addr: {=u8:?}, mapsa: {=bool:?}, sa10b: {=u8:?}, nack: {=bool:?} }}",
            self.ena(),
            self.addr(),
            self.mapsa(),
            self.sa10b(),
            self.nack()
        )
    }
}
#[doc = "Target Maximum Limits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smaxlimits(pub u32);
impl Smaxlimits {
    #[doc = "Maximum Read Length."]
    #[must_use]
    #[inline(always)]
    pub const fn maxrd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Read Length."]
    #[inline(always)]
    pub const fn set_maxrd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Maximum Write Length."]
    #[must_use]
    #[inline(always)]
    pub const fn maxwr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Write Length."]
    #[inline(always)]
    pub const fn set_maxwr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Smaxlimits {
    #[inline(always)]
    fn default() -> Smaxlimits {
        Smaxlimits(0)
    }
}
impl core::fmt::Debug for Smaxlimits {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smaxlimits")
            .field("maxrd", &self.maxrd())
            .field("maxwr", &self.maxwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smaxlimits {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smaxlimits {{ maxrd: {=u16:?}, maxwr: {=u16:?} }}",
            self.maxrd(),
            self.maxwr()
        )
    }
}
#[doc = "Target Message Map Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smsgmapaddr(pub u32);
impl Smsgmapaddr {
    #[doc = "Matched Address Index."]
    #[must_use]
    #[inline(always)]
    pub const fn maplast(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Address Index."]
    #[inline(always)]
    pub const fn set_maplast(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Last Static Address Matched."]
    #[must_use]
    #[inline(always)]
    pub const fn laststatic(&self) -> Laststatic {
        let val = (self.0 >> 4usize) & 0x01;
        Laststatic::from_bits(val as u8)
    }
    #[doc = "Last Static Address Matched."]
    #[inline(always)]
    pub const fn set_laststatic(&mut self, val: Laststatic) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Matched Previous Address Index 1."]
    #[must_use]
    #[inline(always)]
    pub const fn maplastm1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Previous Address Index 1."]
    #[inline(always)]
    pub const fn set_maplastm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Matched Previous Index 2."]
    #[must_use]
    #[inline(always)]
    pub const fn maplastm2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Previous Index 2."]
    #[inline(always)]
    pub const fn set_maplastm2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Smsgmapaddr {
    #[inline(always)]
    fn default() -> Smsgmapaddr {
        Smsgmapaddr(0)
    }
}
impl core::fmt::Debug for Smsgmapaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smsgmapaddr")
            .field("maplast", &self.maplast())
            .field("laststatic", &self.laststatic())
            .field("maplastm1", &self.maplastm1())
            .field("maplastm2", &self.maplastm2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smsgmapaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smsgmapaddr {{ maplast: {=u8:?}, laststatic: {:?}, maplastm1: {=u8:?}, maplastm2: {=u8:?} }}",
            self.maplast(),
            self.laststatic(),
            self.maplastm1(),
            self.maplastm2()
        )
    }
}
#[doc = "Target Read Data Byte."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdatab(pub u32);
impl Srdatab {
    #[doc = "Data 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Srdatab {
    #[inline(always)]
    fn default() -> Srdatab {
        Srdatab(0)
    }
}
impl core::fmt::Debug for Srdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdatab")
            .field("data0", &self.data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srdatab {{ data0: {=u8:?} }}", self.data0())
    }
}
#[doc = "Target Read Data Halfword."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdatah(pub u32);
impl Srdatah {
    #[doc = "Low Byte."]
    #[must_use]
    #[inline(always)]
    pub const fn lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte."]
    #[inline(always)]
    pub const fn set_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High Byte."]
    #[must_use]
    #[inline(always)]
    pub const fn msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High Byte."]
    #[inline(always)]
    pub const fn set_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Srdatah {
    #[inline(always)]
    fn default() -> Srdatah {
        Srdatah(0)
    }
}
impl core::fmt::Debug for Srdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdatah")
            .field("lsb", &self.lsb())
            .field("msb", &self.msb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srdatah {{ lsb: {=u8:?}, msb: {=u8:?} }}",
            self.lsb(),
            self.msb()
        )
    }
}
#[doc = "Target Read Word Data (from Bus)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdataw(pub u32);
impl Srdataw {
    #[doc = "Value."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Srdataw {
    #[inline(always)]
    fn default() -> Srdataw {
        Srdataw(0)
    }
}
impl core::fmt::Debug for Srdataw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdataw")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdataw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srdataw {{ value: {=u32:?} }}", self.value())
    }
}
#[doc = "Target Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sstatus(pub u32);
impl Sstatus {
    #[doc = "Status not Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn stnotstop(&self) -> Stnotstop {
        let val = (self.0 >> 0usize) & 0x01;
        Stnotstop::from_bits(val as u8)
    }
    #[doc = "Status not Stop."]
    #[inline(always)]
    pub const fn set_stnotstop(&mut self, val: Stnotstop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status Message."]
    #[must_use]
    #[inline(always)]
    pub const fn stmsg(&self) -> Stmsg {
        let val = (self.0 >> 1usize) & 0x01;
        Stmsg::from_bits(val as u8)
    }
    #[doc = "Status Message."]
    #[inline(always)]
    pub const fn set_stmsg(&mut self, val: Stmsg) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Status Common Command Code Handler."]
    #[must_use]
    #[inline(always)]
    pub const fn stccch(&self) -> Stccch {
        let val = (self.0 >> 2usize) & 0x01;
        Stccch::from_bits(val as u8)
    }
    #[doc = "Status Common Command Code Handler."]
    #[inline(always)]
    pub const fn set_stccch(&mut self, val: Stccch) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Status Request Read."]
    #[must_use]
    #[inline(always)]
    pub const fn streqrd(&self) -> Streqrd {
        let val = (self.0 >> 3usize) & 0x01;
        Streqrd::from_bits(val as u8)
    }
    #[doc = "Status Request Read."]
    #[inline(always)]
    pub const fn set_streqrd(&mut self, val: Streqrd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status Request Write."]
    #[must_use]
    #[inline(always)]
    pub const fn streqwr(&self) -> Streqwr {
        let val = (self.0 >> 4usize) & 0x01;
        Streqwr::from_bits(val as u8)
    }
    #[doc = "Status Request Write."]
    #[inline(always)]
    pub const fn set_streqwr(&mut self, val: Streqwr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Dynamic Address Assignment."]
    #[must_use]
    #[inline(always)]
    pub const fn stdaa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Dynamic Address Assignment."]
    #[inline(always)]
    pub const fn set_stdaa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Status High Data Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn sthdr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Status High Data Rate."]
    #[inline(always)]
    pub const fn set_sthdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Start Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> SstatusStart {
        let val = (self.0 >> 8usize) & 0x01;
        SstatusStart::from_bits(val as u8)
    }
    #[doc = "Start Flag."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: SstatusStart) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Matched Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Matched Flag."]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Stop Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Flag."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Received Message Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_pend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Received Message Pending."]
    #[inline(always)]
    pub const fn set_rx_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Buffer Not Full."]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> SstatusTxnotfull {
        let val = (self.0 >> 12usize) & 0x01;
        SstatusTxnotfull::from_bits(val as u8)
    }
    #[doc = "Transmit Buffer Not Full."]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: SstatusTxnotfull) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Dynamic Address Change Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic Address Change Flag."]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Common Command Code Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code Flag."]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error Warning."]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Warning."]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "High Data Rate Command Match Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hdrmatch(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate Command Match Flag."]
    #[inline(always)]
    pub const fn set_hdrmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Common Command Code Handled Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code Handled Flag."]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag."]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Event Details."]
    #[must_use]
    #[inline(always)]
    pub const fn evdet(&self) -> Evdet {
        let val = (self.0 >> 20usize) & 0x03;
        Evdet::from_bits(val as u8)
    }
    #[doc = "Event Details."]
    #[inline(always)]
    pub const fn set_evdet(&mut self, val: Evdet) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "In-Band Interrupts Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibidis(&self) -> Ibidis {
        let val = (self.0 >> 24usize) & 0x01;
        Ibidis::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupts Disable."]
    #[inline(always)]
    pub const fn set_ibidis(&mut self, val: Ibidis) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Controller Requests Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn mrdis(&self) -> Mrdis {
        let val = (self.0 >> 25usize) & 0x01;
        Mrdis::from_bits(val as u8)
    }
    #[doc = "Controller Requests Disable."]
    #[inline(always)]
    pub const fn set_mrdis(&mut self, val: Mrdis) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Hot-Join Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn hjdis(&self) -> Hjdis {
        let val = (self.0 >> 27usize) & 0x01;
        Hjdis::from_bits(val as u8)
    }
    #[doc = "Hot-Join Disabled."]
    #[inline(always)]
    pub const fn set_hjdis(&mut self, val: Hjdis) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Activity State from Common Command Codes (CCC)."]
    #[must_use]
    #[inline(always)]
    pub const fn actstate(&self) -> Actstate {
        let val = (self.0 >> 28usize) & 0x03;
        Actstate::from_bits(val as u8)
    }
    #[doc = "Activity State from Common Command Codes (CCC)."]
    #[inline(always)]
    pub const fn set_actstate(&mut self, val: Actstate) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Time Control."]
    #[must_use]
    #[inline(always)]
    pub const fn timectrl(&self) -> SstatusTimectrl {
        let val = (self.0 >> 30usize) & 0x03;
        SstatusTimectrl::from_bits(val as u8)
    }
    #[doc = "Time Control."]
    #[inline(always)]
    pub const fn set_timectrl(&mut self, val: SstatusTimectrl) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Sstatus {
    #[inline(always)]
    fn default() -> Sstatus {
        Sstatus(0)
    }
}
impl core::fmt::Debug for Sstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sstatus")
            .field("stnotstop", &self.stnotstop())
            .field("stmsg", &self.stmsg())
            .field("stccch", &self.stccch())
            .field("streqrd", &self.streqrd())
            .field("streqwr", &self.streqwr())
            .field("stdaa", &self.stdaa())
            .field("sthdr", &self.sthdr())
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rx_pend", &self.rx_pend())
            .field("txnotfull", &self.txnotfull())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("hdrmatch", &self.hdrmatch())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .field("evdet", &self.evdet())
            .field("ibidis", &self.ibidis())
            .field("mrdis", &self.mrdis())
            .field("hjdis", &self.hjdis())
            .field("actstate", &self.actstate())
            .field("timectrl", &self.timectrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sstatus {{ stnotstop: {:?}, stmsg: {:?}, stccch: {:?}, streqrd: {:?}, streqwr: {:?}, stdaa: {=bool:?}, sthdr: {=bool:?}, start: {:?}, matched: {=bool:?}, stop: {=bool:?}, rx_pend: {=bool:?}, txnotfull: {:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, hdrmatch: {=bool:?}, chandled: {=bool:?}, event: {=bool:?}, evdet: {:?}, ibidis: {:?}, mrdis: {:?}, hjdis: {:?}, actstate: {:?}, timectrl: {:?} }}",
            self.stnotstop(),
            self.stmsg(),
            self.stccch(),
            self.streqrd(),
            self.streqwr(),
            self.stdaa(),
            self.sthdr(),
            self.start(),
            self.matched(),
            self.stop(),
            self.rx_pend(),
            self.txnotfull(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.hdrmatch(),
            self.chandled(),
            self.event(),
            self.evdet(),
            self.ibidis(),
            self.mrdis(),
            self.hjdis(),
            self.actstate(),
            self.timectrl()
        )
    }
}
#[doc = "Target Time Control Clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcclock(pub u32);
impl Stcclock {
    #[doc = "Clock Accuracy."]
    #[must_use]
    #[inline(always)]
    pub const fn accuracy(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Accuracy."]
    #[inline(always)]
    pub const fn set_accuracy(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Frequency."]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Stcclock {
    #[inline(always)]
    fn default() -> Stcclock {
        Stcclock(0)
    }
}
impl core::fmt::Debug for Stcclock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stcclock")
            .field("accuracy", &self.accuracy())
            .field("freq", &self.freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stcclock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stcclock {{ accuracy: {=u8:?}, freq: {=u8:?} }}",
            self.accuracy(),
            self.freq()
        )
    }
}
#[doc = "Target Vendor ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svendorid(pub u32);
impl Svendorid {
    #[doc = "Vendor ID."]
    #[must_use]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Vendor ID."]
    #[inline(always)]
    pub const fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Svendorid {
    #[inline(always)]
    fn default() -> Svendorid {
        Svendorid(0)
    }
}
impl core::fmt::Debug for Svendorid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Svendorid")
            .field("vid", &self.vid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Svendorid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Svendorid {{ vid: {=u16:?} }}", self.vid())
    }
}
#[doc = "Target Write Data Byte."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatab(pub u32);
impl Swdatab {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "End."]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End."]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "End Also."]
    #[must_use]
    #[inline(always)]
    pub const fn end_also(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End Also."]
    #[inline(always)]
    pub const fn set_end_also(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Swdatab {
    #[inline(always)]
    fn default() -> Swdatab {
        Swdatab(0)
    }
}
impl core::fmt::Debug for Swdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatab")
            .field("data", &self.data())
            .field("end", &self.end())
            .field("end_also", &self.end_also())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swdatab {{ data: {=u8:?}, end: {=bool:?}, end_also: {=bool:?} }}",
            self.data(),
            self.end(),
            self.end_also()
        )
    }
}
#[doc = "Target Write Data Byte."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatab1(pub u32);
impl Swdatab1 {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Swdatab1 {
    #[inline(always)]
    fn default() -> Swdatab1 {
        Swdatab1(0)
    }
}
impl core::fmt::Debug for Swdatab1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatab1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatab1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swdatab1 {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Target Write Data Byte End."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatabe(pub u32);
impl Swdatabe {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Swdatabe {
    #[inline(always)]
    fn default() -> Swdatabe {
        Swdatabe(0)
    }
}
impl core::fmt::Debug for Swdatabe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatabe")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatabe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swdatabe {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Target Write Data Halfword."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatah(pub u32);
impl Swdatah {
    #[doc = "Data 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data 1."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "End of Message."]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message."]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Swdatah {
    #[inline(always)]
    fn default() -> Swdatah {
        Swdatah(0)
    }
}
impl core::fmt::Debug for Swdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatah")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("end", &self.end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swdatah {{ data0: {=u8:?}, data1: {=u8:?}, end: {=bool:?} }}",
            self.data0(),
            self.data1(),
            self.end()
        )
    }
}
#[doc = "Target Write Data Halfword End."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatahe(pub u32);
impl Swdatahe {
    #[doc = "Data 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data 1."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Swdatahe {
    #[inline(always)]
    fn default() -> Swdatahe {
        Swdatahe(0)
    }
}
impl core::fmt::Debug for Swdatahe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatahe")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatahe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swdatahe {{ data0: {=u8:?}, data1: {=u8:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "Target Write Word Data (to Bus)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdataw(pub u32);
impl Swdataw {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Swdataw {
    #[inline(always)]
    fn default() -> Swdataw {
        Swdataw(0)
    }
}
impl core::fmt::Debug for Swdataw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdataw")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdataw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swdataw {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actstate {
    #[doc = "NO_LATENCY (normal bus operations)."]
    NoLatency = 0x0,
    #[doc = "LATENCY_1MS (1 ms of latency)."]
    Latency1ms = 0x01,
    #[doc = "LATENCY_100MS (100 ms of latency)."]
    Latency100ms = 0x02,
    #[doc = "LATENCY_10S (10 seconds of latency)."]
    Latency10s = 0x03,
}
impl Actstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Actstate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Actstate {
    #[inline(always)]
    fn from(val: u8) -> Actstate {
        Actstate::from_bits(val)
    }
}
impl From<Actstate> for u8 {
    #[inline(always)]
    fn from(val: Actstate) -> u8 {
        Actstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cause {
    #[doc = "No information (this value occurs when not configured to write DA)."]
    None = 0x0,
    #[doc = "Set using ENTDAA."]
    Entdaa = 0x01,
    #[doc = "Set using SETDASA, SETAASA, or SETNEWDA."]
    Setdasa = 0x02,
    #[doc = "Cleared using RSTDAA."]
    Rstdaa = 0x03,
    #[doc = "Auto MAP change happened last."]
    Automap = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cause {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cause {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cause {
    #[inline(always)]
    fn from(val: u8) -> Cause {
        Cause::from_bits(val)
    }
}
impl From<Cause> for u8 {
    #[inline(always)]
    fn from(val: Cause) -> u8 {
        Cause::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccchandle {
    #[doc = "All handling features disabled."]
    AllDisabled = 0x0,
    #[doc = "The I3C module manages events, activities, status, HDR, and if enabled for it, ID and static-address-related items."]
    BlockHandle = 0x01,
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
impl Ccchandle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccchandle {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccchandle {
    #[inline(always)]
    fn from(val: u8) -> Ccchandle {
        Ccchandle::from_bits(val)
    }
}
impl From<Ccchandle> for u8 {
    #[inline(always)]
    fn from(val: Ccchandle) -> u8 {
        Ccchandle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disto {
    #[doc = "Enabled."]
    Enable = 0x0,
    #[doc = "Disabled."]
    Disable = 0x01,
}
impl Disto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disto {
    #[inline(always)]
    fn from(val: u8) -> Disto {
        Disto::from_bits(val)
    }
}
impl From<Disto> for u8 {
    #[inline(always)]
    fn from(val: Disto) -> u8 {
        Disto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma {
    #[doc = "Not supported."]
    Dmano = 0x0,
    #[doc = "Supported."]
    Dmayes = 0x01,
}
impl Dma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma {
    #[inline(always)]
    fn from(val: u8) -> Dma {
        Dma::from_bits(val)
    }
}
impl From<Dma> for u8 {
    #[inline(always)]
    fn from(val: Dma) -> u8 {
        Dma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evdet {
    #[doc = "NONE (no event or no pending event)."]
    None = 0x0,
    #[doc = "NO_REQUEST (request is not sent yet; either there is no START condition yet, or is waiting for Bus-Available or Bus-Idle (HJ))."]
    NoRequest = 0x01,
    #[doc = "NACKed (not acknowledged, request sent and rejected); I3C tries again."]
    Nacked = 0x02,
    #[doc = "ACKed (acknowledged; request sent and accepted), so done (unless the time control data is still being sent)."]
    Acked = 0x03,
}
impl Evdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evdet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evdet {
    #[inline(always)]
    fn from(val: u8) -> Evdet {
        Evdet::from_bits(val)
    }
}
impl From<Evdet> for u8 {
    #[inline(always)]
    fn from(val: Evdet) -> u8 {
        Evdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extfifo {
    #[doc = "No external FIFO available."]
    NoExtFifo = 0x0,
    #[doc = "Standard available or free external FIFO."]
    StdExtFifo = 0x01,
    #[doc = "Request track external FIFO."]
    RequestExtFifo = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Extfifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extfifo {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extfifo {
    #[inline(always)]
    fn from(val: u8) -> Extfifo {
        Extfifo::from_bits(val)
    }
}
impl From<Extfifo> for u8 {
    #[inline(always)]
    fn from(val: Extfifo) -> u8 {
        Extfifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fiforx {
    #[doc = "Two or three."]
    Fifo2byte = 0x0,
    #[doc = "Four."]
    Fifo4byte = 0x01,
    #[doc = "Eight."]
    Fifo8byte = 0x02,
    #[doc = "16 or larger."]
    Fifo16byte = 0x03,
}
impl Fiforx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fiforx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fiforx {
    #[inline(always)]
    fn from(val: u8) -> Fiforx {
        Fiforx::from_bits(val)
    }
}
impl From<Fiforx> for u8 {
    #[inline(always)]
    fn from(val: Fiforx) -> u8 {
        Fiforx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifotx {
    #[doc = "Two."]
    Fifo2byte = 0x0,
    #[doc = "Four."]
    Fifo4byte = 0x01,
    #[doc = "Eight."]
    Fifo8byte = 0x02,
    #[doc = "16 or larger."]
    Fifo16byte = 0x03,
}
impl Fifotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifotx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifotx {
    #[inline(always)]
    fn from(val: u8) -> Fifotx {
        Fifotx::from_bits(val)
    }
}
impl From<Fifotx> for u8 {
    #[inline(always)]
    fn from(val: Fifotx) -> u8 {
        Fifotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Group {
    #[doc = "v1.1 group addressing not supported."]
    Notsupported = 0x0,
    #[doc = "One group supported."]
    One = 0x01,
    #[doc = "Two groups supported."]
    Two = 0x02,
    #[doc = "Three groups supported."]
    Three = 0x03,
}
impl Group {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Group {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Group {
    #[inline(always)]
    fn from(val: u8) -> Group {
        Group::from_bits(val)
    }
}
impl From<Group> for u8 {
    #[inline(always)]
    fn from(val: Group) -> u8 {
        Group::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hdrsupp {
    #[doc = "No HDR modes supported."]
    NoHdr = 0x0,
    #[doc = "DDR mode supported."]
    Ddr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hdrsupp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hdrsupp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hdrsupp {
    #[inline(always)]
    fn from(val: u8) -> Hdrsupp {
        Hdrsupp::from_bits(val)
    }
}
impl From<Hdrsupp> for u8 {
    #[inline(always)]
    fn from(val: Hdrsupp) -> u8 {
        Hdrsupp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hjdis {
    #[doc = "Enabled."]
    MrEnabled = 0x0,
    #[doc = "Disabled."]
    MrDisabled = 0x01,
}
impl Hjdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hjdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hjdis {
    #[inline(always)]
    fn from(val: u8) -> Hjdis {
        Hjdis::from_bits(val)
    }
}
impl From<Hjdis> for u8 {
    #[inline(always)]
    fn from(val: Hjdis) -> u8 {
        Hjdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hkeep {
    #[doc = "None."]
    None = 0x0,
    #[doc = "WIRED_IN."]
    WiredIn = 0x01,
    #[doc = "PASSIVE_SDA (I2C mode, no clock stretches mode)."]
    PassiveSda = 0x02,
    #[doc = "PASSIVE_ON_SDA_SCL."]
    PassiveOnSdaScl = 0x03,
}
impl Hkeep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hkeep {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hkeep {
    #[inline(always)]
    fn from(val: u8) -> Hkeep {
        Hkeep::from_bits(val)
    }
}
impl From<Hkeep> for u8 {
    #[inline(always)]
    fn from(val: Hkeep) -> u8 {
        Hkeep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2c {
    #[doc = "I3C message."]
    I3cmessage = 0x0,
    #[doc = "I2C message."]
    I2cmessage = 0x01,
}
impl I2c {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c {
    #[inline(always)]
    fn from(val: u8) -> I2c {
        I2c::from_bits(val)
    }
}
impl From<I2c> for u8 {
    #[inline(always)]
    fn from(val: I2c) -> u8 {
        I2c::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3cCasDel {
    #[doc = "No delay."]
    NoDelay = 0x0,
    #[doc = "Increases SCL clock period by 1/2."]
    OneHalfClk = 0x01,
    #[doc = "Increases SCL clock period by 1."]
    OneClk = 0x02,
    #[doc = "Increases SCL clock period by 3/2."]
    OneAndOneHalfClk = 0x03,
}
impl I3cCasDel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3cCasDel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3cCasDel {
    #[inline(always)]
    fn from(val: u8) -> I3cCasDel {
        I3cCasDel::from_bits(val)
    }
}
impl From<I3cCasDel> for u8 {
    #[inline(always)]
    fn from(val: I3cCasDel) -> u8 {
        I3cCasDel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3cCasrDel {
    #[doc = "No delay."]
    NoDelay = 0x0,
    #[doc = "Increases SCL clock period by 1/2."]
    OneHalfClk = 0x01,
    #[doc = "Increases SCL clock period by 1."]
    OneClk = 0x02,
    #[doc = "Increases SCL clock period by 1 1/2."]
    OneAndOneHalfClk = 0x03,
}
impl I3cCasrDel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3cCasrDel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3cCasrDel {
    #[inline(always)]
    fn from(val: u8) -> I3cCasrDel {
        I3cCasrDel::from_bits(val)
    }
}
impl From<I3cCasrDel> for u8 {
    #[inline(always)]
    fn from(val: I3cCasrDel) -> u8 {
        I3cCasrDel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IbiMrHj {
    #[doc = "Application cannot generate IBI, CR, or HJ."]
    AllDisabled = 0x0,
    #[doc = "Application can generate an IBI."]
    Ibi = 0x01,
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
impl IbiMrHj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IbiMrHj {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IbiMrHj {
    #[inline(always)]
    fn from(val: u8) -> IbiMrHj {
        IbiMrHj::from_bits(val)
    }
}
impl From<IbiMrHj> for u8 {
    #[inline(always)]
    fn from(val: IbiMrHj) -> u8 {
        IbiMrHj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibidis {
    #[doc = "Enabled."]
    InterruptsEnabled = 0x0,
    #[doc = "Disabled."]
    InterruptsDisabled = 0x01,
}
impl Ibidis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibidis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibidis {
    #[inline(always)]
    fn from(val: u8) -> Ibidis {
        Ibidis::from_bits(val)
    }
}
impl From<Ibidis> for u8 {
    #[inline(always)]
    fn from(val: Ibidis) -> u8 {
        Ibidis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibiresp {
    #[doc = "ACK (acknowledge)."]
    Ack = 0x0,
    #[doc = "NACK (reject)."]
    Nack = 0x01,
    #[doc = "Acknowledge with mandatory byte."]
    AckWithMandatory = 0x02,
    #[doc = "Manual."]
    Manual = 0x03,
}
impl Ibiresp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibiresp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibiresp {
    #[inline(always)]
    fn from(val: u8) -> Ibiresp {
        Ibiresp::from_bits(val)
    }
}
impl From<Ibiresp> for u8 {
    #[inline(always)]
    fn from(val: Ibiresp) -> u8 {
        Ibiresp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibitype {
    #[doc = "NONE (no IBI: this status occurs when MSTATUS\\[IBIWON\\] becomes 0)."]
    None = 0x0,
    #[doc = "IBI."]
    Ibi = 0x01,
    #[doc = "CR."]
    Mr = 0x02,
    #[doc = "HJ."]
    Hj = 0x03,
}
impl Ibitype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibitype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibitype {
    #[inline(always)]
    fn from(val: u8) -> Ibitype {
        Ibitype::from_bits(val)
    }
}
impl From<Ibitype> for u8 {
    #[inline(always)]
    fn from(val: Ibitype) -> u8 {
        Ibitype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idena {
    #[doc = "Application."]
    Application = 0x0,
    #[doc = "Hardware."]
    Hw = 0x01,
    #[doc = "Hardware, but the I3C module instance handles ID 48b."]
    HwBut = 0x02,
    #[doc = "A part number register (PARTNO)."]
    Partno = 0x03,
}
impl Idena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idena {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idena {
    #[inline(always)]
    fn from(val: u8) -> Idena {
        Idena::from_bits(val)
    }
}
impl From<Idena> for u8 {
    #[inline(always)]
    fn from(val: Idena) -> u8 {
        Idena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idreg {
    #[doc = "All ID register features disabled."]
    AllDisabled = 0x0,
    #[doc = "ID Instance is a register; used if there is no PARTNO register."]
    IdInstance = 0x01,
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
impl Idreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idreg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idreg {
    #[inline(always)]
    fn from(val: u8) -> Idreg {
        Idreg::from_bits(val)
    }
}
impl From<Idreg> for u8 {
    #[inline(always)]
    fn from(val: Idreg) -> u8 {
        Idreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int {
    #[doc = "Not supported."]
    Interruptsno = 0x0,
    #[doc = "Supported."]
    Interruptsyes = 0x01,
}
impl Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int {
    #[inline(always)]
    fn from(val: u8) -> Int {
        Int::from_bits(val)
    }
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(val: Int) -> u8 {
        Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Laststatic {
    #[doc = "I3C dynamic address."]
    I3c = 0x0,
    #[doc = "I2C static address."]
    I2c = 0x01,
}
impl Laststatic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Laststatic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Laststatic {
    #[inline(always)]
    fn from(val: u8) -> Laststatic {
        Laststatic::from_bits(val)
    }
}
impl From<Laststatic> for u8 {
    #[inline(always)]
    fn from(val: Laststatic) -> u8 {
        Laststatic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Not supported."]
    Masternotsupported = 0x0,
    #[doc = "Supported."]
    Mastersupported = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctrlDir {
    #[doc = "Write."]
    Dirwrite = 0x0,
    #[doc = "Read."]
    Dirread = 0x01,
}
impl MctrlDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctrlDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctrlDir {
    #[inline(always)]
    fn from(val: u8) -> MctrlDir {
        MctrlDir::from_bits(val)
    }
}
impl From<MctrlDir> for u8 {
    #[inline(always)]
    fn from(val: MctrlDir) -> u8 {
        MctrlDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdatactrlRxtrig {
    #[doc = "Trigger when not empty (default)."]
    NotEmpty = 0x0,
    #[doc = "Trigger when 1/4 full or more."]
    QuarterOrMore = 0x01,
    #[doc = "Trigger when 1/2 full or more."]
    HalfOrMore = 0x02,
    #[doc = "Trigger when 3/4 full or more."]
    ThreeQuarterOrMore = 0x03,
}
impl MdatactrlRxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlRxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlRxtrig {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlRxtrig {
        MdatactrlRxtrig::from_bits(val)
    }
}
impl From<MdatactrlRxtrig> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlRxtrig) -> u8 {
        MdatactrlRxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdatactrlTxtrig {
    #[doc = "Trigger when empty."]
    Empty = 0x0,
    #[doc = "Trigger when 1/4 full or less."]
    QuarterOrLess = 0x01,
    #[doc = "Trigger when 1/2 full or less."]
    HalfOrLess = 0x02,
    #[doc = "Trigger when 1 less than full or less (default)."]
    FullOrLess = 0x03,
}
impl MdatactrlTxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlTxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlTxtrig {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlTxtrig {
        MdatactrlTxtrig::from_bits(val)
    }
}
impl From<MdatactrlTxtrig> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlTxtrig) -> u8 {
        MdatactrlTxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmafb {
    #[doc = "DMA not used."]
    NotUsed = 0x0,
    #[doc = "Enable DMA for one frame."]
    EnableOneFrame = 0x01,
    #[doc = "Enable DMA until DMA is turned off."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MdmactrlDmafb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdmactrlDmafb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdmactrlDmafb {
    #[inline(always)]
    fn from(val: u8) -> MdmactrlDmafb {
        MdmactrlDmafb::from_bits(val)
    }
}
impl From<MdmactrlDmafb> for u8 {
    #[inline(always)]
    fn from(val: MdmactrlDmafb) -> u8 {
        MdmactrlDmafb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmatb {
    #[doc = "DMA not used."]
    NotUsed = 0x0,
    #[doc = "Enable DMA for one frame (ended by DMA or terminated)."]
    EnableOneFrame = 0x01,
    #[doc = "Enable DMA until DMA is turned off."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MdmactrlDmatb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdmactrlDmatb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdmactrlDmatb {
    #[inline(always)]
    fn from(val: u8) -> MdmactrlDmatb {
        MdmactrlDmatb::from_bits(val)
    }
}
impl From<MdmactrlDmatb> for u8 {
    #[inline(always)]
    fn from(val: MdmactrlDmatb) -> u8 {
        MdmactrlDmatb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmawidth {
    #[doc = "Byte."]
    Byte0 = 0x0,
    #[doc = "Byte."]
    Byte1 = 0x01,
    #[doc = "Halfword (16 bits)."]
    HalfWord = 0x02,
    _RESERVED_3 = 0x03,
}
impl MdmactrlDmawidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdmactrlDmawidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdmactrlDmawidth {
    #[inline(always)]
    fn from(val: u8) -> MdmactrlDmawidth {
        MdmactrlDmawidth::from_bits(val)
    }
}
impl From<MdmactrlDmawidth> for u8 {
    #[inline(always)]
    fn from(val: MdmactrlDmawidth) -> u8 {
        MdmactrlDmawidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mldat {
    #[doc = "Single lane for data."]
    Single = 0x0,
    #[doc = "Dual lane for data."]
    Dual = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Quad-lane data."]
    Quad = 0x03,
}
impl Mldat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mldat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mldat {
    #[inline(always)]
    fn from(val: u8) -> Mldat {
        Mldat::from_bits(val)
    }
}
impl From<Mldat> for u8 {
    #[inline(always)]
    fn from(val: Mldat) -> u8 {
        Mldat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mlhdr {
    #[doc = "Single lane for header."]
    Single = 0x0,
    #[doc = "Dual lane for header."]
    Dual = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Quad-lane header."]
    Quad = 0x03,
}
impl Mlhdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mlhdr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mlhdr {
    #[inline(always)]
    fn from(val: u8) -> Mlhdr {
        Mlhdr::from_bits(val)
    }
}
impl From<Mlhdr> for u8 {
    #[inline(always)]
    fn from(val: Mlhdr) -> u8 {
        Mlhdr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrdis {
    #[doc = "Enabled."]
    MrEnabled = 0x0,
    #[doc = "Disabled."]
    MrDisabled = 0x01,
}
impl Mrdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrdis {
    #[inline(always)]
    fn from(val: u8) -> Mrdis {
        Mrdis::from_bits(val)
    }
}
impl From<Mrdis> for u8 {
    #[inline(always)]
    fn from(val: Mrdis) -> u8 {
        Mrdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstena {
    #[doc = "CONTROLLER_OFF."]
    MasterOff = 0x0,
    #[doc = "CONTROLLER_ON."]
    MasterOn = 0x01,
    #[doc = "CONTROLLER_CAPABLE."]
    MasterCapable = 0x02,
    #[doc = "I2C_CONTROLLER_MODE."]
    I2cMasterMode = 0x03,
}
impl Mstena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstena {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstena {
    #[inline(always)]
    fn from(val: u8) -> Mstena {
        Mstena::from_bits(val)
    }
}
impl From<Mstena> for u8 {
    #[inline(always)]
    fn from(val: Mstena) -> u8 {
        Mstena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwmsgSdrControlDir {
    #[doc = "Write."]
    Write = 0x0,
    #[doc = "Read."]
    Read = 0x01,
}
impl MwmsgSdrControlDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwmsgSdrControlDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwmsgSdrControlDir {
    #[inline(always)]
    fn from(val: u8) -> MwmsgSdrControlDir {
        MwmsgSdrControlDir::from_bits(val)
    }
}
impl From<MwmsgSdrControlDir> for u8 {
    #[inline(always)]
    fn from(val: MwmsgSdrControlDir) -> u8 {
        MwmsgSdrControlDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nobyte {
    #[doc = "With mandatory IBI byte."]
    Ibibyte = 0x0,
    #[doc = "Without mandatory IBI byte."]
    NoIbibyte = 0x01,
}
impl Nobyte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nobyte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nobyte {
    #[inline(always)]
    fn from(val: u8) -> Nobyte {
        Nobyte::from_bits(val)
    }
}
impl From<Nobyte> for u8 {
    #[inline(always)]
    fn from(val: Nobyte) -> u8 {
        Nobyte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Request {
    #[doc = "NONE."]
    None = 0x0,
    #[doc = "EMITSTARTADDR."]
    Emitstartaddr = 0x01,
    #[doc = "EMITSTOP."]
    Emitstop = 0x02,
    #[doc = "IBIACKNACK."]
    Ibiacknack = 0x03,
    #[doc = "PROCESSDAA."]
    Processdaa = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Force Exit and Target Reset."]
    Forceexit = 0x06,
    #[doc = "AUTOIBI."]
    Autoibi = 0x07,
}
impl Request {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Request {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Request {
    #[inline(always)]
    fn from(val: u8) -> Request {
        Request::from_bits(val)
    }
}
impl From<Request> for u8 {
    #[inline(always)]
    fn from(val: Request) -> u8 {
        Request::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saddr {
    #[doc = "No static address."]
    NoStatic = 0x0,
    #[doc = "Static address is fixed in hardware."]
    Static = 0x01,
    #[doc = "Hardware controls the static address dynamically (for example, from the pin strap)."]
    HwControl = 0x02,
    #[doc = "SCONFIG register supplies the static address."]
    Config = 0x03,
}
impl Saddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saddr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saddr {
    #[inline(always)]
    fn from(val: u8) -> Saddr {
        Saddr::from_bits(val)
    }
}
impl From<Saddr> for u8 {
    #[inline(always)]
    fn from(val: Saddr) -> u8 {
        Saddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScapabilitiesTimectrl {
    #[doc = "No time control supported."]
    NoTimeControlType = 0x0,
    #[doc = "At least one time-control type supported."]
    Atleast1TimeControl = 0x01,
}
impl ScapabilitiesTimectrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScapabilitiesTimectrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScapabilitiesTimectrl {
    #[inline(always)]
    fn from(val: u8) -> ScapabilitiesTimectrl {
        ScapabilitiesTimectrl::from_bits(val)
    }
}
impl From<ScapabilitiesTimectrl> for u8 {
    #[inline(always)]
    fn from(val: ScapabilitiesTimectrl) -> u8 {
        ScapabilitiesTimectrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctrlEvent {
    #[doc = "NORMAL_MODE."]
    NormalMode = 0x0,
    #[doc = "IBI."]
    Ibi = 0x01,
    #[doc = "CONTROLLER_REQUEST."]
    MasterRequest = 0x02,
    #[doc = "HOT_JOIN_REQUEST."]
    HotJoinRequest = 0x03,
}
impl SctrlEvent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctrlEvent {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctrlEvent {
    #[inline(always)]
    fn from(val: u8) -> SctrlEvent {
        SctrlEvent::from_bits(val)
    }
}
impl From<SctrlEvent> for u8 {
    #[inline(always)]
    fn from(val: SctrlEvent) -> u8 {
        SctrlEvent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlRxempty {
    #[doc = "Not empty."]
    Rxisnotempty = 0x0,
    #[doc = "Empty."]
    Rxisempty = 0x01,
}
impl SdatactrlRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlRxempty {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlRxempty {
        SdatactrlRxempty::from_bits(val)
    }
}
impl From<SdatactrlRxempty> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlRxempty) -> u8 {
        SdatactrlRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlRxtrig {
    #[doc = "Trigger when not empty (default)."]
    Triggrnotempty = 0x0,
    #[doc = "Trigger when 1/4 or more full."]
    Triggronefourth = 0x01,
    #[doc = "Trigger when 1/2 or more full."]
    Triggronehalf = 0x02,
    #[doc = "Trigger when 3/4 or more full."]
    Triggrthreefourths = 0x03,
}
impl SdatactrlRxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlRxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlRxtrig {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlRxtrig {
        SdatactrlRxtrig::from_bits(val)
    }
}
impl From<SdatactrlRxtrig> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlRxtrig) -> u8 {
        SdatactrlRxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlTxfull {
    #[doc = "Not full."]
    Txisnotfull = 0x0,
    #[doc = "Full."]
    Txisfull = 0x01,
}
impl SdatactrlTxfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlTxfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlTxfull {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlTxfull {
        SdatactrlTxfull::from_bits(val)
    }
}
impl From<SdatactrlTxfull> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlTxfull) -> u8 {
        SdatactrlTxfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlTxtrig {
    #[doc = "Trigger when empty."]
    Triggrempty = 0x0,
    #[doc = "Trigger when 1/4 full or less."]
    Triggronefourth = 0x01,
    #[doc = "Trigger when 1/2 full or less."]
    Triggronehalf = 0x02,
    #[doc = "Default (trigger when 1 less than full or less)."]
    Triggroneless = 0x03,
}
impl SdatactrlTxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlTxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlTxtrig {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlTxtrig {
        SdatactrlTxtrig::from_bits(val)
    }
}
impl From<SdatactrlTxtrig> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlTxtrig) -> u8 {
        SdatactrlTxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmafb {
    #[doc = "DMA not used."]
    NotUsed = 0x0,
    #[doc = "DMA enabled for one frame."]
    EnableOneFrame = 0x01,
    #[doc = "DMA enabled until turned off."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl SdmactrlDmafb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdmactrlDmafb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdmactrlDmafb {
    #[inline(always)]
    fn from(val: u8) -> SdmactrlDmafb {
        SdmactrlDmafb::from_bits(val)
    }
}
impl From<SdmactrlDmafb> for u8 {
    #[inline(always)]
    fn from(val: SdmactrlDmafb) -> u8 {
        SdmactrlDmafb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmatb {
    #[doc = "DMA not used."]
    NotUsed = 0x0,
    #[doc = "DMA enabled for one frame."]
    EnableOneFrame = 0x01,
    #[doc = "DMA enabled until turned off."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl SdmactrlDmatb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdmactrlDmatb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdmactrlDmatb {
    #[inline(always)]
    fn from(val: u8) -> SdmactrlDmatb {
        SdmactrlDmatb::from_bits(val)
    }
}
impl From<SdmactrlDmatb> for u8 {
    #[inline(always)]
    fn from(val: SdmactrlDmatb) -> u8 {
        SdmactrlDmatb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmawidth {
    #[doc = "Byte."]
    Byte0 = 0x0,
    #[doc = "Byte."]
    Byte1 = 0x01,
    #[doc = "Halfword (16 bits) (this value ensures that two bytes are available in the FIFO)."]
    HalfWord = 0x02,
    _RESERVED_3 = 0x03,
}
impl SdmactrlDmawidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdmactrlDmawidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdmactrlDmawidth {
    #[inline(always)]
    fn from(val: u8) -> SdmactrlDmawidth {
        SdmactrlDmawidth::from_bits(val)
    }
}
impl From<SdmactrlDmawidth> for u8 {
    #[inline(always)]
    fn from(val: SdmactrlDmawidth) -> u8 {
        SdmactrlDmawidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShdrbtcfgCrc32 {
    #[doc = "CRC16."]
    UseCrc16 = 0x0,
    #[doc = "CRC32."]
    UseCrc32 = 0x01,
}
impl ShdrbtcfgCrc32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShdrbtcfgCrc32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShdrbtcfgCrc32 {
    #[inline(always)]
    fn from(val: u8) -> ShdrbtcfgCrc32 {
        ShdrbtcfgCrc32::from_bits(val)
    }
}
impl From<ShdrbtcfgCrc32> for u8 {
    #[inline(always)]
    fn from(val: ShdrbtcfgCrc32) -> u8 {
        ShdrbtcfgCrc32::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusStart {
    #[doc = "Not detected."]
    StartNotDetected = 0x0,
    #[doc = "Detected."]
    StartDetected = 0x01,
}
impl SstatusStart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusStart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusStart {
    #[inline(always)]
    fn from(val: u8) -> SstatusStart {
        SstatusStart::from_bits(val)
    }
}
impl From<SstatusStart> for u8 {
    #[inline(always)]
    fn from(val: SstatusStart) -> u8 {
        SstatusStart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusTimectrl {
    #[doc = "NO_TIME_CONTROL (no time control is enabled)."]
    NoTimeControl = 0x0,
    #[doc = "SYNC_MODE (Synchronous mode is enabled)."]
    Sync = 0x01,
    #[doc = "ASYNC_MODE (Asynchronous standard mode (0 or 1) is enabled)."]
    AsyncMode = 0x02,
    #[doc = "BOTHSYNCASYNC (both Synchronous and Asynchronous modes are enabled)."]
    Bothsyncasync = 0x03,
}
impl SstatusTimectrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusTimectrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusTimectrl {
    #[inline(always)]
    fn from(val: u8) -> SstatusTimectrl {
        SstatusTimectrl::from_bits(val)
    }
}
impl From<SstatusTimectrl> for u8 {
    #[inline(always)]
    fn from(val: SstatusTimectrl) -> u8 {
        SstatusTimectrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusTxnotfull {
    #[doc = "Transmit buffer full."]
    Full = 0x0,
    #[doc = "Transmit buffer not full."]
    NotFull = 0x01,
}
impl SstatusTxnotfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusTxnotfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusTxnotfull {
    #[inline(always)]
    fn from(val: u8) -> SstatusTxnotfull {
        SstatusTxnotfull::from_bits(val)
    }
}
impl From<SstatusTxnotfull> for u8 {
    #[inline(always)]
    fn from(val: SstatusTxnotfull) -> u8 {
        SstatusTxnotfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    #[doc = "IDLE (bus has stopped)."]
    Idle = 0x0,
    #[doc = "SLVREQ (target request)."]
    Slvreq = 0x01,
    #[doc = "MSGSDR."]
    Msgsdr = 0x02,
    #[doc = "NORMACT."]
    Normact = 0x03,
    #[doc = "MSGDDR."]
    Ddr = 0x04,
    #[doc = "DAA."]
    Daa = 0x05,
    #[doc = "IBIACK."]
    Ibiack = 0x06,
    #[doc = "IBIRCV."]
    Ibircv = 0x07,
}
impl State {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for State {
    #[inline(always)]
    fn from(val: u8) -> State {
        State::from_bits(val)
    }
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(val: State) -> u8 {
        State::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stccch {
    #[doc = "No CCC message handled."]
    Idle = 0x0,
    #[doc = "Handled automatically."]
    Busy = 0x01,
}
impl Stccch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stccch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stccch {
    #[inline(always)]
    fn from(val: u8) -> Stccch {
        Stccch::from_bits(val)
    }
}
impl From<Stccch> for u8 {
    #[inline(always)]
    fn from(val: Stccch) -> u8 {
        Stccch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stmsg {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl Stmsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stmsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stmsg {
    #[inline(always)]
    fn from(val: u8) -> Stmsg {
        Stmsg::from_bits(val)
    }
}
impl From<Stmsg> for u8 {
    #[inline(always)]
    fn from(val: Stmsg) -> u8 {
        Stmsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stnotstop {
    #[doc = "In STOP condition."]
    Stopped = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl Stnotstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stnotstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stnotstop {
    #[inline(always)]
    fn from(val: u8) -> Stnotstop {
        Stnotstop::from_bits(val)
    }
}
impl From<Stnotstop> for u8 {
    #[inline(always)]
    fn from(val: Stnotstop) -> u8 {
        Stnotstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Streqrd {
    #[doc = "Not an SDR read."]
    Idle = 0x0,
    #[doc = "SDR read from this target or an IBI is being pushed out."]
    Busy = 0x01,
}
impl Streqrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Streqrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Streqrd {
    #[inline(always)]
    fn from(val: u8) -> Streqrd {
        Streqrd::from_bits(val)
    }
}
impl From<Streqrd> for u8 {
    #[inline(always)]
    fn from(val: Streqrd) -> u8 {
        Streqrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Streqwr {
    #[doc = "Not an SDR write."]
    Idle = 0x0,
    #[doc = "SDR write data from the controller, but not in ENTDAA mode."]
    Busy = 0x01,
}
impl Streqwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Streqwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Streqwr {
    #[inline(always)]
    fn from(val: u8) -> Streqwr {
        Streqwr::from_bits(val)
    }
}
impl From<Streqwr> for u8 {
    #[inline(always)]
    fn from(val: Streqwr) -> u8 {
        Streqwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Type {
    #[doc = "I3C."]
    I3c = 0x0,
    #[doc = "I2C."]
    I2c = 0x01,
    #[doc = "DDR."]
    Ddr = 0x02,
    _RESERVED_3 = 0x03,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
