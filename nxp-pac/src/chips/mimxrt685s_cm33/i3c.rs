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
    #[doc = "Master Configuration Register"]
    #[inline(always)]
    pub const fn mconfig(self) -> crate::common::Reg<regs::Mconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Slave Configuration Register"]
    #[inline(always)]
    pub const fn sconfig(self) -> crate::common::Reg<regs::Sconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Slave Status Register"]
    #[inline(always)]
    pub const fn sstatus(self) -> crate::common::Reg<regs::Sstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Slave Control Register"]
    #[inline(always)]
    pub const fn sctrl(self) -> crate::common::Reg<regs::Sctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Slave Interrupt Set Register"]
    #[inline(always)]
    pub const fn sintset(self) -> crate::common::Reg<regs::Sintset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Slave Interrupt Clear Register"]
    #[inline(always)]
    pub const fn sintclr(self) -> crate::common::Reg<regs::Sintclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Slave Interrupt Mask Register"]
    #[inline(always)]
    pub const fn sintmasked(self) -> crate::common::Reg<regs::Sintmasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Slave Errors and Warnings Register"]
    #[inline(always)]
    pub const fn serrwarn(self) -> crate::common::Reg<regs::Serrwarn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Slave DMA Control Register"]
    #[inline(always)]
    pub const fn sdmactrl(self) -> crate::common::Reg<regs::Sdmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Slave Data Control Register"]
    #[inline(always)]
    pub const fn sdatactrl(self) -> crate::common::Reg<regs::Sdatactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Slave Write Data Byte Register"]
    #[inline(always)]
    pub const fn swdatab(self) -> crate::common::Reg<regs::Swdatab, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Slave Write Data Byte End"]
    #[inline(always)]
    pub const fn swdatabe(self) -> crate::common::Reg<regs::Swdatabe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Slave Write Data Half-word Register"]
    #[inline(always)]
    pub const fn swdatah(self) -> crate::common::Reg<regs::Swdatah, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Slave Write Data Half-word End Register"]
    #[inline(always)]
    pub const fn swdatahe(self) -> crate::common::Reg<regs::Swdatahe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Slave Read Data Byte Register"]
    #[inline(always)]
    pub const fn srdatab(self) -> crate::common::Reg<regs::Srdatab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Slave Read Data Half-word Register"]
    #[inline(always)]
    pub const fn srdatah(self) -> crate::common::Reg<regs::Srdatah, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Slave Capabilities Register"]
    #[inline(always)]
    pub const fn scapabilities(self) -> crate::common::Reg<regs::Scapabilities, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Slave Dynamic Address Register"]
    #[inline(always)]
    pub const fn sdynaddr(self) -> crate::common::Reg<regs::Sdynaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Slave Maximum Limits Register"]
    #[inline(always)]
    pub const fn smaxlimits(self) -> crate::common::Reg<regs::Smaxlimits, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Slave ID Part Number Register"]
    #[inline(always)]
    pub const fn sidpartno(self) -> crate::common::Reg<regs::Sidpartno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Slave ID Extension Register"]
    #[inline(always)]
    pub const fn sidext(self) -> crate::common::Reg<regs::Sidext, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Slave Vendor ID Register"]
    #[inline(always)]
    pub const fn svendorid(self) -> crate::common::Reg<regs::Svendorid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Slave Time Control Clock Register"]
    #[inline(always)]
    pub const fn stcclock(self) -> crate::common::Reg<regs::Stcclock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Slave Message-Mapped Address Register"]
    #[inline(always)]
    pub const fn smsgmapaddr(self) -> crate::common::Reg<regs::Smsgmapaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Master Main Control Register"]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Master Status Register"]
    #[inline(always)]
    pub const fn mstatus(self) -> crate::common::Reg<regs::Mstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Master In-band Interrupt Registry and Rules Register"]
    #[inline(always)]
    pub const fn mibirules(self) -> crate::common::Reg<regs::Mibirules, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Master Interrupt Set Register"]
    #[inline(always)]
    pub const fn mintset(self) -> crate::common::Reg<regs::Mintset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Master Interrupt Clear Register"]
    #[inline(always)]
    pub const fn mintclr(self) -> crate::common::Reg<regs::Mintclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Master Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mintmasked(self) -> crate::common::Reg<regs::Mintmasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Master Errors and Warnings Register"]
    #[inline(always)]
    pub const fn merrwarn(self) -> crate::common::Reg<regs::Merrwarn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Master DMA Control Register"]
    #[inline(always)]
    pub const fn mdmactrl(self) -> crate::common::Reg<regs::Mdmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Master Data Control Register"]
    #[inline(always)]
    pub const fn mdatactrl(self) -> crate::common::Reg<regs::Mdatactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Master Write Data Byte Register"]
    #[inline(always)]
    pub const fn mwdatab(self) -> crate::common::Reg<regs::Mwdatab, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Master Write Data Byte End Register"]
    #[inline(always)]
    pub const fn mwdatabe(self) -> crate::common::Reg<regs::Mwdatabe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Master Write Data Half-word Register"]
    #[inline(always)]
    pub const fn mwdatah(self) -> crate::common::Reg<regs::Mwdatah, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Master Write Data Byte End Register"]
    #[inline(always)]
    pub const fn mwdatahe(self) -> crate::common::Reg<regs::Mwdatahe, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Master Read Data Byte Register"]
    #[inline(always)]
    pub const fn mrdatab(self) -> crate::common::Reg<regs::Mrdatab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Master Read Data Half-word Register"]
    #[inline(always)]
    pub const fn mrdatah(self) -> crate::common::Reg<regs::Mrdatah, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Master Write Message in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_control(
        self,
    ) -> crate::common::Reg<regs::MwmsgSdrControl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Master Write Message Data in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_data(self) -> crate::common::Reg<regs::MwmsgSdrData, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Master Read Message in SDR mode"]
    #[inline(always)]
    pub const fn mrmsg_sdr(self) -> crate::common::Reg<regs::MrmsgSdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Master Write Message in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_control(
        self,
    ) -> crate::common::Reg<regs::MwmsgDdrControl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Master Write Message Data in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_data(self) -> crate::common::Reg<regs::MwmsgDdrData, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Master Read Message in DDR mode"]
    #[inline(always)]
    pub const fn mrmsg_ddr(self) -> crate::common::Reg<regs::MrmsgDdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Master Dynamic Address Register"]
    #[inline(always)]
    pub const fn mdynaddr(self) -> crate::common::Reg<regs::Mdynaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Slave Module ID Register"]
    #[inline(always)]
    pub const fn sid(self) -> crate::common::Reg<regs::Sid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
