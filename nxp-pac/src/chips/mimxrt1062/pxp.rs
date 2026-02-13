#[doc = "PXP v2.0 Register Reference Index"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pxp {
    ptr: *mut u8,
}
unsafe impl Send for Pxp {}
unsafe impl Sync for Pxp {}
impl Pxp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register 0"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register 0"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register 0"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Control Register 0"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn stat_set(self) -> crate::common::Reg<regs::StatSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn stat_clr(self) -> crate::common::Reg<regs::StatClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn stat_tog(self) -> crate::common::Reg<regs::StatTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Output Buffer Control Register"]
    #[inline(always)]
    pub const fn out_ctrl(self) -> crate::common::Reg<regs::OutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Output Buffer Control Register"]
    #[inline(always)]
    pub const fn out_ctrl_set(self) -> crate::common::Reg<regs::OutCtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Output Buffer Control Register"]
    #[inline(always)]
    pub const fn out_ctrl_clr(self) -> crate::common::Reg<regs::OutCtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Buffer Control Register"]
    #[inline(always)]
    pub const fn out_ctrl_tog(self) -> crate::common::Reg<regs::OutCtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Output Frame Buffer Pointer"]
    #[inline(always)]
    pub const fn out_buf(self) -> crate::common::Reg<regs::OutBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Output Frame Buffer Pointer #2"]
    #[inline(always)]
    pub const fn out_buf2(self) -> crate::common::Reg<regs::OutBuf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Output Buffer Pitch"]
    #[inline(always)]
    pub const fn out_pitch(self) -> crate::common::Reg<regs::OutPitch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Output Surface Lower Right Coordinate"]
    #[inline(always)]
    pub const fn out_lrc(self) -> crate::common::Reg<regs::OutLrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Processed Surface Upper Left Coordinate"]
    #[inline(always)]
    pub const fn out_ps_ulc(self) -> crate::common::Reg<regs::OutPsUlc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Processed Surface Lower Right Coordinate"]
    #[inline(always)]
    pub const fn out_ps_lrc(self) -> crate::common::Reg<regs::OutPsLrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Alpha Surface Upper Left Coordinate"]
    #[inline(always)]
    pub const fn out_as_ulc(self) -> crate::common::Reg<regs::OutAsUlc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Alpha Surface Lower Right Coordinate"]
    #[inline(always)]
    pub const fn out_as_lrc(self) -> crate::common::Reg<regs::OutAsLrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Processed Surface (PS) Control Register"]
    #[inline(always)]
    pub const fn ps_ctrl(self) -> crate::common::Reg<regs::PsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Processed Surface (PS) Control Register"]
    #[inline(always)]
    pub const fn ps_ctrl_set(self) -> crate::common::Reg<regs::PsCtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Processed Surface (PS) Control Register"]
    #[inline(always)]
    pub const fn ps_ctrl_clr(self) -> crate::common::Reg<regs::PsCtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Processed Surface (PS) Control Register"]
    #[inline(always)]
    pub const fn ps_ctrl_tog(self) -> crate::common::Reg<regs::PsCtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "PS Input Buffer Address"]
    #[inline(always)]
    pub const fn ps_buf(self) -> crate::common::Reg<regs::PsBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
    #[inline(always)]
    pub const fn ps_ubuf(self) -> crate::common::Reg<regs::PsUbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "PS V/Cr Input Buffer Address"]
    #[inline(always)]
    pub const fn ps_vbuf(self) -> crate::common::Reg<regs::PsVbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Processed Surface Pitch"]
    #[inline(always)]
    pub const fn ps_pitch(self) -> crate::common::Reg<regs::PsPitch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "PS Background Color"]
    #[inline(always)]
    pub const fn ps_background(self) -> crate::common::Reg<regs::PsBackground, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "PS Scale Factor Register"]
    #[inline(always)]
    pub const fn ps_scale(self) -> crate::common::Reg<regs::PsScale, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "PS Scale Offset Register"]
    #[inline(always)]
    pub const fn ps_offset(self) -> crate::common::Reg<regs::PsOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "PS Color Key Low"]
    #[inline(always)]
    pub const fn ps_clrkeylow(self) -> crate::common::Reg<regs::PsClrkeylow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "PS Color Key High"]
    #[inline(always)]
    pub const fn ps_clrkeyhigh(self) -> crate::common::Reg<regs::PsClrkeyhigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Alpha Surface Control"]
    #[inline(always)]
    pub const fn as_ctrl(self) -> crate::common::Reg<regs::AsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Alpha Surface Buffer Pointer"]
    #[inline(always)]
    pub const fn as_buf(self) -> crate::common::Reg<regs::AsBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Alpha Surface Pitch"]
    #[inline(always)]
    pub const fn as_pitch(self) -> crate::common::Reg<regs::AsPitch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Overlay Color Key Low"]
    #[inline(always)]
    pub const fn as_clrkeylow(self) -> crate::common::Reg<regs::AsClrkeylow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Overlay Color Key High"]
    #[inline(always)]
    pub const fn as_clrkeyhigh(self) -> crate::common::Reg<regs::AsClrkeyhigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Color Space Conversion Coefficient Register 0"]
    #[inline(always)]
    pub const fn csc1_coef0(self) -> crate::common::Reg<regs::Csc1Coef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Color Space Conversion Coefficient Register 1"]
    #[inline(always)]
    pub const fn csc1_coef1(self) -> crate::common::Reg<regs::Csc1Coef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Color Space Conversion Coefficient Register 2"]
    #[inline(always)]
    pub const fn csc1_coef2(self) -> crate::common::Reg<regs::Csc1Coef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "PXP Power Control Register"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Next Frame Pointer"]
    #[inline(always)]
    pub const fn next(self) -> crate::common::Reg<regs::Next, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "PXP Alpha Engine A Control Register."]
    #[inline(always)]
    pub const fn porter_duff_ctrl(
        self,
    ) -> crate::common::Reg<regs::PorterDuffCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
}
pub mod regs;
pub mod vals;
