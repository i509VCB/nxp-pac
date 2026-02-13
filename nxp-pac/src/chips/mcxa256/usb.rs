#[doc = "Array of registers: ENDPT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endpoint {
    ptr: *mut u8,
}
unsafe impl Send for Endpoint {}
unsafe impl Sync for Endpoint {}
impl Endpoint {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Endpoint Control"]
    #[inline(always)]
    pub const fn endpt(self) -> crate::common::Reg<regs::Endpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "USBFS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Peripheral ID"]
    #[inline(always)]
    pub const fn perid(self) -> crate::common::Reg<regs::Perid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peripheral ID Complement"]
    #[inline(always)]
    pub const fn idcomp(self) -> crate::common::Reg<regs::Idcomp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Revision"]
    #[inline(always)]
    pub const fn rev(self) -> crate::common::Reg<regs::Rev, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Peripheral Additional Information"]
    #[inline(always)]
    pub const fn addinfo(self) -> crate::common::Reg<regs::Addinfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "OTG Interrupt Status"]
    #[inline(always)]
    pub const fn otgistat(self) -> crate::common::Reg<regs::Otgistat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "OTG Interrupt Control"]
    #[inline(always)]
    pub const fn otgicr(self) -> crate::common::Reg<regs::Otgicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "OTG Status"]
    #[inline(always)]
    pub const fn otgstat(self) -> crate::common::Reg<regs::Otgstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "OTG Control"]
    #[inline(always)]
    pub const fn otgctl(self) -> crate::common::Reg<regs::Otgctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn istat(self) -> crate::common::Reg<regs::Istat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Error Interrupt Status"]
    #[inline(always)]
    pub const fn errstat(self) -> crate::common::Reg<regs::Errstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Error Interrupt Enable"]
    #[inline(always)]
    pub const fn erren(self) -> crate::common::Reg<regs::Erren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "BDT Page 1"]
    #[inline(always)]
    pub const fn bdtpage1(self) -> crate::common::Reg<regs::Bdtpage1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Frame Number Register Low"]
    #[inline(always)]
    pub const fn frmnuml(self) -> crate::common::Reg<regs::Frmnuml, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Frame Number Register High"]
    #[inline(always)]
    pub const fn frmnumh(self) -> crate::common::Reg<regs::Frmnumh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Token"]
    #[inline(always)]
    pub const fn token(self) -> crate::common::Reg<regs::Token, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "SOF Threshold"]
    #[inline(always)]
    pub const fn softhld(self) -> crate::common::Reg<regs::Softhld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "BDT Page 2"]
    #[inline(always)]
    pub const fn bdtpage2(self) -> crate::common::Reg<regs::Bdtpage2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "BDT Page 3"]
    #[inline(always)]
    pub const fn bdtpage3(self) -> crate::common::Reg<regs::Bdtpage3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Array of registers: ENDPT"]
    #[inline(always)]
    pub const fn endpoint(self, n: usize) -> Endpoint {
        assert!(n < 16usize);
        unsafe { Endpoint::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "USB Control"]
    #[inline(always)]
    pub const fn usbctrl(self) -> crate::common::Reg<regs::Usbctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB OTG Observe"]
    #[inline(always)]
    pub const fn observe(self) -> crate::common::Reg<regs::Observe, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "USB OTG Control"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "USB Transceiver Control 0"]
    #[inline(always)]
    pub const fn usbtrc0(self) -> crate::common::Reg<regs::Usbtrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Frame Adjust"]
    #[inline(always)]
    pub const fn usbfrmadjust(self) -> crate::common::Reg<regs::Usbfrmadjust, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn keep_alive_ctrl_rsvd(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn keep_alive_wkctrl_rsvd(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Miscellaneous Control"]
    #[inline(always)]
    pub const fn miscctrl(self) -> crate::common::Reg<regs::Miscctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction"]
    #[inline(always)]
    pub const fn stall_il_dis(self) -> crate::common::Reg<regs::StallIlDis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction"]
    #[inline(always)]
    pub const fn stall_ih_dis(self) -> crate::common::Reg<regs::StallIhDis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction"]
    #[inline(always)]
    pub const fn stall_ol_dis(self) -> crate::common::Reg<regs::StallOlDis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction"]
    #[inline(always)]
    pub const fn stall_oh_dis(self) -> crate::common::Reg<regs::StallOhDis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "USB Clock Recovery Control"]
    #[inline(always)]
    pub const fn clk_recover_ctrl(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "FIRC Oscillator Enable"]
    #[inline(always)]
    pub const fn clk_recover_irc_en(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverIrcEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Clock Recovery Combined Interrupt Enable"]
    #[inline(always)]
    pub const fn clk_recover_int_en(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Clock Recovery Separated Interrupt Status"]
    #[inline(always)]
    pub const fn clk_recover_int_status(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverIntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
