#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: ENDPT."]
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
    #[doc = "Endpoint Control."]
    #[inline(always)]
    pub const fn endpt(self) -> crate::pac::common::Reg<Endpt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "USBFS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs {
    ptr: *mut u8,
}
unsafe impl Send for Usbfs {}
unsafe impl Sync for Usbfs {}
impl Usbfs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Peripheral ID."]
    #[inline(always)]
    pub const fn perid(self) -> crate::pac::common::Reg<Perid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peripheral ID Complement."]
    #[inline(always)]
    pub const fn idcomp(self) -> crate::pac::common::Reg<Idcomp, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Revision."]
    #[inline(always)]
    pub const fn rev(self) -> crate::pac::common::Reg<Rev, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Peripheral Additional Information."]
    #[inline(always)]
    pub const fn addinfo(self) -> crate::pac::common::Reg<Addinfo, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "OTG Interrupt Status."]
    #[inline(always)]
    pub const fn otgistat(self) -> crate::pac::common::Reg<Otgistat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "OTG Interrupt Control."]
    #[inline(always)]
    pub const fn otgicr(self) -> crate::pac::common::Reg<Otgicr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "OTG Status."]
    #[inline(always)]
    pub const fn otgstat(self) -> crate::pac::common::Reg<Otgstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "OTG Control."]
    #[inline(always)]
    pub const fn otgctl(self) -> crate::pac::common::Reg<Otgctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn istat(self) -> crate::pac::common::Reg<Istat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::pac::common::Reg<Inten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Error Interrupt Status."]
    #[inline(always)]
    pub const fn errstat(self) -> crate::pac::common::Reg<Errstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Error Interrupt Enable."]
    #[inline(always)]
    pub const fn erren(self) -> crate::pac::common::Reg<Erren, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn stat(self) -> crate::pac::common::Reg<Stat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctl(self) -> crate::pac::common::Reg<Ctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Address."]
    #[inline(always)]
    pub const fn addr(self) -> crate::pac::common::Reg<Addr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "BDT Page 1."]
    #[inline(always)]
    pub const fn bdtpage1(self) -> crate::pac::common::Reg<Bdtpage1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Frame Number Register Low."]
    #[inline(always)]
    pub const fn frmnuml(self) -> crate::pac::common::Reg<Frmnuml, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Frame Number Register High."]
    #[inline(always)]
    pub const fn frmnumh(self) -> crate::pac::common::Reg<Frmnumh, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Token."]
    #[inline(always)]
    pub const fn token(self) -> crate::pac::common::Reg<Token, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "SOF Threshold."]
    #[inline(always)]
    pub const fn softhld(self) -> crate::pac::common::Reg<Softhld, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "BDT Page 2."]
    #[inline(always)]
    pub const fn bdtpage2(self) -> crate::pac::common::Reg<Bdtpage2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "BDT Page 3."]
    #[inline(always)]
    pub const fn bdtpage3(self) -> crate::pac::common::Reg<Bdtpage3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Array of registers: ENDPT."]
    #[inline(always)]
    pub const fn endpoint(self, n: usize) -> Endpoint {
        assert!(n < 16usize);
        unsafe { Endpoint::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "USB Control."]
    #[inline(always)]
    pub const fn usbctrl(self) -> crate::pac::common::Reg<Usbctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB OTG Observe."]
    #[inline(always)]
    pub const fn observe(self) -> crate::pac::common::Reg<Observe, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "USB OTG Control."]
    #[inline(always)]
    pub const fn control(self) -> crate::pac::common::Reg<Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "USB Transceiver Control 0."]
    #[inline(always)]
    pub const fn usbtrc0(self) -> crate::pac::common::Reg<Usbtrc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Frame Adjust."]
    #[inline(always)]
    pub const fn usbfrmadjust(
        self,
    ) -> crate::pac::common::Reg<Usbfrmadjust, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Keep Alive Mode Control."]
    #[inline(always)]
    pub const fn keep_alive_ctrl(
        self,
    ) -> crate::pac::common::Reg<KeepAliveCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Keep Alive Mode Wakeup Control."]
    #[inline(always)]
    pub const fn keep_alive_wkctrl(
        self,
    ) -> crate::pac::common::Reg<KeepAliveWkctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn miscctrl(self) -> crate::pac::common::Reg<Miscctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction."]
    #[inline(always)]
    pub const fn stall_il_dis(self) -> crate::pac::common::Reg<StallIlDis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction."]
    #[inline(always)]
    pub const fn stall_ih_dis(self) -> crate::pac::common::Reg<StallIhDis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction."]
    #[inline(always)]
    pub const fn stall_ol_dis(self) -> crate::pac::common::Reg<StallOlDis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction."]
    #[inline(always)]
    pub const fn stall_oh_dis(self) -> crate::pac::common::Reg<StallOhDis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "USB Clock Recovery Control."]
    #[inline(always)]
    pub const fn clk_recover_ctrl(
        self,
    ) -> crate::pac::common::Reg<ClkRecoverCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "FIRC Oscillator Enable."]
    #[inline(always)]
    pub const fn clk_recover_irc_en(
        self,
    ) -> crate::pac::common::Reg<ClkRecoverIrcEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Clock Recovery Combined Interrupt Enable."]
    #[inline(always)]
    pub const fn clk_recover_int_en(
        self,
    ) -> crate::pac::common::Reg<ClkRecoverIntEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Clock Recovery Separated Interrupt Status."]
    #[inline(always)]
    pub const fn clk_recover_int_status(
        self,
    ) -> crate::pac::common::Reg<ClkRecoverIntStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
}
#[doc = "Peripheral Additional Information."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addinfo(pub u8);
impl Addinfo {
    #[doc = "Host Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iehost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Mode Enable."]
    #[inline(always)]
    pub const fn set_iehost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for Addinfo {
    #[inline(always)]
    fn default() -> Addinfo {
        Addinfo(0)
    }
}
impl core::fmt::Debug for Addinfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addinfo")
            .field("iehost", &self.iehost())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addinfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Addinfo {{ iehost: {=bool:?} }}", self.iehost())
    }
}
#[doc = "Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr(pub u8);
impl Addr {
    #[doc = "USB Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USB Address."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "Low Speed Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lsen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low Speed Enable."]
    #[inline(always)]
    pub const fn set_lsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        Addr(0)
    }
}
impl core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addr")
            .field("addr", &self.addr())
            .field("lsen", &self.lsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addr {{ addr: {=u8:?}, lsen: {=bool:?} }}",
            self.addr(),
            self.lsen()
        )
    }
}
#[doc = "BDT Page 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtpage1(pub u8);
impl Bdtpage1 {
    #[doc = "BDT Base Address."]
    #[must_use]
    #[inline(always)]
    pub const fn bdtba(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "BDT Base Address."]
    #[inline(always)]
    pub const fn set_bdtba(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Bdtpage1 {
    #[inline(always)]
    fn default() -> Bdtpage1 {
        Bdtpage1(0)
    }
}
impl core::fmt::Debug for Bdtpage1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtpage1")
            .field("bdtba", &self.bdtba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtpage1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bdtpage1 {{ bdtba: {=u8:?} }}", self.bdtba())
    }
}
#[doc = "BDT Page 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtpage2(pub u8);
impl Bdtpage2 {
    #[doc = "BDT Base Address."]
    #[must_use]
    #[inline(always)]
    pub const fn bdtba(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BDT Base Address."]
    #[inline(always)]
    pub const fn set_bdtba(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bdtpage2 {
    #[inline(always)]
    fn default() -> Bdtpage2 {
        Bdtpage2(0)
    }
}
impl core::fmt::Debug for Bdtpage2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtpage2")
            .field("bdtba", &self.bdtba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtpage2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bdtpage2 {{ bdtba: {=u8:?} }}", self.bdtba())
    }
}
#[doc = "BDT Page 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtpage3(pub u8);
impl Bdtpage3 {
    #[doc = "BDT Base Address."]
    #[must_use]
    #[inline(always)]
    pub const fn bdtba(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BDT Base Address."]
    #[inline(always)]
    pub const fn set_bdtba(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bdtpage3 {
    #[inline(always)]
    fn default() -> Bdtpage3 {
        Bdtpage3(0)
    }
}
impl core::fmt::Debug for Bdtpage3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtpage3")
            .field("bdtba", &self.bdtba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtpage3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bdtpage3 {{ bdtba: {=u8:?} }}", self.bdtba())
    }
}
#[doc = "USB Clock Recovery Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverCtrl(pub u8);
impl ClkRecoverCtrl {
    #[doc = "Selects the source for the initial FIRC trim fine value used after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_init_val_sel(&self) -> TrimInitValSel {
        let val = (self.0 >> 3usize) & 0x01;
        TrimInitValSel::from_bits(val as u8)
    }
    #[doc = "Selects the source for the initial FIRC trim fine value used after a reset."]
    #[inline(always)]
    pub const fn set_trim_init_val_sel(&mut self, val: TrimInitValSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Restart from IFR Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn restart_ifrtrim_en(&self) -> RestartIfrtrimEn {
        let val = (self.0 >> 5usize) & 0x01;
        RestartIfrtrimEn::from_bits(val as u8)
    }
    #[doc = "Restart from IFR Trim Value."]
    #[inline(always)]
    pub const fn set_restart_ifrtrim_en(&mut self, val: RestartIfrtrimEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reset or Resume to Rough Phase Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reset_resume_rough_en(&self) -> ResetResumeRoughEn {
        let val = (self.0 >> 6usize) & 0x01;
        ResetResumeRoughEn::from_bits(val as u8)
    }
    #[doc = "Reset or Resume to Rough Phase Enable."]
    #[inline(always)]
    pub const fn set_reset_resume_rough_en(&mut self, val: ResetResumeRoughEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Crystal-Less USB Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_recover_en(&self) -> ClockRecoverEn {
        let val = (self.0 >> 7usize) & 0x01;
        ClockRecoverEn::from_bits(val as u8)
    }
    #[doc = "Crystal-Less USB Enable."]
    #[inline(always)]
    pub const fn set_clock_recover_en(&mut self, val: ClockRecoverEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for ClkRecoverCtrl {
    #[inline(always)]
    fn default() -> ClkRecoverCtrl {
        ClkRecoverCtrl(0)
    }
}
impl core::fmt::Debug for ClkRecoverCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverCtrl")
            .field("trim_init_val_sel", &self.trim_init_val_sel())
            .field("restart_ifrtrim_en", &self.restart_ifrtrim_en())
            .field("reset_resume_rough_en", &self.reset_resume_rough_en())
            .field("clock_recover_en", &self.clock_recover_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverCtrl {{ trim_init_val_sel: {:?}, restart_ifrtrim_en: {:?}, reset_resume_rough_en: {:?}, clock_recover_en: {:?} }}",
            self.trim_init_val_sel(),
            self.restart_ifrtrim_en(),
            self.reset_resume_rough_en(),
            self.clock_recover_en()
        )
    }
}
#[doc = "Clock Recovery Combined Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIntEn(pub u8);
impl ClkRecoverIntEn {
    #[doc = "Overflow error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ovf_error_en(&self) -> OvfErrorEn {
        let val = (self.0 >> 4usize) & 0x01;
        OvfErrorEn::from_bits(val as u8)
    }
    #[doc = "Overflow error interrupt enable."]
    #[inline(always)]
    pub const fn set_ovf_error_en(&mut self, val: OvfErrorEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for ClkRecoverIntEn {
    #[inline(always)]
    fn default() -> ClkRecoverIntEn {
        ClkRecoverIntEn(0)
    }
}
impl core::fmt::Debug for ClkRecoverIntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIntEn")
            .field("ovf_error_en", &self.ovf_error_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverIntEn {{ ovf_error_en: {:?} }}",
            self.ovf_error_en()
        )
    }
}
#[doc = "Clock Recovery Separated Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIntStatus(pub u8);
impl ClkRecoverIntStatus {
    #[doc = "Overflow Error Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ovf_error(&self) -> OvfError {
        let val = (self.0 >> 4usize) & 0x01;
        OvfError::from_bits(val as u8)
    }
    #[doc = "Overflow Error Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_ovf_error(&mut self, val: OvfError) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for ClkRecoverIntStatus {
    #[inline(always)]
    fn default() -> ClkRecoverIntStatus {
        ClkRecoverIntStatus(0)
    }
}
impl core::fmt::Debug for ClkRecoverIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIntStatus")
            .field("ovf_error", &self.ovf_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverIntStatus {{ ovf_error: {:?} }}",
            self.ovf_error()
        )
    }
}
#[doc = "FIRC Oscillator Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIrcEn(pub u8);
impl ClkRecoverIrcEn {
    #[doc = "Fast IRC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn irc_en(&self) -> IrcEn {
        let val = (self.0 >> 1usize) & 0x01;
        IrcEn::from_bits(val as u8)
    }
    #[doc = "Fast IRC enable."]
    #[inline(always)]
    pub const fn set_irc_en(&mut self, val: IrcEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
}
impl Default for ClkRecoverIrcEn {
    #[inline(always)]
    fn default() -> ClkRecoverIrcEn {
        ClkRecoverIrcEn(0)
    }
}
impl core::fmt::Debug for ClkRecoverIrcEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIrcEn")
            .field("irc_en", &self.irc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIrcEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ClkRecoverIrcEn {{ irc_en: {:?} }}", self.irc_en())
    }
}
#[doc = "USB OTG Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u8);
impl Control {
    #[doc = "VBUS Monitoring Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Monitoring Source Select."]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBUS Session Valid status."]
    #[must_use]
    #[inline(always)]
    pub const fn sess_vld(&self) -> SessVld {
        let val = (self.0 >> 1usize) & 0x01;
        SessVld::from_bits(val as u8)
    }
    #[doc = "VBUS Session Valid status."]
    #[inline(always)]
    pub const fn set_sess_vld(&mut self, val: SessVld) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup in Non-OTG Device Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dppullupnonotg(&self) -> Dppullupnonotg {
        let val = (self.0 >> 4usize) & 0x01;
        Dppullupnonotg::from_bits(val as u8)
    }
    #[doc = "DP Pullup in Non-OTG Device Mode."]
    #[inline(always)]
    pub const fn set_dppullupnonotg(&mut self, val: Dppullupnonotg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("sess_vld", &self.sess_vld())
            .field("dppullupnonotg", &self.dppullupnonotg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Control {{ vbus_source_sel: {=bool:?}, sess_vld: {:?}, dppullupnonotg: {:?} }}",
            self.vbus_source_sel(),
            self.sess_vld(),
            self.dppullupnonotg()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u8);
impl Ctl {
    #[doc = "USB Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usbensofen(&self) -> Usbensofen {
        let val = (self.0 >> 0usize) & 0x01;
        Usbensofen::from_bits(val as u8)
    }
    #[doc = "USB Enable."]
    #[inline(always)]
    pub const fn set_usbensofen(&mut self, val: Usbensofen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Odd Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn oddrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Odd Reset."]
    #[inline(always)]
    pub const fn set_oddrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Resume."]
    #[must_use]
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Resume."]
    #[inline(always)]
    pub const fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Host Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hostmodeen(&self) -> Hostmodeen {
        let val = (self.0 >> 3usize) & 0x01;
        Hostmodeen::from_bits(val as u8)
    }
    #[doc = "Host Mode Enable."]
    #[inline(always)]
    pub const fn set_hostmodeen(&mut self, val: Hostmodeen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Reset Signaling Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Signaling Enable."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "TXD Suspend And Token Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn txsuspendtokenbusy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TXD Suspend And Token Busy."]
    #[inline(always)]
    pub const fn set_txsuspendtokenbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Live USB Single-Ended Zero signal."]
    #[must_use]
    #[inline(always)]
    pub const fn se0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Live USB Single-Ended Zero signal."]
    #[inline(always)]
    pub const fn set_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Live USB Differential Receiver JSTATE Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn jstate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Live USB Differential Receiver JSTATE Signal."]
    #[inline(always)]
    pub const fn set_jstate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
impl core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl")
            .field("usbensofen", &self.usbensofen())
            .field("oddrst", &self.oddrst())
            .field("resume", &self.resume())
            .field("hostmodeen", &self.hostmodeen())
            .field("reset", &self.reset())
            .field("txsuspendtokenbusy", &self.txsuspendtokenbusy())
            .field("se0", &self.se0())
            .field("jstate", &self.jstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl {{ usbensofen: {:?}, oddrst: {=bool:?}, resume: {=bool:?}, hostmodeen: {:?}, reset: {=bool:?}, txsuspendtokenbusy: {=bool:?}, se0: {=bool:?}, jstate: {=bool:?} }}",
            self.usbensofen(),
            self.oddrst(),
            self.resume(),
            self.hostmodeen(),
            self.reset(),
            self.txsuspendtokenbusy(),
            self.se0(),
            self.jstate()
        )
    }
}
#[doc = "Endpoint Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endpt(pub u8);
impl Endpt {
    #[doc = "Endpoint Handshaking Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ephshk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Handshaking Enable."]
    #[inline(always)]
    pub const fn set_ephshk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Stalled."]
    #[must_use]
    #[inline(always)]
    pub const fn epstall(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Stalled."]
    #[inline(always)]
    pub const fn set_epstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Endpoint for TX transfers enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eptxen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint for TX transfers enable."]
    #[inline(always)]
    pub const fn set_eptxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Endpoint for RX transfers enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eprxen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint for RX transfers enable."]
    #[inline(always)]
    pub const fn set_eprxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Control Transfer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn epctldis(&self) -> Epctldis {
        let val = (self.0 >> 4usize) & 0x01;
        Epctldis::from_bits(val as u8)
    }
    #[doc = "Control Transfer Disable."]
    #[inline(always)]
    pub const fn set_epctldis(&mut self, val: Epctldis) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Retry Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn retrydis(&self) -> Retrydis {
        let val = (self.0 >> 6usize) & 0x01;
        Retrydis::from_bits(val as u8)
    }
    #[doc = "Retry Disable."]
    #[inline(always)]
    pub const fn set_retrydis(&mut self, val: Retrydis) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Host Without A Hub."]
    #[must_use]
    #[inline(always)]
    pub const fn hostwohub(&self) -> Hostwohub {
        let val = (self.0 >> 7usize) & 0x01;
        Hostwohub::from_bits(val as u8)
    }
    #[doc = "Host Without A Hub."]
    #[inline(always)]
    pub const fn set_hostwohub(&mut self, val: Hostwohub) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Endpt {
    #[inline(always)]
    fn default() -> Endpt {
        Endpt(0)
    }
}
impl core::fmt::Debug for Endpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endpt")
            .field("ephshk", &self.ephshk())
            .field("epstall", &self.epstall())
            .field("eptxen", &self.eptxen())
            .field("eprxen", &self.eprxen())
            .field("epctldis", &self.epctldis())
            .field("retrydis", &self.retrydis())
            .field("hostwohub", &self.hostwohub())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endpt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endpt {{ ephshk: {=bool:?}, epstall: {=bool:?}, eptxen: {=bool:?}, eprxen: {=bool:?}, epctldis: {:?}, retrydis: {:?}, hostwohub: {:?} }}",
            self.ephshk(),
            self.epstall(),
            self.eptxen(),
            self.eprxen(),
            self.epctldis(),
            self.retrydis(),
            self.hostwohub()
        )
    }
}
#[doc = "Error Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erren(pub u8);
impl Erren {
    #[doc = "PIDERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn piderren(&self) -> Piderren {
        let val = (self.0 >> 0usize) & 0x01;
        Piderren::from_bits(val as u8)
    }
    #[doc = "PIDERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_piderren(&mut self, val: Piderren) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "CRC5/EOF Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crc5eofen(&self) -> Crc5eofen {
        let val = (self.0 >> 1usize) & 0x01;
        Crc5eofen::from_bits(val as u8)
    }
    #[doc = "CRC5/EOF Interrupt Enable."]
    #[inline(always)]
    pub const fn set_crc5eofen(&mut self, val: Crc5eofen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "CRC16 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crc16en(&self) -> Crc16en {
        let val = (self.0 >> 2usize) & 0x01;
        Crc16en::from_bits(val as u8)
    }
    #[doc = "CRC16 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_crc16en(&mut self, val: Crc16en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "DFN8 (Data Field Not Integer Number of Bytes) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dfn8en(&self) -> Dfn8en {
        let val = (self.0 >> 3usize) & 0x01;
        Dfn8en::from_bits(val as u8)
    }
    #[doc = "DFN8 (Data Field Not Integer Number of Bytes) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dfn8en(&mut self, val: Dfn8en) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "BTOERR (Bus Timeout Error) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn btoerren(&self) -> Btoerren {
        let val = (self.0 >> 4usize) & 0x01;
        Btoerren::from_bits(val as u8)
    }
    #[doc = "BTOERR (Bus Timeout Error) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_btoerren(&mut self, val: Btoerren) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "DMAERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerren(&self) -> Dmaerren {
        let val = (self.0 >> 5usize) & 0x01;
        Dmaerren::from_bits(val as u8)
    }
    #[doc = "DMAERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dmaerren(&mut self, val: Dmaerren) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "OWNERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ownerren(&self) -> Ownerren {
        let val = (self.0 >> 6usize) & 0x01;
        Ownerren::from_bits(val as u8)
    }
    #[doc = "OWNERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ownerren(&mut self, val: Ownerren) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "BTSERR (Bit Stuff Error) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn btserren(&self) -> Btserren {
        let val = (self.0 >> 7usize) & 0x01;
        Btserren::from_bits(val as u8)
    }
    #[doc = "BTSERR (Bit Stuff Error) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_btserren(&mut self, val: Btserren) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Erren {
    #[inline(always)]
    fn default() -> Erren {
        Erren(0)
    }
}
impl core::fmt::Debug for Erren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erren")
            .field("piderren", &self.piderren())
            .field("crc5eofen", &self.crc5eofen())
            .field("crc16en", &self.crc16en())
            .field("dfn8en", &self.dfn8en())
            .field("btoerren", &self.btoerren())
            .field("dmaerren", &self.dmaerren())
            .field("ownerren", &self.ownerren())
            .field("btserren", &self.btserren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erren {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erren {{ piderren: {:?}, crc5eofen: {:?}, crc16en: {:?}, dfn8en: {:?}, btoerren: {:?}, dmaerren: {:?}, ownerren: {:?}, btserren: {:?} }}",
            self.piderren(),
            self.crc5eofen(),
            self.crc16en(),
            self.dfn8en(),
            self.btoerren(),
            self.dmaerren(),
            self.ownerren(),
            self.btserren()
        )
    }
}
#[doc = "Error Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errstat(pub u8);
impl Errstat {
    #[doc = "PID Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn piderr(&self) -> Piderr {
        let val = (self.0 >> 0usize) & 0x01;
        Piderr::from_bits(val as u8)
    }
    #[doc = "PID Error Flag."]
    #[inline(always)]
    pub const fn set_piderr(&mut self, val: Piderr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "CRC5 Error or End of Frame Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn crc5eof(&self) -> Crc5eof {
        let val = (self.0 >> 1usize) & 0x01;
        Crc5eof::from_bits(val as u8)
    }
    #[doc = "CRC5 Error or End of Frame Error Flag."]
    #[inline(always)]
    pub const fn set_crc5eof(&mut self, val: Crc5eof) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "CRC16 Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn crc16(&self) -> Crc16 {
        let val = (self.0 >> 2usize) & 0x01;
        Crc16::from_bits(val as u8)
    }
    #[doc = "CRC16 Error Flag."]
    #[inline(always)]
    pub const fn set_crc16(&mut self, val: Crc16) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Data Field Not 8 Bits Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dfn8(&self) -> Dfn8 {
        let val = (self.0 >> 3usize) & 0x01;
        Dfn8::from_bits(val as u8)
    }
    #[doc = "Data Field Not 8 Bits Flag."]
    #[inline(always)]
    pub const fn set_dfn8(&mut self, val: Dfn8) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Bus Turnaround Timeout Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn btoerr(&self) -> Btoerr {
        let val = (self.0 >> 4usize) & 0x01;
        Btoerr::from_bits(val as u8)
    }
    #[doc = "Bus Turnaround Timeout Error Flag."]
    #[inline(always)]
    pub const fn set_btoerr(&mut self, val: Btoerr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "DMA Access Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerr(&self) -> Dmaerr {
        let val = (self.0 >> 5usize) & 0x01;
        Dmaerr::from_bits(val as u8)
    }
    #[doc = "DMA Access Error Flag."]
    #[inline(always)]
    pub const fn set_dmaerr(&mut self, val: Dmaerr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "BD Unavailable Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ownerr(&self) -> Ownerr {
        let val = (self.0 >> 6usize) & 0x01;
        Ownerr::from_bits(val as u8)
    }
    #[doc = "BD Unavailable Error Flag."]
    #[inline(always)]
    pub const fn set_ownerr(&mut self, val: Ownerr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Bit Stuff Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn btserr(&self) -> Btserr {
        let val = (self.0 >> 7usize) & 0x01;
        Btserr::from_bits(val as u8)
    }
    #[doc = "Bit Stuff Error Flag."]
    #[inline(always)]
    pub const fn set_btserr(&mut self, val: Btserr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Errstat {
    #[inline(always)]
    fn default() -> Errstat {
        Errstat(0)
    }
}
impl core::fmt::Debug for Errstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Errstat")
            .field("piderr", &self.piderr())
            .field("crc5eof", &self.crc5eof())
            .field("crc16", &self.crc16())
            .field("dfn8", &self.dfn8())
            .field("btoerr", &self.btoerr())
            .field("dmaerr", &self.dmaerr())
            .field("ownerr", &self.ownerr())
            .field("btserr", &self.btserr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Errstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Errstat {{ piderr: {:?}, crc5eof: {:?}, crc16: {:?}, dfn8: {:?}, btoerr: {:?}, dmaerr: {:?}, ownerr: {:?}, btserr: {:?} }}",
            self.piderr(),
            self.crc5eof(),
            self.crc16(),
            self.dfn8(),
            self.btoerr(),
            self.dmaerr(),
            self.ownerr(),
            self.btserr()
        )
    }
}
#[doc = "Frame Number Register High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frmnumh(pub u8);
impl Frmnumh {
    #[doc = "Frame Number, Bits 8-10."]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Frame Number, Bits 8-10."]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
}
impl Default for Frmnumh {
    #[inline(always)]
    fn default() -> Frmnumh {
        Frmnumh(0)
    }
}
impl core::fmt::Debug for Frmnumh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frmnumh").field("frm", &self.frm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frmnumh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frmnumh {{ frm: {=u8:?} }}", self.frm())
    }
}
#[doc = "Frame Number Register Low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frmnuml(pub u8);
impl Frmnuml {
    #[doc = "Frame Number, Bits 0-7."]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frame Number, Bits 0-7."]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Frmnuml {
    #[inline(always)]
    fn default() -> Frmnuml {
        Frmnuml(0)
    }
}
impl core::fmt::Debug for Frmnuml {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frmnuml").field("frm", &self.frm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frmnuml {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frmnuml {{ frm: {=u8:?} }}", self.frm())
    }
}
#[doc = "Peripheral ID Complement."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idcomp(pub u8);
impl Idcomp {
    #[doc = "Negative Peripheral ID."]
    #[must_use]
    #[inline(always)]
    pub const fn nid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Negative Peripheral ID."]
    #[inline(always)]
    pub const fn set_nid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
}
impl Default for Idcomp {
    #[inline(always)]
    fn default() -> Idcomp {
        Idcomp(0)
    }
}
impl core::fmt::Debug for Idcomp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idcomp").field("nid", &self.nid()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idcomp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Idcomp {{ nid: {=u8:?} }}", self.nid())
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u8);
impl Inten {
    #[doc = "USBRST Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usbrsten(&self) -> Usbrsten {
        let val = (self.0 >> 0usize) & 0x01;
        Usbrsten::from_bits(val as u8)
    }
    #[doc = "USBRST Interrupt Enable."]
    #[inline(always)]
    pub const fn set_usbrsten(&mut self, val: Usbrsten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "ERROR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erroren(&self) -> Erroren {
        let val = (self.0 >> 1usize) & 0x01;
        Erroren::from_bits(val as u8)
    }
    #[doc = "ERROR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erroren(&mut self, val: Erroren) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "SOFTOK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn softoken(&self) -> Softoken {
        let val = (self.0 >> 2usize) & 0x01;
        Softoken::from_bits(val as u8)
    }
    #[doc = "SOFTOK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_softoken(&mut self, val: Softoken) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "TOKDNE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tokdneen(&self) -> Tokdneen {
        let val = (self.0 >> 3usize) & 0x01;
        Tokdneen::from_bits(val as u8)
    }
    #[doc = "TOKDNE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tokdneen(&mut self, val: Tokdneen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "SLEEP Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sleepen(&self) -> Sleepen {
        let val = (self.0 >> 4usize) & 0x01;
        Sleepen::from_bits(val as u8)
    }
    #[doc = "SLEEP Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sleepen(&mut self, val: Sleepen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "RESUME Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn resumeen(&self) -> Resumeen {
        let val = (self.0 >> 5usize) & 0x01;
        Resumeen::from_bits(val as u8)
    }
    #[doc = "RESUME Interrupt Enable."]
    #[inline(always)]
    pub const fn set_resumeen(&mut self, val: Resumeen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "ATTACH Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn attachen(&self) -> Attachen {
        let val = (self.0 >> 6usize) & 0x01;
        Attachen::from_bits(val as u8)
    }
    #[doc = "ATTACH Interrupt Enable."]
    #[inline(always)]
    pub const fn set_attachen(&mut self, val: Attachen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "STALL Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stallen(&self) -> Stallen {
        let val = (self.0 >> 7usize) & 0x01;
        Stallen::from_bits(val as u8)
    }
    #[doc = "STALL Interrupt Enable."]
    #[inline(always)]
    pub const fn set_stallen(&mut self, val: Stallen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("usbrsten", &self.usbrsten())
            .field("erroren", &self.erroren())
            .field("softoken", &self.softoken())
            .field("tokdneen", &self.tokdneen())
            .field("sleepen", &self.sleepen())
            .field("resumeen", &self.resumeen())
            .field("attachen", &self.attachen())
            .field("stallen", &self.stallen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ usbrsten: {:?}, erroren: {:?}, softoken: {:?}, tokdneen: {:?}, sleepen: {:?}, resumeen: {:?}, attachen: {:?}, stallen: {:?} }}",
            self.usbrsten(),
            self.erroren(),
            self.softoken(),
            self.tokdneen(),
            self.sleepen(),
            self.resumeen(),
            self.attachen(),
            self.stallen()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Istat(pub u8);
impl Istat {
    #[doc = "USB Reset Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn usbrst(&self) -> Usbrst {
        let val = (self.0 >> 0usize) & 0x01;
        Usbrst::from_bits(val as u8)
    }
    #[doc = "USB Reset Flag."]
    #[inline(always)]
    pub const fn set_usbrst(&mut self, val: Usbrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> Error {
        let val = (self.0 >> 1usize) & 0x01;
        Error::from_bits(val as u8)
    }
    #[doc = "Error Flag."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: Error) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Start Of Frame (SOF) Token Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn softok(&self) -> Softok {
        let val = (self.0 >> 2usize) & 0x01;
        Softok::from_bits(val as u8)
    }
    #[doc = "Start Of Frame (SOF) Token Flag."]
    #[inline(always)]
    pub const fn set_softok(&mut self, val: Softok) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Current Token Processing Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tokdne(&self) -> Tokdne {
        let val = (self.0 >> 3usize) & 0x01;
        Tokdne::from_bits(val as u8)
    }
    #[doc = "Current Token Processing Flag."]
    #[inline(always)]
    pub const fn set_tokdne(&mut self, val: Tokdne) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Sleep Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sleep(&self) -> Sleep {
        let val = (self.0 >> 4usize) & 0x01;
        Sleep::from_bits(val as u8)
    }
    #[doc = "Sleep Flag."]
    #[inline(always)]
    pub const fn set_sleep(&mut self, val: Sleep) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Resume Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn resume(&self) -> Resume {
        let val = (self.0 >> 5usize) & 0x01;
        Resume::from_bits(val as u8)
    }
    #[doc = "Resume Flag."]
    #[inline(always)]
    pub const fn set_resume(&mut self, val: Resume) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Attach Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn attach(&self) -> Attach {
        let val = (self.0 >> 6usize) & 0x01;
        Attach::from_bits(val as u8)
    }
    #[doc = "Attach Interrupt Flag."]
    #[inline(always)]
    pub const fn set_attach(&mut self, val: Attach) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Stall Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> Stall {
        let val = (self.0 >> 7usize) & 0x01;
        Stall::from_bits(val as u8)
    }
    #[doc = "Stall Interrupt Flag."]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: Stall) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Istat {
    #[inline(always)]
    fn default() -> Istat {
        Istat(0)
    }
}
impl core::fmt::Debug for Istat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Istat")
            .field("usbrst", &self.usbrst())
            .field("error", &self.error())
            .field("softok", &self.softok())
            .field("tokdne", &self.tokdne())
            .field("sleep", &self.sleep())
            .field("resume", &self.resume())
            .field("attach", &self.attach())
            .field("stall", &self.stall())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Istat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Istat {{ usbrst: {:?}, error: {:?}, softok: {:?}, tokdne: {:?}, sleep: {:?}, resume: {:?}, attach: {:?}, stall: {:?} }}",
            self.usbrst(),
            self.error(),
            self.softok(),
            self.tokdne(),
            self.sleep(),
            self.resume(),
            self.attach(),
            self.stall()
        )
    }
}
#[doc = "Keep Alive Mode Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeepAliveCtrl(pub u8);
impl KeepAliveCtrl {
    #[doc = "Keep Alive Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn keep_alive_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Keep Alive Mode Enable."]
    #[inline(always)]
    pub const fn set_keep_alive_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "OWN Bit Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn own_overrd_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OWN Bit Override Enable."]
    #[inline(always)]
    pub const fn set_own_overrd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Stop Acknowledge Delay Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_ack_dly_en(&self) -> StopAckDlyEn {
        let val = (self.0 >> 2usize) & 0x01;
        StopAckDlyEn::from_bits(val as u8)
    }
    #[doc = "Stop Acknowledge Delay Enable."]
    #[inline(always)]
    pub const fn set_stop_ack_dly_en(&mut self, val: StopAckDlyEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Wakeup Request Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_req_en(&self) -> WakeReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        WakeReqEn::from_bits(val as u8)
    }
    #[doc = "Wakeup Request Enable."]
    #[inline(always)]
    pub const fn set_wake_req_en(&mut self, val: WakeReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Wakeup Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wake_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Keep Alive Status."]
    #[must_use]
    #[inline(always)]
    pub const fn keep_alive_sts(&self) -> KeepAliveSts {
        let val = (self.0 >> 6usize) & 0x01;
        KeepAliveSts::from_bits(val as u8)
    }
    #[doc = "Keep Alive Status."]
    #[inline(always)]
    pub const fn set_keep_alive_sts(&mut self, val: KeepAliveSts) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Wakeup Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_int_sts(&self) -> WakeIntSts {
        let val = (self.0 >> 7usize) & 0x01;
        WakeIntSts::from_bits(val as u8)
    }
    #[doc = "Wakeup Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_wake_int_sts(&mut self, val: WakeIntSts) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for KeepAliveCtrl {
    #[inline(always)]
    fn default() -> KeepAliveCtrl {
        KeepAliveCtrl(0)
    }
}
impl core::fmt::Debug for KeepAliveCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeepAliveCtrl")
            .field("keep_alive_en", &self.keep_alive_en())
            .field("own_overrd_en", &self.own_overrd_en())
            .field("stop_ack_dly_en", &self.stop_ack_dly_en())
            .field("wake_req_en", &self.wake_req_en())
            .field("wake_int_en", &self.wake_int_en())
            .field("keep_alive_sts", &self.keep_alive_sts())
            .field("wake_int_sts", &self.wake_int_sts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeepAliveCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KeepAliveCtrl {{ keep_alive_en: {=bool:?}, own_overrd_en: {=bool:?}, stop_ack_dly_en: {:?}, wake_req_en: {:?}, wake_int_en: {=bool:?}, keep_alive_sts: {:?}, wake_int_sts: {:?} }}",
            self.keep_alive_en(),
            self.own_overrd_en(),
            self.stop_ack_dly_en(),
            self.wake_req_en(),
            self.wake_int_en(),
            self.keep_alive_sts(),
            self.wake_int_sts()
        )
    }
}
#[doc = "Keep Alive Mode Wakeup Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeepAliveWkctrl(pub u8);
impl KeepAliveWkctrl {
    #[doc = "Token PID for the wakeup request."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_on_this(&self) -> WakeOnThis {
        let val = (self.0 >> 0usize) & 0x0f;
        WakeOnThis::from_bits(val as u8)
    }
    #[doc = "Token PID for the wakeup request."]
    #[inline(always)]
    pub const fn set_wake_on_this(&mut self, val: WakeOnThis) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
    #[doc = "Endpoint address for the wakeup request."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_endpt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint address for the wakeup request."]
    #[inline(always)]
    pub const fn set_wake_endpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for KeepAliveWkctrl {
    #[inline(always)]
    fn default() -> KeepAliveWkctrl {
        KeepAliveWkctrl(0)
    }
}
impl core::fmt::Debug for KeepAliveWkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeepAliveWkctrl")
            .field("wake_on_this", &self.wake_on_this())
            .field("wake_endpt", &self.wake_endpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeepAliveWkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KeepAliveWkctrl {{ wake_on_this: {:?}, wake_endpt: {=u8:?} }}",
            self.wake_on_this(),
            self.wake_endpt()
        )
    }
}
#[doc = "Miscellaneous Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miscctrl(pub u8);
impl Miscctrl {
    #[doc = "Dynamic SOF Threshold Compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sofdynthld(&self) -> Sofdynthld {
        let val = (self.0 >> 0usize) & 0x01;
        Sofdynthld::from_bits(val as u8)
    }
    #[doc = "Dynamic SOF Threshold Compare mode."]
    #[inline(always)]
    pub const fn set_sofdynthld(&mut self, val: Sofdynthld) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "SOF_TOK Interrupt Generation Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sofbusset(&self) -> Sofbusset {
        let val = (self.0 >> 1usize) & 0x01;
        Sofbusset::from_bits(val as u8)
    }
    #[doc = "SOF_TOK Interrupt Generation Mode Select."]
    #[inline(always)]
    pub const fn set_sofbusset(&mut self, val: Sofbusset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "OWN Error Detect for ISO IN and ISO OUT Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ownerrisodis(&self) -> Ownerrisodis {
        let val = (self.0 >> 2usize) & 0x01;
        Ownerrisodis::from_bits(val as u8)
    }
    #[doc = "OWN Error Detect for ISO IN and ISO OUT Disable."]
    #[inline(always)]
    pub const fn set_ownerrisodis(&mut self, val: Ownerrisodis) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "VREGIN Rising Edge Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vredg_en(&self) -> VredgEn {
        let val = (self.0 >> 3usize) & 0x01;
        VredgEn::from_bits(val as u8)
    }
    #[doc = "VREGIN Rising Edge Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vredg_en(&mut self, val: VredgEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "VREGIN Falling Edge Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vfedg_en(&self) -> VfedgEn {
        let val = (self.0 >> 4usize) & 0x01;
        VfedgEn::from_bits(val as u8)
    }
    #[doc = "VREGIN Falling Edge Interrupt Enable."]
    #[inline(always)]
    pub const fn set_vfedg_en(&mut self, val: VfedgEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "USB Peripheral Mode Stall Adjust Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stl_adj_en(&self) -> StlAdjEn {
        let val = (self.0 >> 7usize) & 0x01;
        StlAdjEn::from_bits(val as u8)
    }
    #[doc = "USB Peripheral Mode Stall Adjust Enable."]
    #[inline(always)]
    pub const fn set_stl_adj_en(&mut self, val: StlAdjEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Miscctrl {
    #[inline(always)]
    fn default() -> Miscctrl {
        Miscctrl(0)
    }
}
impl core::fmt::Debug for Miscctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Miscctrl")
            .field("sofdynthld", &self.sofdynthld())
            .field("sofbusset", &self.sofbusset())
            .field("ownerrisodis", &self.ownerrisodis())
            .field("vredg_en", &self.vredg_en())
            .field("vfedg_en", &self.vfedg_en())
            .field("stl_adj_en", &self.stl_adj_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Miscctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Miscctrl {{ sofdynthld: {:?}, sofbusset: {:?}, ownerrisodis: {:?}, vredg_en: {:?}, vfedg_en: {:?}, stl_adj_en: {:?} }}",
            self.sofdynthld(),
            self.sofbusset(),
            self.ownerrisodis(),
            self.vredg_en(),
            self.vfedg_en(),
            self.stl_adj_en()
        )
    }
}
#[doc = "USB OTG Observe."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Observe(pub u8);
impl Observe {
    #[doc = "D- Pulldown."]
    #[must_use]
    #[inline(always)]
    pub const fn dmpd(&self) -> Dmpd {
        let val = (self.0 >> 4usize) & 0x01;
        Dmpd::from_bits(val as u8)
    }
    #[doc = "D- Pulldown."]
    #[inline(always)]
    pub const fn set_dmpd(&mut self, val: Dmpd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "D+ Pulldown."]
    #[must_use]
    #[inline(always)]
    pub const fn dppd(&self) -> Dppd {
        let val = (self.0 >> 6usize) & 0x01;
        Dppd::from_bits(val as u8)
    }
    #[doc = "D+ Pulldown."]
    #[inline(always)]
    pub const fn set_dppd(&mut self, val: Dppd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "D+ Pullup."]
    #[must_use]
    #[inline(always)]
    pub const fn dppu(&self) -> Dppu {
        let val = (self.0 >> 7usize) & 0x01;
        Dppu::from_bits(val as u8)
    }
    #[doc = "D+ Pullup."]
    #[inline(always)]
    pub const fn set_dppu(&mut self, val: Dppu) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Observe {
    #[inline(always)]
    fn default() -> Observe {
        Observe(0)
    }
}
impl core::fmt::Debug for Observe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Observe")
            .field("dmpd", &self.dmpd())
            .field("dppd", &self.dppd())
            .field("dppu", &self.dppu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Observe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Observe {{ dmpd: {:?}, dppd: {:?}, dppu: {:?} }}",
            self.dmpd(),
            self.dppd(),
            self.dppu()
        )
    }
}
#[doc = "OTG Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgctl(pub u8);
impl Otgctl {
    #[doc = "On-The-Go Pullup and Pulldown Resistor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn otgen(&self) -> Otgen {
        let val = (self.0 >> 2usize) & 0x01;
        Otgen::from_bits(val as u8)
    }
    #[doc = "On-The-Go Pullup and Pulldown Resistor Enable."]
    #[inline(always)]
    pub const fn set_otgen(&mut self, val: Otgen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "D- Data Line Pulldown Resistor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmlow(&self) -> Dmlow {
        let val = (self.0 >> 4usize) & 0x01;
        Dmlow::from_bits(val as u8)
    }
    #[doc = "D- Data Line Pulldown Resistor Enable."]
    #[inline(always)]
    pub const fn set_dmlow(&mut self, val: Dmlow) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "D+ Data Line pulldown Resistor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dplow(&self) -> Dplow {
        let val = (self.0 >> 5usize) & 0x01;
        Dplow::from_bits(val as u8)
    }
    #[doc = "D+ Data Line pulldown Resistor Enable."]
    #[inline(always)]
    pub const fn set_dplow(&mut self, val: Dplow) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "D+ Data Line Pullup Resistor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dphigh(&self) -> Dphigh {
        let val = (self.0 >> 7usize) & 0x01;
        Dphigh::from_bits(val as u8)
    }
    #[doc = "D+ Data Line Pullup Resistor Enable."]
    #[inline(always)]
    pub const fn set_dphigh(&mut self, val: Dphigh) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Otgctl {
    #[inline(always)]
    fn default() -> Otgctl {
        Otgctl(0)
    }
}
impl core::fmt::Debug for Otgctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgctl")
            .field("otgen", &self.otgen())
            .field("dmlow", &self.dmlow())
            .field("dplow", &self.dplow())
            .field("dphigh", &self.dphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgctl {{ otgen: {:?}, dmlow: {:?}, dplow: {:?}, dphigh: {:?} }}",
            self.otgen(),
            self.dmlow(),
            self.dplow(),
            self.dphigh()
        )
    }
}
#[doc = "OTG Interrupt Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgicr(pub u8);
impl Otgicr {
    #[doc = "Line State Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn linestateen(&self) -> Linestateen {
        let val = (self.0 >> 5usize) & 0x01;
        Linestateen::from_bits(val as u8)
    }
    #[doc = "Line State Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_linestateen(&mut self, val: Linestateen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "1-Millisecond Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn onemsecen(&self) -> Onemsecen {
        let val = (self.0 >> 6usize) & 0x01;
        Onemsecen::from_bits(val as u8)
    }
    #[doc = "1-Millisecond Interrupt Enable."]
    #[inline(always)]
    pub const fn set_onemsecen(&mut self, val: Onemsecen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
}
impl Default for Otgicr {
    #[inline(always)]
    fn default() -> Otgicr {
        Otgicr(0)
    }
}
impl core::fmt::Debug for Otgicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgicr")
            .field("linestateen", &self.linestateen())
            .field("onemsecen", &self.onemsecen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgicr {{ linestateen: {:?}, onemsecen: {:?} }}",
            self.linestateen(),
            self.onemsecen()
        )
    }
}
#[doc = "OTG Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgistat(pub u8);
impl Otgistat {
    #[doc = "Line State Change Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn line_state_chg(&self) -> LineStateChg {
        let val = (self.0 >> 5usize) & 0x01;
        LineStateChg::from_bits(val as u8)
    }
    #[doc = "Line State Change Interrupt Flag."]
    #[inline(always)]
    pub const fn set_line_state_chg(&mut self, val: LineStateChg) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "One Millisecond Timer Timeout Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn onemsec(&self) -> Onemsec {
        let val = (self.0 >> 6usize) & 0x01;
        Onemsec::from_bits(val as u8)
    }
    #[doc = "One Millisecond Timer Timeout Flag."]
    #[inline(always)]
    pub const fn set_onemsec(&mut self, val: Onemsec) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
}
impl Default for Otgistat {
    #[inline(always)]
    fn default() -> Otgistat {
        Otgistat(0)
    }
}
impl core::fmt::Debug for Otgistat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgistat")
            .field("line_state_chg", &self.line_state_chg())
            .field("onemsec", &self.onemsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgistat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgistat {{ line_state_chg: {:?}, onemsec: {:?} }}",
            self.line_state_chg(),
            self.onemsec()
        )
    }
}
#[doc = "OTG Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgstat(pub u8);
impl Otgstat {
    #[doc = "Line State Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn linestatestable(&self) -> Linestatestable {
        let val = (self.0 >> 5usize) & 0x01;
        Linestatestable::from_bits(val as u8)
    }
    #[doc = "Line State Stable."]
    #[inline(always)]
    pub const fn set_linestatestable(&mut self, val: Linestatestable) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reserved for 1 ms count."]
    #[must_use]
    #[inline(always)]
    pub const fn onemsec(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for 1 ms count."]
    #[inline(always)]
    pub const fn set_onemsec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
}
impl Default for Otgstat {
    #[inline(always)]
    fn default() -> Otgstat {
        Otgstat(0)
    }
}
impl core::fmt::Debug for Otgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgstat")
            .field("linestatestable", &self.linestatestable())
            .field("onemsec", &self.onemsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgstat {{ linestatestable: {:?}, onemsec: {=bool:?} }}",
            self.linestatestable(),
            self.onemsec()
        )
    }
}
#[doc = "Peripheral ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perid(pub u8);
impl Perid {
    #[doc = "Peripheral Identification."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Peripheral Identification."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
}
impl Default for Perid {
    #[inline(always)]
    fn default() -> Perid {
        Perid(0)
    }
}
impl core::fmt::Debug for Perid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Perid").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Perid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Perid {{ id: {=u8:?} }}", self.id())
    }
}
#[doc = "Peripheral Revision."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rev(pub u8);
impl Rev {
    #[doc = "Revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision."]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Rev {
    #[inline(always)]
    fn default() -> Rev {
        Rev(0)
    }
}
impl core::fmt::Debug for Rev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rev").field("rev", &self.rev()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rev {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rev {{ rev: {=u8:?} }}", self.rev())
    }
}
#[doc = "SOF Threshold."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Softhld(pub u8);
impl Softhld {
    #[doc = "SOF Count Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SOF Count Threshold."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Softhld {
    #[inline(always)]
    fn default() -> Softhld {
        Softhld(0)
    }
}
impl core::fmt::Debug for Softhld {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Softhld").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Softhld {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Softhld {{ cnt: {=u8:?} }}", self.cnt())
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallIhDis(pub u8);
impl StallIhDis {
    #[doc = "Disable Endpoint IN Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis(&self, n: usize) -> StallIDisHi {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        StallIDisHi::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint IN Direction."]
    #[inline(always)]
    pub const fn set_stall_i_dis(&mut self, n: usize, val: StallIDisHi) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u8) & 0x01) << offs);
    }
}
impl Default for StallIhDis {
    #[inline(always)]
    fn default() -> StallIhDis {
        StallIhDis(0)
    }
}
impl core::fmt::Debug for StallIhDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallIhDis")
            .field("stall_i_dis[0]", &self.stall_i_dis(0usize))
            .field("stall_i_dis[1]", &self.stall_i_dis(1usize))
            .field("stall_i_dis[2]", &self.stall_i_dis(2usize))
            .field("stall_i_dis[3]", &self.stall_i_dis(3usize))
            .field("stall_i_dis[4]", &self.stall_i_dis(4usize))
            .field("stall_i_dis[5]", &self.stall_i_dis(5usize))
            .field("stall_i_dis[6]", &self.stall_i_dis(6usize))
            .field("stall_i_dis[7]", &self.stall_i_dis(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallIhDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallIhDis {{ stall_i_dis[0]: {:?}, stall_i_dis[1]: {:?}, stall_i_dis[2]: {:?}, stall_i_dis[3]: {:?}, stall_i_dis[4]: {:?}, stall_i_dis[5]: {:?}, stall_i_dis[6]: {:?}, stall_i_dis[7]: {:?} }}",
            self.stall_i_dis(0usize),
            self.stall_i_dis(1usize),
            self.stall_i_dis(2usize),
            self.stall_i_dis(3usize),
            self.stall_i_dis(4usize),
            self.stall_i_dis(5usize),
            self.stall_i_dis(6usize),
            self.stall_i_dis(7usize)
        )
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallIlDis(pub u8);
impl StallIlDis {
    #[doc = "Disable Endpoint IN Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis(&self, n: usize) -> StallIDisLo {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        StallIDisLo::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint IN Direction."]
    #[inline(always)]
    pub const fn set_stall_i_dis(&mut self, n: usize, val: StallIDisLo) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u8) & 0x01) << offs);
    }
}
impl Default for StallIlDis {
    #[inline(always)]
    fn default() -> StallIlDis {
        StallIlDis(0)
    }
}
impl core::fmt::Debug for StallIlDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallIlDis")
            .field("stall_i_dis[0]", &self.stall_i_dis(0usize))
            .field("stall_i_dis[1]", &self.stall_i_dis(1usize))
            .field("stall_i_dis[2]", &self.stall_i_dis(2usize))
            .field("stall_i_dis[3]", &self.stall_i_dis(3usize))
            .field("stall_i_dis[4]", &self.stall_i_dis(4usize))
            .field("stall_i_dis[5]", &self.stall_i_dis(5usize))
            .field("stall_i_dis[6]", &self.stall_i_dis(6usize))
            .field("stall_i_dis[7]", &self.stall_i_dis(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallIlDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallIlDis {{ stall_i_dis[0]: {:?}, stall_i_dis[1]: {:?}, stall_i_dis[2]: {:?}, stall_i_dis[3]: {:?}, stall_i_dis[4]: {:?}, stall_i_dis[5]: {:?}, stall_i_dis[6]: {:?}, stall_i_dis[7]: {:?} }}",
            self.stall_i_dis(0usize),
            self.stall_i_dis(1usize),
            self.stall_i_dis(2usize),
            self.stall_i_dis(3usize),
            self.stall_i_dis(4usize),
            self.stall_i_dis(5usize),
            self.stall_i_dis(6usize),
            self.stall_i_dis(7usize)
        )
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallOhDis(pub u8);
impl StallOhDis {
    #[doc = "Disable Endpoint OUT Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis(&self, n: usize) -> StallODisHi {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        StallODisHi::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint OUT Direction."]
    #[inline(always)]
    pub const fn set_stall_o_dis(&mut self, n: usize, val: StallODisHi) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u8) & 0x01) << offs);
    }
}
impl Default for StallOhDis {
    #[inline(always)]
    fn default() -> StallOhDis {
        StallOhDis(0)
    }
}
impl core::fmt::Debug for StallOhDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallOhDis")
            .field("stall_o_dis[0]", &self.stall_o_dis(0usize))
            .field("stall_o_dis[1]", &self.stall_o_dis(1usize))
            .field("stall_o_dis[2]", &self.stall_o_dis(2usize))
            .field("stall_o_dis[3]", &self.stall_o_dis(3usize))
            .field("stall_o_dis[4]", &self.stall_o_dis(4usize))
            .field("stall_o_dis[5]", &self.stall_o_dis(5usize))
            .field("stall_o_dis[6]", &self.stall_o_dis(6usize))
            .field("stall_o_dis[7]", &self.stall_o_dis(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallOhDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallOhDis {{ stall_o_dis[0]: {:?}, stall_o_dis[1]: {:?}, stall_o_dis[2]: {:?}, stall_o_dis[3]: {:?}, stall_o_dis[4]: {:?}, stall_o_dis[5]: {:?}, stall_o_dis[6]: {:?}, stall_o_dis[7]: {:?} }}",
            self.stall_o_dis(0usize),
            self.stall_o_dis(1usize),
            self.stall_o_dis(2usize),
            self.stall_o_dis(3usize),
            self.stall_o_dis(4usize),
            self.stall_o_dis(5usize),
            self.stall_o_dis(6usize),
            self.stall_o_dis(7usize)
        )
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallOlDis(pub u8);
impl StallOlDis {
    #[doc = "Disable Endpoint OUT Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis(&self, n: usize) -> StallODisLo {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        StallODisLo::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint OUT Direction."]
    #[inline(always)]
    pub const fn set_stall_o_dis(&mut self, n: usize, val: StallODisLo) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u8) & 0x01) << offs);
    }
}
impl Default for StallOlDis {
    #[inline(always)]
    fn default() -> StallOlDis {
        StallOlDis(0)
    }
}
impl core::fmt::Debug for StallOlDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallOlDis")
            .field("stall_o_dis[0]", &self.stall_o_dis(0usize))
            .field("stall_o_dis[1]", &self.stall_o_dis(1usize))
            .field("stall_o_dis[2]", &self.stall_o_dis(2usize))
            .field("stall_o_dis[3]", &self.stall_o_dis(3usize))
            .field("stall_o_dis[4]", &self.stall_o_dis(4usize))
            .field("stall_o_dis[5]", &self.stall_o_dis(5usize))
            .field("stall_o_dis[6]", &self.stall_o_dis(6usize))
            .field("stall_o_dis[7]", &self.stall_o_dis(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallOlDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallOlDis {{ stall_o_dis[0]: {:?}, stall_o_dis[1]: {:?}, stall_o_dis[2]: {:?}, stall_o_dis[3]: {:?}, stall_o_dis[4]: {:?}, stall_o_dis[5]: {:?}, stall_o_dis[6]: {:?}, stall_o_dis[7]: {:?} }}",
            self.stall_o_dis(0usize),
            self.stall_o_dis(1usize),
            self.stall_o_dis(2usize),
            self.stall_o_dis(3usize),
            self.stall_o_dis(4usize),
            self.stall_o_dis(5usize),
            self.stall_o_dis(6usize),
            self.stall_o_dis(7usize)
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u8);
impl Stat {
    #[doc = "Odd Bank."]
    #[must_use]
    #[inline(always)]
    pub const fn odd(&self) -> Odd {
        let val = (self.0 >> 2usize) & 0x01;
        Odd::from_bits(val as u8)
    }
    #[doc = "Odd Bank."]
    #[inline(always)]
    pub const fn set_odd(&mut self, val: Odd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Transmit Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> Tx {
        let val = (self.0 >> 3usize) & 0x01;
        Tx::from_bits(val as u8)
    }
    #[doc = "Transmit Indicator."]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Endpoint address."]
    #[must_use]
    #[inline(always)]
    pub const fn endp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint address."]
    #[inline(always)]
    pub const fn set_endp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("odd", &self.odd())
            .field("tx", &self.tx())
            .field("endp", &self.endp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ odd: {:?}, tx: {:?}, endp: {=u8:?} }}",
            self.odd(),
            self.tx(),
            self.endp()
        )
    }
}
#[doc = "Token."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Token(pub u8);
impl Token {
    #[doc = "Token Endpoint Address."]
    #[must_use]
    #[inline(always)]
    pub const fn tokenendpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Token Endpoint Address."]
    #[inline(always)]
    pub const fn set_tokenendpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Token Type."]
    #[must_use]
    #[inline(always)]
    pub const fn tokenpid(&self) -> Tokenpid {
        let val = (self.0 >> 4usize) & 0x0f;
        Tokenpid::from_bits(val as u8)
    }
    #[doc = "Token Type."]
    #[inline(always)]
    pub const fn set_tokenpid(&mut self, val: Tokenpid) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u8) & 0x0f) << 4usize);
    }
}
impl Default for Token {
    #[inline(always)]
    fn default() -> Token {
        Token(0)
    }
}
impl core::fmt::Debug for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Token")
            .field("tokenendpt", &self.tokenendpt())
            .field("tokenpid", &self.tokenpid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Token {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Token {{ tokenendpt: {=u8:?}, tokenpid: {:?} }}",
            self.tokenendpt(),
            self.tokenpid()
        )
    }
}
#[doc = "USB Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbctrl(pub u8);
impl Usbctrl {
    #[doc = "DP and DM Lane Reversal Control."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdm_lane_reverse(&self) -> DpdmLaneReverse {
        let val = (self.0 >> 2usize) & 0x01;
        DpdmLaneReverse::from_bits(val as u8)
    }
    #[doc = "DP and DM Lane Reversal Control."]
    #[inline(always)]
    pub const fn set_dpdm_lane_reverse(&mut self, val: DpdmLaneReverse) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Host-Mode-Only Low-Speed Device EOP Signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn host_ls_eop(&self) -> HostLsEop {
        let val = (self.0 >> 3usize) & 0x01;
        HostLsEop::from_bits(val as u8)
    }
    #[doc = "Host-Mode-Only Low-Speed Device EOP Signaling."]
    #[inline(always)]
    pub const fn set_host_ls_eop(&mut self, val: HostLsEop) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "UART Select."]
    #[must_use]
    #[inline(always)]
    pub const fn uartsel(&self) -> Uartsel {
        let val = (self.0 >> 4usize) & 0x01;
        Uartsel::from_bits(val as u8)
    }
    #[doc = "UART Select."]
    #[inline(always)]
    pub const fn set_uartsel(&mut self, val: Uartsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "UART Signal Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn uartchls(&self) -> Uartchls {
        let val = (self.0 >> 5usize) & 0x01;
        Uartchls::from_bits(val as u8)
    }
    #[doc = "UART Signal Channel Select."]
    #[inline(always)]
    pub const fn set_uartchls(&mut self, val: Uartchls) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pde(&self) -> Pde {
        let val = (self.0 >> 6usize) & 0x01;
        Pde::from_bits(val as u8)
    }
    #[doc = "Pulldown Enable."]
    #[inline(always)]
    pub const fn set_pde(&mut self, val: Pde) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> Susp {
        let val = (self.0 >> 7usize) & 0x01;
        Susp::from_bits(val as u8)
    }
    #[doc = "Suspend."]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: Susp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Usbctrl {
    #[inline(always)]
    fn default() -> Usbctrl {
        Usbctrl(0)
    }
}
impl core::fmt::Debug for Usbctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbctrl")
            .field("dpdm_lane_reverse", &self.dpdm_lane_reverse())
            .field("host_ls_eop", &self.host_ls_eop())
            .field("uartsel", &self.uartsel())
            .field("uartchls", &self.uartchls())
            .field("pde", &self.pde())
            .field("susp", &self.susp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbctrl {{ dpdm_lane_reverse: {:?}, host_ls_eop: {:?}, uartsel: {:?}, uartchls: {:?}, pde: {:?}, susp: {:?} }}",
            self.dpdm_lane_reverse(),
            self.host_ls_eop(),
            self.uartsel(),
            self.uartchls(),
            self.pde(),
            self.susp()
        )
    }
}
#[doc = "Frame Adjust."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfrmadjust(pub u8);
impl Usbfrmadjust {
    #[doc = "Frame Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn adj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frame Adjustment."]
    #[inline(always)]
    pub const fn set_adj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Usbfrmadjust {
    #[inline(always)]
    fn default() -> Usbfrmadjust {
        Usbfrmadjust(0)
    }
}
impl core::fmt::Debug for Usbfrmadjust {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbfrmadjust")
            .field("adj", &self.adj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbfrmadjust {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usbfrmadjust {{ adj: {=u8:?} }}", self.adj())
    }
}
#[doc = "USB Transceiver Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbtrc0(pub u8);
impl Usbtrc0 {
    #[doc = "USB Asynchronous Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_resume_int(&self) -> UsbResumeInt {
        let val = (self.0 >> 0usize) & 0x01;
        UsbResumeInt::from_bits(val as u8)
    }
    #[doc = "USB Asynchronous Interrupt."]
    #[inline(always)]
    pub const fn set_usb_resume_int(&mut self, val: UsbResumeInt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Synchronous USB Interrupt Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn sync_det(&self) -> SyncDet {
        let val = (self.0 >> 1usize) & 0x01;
        SyncDet::from_bits(val as u8)
    }
    #[doc = "Synchronous USB Interrupt Detect."]
    #[inline(always)]
    pub const fn set_sync_det(&mut self, val: SyncDet) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Combined USB Clock Recovery interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_clk_recovery_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Combined USB Clock Recovery interrupt status."]
    #[inline(always)]
    pub const fn set_usb_clk_recovery_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VREGIN Rising Edge Interrupt Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn vredg_det(&self) -> VredgDet {
        let val = (self.0 >> 3usize) & 0x01;
        VredgDet::from_bits(val as u8)
    }
    #[doc = "VREGIN Rising Edge Interrupt Detect."]
    #[inline(always)]
    pub const fn set_vredg_det(&mut self, val: VredgDet) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "VREGIN Falling Edge Interrupt Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn vfedg_det(&self) -> VfedgDet {
        let val = (self.0 >> 4usize) & 0x01;
        VfedgDet::from_bits(val as u8)
    }
    #[doc = "VREGIN Falling Edge Interrupt Detect."]
    #[inline(always)]
    pub const fn set_vfedg_det(&mut self, val: VfedgDet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Resume Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usbresmen(&self) -> Usbresmen {
        let val = (self.0 >> 5usize) & 0x01;
        Usbresmen::from_bits(val as u8)
    }
    #[doc = "Asynchronous Resume Interrupt Enable."]
    #[inline(always)]
    pub const fn set_usbresmen(&mut self, val: Usbresmen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "VREGIN Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vregin_sts(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "VREGIN Status."]
    #[inline(always)]
    pub const fn set_vregin_sts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "USB Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn usbreset(&self) -> Usbreset {
        let val = (self.0 >> 7usize) & 0x01;
        Usbreset::from_bits(val as u8)
    }
    #[doc = "USB Reset."]
    #[inline(always)]
    pub const fn set_usbreset(&mut self, val: Usbreset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Usbtrc0 {
    #[inline(always)]
    fn default() -> Usbtrc0 {
        Usbtrc0(0)
    }
}
impl core::fmt::Debug for Usbtrc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbtrc0")
            .field("usb_resume_int", &self.usb_resume_int())
            .field("sync_det", &self.sync_det())
            .field("usb_clk_recovery_int", &self.usb_clk_recovery_int())
            .field("vredg_det", &self.vredg_det())
            .field("vfedg_det", &self.vfedg_det())
            .field("usbresmen", &self.usbresmen())
            .field("vregin_sts", &self.vregin_sts())
            .field("usbreset", &self.usbreset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbtrc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbtrc0 {{ usb_resume_int: {:?}, sync_det: {:?}, usb_clk_recovery_int: {=bool:?}, vredg_det: {:?}, vfedg_det: {:?}, usbresmen: {:?}, vregin_sts: {=bool:?}, usbreset: {:?} }}",
            self.usb_resume_int(),
            self.sync_det(),
            self.usb_clk_recovery_int(),
            self.vredg_det(),
            self.vfedg_det(),
            self.usbresmen(),
            self.vregin_sts(),
            self.usbreset()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Attach {
    #[doc = "Not detected."]
    IntNo = 0x0,
    #[doc = "Detected."]
    IntYes = 0x01,
}
impl Attach {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Attach {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Attach {
    #[inline(always)]
    fn from(val: u8) -> Attach {
        Attach::from_bits(val)
    }
}
impl From<Attach> for u8 {
    #[inline(always)]
    fn from(val: Attach) -> u8 {
        Attach::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Attachen {
    #[doc = "Disable."]
    DisAttachInt = 0x0,
    #[doc = "Enable."]
    EnAttachInt = 0x01,
}
impl Attachen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Attachen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Attachen {
    #[inline(always)]
    fn from(val: u8) -> Attachen {
        Attachen::from_bits(val)
    }
}
impl From<Attachen> for u8 {
    #[inline(always)]
    fn from(val: Attachen) -> u8 {
        Attachen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btoerr {
    #[doc = "Not timed out."]
    IntNo = 0x0,
    #[doc = "Timed out."]
    IntYes = 0x01,
}
impl Btoerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btoerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btoerr {
    #[inline(always)]
    fn from(val: u8) -> Btoerr {
        Btoerr::from_bits(val)
    }
}
impl From<Btoerr> for u8 {
    #[inline(always)]
    fn from(val: Btoerr) -> u8 {
        Btoerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btoerren {
    #[doc = "Disable."]
    DisBtoerrInt = 0x0,
    #[doc = "Enable."]
    EnBtoerrInt = 0x01,
}
impl Btoerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btoerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btoerren {
    #[inline(always)]
    fn from(val: u8) -> Btoerren {
        Btoerren::from_bits(val)
    }
}
impl From<Btoerren> for u8 {
    #[inline(always)]
    fn from(val: Btoerren) -> u8 {
        Btoerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btserr {
    #[doc = "Packet not rejected due to the error."]
    IntNo = 0x0,
    #[doc = "Packet rejected due to the error."]
    IntYes = 0x01,
}
impl Btserr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btserr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btserr {
    #[inline(always)]
    fn from(val: u8) -> Btserr {
        Btserr::from_bits(val)
    }
}
impl From<Btserr> for u8 {
    #[inline(always)]
    fn from(val: Btserr) -> u8 {
        Btserr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btserren {
    #[doc = "Disable."]
    DisBtserrenInt = 0x0,
    #[doc = "Enable."]
    EnBtserrenInt = 0x01,
}
impl Btserren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btserren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btserren {
    #[inline(always)]
    fn from(val: u8) -> Btserren {
        Btserren::from_bits(val)
    }
}
impl From<Btserren> for u8 {
    #[inline(always)]
    fn from(val: Btserren) -> u8 {
        Btserren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockRecoverEn {
    #[doc = "Disable."]
    DisClkRecover = 0x0,
    #[doc = "Enable."]
    EnClkRecover = 0x01,
}
impl ClockRecoverEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRecoverEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRecoverEn {
    #[inline(always)]
    fn from(val: u8) -> ClockRecoverEn {
        ClockRecoverEn::from_bits(val)
    }
}
impl From<ClockRecoverEn> for u8 {
    #[inline(always)]
    fn from(val: ClockRecoverEn) -> u8 {
        ClockRecoverEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc16 {
    #[doc = "Not rejected."]
    IntNo = 0x0,
    #[doc = "Rejected."]
    IntYes = 0x01,
}
impl Crc16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc16 {
    #[inline(always)]
    fn from(val: u8) -> Crc16 {
        Crc16::from_bits(val)
    }
}
impl From<Crc16> for u8 {
    #[inline(always)]
    fn from(val: Crc16) -> u8 {
        Crc16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc16en {
    #[doc = "Disable."]
    DisCrc16Int = 0x0,
    #[doc = "Enable."]
    EnCrc16Int = 0x01,
}
impl Crc16en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc16en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc16en {
    #[inline(always)]
    fn from(val: u8) -> Crc16en {
        Crc16en::from_bits(val)
    }
}
impl From<Crc16en> for u8 {
    #[inline(always)]
    fn from(val: Crc16en) -> u8 {
        Crc16en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc5eof {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl Crc5eof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc5eof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc5eof {
    #[inline(always)]
    fn from(val: u8) -> Crc5eof {
        Crc5eof::from_bits(val)
    }
}
impl From<Crc5eof> for u8 {
    #[inline(always)]
    fn from(val: Crc5eof) -> u8 {
        Crc5eof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc5eofen {
    #[doc = "Disable."]
    DisCrc5Int = 0x0,
    #[doc = "Enable."]
    EnCrc5Int = 0x01,
}
impl Crc5eofen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc5eofen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc5eofen {
    #[inline(always)]
    fn from(val: u8) -> Crc5eofen {
        Crc5eofen::from_bits(val)
    }
}
impl From<Crc5eofen> for u8 {
    #[inline(always)]
    fn from(val: Crc5eofen) -> u8 {
        Crc5eofen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfn8 {
    #[doc = "Integer number of bytes."]
    IntNo = 0x0,
    #[doc = "Not an integer number of bytes."]
    IntYes = 0x01,
}
impl Dfn8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfn8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfn8 {
    #[inline(always)]
    fn from(val: u8) -> Dfn8 {
        Dfn8::from_bits(val)
    }
}
impl From<Dfn8> for u8 {
    #[inline(always)]
    fn from(val: Dfn8) -> u8 {
        Dfn8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfn8en {
    #[doc = "Disable."]
    DisDfn8Int = 0x0,
    #[doc = "Enable."]
    EnDfn8Int = 0x01,
}
impl Dfn8en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfn8en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfn8en {
    #[inline(always)]
    fn from(val: u8) -> Dfn8en {
        Dfn8en::from_bits(val)
    }
}
impl From<Dfn8en> for u8 {
    #[inline(always)]
    fn from(val: Dfn8en) -> u8 {
        Dfn8en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaerr {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl Dmaerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaerr {
    #[inline(always)]
    fn from(val: u8) -> Dmaerr {
        Dmaerr::from_bits(val)
    }
}
impl From<Dmaerr> for u8 {
    #[inline(always)]
    fn from(val: Dmaerr) -> u8 {
        Dmaerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaerren {
    #[doc = "Disable."]
    DisDmaerrInt = 0x0,
    #[doc = "Enable."]
    EnDmaerrInt = 0x01,
}
impl Dmaerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaerren {
    #[inline(always)]
    fn from(val: u8) -> Dmaerren {
        Dmaerren::from_bits(val)
    }
}
impl From<Dmaerren> for u8 {
    #[inline(always)]
    fn from(val: Dmaerren) -> u8 {
        Dmaerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmlow {
    #[doc = "Disable."]
    DisDmPulldown = 0x0,
    #[doc = "Enable."]
    EnDmPulldown = 0x01,
}
impl Dmlow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmlow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmlow {
    #[inline(always)]
    fn from(val: u8) -> Dmlow {
        Dmlow::from_bits(val)
    }
}
impl From<Dmlow> for u8 {
    #[inline(always)]
    fn from(val: Dmlow) -> u8 {
        Dmlow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmpd {
    #[doc = "Disabled."]
    DmPdDisStat = 0x0,
    #[doc = "Enabled."]
    DmPdEnStat = 0x01,
}
impl Dmpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmpd {
    #[inline(always)]
    fn from(val: u8) -> Dmpd {
        Dmpd::from_bits(val)
    }
}
impl From<Dmpd> for u8 {
    #[inline(always)]
    fn from(val: Dmpd) -> u8 {
        Dmpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdmLaneReverse {
    #[doc = "Standard USB DP and DM package pin assignment."]
    DpDmStandard = 0x0,
    #[doc = "Reverse roles of USB DP and DM package pins."]
    DpDmReversed = 0x01,
}
impl DpdmLaneReverse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdmLaneReverse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdmLaneReverse {
    #[inline(always)]
    fn from(val: u8) -> DpdmLaneReverse {
        DpdmLaneReverse::from_bits(val)
    }
}
impl From<DpdmLaneReverse> for u8 {
    #[inline(always)]
    fn from(val: DpdmLaneReverse) -> u8 {
        DpdmLaneReverse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dphigh {
    #[doc = "Disable."]
    DisDpPullup = 0x0,
    #[doc = "Enable."]
    EnDpPullup = 0x01,
}
impl Dphigh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dphigh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dphigh {
    #[inline(always)]
    fn from(val: u8) -> Dphigh {
        Dphigh::from_bits(val)
    }
}
impl From<Dphigh> for u8 {
    #[inline(always)]
    fn from(val: Dphigh) -> u8 {
        Dphigh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dplow {
    #[doc = "Disable."]
    DisDpPulldown = 0x0,
    #[doc = "Enable."]
    EnDpPulldown = 0x01,
}
impl Dplow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dplow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dplow {
    #[inline(always)]
    fn from(val: u8) -> Dplow {
        Dplow::from_bits(val)
    }
}
impl From<Dplow> for u8 {
    #[inline(always)]
    fn from(val: Dplow) -> u8 {
        Dplow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dppd {
    #[doc = "Disabled."]
    DpPdDisStat = 0x0,
    #[doc = "Enabled."]
    DpPdEnStat = 0x01,
}
impl Dppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dppd {
    #[inline(always)]
    fn from(val: u8) -> Dppd {
        Dppd::from_bits(val)
    }
}
impl From<Dppd> for u8 {
    #[inline(always)]
    fn from(val: Dppd) -> u8 {
        Dppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dppu {
    #[doc = "Disabled."]
    DpPuDisStat = 0x0,
    #[doc = "Enabled."]
    DpPuEnStat = 0x01,
}
impl Dppu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dppu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dppu {
    #[inline(always)]
    fn from(val: u8) -> Dppu {
        Dppu::from_bits(val)
    }
}
impl From<Dppu> for u8 {
    #[inline(always)]
    fn from(val: Dppu) -> u8 {
        Dppu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dppullupnonotg {
    #[doc = "Disable."]
    DisDeviceDpPu = 0x0,
    #[doc = "Enabled."]
    EnDeviceDpPu = 0x01,
}
impl Dppullupnonotg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dppullupnonotg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dppullupnonotg {
    #[inline(always)]
    fn from(val: u8) -> Dppullupnonotg {
        Dppullupnonotg::from_bits(val)
    }
}
impl From<Dppullupnonotg> for u8 {
    #[inline(always)]
    fn from(val: Dppullupnonotg) -> u8 {
        Dppullupnonotg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epctldis {
    #[doc = "Enable."]
    Enable = 0x0,
    #[doc = "Disable."]
    Disable = 0x01,
}
impl Epctldis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epctldis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epctldis {
    #[inline(always)]
    fn from(val: u8) -> Epctldis {
        Epctldis::from_bits(val)
    }
}
impl From<Epctldis> for u8 {
    #[inline(always)]
    fn from(val: Epctldis) -> u8 {
        Epctldis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[doc = "Error did not occur."]
    IntNo = 0x0,
    #[doc = "Error occurred."]
    IntYes = 0x01,
}
impl Error {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Error {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Error {
    #[inline(always)]
    fn from(val: u8) -> Error {
        Error::from_bits(val)
    }
}
impl From<Error> for u8 {
    #[inline(always)]
    fn from(val: Error) -> u8 {
        Error::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erroren {
    #[doc = "Disable."]
    DisErrorInt = 0x0,
    #[doc = "Enable."]
    EnErrorInt = 0x01,
}
impl Erroren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erroren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erroren {
    #[inline(always)]
    fn from(val: u8) -> Erroren {
        Erroren::from_bits(val)
    }
}
impl From<Erroren> for u8 {
    #[inline(always)]
    fn from(val: Erroren) -> u8 {
        Erroren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostLsEop {
    #[doc = "Full-speed device or a low-speed device through a hub."]
    HostFsResumeEop = 0x0,
    #[doc = "Directly-connected low-speed device."]
    HostLsResumeEop = 0x01,
}
impl HostLsEop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostLsEop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostLsEop {
    #[inline(always)]
    fn from(val: u8) -> HostLsEop {
        HostLsEop::from_bits(val)
    }
}
impl From<HostLsEop> for u8 {
    #[inline(always)]
    fn from(val: HostLsEop) -> u8 {
        HostLsEop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hostmodeen {
    #[doc = "USBFS operates in Device mode."]
    EnDeviceMode = 0x0,
    #[doc = "USBFS operates in Host mode. In Host mode, USBFS performs USB transactions under the programmed control of the host processor."]
    EnHostMode = 0x01,
}
impl Hostmodeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hostmodeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hostmodeen {
    #[inline(always)]
    fn from(val: u8) -> Hostmodeen {
        Hostmodeen::from_bits(val)
    }
}
impl From<Hostmodeen> for u8 {
    #[inline(always)]
    fn from(val: Hostmodeen) -> u8 {
        Hostmodeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hostwohub {
    #[doc = "Connected using a hub (USBFS generates PRE_PID as required)."]
    LsThruHub = 0x0,
    #[doc = "Connected directly to host without a hub, or was used to attach."]
    LsDirectConnect = 0x01,
}
impl Hostwohub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hostwohub {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hostwohub {
    #[inline(always)]
    fn from(val: u8) -> Hostwohub {
        Hostwohub::from_bits(val)
    }
}
impl From<Hostwohub> for u8 {
    #[inline(always)]
    fn from(val: Hostwohub) -> u8 {
        Hostwohub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrcEn {
    #[doc = "Disable."]
    DisIrc = 0x0,
    #[doc = "Enable."]
    EnIrc = 0x01,
}
impl IrcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrcEn {
    #[inline(always)]
    fn from(val: u8) -> IrcEn {
        IrcEn::from_bits(val)
    }
}
impl From<IrcEn> for u8 {
    #[inline(always)]
    fn from(val: IrcEn) -> u8 {
        IrcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeepAliveSts {
    #[doc = "Not in Keep Alive mode."]
    KeepAliveDisStat = 0x0,
    #[doc = "In Keep Alive mode."]
    KeepAliveEnStat = 0x01,
}
impl KeepAliveSts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeepAliveSts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeepAliveSts {
    #[inline(always)]
    fn from(val: u8) -> KeepAliveSts {
        KeepAliveSts::from_bits(val)
    }
}
impl From<KeepAliveSts> for u8 {
    #[inline(always)]
    fn from(val: KeepAliveSts) -> u8 {
        KeepAliveSts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LineStateChg {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl LineStateChg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LineStateChg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LineStateChg {
    #[inline(always)]
    fn from(val: u8) -> LineStateChg {
        LineStateChg::from_bits(val)
    }
}
impl From<LineStateChg> for u8 {
    #[inline(always)]
    fn from(val: LineStateChg) -> u8 {
        LineStateChg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linestateen {
    #[doc = "Disable."]
    DisLinestInt = 0x0,
    #[doc = "Enable."]
    EnLinestInt = 0x01,
}
impl Linestateen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linestateen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linestateen {
    #[inline(always)]
    fn from(val: u8) -> Linestateen {
        Linestateen::from_bits(val)
    }
}
impl From<Linestateen> for u8 {
    #[inline(always)]
    fn from(val: Linestateen) -> u8 {
        Linestateen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linestatestable {
    #[doc = "Unstable."]
    LinestNotStable = 0x0,
    #[doc = "Stable."]
    LinestStable = 0x01,
}
impl Linestatestable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linestatestable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linestatestable {
    #[inline(always)]
    fn from(val: u8) -> Linestatestable {
        Linestatestable::from_bits(val)
    }
}
impl From<Linestatestable> for u8 {
    #[inline(always)]
    fn from(val: Linestatestable) -> u8 {
        Linestatestable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Odd {
    #[doc = "Not in the odd bank."]
    NotInOddBank = 0x0,
    #[doc = "In the odd bank."]
    OddBank = 0x01,
}
impl Odd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Odd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Odd {
    #[inline(always)]
    fn from(val: u8) -> Odd {
        Odd::from_bits(val)
    }
}
impl From<Odd> for u8 {
    #[inline(always)]
    fn from(val: Odd) -> u8 {
        Odd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Onemsec {
    #[doc = "Not timed out."]
    IntNo = 0x0,
    #[doc = "Timed out."]
    IntYes = 0x01,
}
impl Onemsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Onemsec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Onemsec {
    #[inline(always)]
    fn from(val: u8) -> Onemsec {
        Onemsec::from_bits(val)
    }
}
impl From<Onemsec> for u8 {
    #[inline(always)]
    fn from(val: Onemsec) -> u8 {
        Onemsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Onemsecen {
    #[doc = "Disable."]
    DisTimerInt = 0x0,
    #[doc = "Enable."]
    EnTimerInt = 0x01,
}
impl Onemsecen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Onemsecen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Onemsecen {
    #[inline(always)]
    fn from(val: u8) -> Onemsecen {
        Onemsecen::from_bits(val)
    }
}
impl From<Onemsecen> for u8 {
    #[inline(always)]
    fn from(val: Onemsecen) -> u8 {
        Onemsecen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Otgen {
    #[doc = "If USBENSOFEN is 1 and HOSTMODEEN is 0 in the Control Register (CTL), then the D+ Data line pullup resistors are enabled. If HOSTMODEEN is 1, then the D+ and D- Data line pulldown resistors are engaged."]
    ConfigResistorsCtl = 0x0,
    #[doc = "Uses the pullup and pulldown controls in this register."]
    ConfigResistorsHere = 0x01,
}
impl Otgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Otgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Otgen {
    #[inline(always)]
    fn from(val: u8) -> Otgen {
        Otgen::from_bits(val)
    }
}
impl From<Otgen> for u8 {
    #[inline(always)]
    fn from(val: Otgen) -> u8 {
        Otgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvfError {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Unmasked interrupt occurred."]
    IntYes = 0x01,
}
impl OvfError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvfError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvfError {
    #[inline(always)]
    fn from(val: u8) -> OvfError {
        OvfError::from_bits(val)
    }
}
impl From<OvfError> for u8 {
    #[inline(always)]
    fn from(val: OvfError) -> u8 {
        OvfError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvfErrorEn {
    #[doc = "The interrupt is masked."]
    MaskOvfErrInt = 0x0,
    #[doc = "The interrupt is enabled."]
    EnOvfErrInt = 0x01,
}
impl OvfErrorEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvfErrorEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvfErrorEn {
    #[inline(always)]
    fn from(val: u8) -> OvfErrorEn {
        OvfErrorEn::from_bits(val)
    }
}
impl From<OvfErrorEn> for u8 {
    #[inline(always)]
    fn from(val: OvfErrorEn) -> u8 {
        OvfErrorEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ownerr {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl Ownerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ownerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ownerr {
    #[inline(always)]
    fn from(val: u8) -> Ownerr {
        Ownerr::from_bits(val)
    }
}
impl From<Ownerr> for u8 {
    #[inline(always)]
    fn from(val: Ownerr) -> u8 {
        Ownerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ownerren {
    #[doc = "Disable."]
    DisOwnerrInt = 0x0,
    #[doc = "Enable."]
    EnOwnerrInt = 0x01,
}
impl Ownerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ownerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ownerren {
    #[inline(always)]
    fn from(val: u8) -> Ownerren {
        Ownerren::from_bits(val)
    }
}
impl From<Ownerren> for u8 {
    #[inline(always)]
    fn from(val: Ownerren) -> u8 {
        Ownerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ownerrisodis {
    #[doc = "Enable."]
    DisOwnErrorDetectIso = 0x0,
    #[doc = "Disable."]
    EnOwnErrorDetectIso = 0x01,
}
impl Ownerrisodis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ownerrisodis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ownerrisodis {
    #[inline(always)]
    fn from(val: u8) -> Ownerrisodis {
        Ownerrisodis::from_bits(val)
    }
}
impl From<Ownerrisodis> for u8 {
    #[inline(always)]
    fn from(val: Ownerrisodis) -> u8 {
        Ownerrisodis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pde {
    #[doc = "Disable on D+ and D-."]
    DisPulldowns = 0x0,
    #[doc = "Enable on D+ and D-."]
    EnPulldowns = 0x01,
}
impl Pde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pde {
    #[inline(always)]
    fn from(val: u8) -> Pde {
        Pde::from_bits(val)
    }
}
impl From<Pde> for u8 {
    #[inline(always)]
    fn from(val: Pde) -> u8 {
        Pde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Piderr {
    #[doc = "Did not fail."]
    IntNo = 0x0,
    #[doc = "Failed."]
    IntYes = 0x01,
}
impl Piderr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Piderr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Piderr {
    #[inline(always)]
    fn from(val: u8) -> Piderr {
        Piderr::from_bits(val)
    }
}
impl From<Piderr> for u8 {
    #[inline(always)]
    fn from(val: Piderr) -> u8 {
        Piderr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Piderren {
    #[doc = "Disable."]
    DisPiderrInt = 0x0,
    #[doc = "Enable."]
    EnPiderrInt = 0x01,
}
impl Piderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Piderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Piderren {
    #[inline(always)]
    fn from(val: u8) -> Piderren {
        Piderren::from_bits(val)
    }
}
impl From<Piderren> for u8 {
    #[inline(always)]
    fn from(val: Piderren) -> u8 {
        Piderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetResumeRoughEn {
    #[doc = "Always works in tracking phase after the first time rough phase, to track transition."]
    KeepTrimFineOnReset = 0x0,
    #[doc = "Go back to rough stage whenever a bus reset or bus resume occurs."]
    UseIfrTrimFineOnReset = 0x01,
}
impl ResetResumeRoughEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResetResumeRoughEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResetResumeRoughEn {
    #[inline(always)]
    fn from(val: u8) -> ResetResumeRoughEn {
        ResetResumeRoughEn::from_bits(val)
    }
}
impl From<ResetResumeRoughEn> for u8 {
    #[inline(always)]
    fn from(val: ResetResumeRoughEn) -> u8 {
        ResetResumeRoughEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RestartIfrtrimEn {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value."]
    LoadTrimFineMid = 0x0,
    #[doc = "Trim fine restarts from the IFR trim value whenever you detect bus_reset or bus_resume or deassert module enable."]
    LoadTrimFineIfr = 0x01,
}
impl RestartIfrtrimEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RestartIfrtrimEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RestartIfrtrimEn {
    #[inline(always)]
    fn from(val: u8) -> RestartIfrtrimEn {
        RestartIfrtrimEn::from_bits(val)
    }
}
impl From<RestartIfrtrimEn> for u8 {
    #[inline(always)]
    fn from(val: RestartIfrtrimEn) -> u8 {
        RestartIfrtrimEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resume {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl Resume {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resume {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resume {
    #[inline(always)]
    fn from(val: u8) -> Resume {
        Resume::from_bits(val)
    }
}
impl From<Resume> for u8 {
    #[inline(always)]
    fn from(val: Resume) -> u8 {
        Resume::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resumeen {
    #[doc = "Disable."]
    DisResumeInt = 0x0,
    #[doc = "Enable."]
    EnResumeInt = 0x01,
}
impl Resumeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resumeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resumeen {
    #[inline(always)]
    fn from(val: u8) -> Resumeen {
        Resumeen::from_bits(val)
    }
}
impl From<Resumeen> for u8 {
    #[inline(always)]
    fn from(val: Resumeen) -> u8 {
        Resumeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Retrydis {
    #[doc = "Retried NAK'ed transactions in hardware."]
    Retried = 0x0,
    #[doc = "Do not retry NAK'ed transactions. When a transaction is NAK'ed, the BDT PID field is updated with the NAK PID, and the TOKEN_DNE interrupt becomes 1."]
    DoNotRetried = 0x01,
}
impl Retrydis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Retrydis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Retrydis {
    #[inline(always)]
    fn from(val: u8) -> Retrydis {
        Retrydis::from_bits(val)
    }
}
impl From<Retrydis> for u8 {
    #[inline(always)]
    fn from(val: Retrydis) -> u8 {
        Retrydis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SessVld {
    #[doc = "Below."]
    SessVldLow = 0x0,
    #[doc = "Above."]
    SessVldHigh = 0x01,
}
impl SessVld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SessVld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SessVld {
    #[inline(always)]
    fn from(val: u8) -> SessVld {
        SessVld::from_bits(val)
    }
}
impl From<SessVld> for u8 {
    #[inline(always)]
    fn from(val: SessVld) -> u8 {
        SessVld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleep {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl Sleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleep {
    #[inline(always)]
    fn from(val: u8) -> Sleep {
        Sleep::from_bits(val)
    }
}
impl From<Sleep> for u8 {
    #[inline(always)]
    fn from(val: Sleep) -> u8 {
        Sleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepen {
    #[doc = "Disable."]
    DisSleepInt = 0x0,
    #[doc = "Enable."]
    EnSleepInt = 0x01,
}
impl Sleepen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepen {
    #[inline(always)]
    fn from(val: u8) -> Sleepen {
        Sleepen::from_bits(val)
    }
}
impl From<Sleepen> for u8 {
    #[inline(always)]
    fn from(val: Sleepen) -> u8 {
        Sleepen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sofbusset {
    #[doc = "According to the SOF threshold value."]
    SofTokIntFromThreshold = 0x0,
    #[doc = "When the SOF counter reaches 0."]
    SofTokIntCounter0 = 0x01,
}
impl Sofbusset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sofbusset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sofbusset {
    #[inline(always)]
    fn from(val: u8) -> Sofbusset {
        Sofbusset::from_bits(val)
    }
}
impl From<Sofbusset> for u8 {
    #[inline(always)]
    fn from(val: Sofbusset) -> u8 {
        Sofbusset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sofdynthld {
    #[doc = "When the byte-times SOF threshold is reached."]
    UseDynSofThreshold = 0x0,
    #[doc = "When 8 byte-times SOF threshold is reached or overstepped."]
    UseFixedSofThreshold = 0x01,
}
impl Sofdynthld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sofdynthld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sofdynthld {
    #[inline(always)]
    fn from(val: u8) -> Sofdynthld {
        Sofdynthld::from_bits(val)
    }
}
impl From<Sofdynthld> for u8 {
    #[inline(always)]
    fn from(val: Sofdynthld) -> u8 {
        Sofdynthld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softok {
    #[doc = "Did not receive."]
    IntNo = 0x0,
    #[doc = "Received."]
    IntYes = 0x01,
}
impl Softok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softok {
    #[inline(always)]
    fn from(val: u8) -> Softok {
        Softok::from_bits(val)
    }
}
impl From<Softok> for u8 {
    #[inline(always)]
    fn from(val: Softok) -> u8 {
        Softok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softoken {
    #[doc = "Disable."]
    DisSoftokInt = 0x0,
    #[doc = "Enable."]
    EnSoftokInt = 0x01,
}
impl Softoken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softoken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softoken {
    #[inline(always)]
    fn from(val: u8) -> Softoken {
        Softoken::from_bits(val)
    }
}
impl From<Softoken> for u8 {
    #[inline(always)]
    fn from(val: Softoken) -> u8 {
        Softoken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stall {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl Stall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stall {
    #[inline(always)]
    fn from(val: u8) -> Stall {
        Stall::from_bits(val)
    }
}
impl From<Stall> for u8 {
    #[inline(always)]
    fn from(val: Stall) -> u8 {
        Stall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDisHi {
    #[doc = "Enable."]
    EnEpInStall = 0x0,
    #[doc = "Disable."]
    DisEpInStall = 0x01,
}
impl StallIDisHi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDisHi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDisHi {
    #[inline(always)]
    fn from(val: u8) -> StallIDisHi {
        StallIDisHi::from_bits(val)
    }
}
impl From<StallIDisHi> for u8 {
    #[inline(always)]
    fn from(val: StallIDisHi) -> u8 {
        StallIDisHi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDisLo {
    #[doc = "Enable."]
    EnEpInStall = 0x0,
    #[doc = "Disable."]
    DisEpInStall = 0x01,
}
impl StallIDisLo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDisLo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDisLo {
    #[inline(always)]
    fn from(val: u8) -> StallIDisLo {
        StallIDisLo::from_bits(val)
    }
}
impl From<StallIDisLo> for u8 {
    #[inline(always)]
    fn from(val: StallIDisLo) -> u8 {
        StallIDisLo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODisHi {
    #[doc = "Enable."]
    EnEpOutStall = 0x0,
    #[doc = "Disable."]
    DisEpOutStall = 0x01,
}
impl StallODisHi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODisHi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODisHi {
    #[inline(always)]
    fn from(val: u8) -> StallODisHi {
        StallODisHi::from_bits(val)
    }
}
impl From<StallODisHi> for u8 {
    #[inline(always)]
    fn from(val: StallODisHi) -> u8 {
        StallODisHi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODisLo {
    #[doc = "Enable."]
    EnEpOutStall = 0x0,
    #[doc = "Disable."]
    DisEpOutStall = 0x01,
}
impl StallODisLo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODisLo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODisLo {
    #[inline(always)]
    fn from(val: u8) -> StallODisLo {
        StallODisLo::from_bits(val)
    }
}
impl From<StallODisLo> for u8 {
    #[inline(always)]
    fn from(val: StallODisLo) -> u8 {
        StallODisLo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stallen {
    #[doc = "Disable."]
    DisStallInt = 0x0,
    #[doc = "Enable."]
    EnStallInt = 0x01,
}
impl Stallen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stallen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stallen {
    #[inline(always)]
    fn from(val: u8) -> Stallen {
        Stallen::from_bits(val)
    }
}
impl From<Stallen> for u8 {
    #[inline(always)]
    fn from(val: Stallen) -> u8 {
        Stallen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StlAdjEn {
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint stalls."]
    StallBothInOut = 0x0,
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, the STALL_xx_DIS registers control which directions for the associated endpoint stalls."]
    StallSingleDirection = 0x01,
}
impl StlAdjEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StlAdjEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StlAdjEn {
    #[inline(always)]
    fn from(val: u8) -> StlAdjEn {
        StlAdjEn::from_bits(val)
    }
}
impl From<StlAdjEn> for u8 {
    #[inline(always)]
    fn from(val: StlAdjEn) -> u8 {
        StlAdjEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopAckDlyEn {
    #[doc = "Enter KEEP_ALIVE mode immediately when there is no USB AHB transfer."]
    EnterKeepAliveOnIdle = 0x0,
    #[doc = "Enter KEEP_ALIVE mode until the USB core is idle and there is no USB AHB transfer."]
    EnterKeepAliveImmediate = 0x01,
}
impl StopAckDlyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StopAckDlyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StopAckDlyEn {
    #[inline(always)]
    fn from(val: u8) -> StopAckDlyEn {
        StopAckDlyEn::from_bits(val)
    }
}
impl From<StopAckDlyEn> for u8 {
    #[inline(always)]
    fn from(val: StopAckDlyEn) -> u8 {
        StopAckDlyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Susp {
    #[doc = "Not in Suspend state."]
    XcvrNotSuspend = 0x0,
    #[doc = "In Suspend state."]
    XcvrSuspend = 0x01,
}
impl Susp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Susp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Susp {
    #[inline(always)]
    fn from(val: u8) -> Susp {
        Susp::from_bits(val)
    }
}
impl From<Susp> for u8 {
    #[inline(always)]
    fn from(val: Susp) -> u8 {
        Susp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncDet {
    #[doc = "Not detected."]
    NoSyncInt = 0x0,
    #[doc = "Detected."]
    SyncIntDetected = 0x01,
}
impl SyncDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncDet {
    #[inline(always)]
    fn from(val: u8) -> SyncDet {
        SyncDet::from_bits(val)
    }
}
impl From<SyncDet> for u8 {
    #[inline(always)]
    fn from(val: SyncDet) -> u8 {
        SyncDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tokdne {
    #[doc = "Not processed."]
    IntNo = 0x0,
    #[doc = "Processed."]
    IntYes = 0x01,
}
impl Tokdne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tokdne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tokdne {
    #[inline(always)]
    fn from(val: u8) -> Tokdne {
        Tokdne::from_bits(val)
    }
}
impl From<Tokdne> for u8 {
    #[inline(always)]
    fn from(val: Tokdne) -> u8 {
        Tokdne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tokdneen {
    #[doc = "Disable."]
    DisTokdneInt = 0x0,
    #[doc = "Enable."]
    EnTokdneInt = 0x01,
}
impl Tokdneen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tokdneen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tokdneen {
    #[inline(always)]
    fn from(val: u8) -> Tokdneen {
        Tokdneen::from_bits(val)
    }
}
impl From<Tokdneen> for u8 {
    #[inline(always)]
    fn from(val: Tokdneen) -> u8 {
        Tokdneen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tokenpid {
    _RESERVED_0 = 0x0,
    #[doc = "OUT token. USBFS performs an OUT (TX) transaction."]
    EnOutToken = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "IN token. USBFS performs an IN (RX) transaction."]
    EnInToken = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "SETUP token. USBFS performs a SETUP (TX) transaction."]
    EnSetupToken = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tokenpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tokenpid {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tokenpid {
    #[inline(always)]
    fn from(val: u8) -> Tokenpid {
        Tokenpid::from_bits(val)
    }
}
impl From<Tokenpid> for u8 {
    #[inline(always)]
    fn from(val: Tokenpid) -> u8 {
        Tokenpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimInitValSel {
    #[doc = "Mid-scale."]
    InitTrimFineMid = 0x0,
    #[doc = "IFR."]
    InitTrimFineIfr = 0x01,
}
impl TrimInitValSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimInitValSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimInitValSel {
    #[inline(always)]
    fn from(val: u8) -> TrimInitValSel {
        TrimInitValSel::from_bits(val)
    }
}
impl From<TrimInitValSel> for u8 {
    #[inline(always)]
    fn from(val: TrimInitValSel) -> u8 {
        TrimInitValSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tx {
    #[doc = "Receive."]
    RxTransaction = 0x0,
    #[doc = "Transmit."]
    TxTransaction = 0x01,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartchls {
    #[doc = "USB DP and DM signals are used as UART TX/RX."]
    UartDpTx = 0x0,
    #[doc = "USB DP and DM signals are used as UART RX/TX."]
    UartDmTx = 0x01,
}
impl Uartchls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartchls {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartchls {
    #[inline(always)]
    fn from(val: u8) -> Uartchls {
        Uartchls::from_bits(val)
    }
}
impl From<Uartchls> for u8 {
    #[inline(always)]
    fn from(val: Uartchls) -> u8 {
        Uartchls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartsel {
    #[doc = "USB DP and DM external package pins are used for USB signaling."]
    UsbMode = 0x0,
    #[doc = "USB DP and DM external package pins are used for UART signaling."]
    UartMode = 0x01,
}
impl Uartsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartsel {
    #[inline(always)]
    fn from(val: u8) -> Uartsel {
        Uartsel::from_bits(val)
    }
}
impl From<Uartsel> for u8 {
    #[inline(always)]
    fn from(val: Uartsel) -> u8 {
        Uartsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbResumeInt {
    #[doc = "Not generated."]
    NoAsyncInt = 0x0,
    #[doc = "Generated because of the USB asynchronous interrupt."]
    SyncIntGenerated = 0x01,
}
impl UsbResumeInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbResumeInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbResumeInt {
    #[inline(always)]
    fn from(val: u8) -> UsbResumeInt {
        UsbResumeInt::from_bits(val)
    }
}
impl From<UsbResumeInt> for u8 {
    #[inline(always)]
    fn from(val: UsbResumeInt) -> u8 {
        UsbResumeInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbensofen {
    #[doc = "Disable."]
    DisUsbSof = 0x0,
    #[doc = "Enable."]
    EnUsbSof = 0x01,
}
impl Usbensofen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbensofen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbensofen {
    #[inline(always)]
    fn from(val: u8) -> Usbensofen {
        Usbensofen::from_bits(val)
    }
}
impl From<Usbensofen> for u8 {
    #[inline(always)]
    fn from(val: Usbensofen) -> u8 {
        Usbensofen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbreset {
    #[doc = "Normal USBFS operation."]
    NormalOperation = 0x0,
    #[doc = "Returns USBFS to its reset state."]
    ForceHardReset = 0x01,
}
impl Usbreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbreset {
    #[inline(always)]
    fn from(val: u8) -> Usbreset {
        Usbreset::from_bits(val)
    }
}
impl From<Usbreset> for u8 {
    #[inline(always)]
    fn from(val: Usbreset) -> u8 {
        Usbreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbresmen {
    #[doc = "Disable."]
    DisAsyncWakeup = 0x0,
    #[doc = "Enable."]
    EnAsyncWakeup = 0x01,
}
impl Usbresmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbresmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbresmen {
    #[inline(always)]
    fn from(val: u8) -> Usbresmen {
        Usbresmen::from_bits(val)
    }
}
impl From<Usbresmen> for u8 {
    #[inline(always)]
    fn from(val: Usbresmen) -> u8 {
        Usbresmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbrst {
    #[doc = "Not detected."]
    IntNo = 0x0,
    #[doc = "Detected."]
    IntYes = 0x01,
}
impl Usbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbrst {
    #[inline(always)]
    fn from(val: u8) -> Usbrst {
        Usbrst::from_bits(val)
    }
}
impl From<Usbrst> for u8 {
    #[inline(always)]
    fn from(val: Usbrst) -> u8 {
        Usbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbrsten {
    #[doc = "Disable."]
    DisUsbrstInt = 0x0,
    #[doc = "Enable."]
    EnUsbrstInt = 0x01,
}
impl Usbrsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbrsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbrsten {
    #[inline(always)]
    fn from(val: u8) -> Usbrsten {
        Usbrsten::from_bits(val)
    }
}
impl From<Usbrsten> for u8 {
    #[inline(always)]
    fn from(val: Usbrsten) -> u8 {
        Usbrsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VfedgDet {
    #[doc = "Not detected."]
    NoVregFeInt = 0x0,
    #[doc = "Detected."]
    VregFeIntDetected = 0x01,
}
impl VfedgDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VfedgDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VfedgDet {
    #[inline(always)]
    fn from(val: u8) -> VfedgDet {
        VfedgDet::from_bits(val)
    }
}
impl From<VfedgDet> for u8 {
    #[inline(always)]
    fn from(val: VfedgDet) -> u8 {
        VfedgDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VfedgEn {
    #[doc = "Disable."]
    DisVreginFeInt = 0x0,
    #[doc = "Enable."]
    EnVreginFeInt = 0x01,
}
impl VfedgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VfedgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VfedgEn {
    #[inline(always)]
    fn from(val: u8) -> VfedgEn {
        VfedgEn::from_bits(val)
    }
}
impl From<VfedgEn> for u8 {
    #[inline(always)]
    fn from(val: VfedgEn) -> u8 {
        VfedgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VredgDet {
    #[doc = "Not detected."]
    NoVregReInt = 0x0,
    #[doc = "Detected."]
    VregReIntDetected = 0x01,
}
impl VredgDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VredgDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VredgDet {
    #[inline(always)]
    fn from(val: u8) -> VredgDet {
        VredgDet::from_bits(val)
    }
}
impl From<VredgDet> for u8 {
    #[inline(always)]
    fn from(val: VredgDet) -> u8 {
        VredgDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VredgEn {
    #[doc = "Disable."]
    DisVreginReInt = 0x0,
    #[doc = "Enable."]
    EnVreginReInt = 0x01,
}
impl VredgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VredgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VredgEn {
    #[inline(always)]
    fn from(val: u8) -> VredgEn {
        VredgEn::from_bits(val)
    }
}
impl From<VredgEn> for u8 {
    #[inline(always)]
    fn from(val: VredgEn) -> u8 {
        VredgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeIntSts {
    #[doc = "Interrupt did not occur."]
    IntNo = 0x0,
    #[doc = "Interrupt occurred."]
    IntYes = 0x01,
}
impl WakeIntSts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeIntSts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeIntSts {
    #[inline(always)]
    fn from(val: u8) -> WakeIntSts {
        WakeIntSts::from_bits(val)
    }
}
impl From<WakeIntSts> for u8 {
    #[inline(always)]
    fn from(val: WakeIntSts) -> u8 {
        WakeIntSts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeOnThis {
    _RESERVED_0 = 0x0,
    #[doc = "Wake up after receiving OUT or SETUP token packet."]
    WakeOnOutSetup = 0x01,
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
    #[doc = "Wake up after receiving SETUP token packet. All other values are reserved."]
    WakeOnSetupOnly = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WakeOnThis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeOnThis {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeOnThis {
    #[inline(always)]
    fn from(val: u8) -> WakeOnThis {
        WakeOnThis::from_bits(val)
    }
}
impl From<WakeOnThis> for u8 {
    #[inline(always)]
    fn from(val: WakeOnThis) -> u8 {
        WakeOnThis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeReqEn {
    #[doc = "Disable."]
    DisWakeRequest = 0x0,
    #[doc = "Enable."]
    EnWakeRequest = 0x01,
}
impl WakeReqEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeReqEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeReqEn {
    #[inline(always)]
    fn from(val: u8) -> WakeReqEn {
        WakeReqEn::from_bits(val)
    }
}
impl From<WakeReqEn> for u8 {
    #[inline(always)]
    fn from(val: WakeReqEn) -> u8 {
        WakeReqEn::to_bits(val)
    }
}
