#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "RTC_SUBSYSTEM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcSubsystem {
    ptr: *mut u8,
}
unsafe impl Send for RtcSubsystem {}
unsafe impl Sync for RtcSubsystem {}
impl RtcSubsystem {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Subsecond Control."]
    #[inline(always)]
    pub const fn subsecond_ctrl(
        self,
    ) -> crate::pac::common::Reg<SubsecondCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Subsecond Counter."]
    #[inline(always)]
    pub const fn subsecond_cnt(
        self,
    ) -> crate::pac::common::Reg<SubsecondCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Wake Timer Control."]
    #[inline(always)]
    pub const fn wake_timer_ctrl(
        self,
    ) -> crate::pac::common::Reg<WakeTimerCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Wake Timer Counter."]
    #[inline(always)]
    pub const fn wake_timer_cnt(
        self,
    ) -> crate::pac::common::Reg<WakeTimerCnt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c0cusize) as _) }
    }
}
#[doc = "Subsecond Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubsecondCnt(pub u32);
impl SubsecondCnt {
    #[doc = "Current Subsecond Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn subsecond_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current Subsecond Counter Value."]
    #[inline(always)]
    pub const fn set_subsecond_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SubsecondCnt {
    #[inline(always)]
    fn default() -> SubsecondCnt {
        SubsecondCnt(0)
    }
}
impl core::fmt::Debug for SubsecondCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SubsecondCnt")
            .field("subsecond_cnt", &self.subsecond_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SubsecondCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SubsecondCnt {{ subsecond_cnt: {=u16:?} }}",
            self.subsecond_cnt()
        )
    }
}
#[doc = "Subsecond Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubsecondCtrl(pub u32);
impl SubsecondCtrl {
    #[doc = "Subsecond Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sub_second_cnt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Subsecond Counter Enable."]
    #[inline(always)]
    pub const fn set_sub_second_cnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SubsecondCtrl {
    #[inline(always)]
    fn default() -> SubsecondCtrl {
        SubsecondCtrl(0)
    }
}
impl core::fmt::Debug for SubsecondCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SubsecondCtrl")
            .field("sub_second_cnt_en", &self.sub_second_cnt_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SubsecondCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SubsecondCtrl {{ sub_second_cnt_en: {=bool:?} }}",
            self.sub_second_cnt_en()
        )
    }
}
#[doc = "Wake Timer Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeTimerCnt(pub u32);
impl WakeTimerCnt {
    #[doc = "Wake Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Wake Counter."]
    #[inline(always)]
    pub const fn set_wake_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WakeTimerCnt {
    #[inline(always)]
    fn default() -> WakeTimerCnt {
        WakeTimerCnt(0)
    }
}
impl core::fmt::Debug for WakeTimerCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WakeTimerCnt")
            .field("wake_cnt", &self.wake_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WakeTimerCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WakeTimerCnt {{ wake_cnt: {=u32:?} }}", self.wake_cnt())
    }
}
#[doc = "Wake Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeTimerCtrl(pub u32);
impl WakeTimerCtrl {
    #[doc = "Wake Timer Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_flag(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wake Timer Status Flag."]
    #[inline(always)]
    pub const fn set_wake_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear Wake Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_wake_timer(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Wake Timer."]
    #[inline(always)]
    pub const fn set_clr_wake_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OSC Divide Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_div_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OSC Divide Enable."]
    #[inline(always)]
    pub const fn set_osc_div_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt."]
    #[inline(always)]
    pub const fn set_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for WakeTimerCtrl {
    #[inline(always)]
    fn default() -> WakeTimerCtrl {
        WakeTimerCtrl(0)
    }
}
impl core::fmt::Debug for WakeTimerCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WakeTimerCtrl")
            .field("wake_flag", &self.wake_flag())
            .field("clr_wake_timer", &self.clr_wake_timer())
            .field("osc_div_ena", &self.osc_div_ena())
            .field("intr_en", &self.intr_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WakeTimerCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WakeTimerCtrl {{ wake_flag: {=bool:?}, clr_wake_timer: {=bool:?}, osc_div_ena: {=bool:?}, intr_en: {=bool:?} }}",
            self.wake_flag(),
            self.clr_wake_timer(),
            self.osc_div_ena(),
            self.intr_en()
        )
    }
}
