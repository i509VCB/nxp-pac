#[doc = "CCM_ANALOG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcmAnalog {
    ptr: *mut u8,
}
unsafe impl Send for CcmAnalog {}
unsafe impl Sync for CcmAnalog {}
impl CcmAnalog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Analog ARM PLL control Register"]
    #[inline(always)]
    pub const fn pll_arm(self) -> crate::common::Reg<regs::PllArm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Analog ARM PLL control Register"]
    #[inline(always)]
    pub const fn pll_arm_set(self) -> crate::common::Reg<regs::PllArmSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Analog ARM PLL control Register"]
    #[inline(always)]
    pub const fn pll_arm_clr(self) -> crate::common::Reg<regs::PllArmClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Analog ARM PLL control Register"]
    #[inline(always)]
    pub const fn pll_arm_tog(self) -> crate::common::Reg<regs::PllArmTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1(self) -> crate::common::Reg<regs::PllUsb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1_set(self) -> crate::common::Reg<regs::PllUsb1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1_clr(self) -> crate::common::Reg<regs::PllUsb1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1_tog(self) -> crate::common::Reg<regs::PllUsb1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb2(self) -> crate::common::Reg<regs::PllUsb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb2_set(self) -> crate::common::Reg<regs::PllUsb2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb2_clr(self) -> crate::common::Reg<regs::PllUsb2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb2_tog(self) -> crate::common::Reg<regs::PllUsb2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys(self) -> crate::common::Reg<regs::PllSys, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys_set(self) -> crate::common::Reg<regs::PllSysSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys_clr(self) -> crate::common::Reg<regs::PllSysClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys_tog(self) -> crate::common::Reg<regs::PllSysTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "528MHz System PLL Spread Spectrum Register"]
    #[inline(always)]
    pub const fn pll_sys_ss(self) -> crate::common::Reg<regs::PllSysSs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_sys_num(self) -> crate::common::Reg<regs::PllSysNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_sys_denom(self) -> crate::common::Reg<regs::PllSysDenom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio(self) -> crate::common::Reg<regs::PllAudio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio_set(self) -> crate::common::Reg<regs::PllAudioSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio_clr(self) -> crate::common::Reg<regs::PllAudioClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio_tog(self) -> crate::common::Reg<regs::PllAudioTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_audio_num(self) -> crate::common::Reg<regs::PllAudioNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_audio_denom(
        self,
    ) -> crate::common::Reg<regs::PllAudioDenom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Analog Video PLL control Register"]
    #[inline(always)]
    pub const fn pll_video(self) -> crate::common::Reg<regs::PllVideo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Analog Video PLL control Register"]
    #[inline(always)]
    pub const fn pll_video_set(self) -> crate::common::Reg<regs::PllVideoSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Analog Video PLL control Register"]
    #[inline(always)]
    pub const fn pll_video_clr(self) -> crate::common::Reg<regs::PllVideoClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Analog Video PLL control Register"]
    #[inline(always)]
    pub const fn pll_video_tog(self) -> crate::common::Reg<regs::PllVideoTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_video_num(self) -> crate::common::Reg<regs::PllVideoNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_video_denom(
        self,
    ) -> crate::common::Reg<regs::PllVideoDenom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet(self) -> crate::common::Reg<regs::PllEnet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet_set(self) -> crate::common::Reg<regs::PllEnetSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet_clr(self) -> crate::common::Reg<regs::PllEnetClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet_tog(self) -> crate::common::Reg<regs::PllEnetTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480(self) -> crate::common::Reg<regs::Pfd480, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480_set(self) -> crate::common::Reg<regs::Pfd480Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480_clr(self) -> crate::common::Reg<regs::Pfd480Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480_tog(self) -> crate::common::Reg<regs::Pfd480Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528(self) -> crate::common::Reg<regs::Pfd528, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528_set(self) -> crate::common::Reg<regs::Pfd528Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528_clr(self) -> crate::common::Reg<regs::Pfd528Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528_tog(self) -> crate::common::Reg<regs::Pfd528Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0(self) -> crate::common::Reg<regs::Misc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_set(self) -> crate::common::Reg<regs::Misc0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_clr(self) -> crate::common::Reg<regs::Misc0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_tog(self) -> crate::common::Reg<regs::Misc0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1(self) -> crate::common::Reg<regs::Misc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_set(self) -> crate::common::Reg<regs::Misc1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_clr(self) -> crate::common::Reg<regs::Misc1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_tog(self) -> crate::common::Reg<regs::Misc1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2(self) -> crate::common::Reg<regs::Misc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2_set(self) -> crate::common::Reg<regs::Misc2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2_clr(self) -> crate::common::Reg<regs::Misc2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2_tog(self) -> crate::common::Reg<regs::Misc2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
