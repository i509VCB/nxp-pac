#[doc = "Wake Timer Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeTimerCnt(pub u32);
impl WakeTimerCnt {
    #[doc = "Wake Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn wake_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Wake Counter"]
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
#[doc = "Wake Timer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeTimerCtrl(pub u32);
impl WakeTimerCtrl {
    #[doc = "Wake Timer Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wake_flag(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wake Timer Status Flag"]
    #[inline(always)]
    pub const fn set_wake_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear Wake Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_wake_timer(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Wake Timer"]
    #[inline(always)]
    pub const fn set_clr_wake_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OSC Divide Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_div_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OSC Divide Enable"]
    #[inline(always)]
    pub const fn set_osc_div_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt"]
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
