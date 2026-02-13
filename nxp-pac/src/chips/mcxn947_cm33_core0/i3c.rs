#[doc = "I3C"]
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
    #[doc = "Controller Configuration"]
    #[inline(always)]
    pub const fn mconfig(self) -> crate::common::Reg<regs::Mconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Target Configuration"]
    #[inline(always)]
    pub const fn sconfig(self) -> crate::common::Reg<regs::Sconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Target Status"]
    #[inline(always)]
    pub const fn sstatus(self) -> crate::common::Reg<regs::Sstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Target Control"]
    #[inline(always)]
    pub const fn sctrl(self) -> crate::common::Reg<regs::Sctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Target Interrupt Set"]
    #[inline(always)]
    pub const fn sintset(self) -> crate::common::Reg<regs::Sintset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Target Interrupt Clear"]
    #[inline(always)]
    pub const fn sintclr(self) -> crate::common::Reg<regs::Sintclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Target Interrupt Mask"]
    #[inline(always)]
    pub const fn sintmasked(self) -> crate::common::Reg<regs::Sintmasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Target Errors and Warnings"]
    #[inline(always)]
    pub const fn serrwarn(self) -> crate::common::Reg<regs::Serrwarn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Target DMA Control"]
    #[inline(always)]
    pub const fn sdmactrl(self) -> crate::common::Reg<regs::Sdmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Target Data Control"]
    #[inline(always)]
    pub const fn sdatactrl(self) -> crate::common::Reg<regs::Sdatactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Target Write Data Byte"]
    #[inline(always)]
    pub const fn swdatab(self) -> crate::common::Reg<regs::Swdatab, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Target Write Data Byte End"]
    #[inline(always)]
    pub const fn swdatabe(self) -> crate::common::Reg<regs::Swdatabe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Target Write Data Halfword"]
    #[inline(always)]
    pub const fn swdatah(self) -> crate::common::Reg<regs::Swdatah, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Target Write Data Halfword End"]
    #[inline(always)]
    pub const fn swdatahe(self) -> crate::common::Reg<regs::Swdatahe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Target Read Data Byte"]
    #[inline(always)]
    pub const fn srdatab(self) -> crate::common::Reg<regs::Srdatab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Target Read Data Halfword"]
    #[inline(always)]
    pub const fn srdatah(self) -> crate::common::Reg<regs::Srdatah, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Target Write Data Byte"]
    #[inline(always)]
    pub const fn swdatab1(self) -> crate::common::Reg<regs::Swdatab1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Target Capabilities 2"]
    #[inline(always)]
    pub const fn scapabilities2(
        self,
    ) -> crate::common::Reg<regs::Scapabilities2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Target Capabilities"]
    #[inline(always)]
    pub const fn scapabilities(self) -> crate::common::Reg<regs::Scapabilities, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Target Dynamic Address"]
    #[inline(always)]
    pub const fn sdynaddr(self) -> crate::common::Reg<regs::Sdynaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Target Maximum Limits"]
    #[inline(always)]
    pub const fn smaxlimits(self) -> crate::common::Reg<regs::Smaxlimits, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Target ID Part Number"]
    #[inline(always)]
    pub const fn sidpartno(self) -> crate::common::Reg<regs::Sidpartno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Target ID Extension"]
    #[inline(always)]
    pub const fn sidext(self) -> crate::common::Reg<regs::Sidext, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Target Vendor ID"]
    #[inline(always)]
    pub const fn svendorid(self) -> crate::common::Reg<regs::Svendorid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Target Time Control Clock"]
    #[inline(always)]
    pub const fn stcclock(self) -> crate::common::Reg<regs::Stcclock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Target Message Map Address"]
    #[inline(always)]
    pub const fn smsgmapaddr(self) -> crate::common::Reg<regs::Smsgmapaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Controller Control"]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Controller Status"]
    #[inline(always)]
    pub const fn mstatus(self) -> crate::common::Reg<regs::Mstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Controller In-band Interrupt Registry and Rules"]
    #[inline(always)]
    pub const fn mibirules(self) -> crate::common::Reg<regs::Mibirules, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Controller Interrupt Set"]
    #[inline(always)]
    pub const fn mintset(self) -> crate::common::Reg<regs::Mintset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Controller Interrupt Clear"]
    #[inline(always)]
    pub const fn mintclr(self) -> crate::common::Reg<regs::Mintclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Controller Interrupt Mask"]
    #[inline(always)]
    pub const fn mintmasked(self) -> crate::common::Reg<regs::Mintmasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Controller Errors and Warnings"]
    #[inline(always)]
    pub const fn merrwarn(self) -> crate::common::Reg<regs::Merrwarn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Controller DMA Control"]
    #[inline(always)]
    pub const fn mdmactrl(self) -> crate::common::Reg<regs::Mdmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Controller Data Control"]
    #[inline(always)]
    pub const fn mdatactrl(self) -> crate::common::Reg<regs::Mdatactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Controller Write Data Byte"]
    #[inline(always)]
    pub const fn mwdatab(self) -> crate::common::Reg<regs::Mwdatab, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Controller Write Data Byte End"]
    #[inline(always)]
    pub const fn mwdatabe(self) -> crate::common::Reg<regs::Mwdatabe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Controller Write Data Halfword"]
    #[inline(always)]
    pub const fn mwdatah(self) -> crate::common::Reg<regs::Mwdatah, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Controller Write Data Halfword End"]
    #[inline(always)]
    pub const fn mwdatahe(self) -> crate::common::Reg<regs::Mwdatahe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Controller Read Data Byte"]
    #[inline(always)]
    pub const fn mrdatab(self) -> crate::common::Reg<regs::Mrdatab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Controller Read Data Halfword"]
    #[inline(always)]
    pub const fn mrdatah(self) -> crate::common::Reg<regs::Mrdatah, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Controller Write Byte Data 1 (to Bus)"]
    #[inline(always)]
    pub const fn mwdatab1(self) -> crate::common::Reg<regs::Mwdatab1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Controller Write Message Control in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_control(
        self,
    ) -> crate::common::Reg<regs::MwmsgSdrControl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Controller Write Message Data in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_data(self) -> crate::common::Reg<regs::MwmsgSdrData, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Controller Read Message in SDR mode"]
    #[inline(always)]
    pub const fn mrmsg_sdr(self) -> crate::common::Reg<regs::MrmsgSdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Controller Write Message in DDR mode: First Control Word"]
    #[inline(always)]
    pub const fn mwmsg_ddr_control(
        self,
    ) -> crate::common::Reg<regs::MwmsgDdrControl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Controller Write Message in DDR Mode Control 2"]
    #[inline(always)]
    pub const fn mwmsg_ddr_control2(
        self,
    ) -> crate::common::Reg<regs::MwmsgDdrControl2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Controller Write Message Data in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_data(self) -> crate::common::Reg<regs::MwmsgDdrData, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Controller Read Message in DDR mode"]
    #[inline(always)]
    pub const fn mrmsg_ddr(self) -> crate::common::Reg<regs::MrmsgDdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Controller Dynamic Address"]
    #[inline(always)]
    pub const fn mdynaddr(self) -> crate::common::Reg<regs::Mdynaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Map Feature Control 0"]
    #[inline(always)]
    pub const fn smapctrl0(self) -> crate::common::Reg<regs::Smapctrl0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Extended IBI Data 1"]
    #[inline(always)]
    pub const fn ibiext1(self) -> crate::common::Reg<regs::Ibiext1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Extended IBI Data 2"]
    #[inline(always)]
    pub const fn ibiext2(self) -> crate::common::Reg<regs::Ibiext2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Target Module ID"]
    #[inline(always)]
    pub const fn sid(self) -> crate::common::Reg<regs::Sid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
