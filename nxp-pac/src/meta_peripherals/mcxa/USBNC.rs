#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "USBNC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbnc {
    ptr: *mut u8,
}
unsafe impl Send for Usbnc {}
unsafe impl Sync for Usbnc {}
impl Usbnc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB Control 1."]
    #[inline(always)]
    pub const fn CTRL1(self) -> crate::pac::common::Reg<CTRL1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB Control 2."]
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::pac::common::Reg<CTRL2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "HSIC DLL Configure Register 4."]
    #[inline(always)]
    pub const fn HSIC_DLL_CFG4(
        self,
    ) -> crate::pac::common::Reg<HSIC_DLL_CFG4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "USB LPM Control and Status 0."]
    #[inline(always)]
    pub const fn LPM_CSR0(self) -> crate::pac::common::Reg<LPM_CSR0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "USB LPM Control and Status 1."]
    #[inline(always)]
    pub const fn LPM_CSR1(self) -> crate::pac::common::Reg<LPM_CSR1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "USB LPM Control and Status 2."]
    #[inline(always)]
    pub const fn LPM_CSR2(self) -> crate::pac::common::Reg<LPM_CSR2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "USB Clock Recovery Control."]
    #[inline(always)]
    pub const fn CLK_RECOVER_CTRL(
        self,
    ) -> crate::pac::common::Reg<CLK_RECOVER_CTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FIRC Oscillator Enable."]
    #[inline(always)]
    pub const fn CLK_RECOVER_IRC_EN(
        self,
    ) -> crate::pac::common::Reg<CLK_RECOVER_IRC_EN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Clock Recovery Combined Interrupt Enable."]
    #[inline(always)]
    pub const fn CLK_RECOVER_INT_EN(
        self,
    ) -> crate::pac::common::Reg<CLK_RECOVER_INT_EN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Clock Recovery Separated Interrupt Status."]
    #[inline(always)]
    pub const fn CLK_RECOVER_INT_STATUS(
        self,
    ) -> crate::pac::common::Reg<CLK_RECOVER_INT_STATUS, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
}
#[doc = "USB Clock Recovery Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLK_RECOVER_CTRL(pub u8);
impl CLK_RECOVER_CTRL {
    #[doc = "Selects the source for the initial FIRC192M trim fine value used after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_INIT_VAL_SEL(&self) -> TRIM_INIT_VAL_SEL {
        let val = (self.0 >> 3usize) & 0x01;
        TRIM_INIT_VAL_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for the initial FIRC192M trim fine value used after a reset."]
    #[inline(always)]
    pub const fn set_TRIM_INIT_VAL_SEL(&mut self, val: TRIM_INIT_VAL_SEL) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Restart from IFR Trim Value."]
    #[must_use]
    #[inline(always)]
    pub const fn RESTART_IFRTRIM_EN(&self) -> RESTART_IFRTRIM_EN {
        let val = (self.0 >> 5usize) & 0x01;
        RESTART_IFRTRIM_EN::from_bits(val as u8)
    }
    #[doc = "Restart from IFR Trim Value."]
    #[inline(always)]
    pub const fn set_RESTART_IFRTRIM_EN(&mut self, val: RESTART_IFRTRIM_EN) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reset or Resume to Rough Phase Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_RESUME_ROUGH_EN(&self) -> RESET_RESUME_ROUGH_EN {
        let val = (self.0 >> 6usize) & 0x01;
        RESET_RESUME_ROUGH_EN::from_bits(val as u8)
    }
    #[doc = "Reset or Resume to Rough Phase Enable."]
    #[inline(always)]
    pub const fn set_RESET_RESUME_ROUGH_EN(&mut self, val: RESET_RESUME_ROUGH_EN) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Crystal-Less USB Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CLOCK_RECOVER_EN(&self) -> CLOCK_RECOVER_EN {
        let val = (self.0 >> 7usize) & 0x01;
        CLOCK_RECOVER_EN::from_bits(val as u8)
    }
    #[doc = "Crystal-Less USB Enable."]
    #[inline(always)]
    pub const fn set_CLOCK_RECOVER_EN(&mut self, val: CLOCK_RECOVER_EN) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for CLK_RECOVER_CTRL {
    #[inline(always)]
    fn default() -> CLK_RECOVER_CTRL {
        CLK_RECOVER_CTRL(0)
    }
}
impl core::fmt::Debug for CLK_RECOVER_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_RECOVER_CTRL")
            .field("TRIM_INIT_VAL_SEL", &self.TRIM_INIT_VAL_SEL())
            .field("RESTART_IFRTRIM_EN", &self.RESTART_IFRTRIM_EN())
            .field("RESET_RESUME_ROUGH_EN", &self.RESET_RESUME_ROUGH_EN())
            .field("CLOCK_RECOVER_EN", &self.CLOCK_RECOVER_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLK_RECOVER_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLK_RECOVER_CTRL {{ TRIM_INIT_VAL_SEL: {:?}, RESTART_IFRTRIM_EN: {:?}, RESET_RESUME_ROUGH_EN: {:?}, CLOCK_RECOVER_EN: {:?} }}",
            self.TRIM_INIT_VAL_SEL(),
            self.RESTART_IFRTRIM_EN(),
            self.RESET_RESUME_ROUGH_EN(),
            self.CLOCK_RECOVER_EN()
        )
    }
}
#[doc = "Clock Recovery Combined Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLK_RECOVER_INT_EN(pub u8);
impl CLK_RECOVER_INT_EN {
    #[doc = "Overflow error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn OVF_ERROR_EN(&self) -> OVF_ERROR_EN {
        let val = (self.0 >> 4usize) & 0x01;
        OVF_ERROR_EN::from_bits(val as u8)
    }
    #[doc = "Overflow error interrupt enable."]
    #[inline(always)]
    pub const fn set_OVF_ERROR_EN(&mut self, val: OVF_ERROR_EN) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for CLK_RECOVER_INT_EN {
    #[inline(always)]
    fn default() -> CLK_RECOVER_INT_EN {
        CLK_RECOVER_INT_EN(0)
    }
}
impl core::fmt::Debug for CLK_RECOVER_INT_EN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_RECOVER_INT_EN")
            .field("OVF_ERROR_EN", &self.OVF_ERROR_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLK_RECOVER_INT_EN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLK_RECOVER_INT_EN {{ OVF_ERROR_EN: {:?} }}",
            self.OVF_ERROR_EN()
        )
    }
}
#[doc = "Clock Recovery Separated Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLK_RECOVER_INT_STATUS(pub u8);
impl CLK_RECOVER_INT_STATUS {
    #[doc = "Overflow Error Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn OVF_ERROR(&self) -> OVF_ERROR {
        let val = (self.0 >> 4usize) & 0x01;
        OVF_ERROR::from_bits(val as u8)
    }
    #[doc = "Overflow Error Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_OVF_ERROR(&mut self, val: OVF_ERROR) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for CLK_RECOVER_INT_STATUS {
    #[inline(always)]
    fn default() -> CLK_RECOVER_INT_STATUS {
        CLK_RECOVER_INT_STATUS(0)
    }
}
impl core::fmt::Debug for CLK_RECOVER_INT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_RECOVER_INT_STATUS")
            .field("OVF_ERROR", &self.OVF_ERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLK_RECOVER_INT_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLK_RECOVER_INT_STATUS {{ OVF_ERROR: {:?} }}",
            self.OVF_ERROR()
        )
    }
}
#[doc = "FIRC Oscillator Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLK_RECOVER_IRC_EN(pub u8);
impl CLK_RECOVER_IRC_EN {
    #[doc = "Fast IRC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn IRC_EN(&self) -> IRC_EN {
        let val = (self.0 >> 1usize) & 0x01;
        IRC_EN::from_bits(val as u8)
    }
    #[doc = "Fast IRC enable."]
    #[inline(always)]
    pub const fn set_IRC_EN(&mut self, val: IRC_EN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
}
impl Default for CLK_RECOVER_IRC_EN {
    #[inline(always)]
    fn default() -> CLK_RECOVER_IRC_EN {
        CLK_RECOVER_IRC_EN(0)
    }
}
impl core::fmt::Debug for CLK_RECOVER_IRC_EN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_RECOVER_IRC_EN")
            .field("IRC_EN", &self.IRC_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLK_RECOVER_IRC_EN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLK_RECOVER_IRC_EN {{ IRC_EN: {:?} }}", self.IRC_EN())
    }
}
#[doc = "USB Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL1(pub u32);
impl CTRL1 {
    #[doc = "Overcurrent Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn OVER_CUR_DIS(&self) -> OVER_CUR_DIS {
        let val = (self.0 >> 7usize) & 0x01;
        OVER_CUR_DIS::from_bits(val as u8)
    }
    #[doc = "Overcurrent Disable."]
    #[inline(always)]
    pub const fn set_OVER_CUR_DIS(&mut self, val: OVER_CUR_DIS) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Overcurrent Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn OVER_CUR_POL(&self) -> OVER_CUR_POL {
        let val = (self.0 >> 8usize) & 0x01;
        OVER_CUR_POL::from_bits(val as u8)
    }
    #[doc = "Overcurrent Polarity."]
    #[inline(always)]
    pub const fn set_OVER_CUR_POL(&mut self, val: OVER_CUR_POL) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Power Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn PWR_POL(&self) -> PWR_POL {
        let val = (self.0 >> 9usize) & 0x01;
        PWR_POL::from_bits(val as u8)
    }
    #[doc = "Power Polarity."]
    #[inline(always)]
    pub const fn set_PWR_POL(&mut self, val: PWR_POL) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WIE(&self) -> WIE {
        let val = (self.0 >> 10usize) & 0x01;
        WIE::from_bits(val as u8)
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_WIE(&mut self, val: WIE) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Wake-Up Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WKUP_SW_EN(&self) -> WKUP_SW_EN {
        let val = (self.0 >> 14usize) & 0x01;
        WKUP_SW_EN::from_bits(val as u8)
    }
    #[doc = "Software Wake-Up Enable."]
    #[inline(always)]
    pub const fn set_WKUP_SW_EN(&mut self, val: WKUP_SW_EN) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software Wake-Up."]
    #[must_use]
    #[inline(always)]
    pub const fn WKUP_SW(&self) -> WKUP_SW {
        let val = (self.0 >> 15usize) & 0x01;
        WKUP_SW::from_bits(val as u8)
    }
    #[doc = "Software Wake-Up."]
    #[inline(always)]
    pub const fn set_WKUP_SW(&mut self, val: WKUP_SW) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up After ID Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WKUP_ID_EN(&self) -> WKUP_ID_EN {
        let val = (self.0 >> 16usize) & 0x01;
        WKUP_ID_EN::from_bits(val as u8)
    }
    #[doc = "Wake-Up After ID Change Enable."]
    #[inline(always)]
    pub const fn set_WKUP_ID_EN(&mut self, val: WKUP_ID_EN) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up After VBUS Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WKUP_VBUS_EN(&self) -> WKUP_VBUS_EN {
        let val = (self.0 >> 17usize) & 0x01;
        WKUP_VBUS_EN::from_bits(val as u8)
    }
    #[doc = "Wake-Up After VBUS Change Enable."]
    #[inline(always)]
    pub const fn set_WKUP_VBUS_EN(&mut self, val: WKUP_VBUS_EN) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Remote Wake-Up Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn REMOTE_WAKEUP_EN(&self) -> REMOTE_WAKEUP_EN {
        let val = (self.0 >> 28usize) & 0x01;
        REMOTE_WAKEUP_EN::from_bits(val as u8)
    }
    #[doc = "Remote Wake-Up Enable."]
    #[inline(always)]
    pub const fn set_REMOTE_WAKEUP_EN(&mut self, val: REMOTE_WAKEUP_EN) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Wake-Up After DP or DM Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WKUP_DPDM_EN(&self) -> WKUP_DPDM_EN {
        let val = (self.0 >> 29usize) & 0x01;
        WKUP_DPDM_EN::from_bits(val as u8)
    }
    #[doc = "Wake-Up After DP or DM Change Enable."]
    #[inline(always)]
    pub const fn set_WKUP_DPDM_EN(&mut self, val: WKUP_DPDM_EN) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-Up Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn WIR(&self) -> WIR {
        let val = (self.0 >> 31usize) & 0x01;
        WIR::from_bits(val as u8)
    }
    #[doc = "Wake-Up Interrupt Request."]
    #[inline(always)]
    pub const fn set_WIR(&mut self, val: WIR) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL1 {
    #[inline(always)]
    fn default() -> CTRL1 {
        CTRL1(0)
    }
}
impl core::fmt::Debug for CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("OVER_CUR_DIS", &self.OVER_CUR_DIS())
            .field("OVER_CUR_POL", &self.OVER_CUR_POL())
            .field("PWR_POL", &self.PWR_POL())
            .field("WIE", &self.WIE())
            .field("WKUP_SW_EN", &self.WKUP_SW_EN())
            .field("WKUP_SW", &self.WKUP_SW())
            .field("WKUP_ID_EN", &self.WKUP_ID_EN())
            .field("WKUP_VBUS_EN", &self.WKUP_VBUS_EN())
            .field("REMOTE_WAKEUP_EN", &self.REMOTE_WAKEUP_EN())
            .field("WKUP_DPDM_EN", &self.WKUP_DPDM_EN())
            .field("WIR", &self.WIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL1 {{ OVER_CUR_DIS: {:?}, OVER_CUR_POL: {:?}, PWR_POL: {:?}, WIE: {:?}, WKUP_SW_EN: {:?}, WKUP_SW: {:?}, WKUP_ID_EN: {:?}, WKUP_VBUS_EN: {:?}, REMOTE_WAKEUP_EN: {:?}, WKUP_DPDM_EN: {:?}, WIR: {:?} }}",
            self.OVER_CUR_DIS(),
            self.OVER_CUR_POL(),
            self.PWR_POL(),
            self.WIE(),
            self.WKUP_SW_EN(),
            self.WKUP_SW(),
            self.WKUP_ID_EN(),
            self.WKUP_VBUS_EN(),
            self.REMOTE_WAKEUP_EN(),
            self.WKUP_DPDM_EN(),
            self.WIR()
        )
    }
}
#[doc = "USB Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL2(pub u32);
impl CTRL2 {
    #[doc = "VBUS Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> VBUS_SOURCE_SEL {
        let val = (self.0 >> 0usize) & 0x03;
        VBUS_SOURCE_SEL::from_bits(val as u8)
    }
    #[doc = "VBUS Source Select."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(&mut self, val: VBUS_SOURCE_SEL) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "UTMI Clock Valid Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_CLK_VLD(&self) -> UTMI_CLK_VLD {
        let val = (self.0 >> 31usize) & 0x01;
        UTMI_CLK_VLD::from_bits(val as u8)
    }
    #[doc = "UTMI Clock Valid Flag."]
    #[inline(always)]
    pub const fn set_UTMI_CLK_VLD(&mut self, val: UTMI_CLK_VLD) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL2 {
    #[inline(always)]
    fn default() -> CTRL2 {
        CTRL2(0)
    }
}
impl core::fmt::Debug for CTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("UTMI_CLK_VLD", &self.UTMI_CLK_VLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL2 {{ VBUS_SOURCE_SEL: {:?}, UTMI_CLK_VLD: {:?} }}",
            self.VBUS_SOURCE_SEL(),
            self.UTMI_CLK_VLD()
        )
    }
}
#[doc = "HSIC DLL Configure Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSIC_DLL_CFG4(pub u32);
impl HSIC_DLL_CFG4 {
    #[doc = "LPM EXT token ENDP check enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_EN_ENDP_CHK(&self) -> LPM_EN_ENDP_CHK {
        let val = (self.0 >> 30usize) & 0x01;
        LPM_EN_ENDP_CHK::from_bits(val as u8)
    }
    #[doc = "LPM EXT token ENDP check enable."]
    #[inline(always)]
    pub const fn set_LPM_EN_ENDP_CHK(&mut self, val: LPM_EN_ENDP_CHK) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FS Isochronous back to back transfer enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FS_ISO_B2B_FIXEN(&self) -> FS_ISO_B2B_FIXEN {
        let val = (self.0 >> 31usize) & 0x01;
        FS_ISO_B2B_FIXEN::from_bits(val as u8)
    }
    #[doc = "FS Isochronous back to back transfer enable."]
    #[inline(always)]
    pub const fn set_FS_ISO_B2B_FIXEN(&mut self, val: FS_ISO_B2B_FIXEN) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for HSIC_DLL_CFG4 {
    #[inline(always)]
    fn default() -> HSIC_DLL_CFG4 {
        HSIC_DLL_CFG4(0)
    }
}
impl core::fmt::Debug for HSIC_DLL_CFG4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSIC_DLL_CFG4")
            .field("LPM_EN_ENDP_CHK", &self.LPM_EN_ENDP_CHK())
            .field("FS_ISO_B2B_FIXEN", &self.FS_ISO_B2B_FIXEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSIC_DLL_CFG4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSIC_DLL_CFG4 {{ LPM_EN_ENDP_CHK: {:?}, FS_ISO_B2B_FIXEN: {:?} }}",
            self.LPM_EN_ENDP_CHK(),
            self.FS_ISO_B2B_FIXEN()
        )
    }
}
#[doc = "USB LPM Control and Status 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPM_CSR0(pub u32);
impl LPM_CSR0 {
    #[doc = "Link Power Management Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_EN(&self) -> LPM_EN {
        let val = (self.0 >> 0usize) & 0x01;
        LPM_EN::from_bits(val as u8)
    }
    #[doc = "Link Power Management Feature Enable."]
    #[inline(always)]
    pub const fn set_LPM_EN(&mut self, val: LPM_EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Power Management ECN Errata Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_ERRATA_EN(&self) -> LPM_ERRATA_EN {
        let val = (self.0 >> 1usize) & 0x01;
        LPM_ERRATA_EN::from_bits(val as u8)
    }
    #[doc = "Link Power Management ECN Errata Feature Enable."]
    #[inline(always)]
    pub const fn set_LPM_ERRATA_EN(&mut self, val: LPM_ERRATA_EN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto Low-Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_AUTO_PHCD(&self) -> LPM_AUTO_PHCD {
        let val = (self.0 >> 3usize) & 0x01;
        LPM_AUTO_PHCD::from_bits(val as u8)
    }
    #[doc = "Auto Low-Power Mode."]
    #[inline(always)]
    pub const fn set_LPM_AUTO_PHCD(&mut self, val: LPM_AUTO_PHCD) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPM Resume OK."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_RESUMEOK(&self) -> LPM_RESUMEOK {
        let val = (self.0 >> 30usize) & 0x01;
        LPM_RESUMEOK::from_bits(val as u8)
    }
    #[doc = "LPM Resume OK."]
    #[inline(always)]
    pub const fn set_LPM_RESUMEOK(&mut self, val: LPM_RESUMEOK) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "LPM Active."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_L1_ACTIVE(&self) -> LPM_L1_ACTIVE {
        let val = (self.0 >> 31usize) & 0x01;
        LPM_L1_ACTIVE::from_bits(val as u8)
    }
    #[doc = "LPM Active."]
    #[inline(always)]
    pub const fn set_LPM_L1_ACTIVE(&mut self, val: LPM_L1_ACTIVE) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for LPM_CSR0 {
    #[inline(always)]
    fn default() -> LPM_CSR0 {
        LPM_CSR0(0)
    }
}
impl core::fmt::Debug for LPM_CSR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPM_CSR0")
            .field("LPM_EN", &self.LPM_EN())
            .field("LPM_ERRATA_EN", &self.LPM_ERRATA_EN())
            .field("LPM_AUTO_PHCD", &self.LPM_AUTO_PHCD())
            .field("LPM_RESUMEOK", &self.LPM_RESUMEOK())
            .field("LPM_L1_ACTIVE", &self.LPM_L1_ACTIVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPM_CSR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPM_CSR0 {{ LPM_EN: {:?}, LPM_ERRATA_EN: {:?}, LPM_AUTO_PHCD: {:?}, LPM_RESUMEOK: {:?}, LPM_L1_ACTIVE: {:?} }}",
            self.LPM_EN(),
            self.LPM_ERRATA_EN(),
            self.LPM_AUTO_PHCD(),
            self.LPM_RESUMEOK(),
            self.LPM_L1_ACTIVE()
        )
    }
}
#[doc = "USB LPM Control and Status 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPM_CSR1(pub u32);
impl LPM_CSR1 {
    #[doc = "Device Required Host Initiated Resume Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_BESLTHRES(&self) -> LPM_DEV_BESLTHRES {
        let val = (self.0 >> 0usize) & 0x0f;
        LPM_DEV_BESLTHRES::from_bits(val as u8)
    }
    #[doc = "Device Required Host Initiated Resume Duration."]
    #[inline(always)]
    pub const fn set_LPM_DEV_BESLTHRES(&mut self, val: LPM_DEV_BESLTHRES) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "LPM Device Response."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_RES(&self) -> LPM_DEV_RES {
        let val = (self.0 >> 4usize) & 0x01;
        LPM_DEV_RES::from_bits(val as u8)
    }
    #[doc = "LPM Device Response."]
    #[inline(always)]
    pub const fn set_LPM_DEV_RES(&mut self, val: LPM_DEV_RES) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LPM Device Data Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_DP(&self) -> LPM_DEV_DP {
        let val = (self.0 >> 5usize) & 0x01;
        LPM_DEV_DP::from_bits(val as u8)
    }
    #[doc = "LPM Device Data Pending."]
    #[inline(always)]
    pub const fn set_LPM_DEV_DP(&mut self, val: LPM_DEV_DP) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LPM Device Response Status."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_RSPSTS(&self) -> LPM_DEV_RSPSTS {
        let val = (self.0 >> 20usize) & 0x03;
        LPM_DEV_RSPSTS::from_bits(val as u8)
    }
    #[doc = "LPM Device Response Status."]
    #[inline(always)]
    pub const fn set_LPM_DEV_RSPSTS(&mut self, val: LPM_DEV_RSPSTS) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LPM Device Received bRemoteWake."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_RWKENRCVD(&self) -> LPM_DEV_RWKENRCVD {
        let val = (self.0 >> 23usize) & 0x01;
        LPM_DEV_RWKENRCVD::from_bits(val as u8)
    }
    #[doc = "LPM Device Received bRemoteWake."]
    #[inline(always)]
    pub const fn set_LPM_DEV_RWKENRCVD(&mut self, val: LPM_DEV_RWKENRCVD) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "LPM Device Received bLinkState."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_LNKSTRCVD(&self) -> LPM_DEV_LNKSTRCVD {
        let val = (self.0 >> 24usize) & 0x0f;
        LPM_DEV_LNKSTRCVD::from_bits(val as u8)
    }
    #[doc = "LPM Device Received bLinkState."]
    #[inline(always)]
    pub const fn set_LPM_DEV_LNKSTRCVD(&mut self, val: LPM_DEV_LNKSTRCVD) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "LPM Device Received BESL."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_BESLRCVD(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "LPM Device Received BESL."]
    #[inline(always)]
    pub const fn set_LPM_DEV_BESLRCVD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for LPM_CSR1 {
    #[inline(always)]
    fn default() -> LPM_CSR1 {
        LPM_CSR1(0)
    }
}
impl core::fmt::Debug for LPM_CSR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPM_CSR1")
            .field("LPM_DEV_BESLTHRES", &self.LPM_DEV_BESLTHRES())
            .field("LPM_DEV_RES", &self.LPM_DEV_RES())
            .field("LPM_DEV_DP", &self.LPM_DEV_DP())
            .field("LPM_DEV_RSPSTS", &self.LPM_DEV_RSPSTS())
            .field("LPM_DEV_RWKENRCVD", &self.LPM_DEV_RWKENRCVD())
            .field("LPM_DEV_LNKSTRCVD", &self.LPM_DEV_LNKSTRCVD())
            .field("LPM_DEV_BESLRCVD", &self.LPM_DEV_BESLRCVD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPM_CSR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPM_CSR1 {{ LPM_DEV_BESLTHRES: {:?}, LPM_DEV_RES: {:?}, LPM_DEV_DP: {:?}, LPM_DEV_RSPSTS: {:?}, LPM_DEV_RWKENRCVD: {:?}, LPM_DEV_LNKSTRCVD: {:?}, LPM_DEV_BESLRCVD: {=u8:?} }}",
            self.LPM_DEV_BESLTHRES(),
            self.LPM_DEV_RES(),
            self.LPM_DEV_DP(),
            self.LPM_DEV_RSPSTS(),
            self.LPM_DEV_RWKENRCVD(),
            self.LPM_DEV_LNKSTRCVD(),
            self.LPM_DEV_BESLRCVD()
        )
    }
}
#[doc = "USB LPM Control and Status 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPM_CSR2(pub u32);
impl LPM_CSR2 {
    #[doc = "LPM Host Send Extension Token."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_SEND(&self) -> LPM_HST_SEND {
        let val = (self.0 >> 0usize) & 0x01;
        LPM_HST_SEND::from_bits(val as u8)
    }
    #[doc = "LPM Host Send Extension Token."]
    #[inline(always)]
    pub const fn set_LPM_HST_SEND(&mut self, val: LPM_HST_SEND) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LPM Host Extension Token's Device Address."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_DEVADD(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "LPM Host Extension Token's Device Address."]
    #[inline(always)]
    pub const fn set_LPM_HST_DEVADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "LPM Host Extension Token's BESL or HIRD."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_BESL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "LPM Host Extension Token's BESL or HIRD."]
    #[inline(always)]
    pub const fn set_LPM_HST_BESL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "LPM Host Extension Token's bRemoteWake."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_RWKEN(&self) -> LPM_HST_RWKEN {
        let val = (self.0 >> 12usize) & 0x01;
        LPM_HST_RWKEN::from_bits(val as u8)
    }
    #[doc = "LPM Host Extension Token's bRemoteWake."]
    #[inline(always)]
    pub const fn set_LPM_HST_RWKEN(&mut self, val: LPM_HST_RWKEN) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "LPM Host Response Status from the Device."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_STSRCVD(&self) -> LPM_HST_STSRCVD {
        let val = (self.0 >> 28usize) & 0x07;
        LPM_HST_STSRCVD::from_bits(val as u8)
    }
    #[doc = "LPM Host Response Status from the Device."]
    #[inline(always)]
    pub const fn set_LPM_HST_STSRCVD(&mut self, val: LPM_HST_STSRCVD) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for LPM_CSR2 {
    #[inline(always)]
    fn default() -> LPM_CSR2 {
        LPM_CSR2(0)
    }
}
impl core::fmt::Debug for LPM_CSR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPM_CSR2")
            .field("LPM_HST_SEND", &self.LPM_HST_SEND())
            .field("LPM_HST_DEVADD", &self.LPM_HST_DEVADD())
            .field("LPM_HST_BESL", &self.LPM_HST_BESL())
            .field("LPM_HST_RWKEN", &self.LPM_HST_RWKEN())
            .field("LPM_HST_STSRCVD", &self.LPM_HST_STSRCVD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPM_CSR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPM_CSR2 {{ LPM_HST_SEND: {:?}, LPM_HST_DEVADD: {=u8:?}, LPM_HST_BESL: {=u8:?}, LPM_HST_RWKEN: {:?}, LPM_HST_STSRCVD: {:?} }}",
            self.LPM_HST_SEND(),
            self.LPM_HST_DEVADD(),
            self.LPM_HST_BESL(),
            self.LPM_HST_RWKEN(),
            self.LPM_HST_STSRCVD()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLOCK_RECOVER_EN {
    #[doc = "Disable."]
    DIS_CLK_RECOVER = 0x0,
    #[doc = "Enable."]
    EN_CLK_RECOVER = 0x01,
}
impl CLOCK_RECOVER_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLOCK_RECOVER_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLOCK_RECOVER_EN {
    #[inline(always)]
    fn from(val: u8) -> CLOCK_RECOVER_EN {
        CLOCK_RECOVER_EN::from_bits(val)
    }
}
impl From<CLOCK_RECOVER_EN> for u8 {
    #[inline(always)]
    fn from(val: CLOCK_RECOVER_EN) -> u8 {
        CLOCK_RECOVER_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FS_ISO_B2B_FIXEN {
    #[doc = "Disabled."]
    FS_ISO_B2B_FIXEN_0 = 0x0,
    #[doc = "Enabled."]
    FS_ISO_B2B_FIXEN_1 = 0x01,
}
impl FS_ISO_B2B_FIXEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FS_ISO_B2B_FIXEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FS_ISO_B2B_FIXEN {
    #[inline(always)]
    fn from(val: u8) -> FS_ISO_B2B_FIXEN {
        FS_ISO_B2B_FIXEN::from_bits(val)
    }
}
impl From<FS_ISO_B2B_FIXEN> for u8 {
    #[inline(always)]
    fn from(val: FS_ISO_B2B_FIXEN) -> u8 {
        FS_ISO_B2B_FIXEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRC_EN {
    #[doc = "Disable."]
    DIS_IRC = 0x0,
    #[doc = "Enable."]
    EN_IRC = 0x01,
}
impl IRC_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRC_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRC_EN {
    #[inline(always)]
    fn from(val: u8) -> IRC_EN {
        IRC_EN::from_bits(val)
    }
}
impl From<IRC_EN> for u8 {
    #[inline(always)]
    fn from(val: IRC_EN) -> u8 {
        IRC_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_AUTO_PHCD {
    #[doc = "Disable."]
    LPM_AUTO_PHCD0 = 0x0,
    #[doc = "Enable."]
    LPM_AUTO_PHCD1 = 0x01,
}
impl LPM_AUTO_PHCD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_AUTO_PHCD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_AUTO_PHCD {
    #[inline(always)]
    fn from(val: u8) -> LPM_AUTO_PHCD {
        LPM_AUTO_PHCD::from_bits(val)
    }
}
impl From<LPM_AUTO_PHCD> for u8 {
    #[inline(always)]
    fn from(val: LPM_AUTO_PHCD) -> u8 {
        LPM_AUTO_PHCD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_BESLTHRES {
    #[doc = "75 us, if LPM_ERRATA_EN = 1; 50 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES0 = 0x0,
    #[doc = "100 us, if LPM_ERRATA_EN = 1; 125 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES1 = 0x01,
    #[doc = "150 us, if LPM_ERRATA_EN = 1; 200 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES2 = 0x02,
    #[doc = "250 us, if LPM_ERRATA_EN = 1; 275 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES3 = 0x03,
    #[doc = "350 us, if LPM_ERRATA_EN = 1; 350 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES4 = 0x04,
    #[doc = "450 us, if LPM_ERRATA_EN = 1; 425 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES5 = 0x05,
    #[doc = "950 us, if LPM_ERRATA_EN = 1; 500 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES6 = 0x06,
    #[doc = "1950 us, if LPM_ERRATA_EN = 1; 575 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES7 = 0x07,
    #[doc = "2950 us, if LPM_ERRATA_EN = 1; 650 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES8 = 0x08,
    #[doc = "3950 us, if LPM_ERRATA_EN = 1; 725 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRES9 = 0x09,
    #[doc = "4950 us, if LPM_ERRATA_EN = 1; 800 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRESA = 0x0a,
    #[doc = "5950 us, if LPM_ERRATA_EN = 1; 875 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRESB = 0x0b,
    #[doc = "6950 us, if LPM_ERRATA_EN = 1; 950 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRESC = 0x0c,
    #[doc = "7950 us, if LPM_ERRATA_EN = 1; 1025 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRESD = 0x0d,
    #[doc = "8950 us, if LPM_ERRATA_EN = 1; 1100 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRESE = 0x0e,
    #[doc = "9950 us, if LPM_ERRATA_EN = 1; 1175 us, if LPM_ERRATA_EN = 0."]
    LPM_DEV_BESLTHRESF = 0x0f,
}
impl LPM_DEV_BESLTHRES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_BESLTHRES {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_BESLTHRES {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_BESLTHRES {
        LPM_DEV_BESLTHRES::from_bits(val)
    }
}
impl From<LPM_DEV_BESLTHRES> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_BESLTHRES) -> u8 {
        LPM_DEV_BESLTHRES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_DP {
    #[doc = "Not pending."]
    LPM_DEV_DP0 = 0x0,
    #[doc = "Pending."]
    LPM_DEV_DP1 = 0x01,
}
impl LPM_DEV_DP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_DP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_DP {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_DP {
        LPM_DEV_DP::from_bits(val)
    }
}
impl From<LPM_DEV_DP> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_DP) -> u8 {
        LPM_DEV_DP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_LNKSTRCVD {
    _RESERVED_0 = 0x0,
    #[doc = "L1 (Sleep mode)."]
    LPM_DEV_LNKSTRCVD1 = 0x01,
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
impl LPM_DEV_LNKSTRCVD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_LNKSTRCVD {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_LNKSTRCVD {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_LNKSTRCVD {
        LPM_DEV_LNKSTRCVD::from_bits(val)
    }
}
impl From<LPM_DEV_LNKSTRCVD> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_LNKSTRCVD) -> u8 {
        LPM_DEV_LNKSTRCVD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_RES {
    #[doc = "Fourth condition not needed."]
    LPM_DEV_RES0 = 0x0,
    #[doc = "Fourth condition needed."]
    LPM_DEV_RES1 = 0x01,
}
impl LPM_DEV_RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_RES {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_RES {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_RES {
        LPM_DEV_RES::from_bits(val)
    }
}
impl From<LPM_DEV_RES> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_RES) -> u8 {
        LPM_DEV_RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_RSPSTS {
    #[doc = "Invalid."]
    LPM_DEV_RSPSTS0 = 0x0,
    #[doc = "ACK."]
    LPM_DEV_RSPSTS1 = 0x01,
    #[doc = "NYET."]
    LPM_DEV_RSPSTS2 = 0x02,
    #[doc = "STALL."]
    LPM_DEV_RSPSTS3 = 0x03,
}
impl LPM_DEV_RSPSTS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_RSPSTS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_RSPSTS {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_RSPSTS {
        LPM_DEV_RSPSTS::from_bits(val)
    }
}
impl From<LPM_DEV_RSPSTS> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_RSPSTS) -> u8 {
        LPM_DEV_RSPSTS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_RWKENRCVD {
    #[doc = "0."]
    LPM_DEV_RWKENRCVD0 = 0x0,
    #[doc = "1."]
    LPM_DEV_RWKENRCVD1 = 0x01,
}
impl LPM_DEV_RWKENRCVD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_RWKENRCVD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_RWKENRCVD {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_RWKENRCVD {
        LPM_DEV_RWKENRCVD::from_bits(val)
    }
}
impl From<LPM_DEV_RWKENRCVD> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_RWKENRCVD) -> u8 {
        LPM_DEV_RWKENRCVD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_EN {
    #[doc = "Disable."]
    LPM_EN0 = 0x0,
    #[doc = "Enable."]
    LPM_EN1 = 0x01,
}
impl LPM_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_EN {
    #[inline(always)]
    fn from(val: u8) -> LPM_EN {
        LPM_EN::from_bits(val)
    }
}
impl From<LPM_EN> for u8 {
    #[inline(always)]
    fn from(val: LPM_EN) -> u8 {
        LPM_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_EN_ENDP_CHK {
    #[doc = "Disabled."]
    LPM_EN_ENDP_CHK_0 = 0x0,
    #[doc = "Enabled."]
    LPM_EN_ENDP_CHK_1 = 0x01,
}
impl LPM_EN_ENDP_CHK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_EN_ENDP_CHK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_EN_ENDP_CHK {
    #[inline(always)]
    fn from(val: u8) -> LPM_EN_ENDP_CHK {
        LPM_EN_ENDP_CHK::from_bits(val)
    }
}
impl From<LPM_EN_ENDP_CHK> for u8 {
    #[inline(always)]
    fn from(val: LPM_EN_ENDP_CHK) -> u8 {
        LPM_EN_ENDP_CHK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_ERRATA_EN {
    #[doc = "Disable."]
    LPM_ERRATA_EN0 = 0x0,
    #[doc = "Enable."]
    LPM_ERRATA_EN1 = 0x01,
}
impl LPM_ERRATA_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_ERRATA_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_ERRATA_EN {
    #[inline(always)]
    fn from(val: u8) -> LPM_ERRATA_EN {
        LPM_ERRATA_EN::from_bits(val)
    }
}
impl From<LPM_ERRATA_EN> for u8 {
    #[inline(always)]
    fn from(val: LPM_ERRATA_EN) -> u8 {
        LPM_ERRATA_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_HST_RWKEN {
    #[doc = "Disable."]
    LPM_HST_RWKEN0 = 0x0,
    #[doc = "Enable."]
    LPM_HST_RWKEN1 = 0x01,
}
impl LPM_HST_RWKEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_HST_RWKEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_HST_RWKEN {
    #[inline(always)]
    fn from(val: u8) -> LPM_HST_RWKEN {
        LPM_HST_RWKEN::from_bits(val)
    }
}
impl From<LPM_HST_RWKEN> for u8 {
    #[inline(always)]
    fn from(val: LPM_HST_RWKEN) -> u8 {
        LPM_HST_RWKEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_HST_SEND {
    #[doc = "LPM transaction did not happen or is complete."]
    LPM_HST_SEND0 = 0x0,
    #[doc = "LPM transaction is ongoing."]
    LPM_HST_SEND1 = 0x01,
}
impl LPM_HST_SEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_HST_SEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_HST_SEND {
    #[inline(always)]
    fn from(val: u8) -> LPM_HST_SEND {
        LPM_HST_SEND::from_bits(val)
    }
}
impl From<LPM_HST_SEND> for u8 {
    #[inline(always)]
    fn from(val: LPM_HST_SEND) -> u8 {
        LPM_HST_SEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_HST_STSRCVD {
    #[doc = "Invalid."]
    LPM_HST_STSRCVD0 = 0x0,
    #[doc = "ACK."]
    LPM_HST_STSRCVD1 = 0x01,
    #[doc = "NYET."]
    LPM_HST_STSRCVD2 = 0x02,
    #[doc = "STALL."]
    LPM_HST_STSRCVD3 = 0x03,
    #[doc = "Timeout."]
    LPM_HST_STSRCVD4 = 0x04,
    #[doc = "ERR."]
    LPM_HST_STSRCVD5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LPM_HST_STSRCVD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_HST_STSRCVD {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_HST_STSRCVD {
    #[inline(always)]
    fn from(val: u8) -> LPM_HST_STSRCVD {
        LPM_HST_STSRCVD::from_bits(val)
    }
}
impl From<LPM_HST_STSRCVD> for u8 {
    #[inline(always)]
    fn from(val: LPM_HST_STSRCVD) -> u8 {
        LPM_HST_STSRCVD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_L1_ACTIVE {
    #[doc = "Inactive."]
    LPM_L1_ACTIVE0 = 0x0,
    #[doc = "Active."]
    LPM_L1_ACTIVE1 = 0x01,
}
impl LPM_L1_ACTIVE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_L1_ACTIVE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_L1_ACTIVE {
    #[inline(always)]
    fn from(val: u8) -> LPM_L1_ACTIVE {
        LPM_L1_ACTIVE::from_bits(val)
    }
}
impl From<LPM_L1_ACTIVE> for u8 {
    #[inline(always)]
    fn from(val: LPM_L1_ACTIVE) -> u8 {
        LPM_L1_ACTIVE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_RESUMEOK {
    #[doc = "Cannot resume."]
    LPM_RESUMEOK0 = 0x0,
    #[doc = "Can resume."]
    LPM_RESUMEOK1 = 0x01,
}
impl LPM_RESUMEOK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_RESUMEOK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_RESUMEOK {
    #[inline(always)]
    fn from(val: u8) -> LPM_RESUMEOK {
        LPM_RESUMEOK::from_bits(val)
    }
}
impl From<LPM_RESUMEOK> for u8 {
    #[inline(always)]
    fn from(val: LPM_RESUMEOK) -> u8 {
        LPM_RESUMEOK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OVER_CUR_DIS {
    #[doc = "Enable."]
    OVRCRNT_DETCT_EN = 0x0,
    #[doc = "Disable."]
    OVRCRNT_DETCT_DIS = 0x01,
}
impl OVER_CUR_DIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OVER_CUR_DIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OVER_CUR_DIS {
    #[inline(always)]
    fn from(val: u8) -> OVER_CUR_DIS {
        OVER_CUR_DIS::from_bits(val)
    }
}
impl From<OVER_CUR_DIS> for u8 {
    #[inline(always)]
    fn from(val: OVER_CUR_DIS) -> u8 {
        OVER_CUR_DIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OVER_CUR_POL {
    #[doc = "Active high."]
    ACTIVE_HI_OVRCRNT = 0x0,
    #[doc = "Active low."]
    ACTIVE_LOW_OVRCRNT = 0x01,
}
impl OVER_CUR_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OVER_CUR_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OVER_CUR_POL {
    #[inline(always)]
    fn from(val: u8) -> OVER_CUR_POL {
        OVER_CUR_POL::from_bits(val)
    }
}
impl From<OVER_CUR_POL> for u8 {
    #[inline(always)]
    fn from(val: OVER_CUR_POL) -> u8 {
        OVER_CUR_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OVF_ERROR {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Unmasked interrupt occurred."]
    INT_YES = 0x01,
}
impl OVF_ERROR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OVF_ERROR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OVF_ERROR {
    #[inline(always)]
    fn from(val: u8) -> OVF_ERROR {
        OVF_ERROR::from_bits(val)
    }
}
impl From<OVF_ERROR> for u8 {
    #[inline(always)]
    fn from(val: OVF_ERROR) -> u8 {
        OVF_ERROR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OVF_ERROR_EN {
    #[doc = "The interrupt is masked."]
    MASK_OVF_ERR_INT = 0x0,
    #[doc = "The interrupt is enabled."]
    EN_OVF_ERR_INT = 0x01,
}
impl OVF_ERROR_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OVF_ERROR_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OVF_ERROR_EN {
    #[inline(always)]
    fn from(val: u8) -> OVF_ERROR_EN {
        OVF_ERROR_EN::from_bits(val)
    }
}
impl From<OVF_ERROR_EN> for u8 {
    #[inline(always)]
    fn from(val: OVF_ERROR_EN) -> u8 {
        OVF_ERROR_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWR_POL {
    #[doc = "Active low."]
    ACTIVE_LO_PMIC = 0x0,
    #[doc = "Active high."]
    ACTIVE_HI_PMIC = 0x01,
}
impl PWR_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWR_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWR_POL {
    #[inline(always)]
    fn from(val: u8) -> PWR_POL {
        PWR_POL::from_bits(val)
    }
}
impl From<PWR_POL> for u8 {
    #[inline(always)]
    fn from(val: PWR_POL) -> u8 {
        PWR_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REMOTE_WAKEUP_EN {
    #[doc = "Disable."]
    REMOTE_WKUP_DIS = 0x0,
    #[doc = "Enable."]
    REMOTE_WKUP_EN = 0x01,
}
impl REMOTE_WAKEUP_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REMOTE_WAKEUP_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REMOTE_WAKEUP_EN {
    #[inline(always)]
    fn from(val: u8) -> REMOTE_WAKEUP_EN {
        REMOTE_WAKEUP_EN::from_bits(val)
    }
}
impl From<REMOTE_WAKEUP_EN> for u8 {
    #[inline(always)]
    fn from(val: REMOTE_WAKEUP_EN) -> u8 {
        REMOTE_WAKEUP_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESET_RESUME_ROUGH_EN {
    #[doc = "Always works in tracking phase after the first time rough phase, to track transition."]
    KEEP_TRIM_FINE_ON_RESET = 0x0,
    #[doc = "Go back to rough stage whenever a bus reset or bus resume occurs."]
    USE_IFR_TRIM_FINE_ON_RESET = 0x01,
}
impl RESET_RESUME_ROUGH_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESET_RESUME_ROUGH_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESET_RESUME_ROUGH_EN {
    #[inline(always)]
    fn from(val: u8) -> RESET_RESUME_ROUGH_EN {
        RESET_RESUME_ROUGH_EN::from_bits(val)
    }
}
impl From<RESET_RESUME_ROUGH_EN> for u8 {
    #[inline(always)]
    fn from(val: RESET_RESUME_ROUGH_EN) -> u8 {
        RESET_RESUME_ROUGH_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESTART_IFRTRIM_EN {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value."]
    LOAD_TRIM_FINE_MID = 0x0,
    #[doc = "Trim fine restarts from the IFR trim value whenever you detect bus_reset or bus_resume or deassert module enable."]
    LOAD_TRIM_FINE_IFR = 0x01,
}
impl RESTART_IFRTRIM_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESTART_IFRTRIM_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESTART_IFRTRIM_EN {
    #[inline(always)]
    fn from(val: u8) -> RESTART_IFRTRIM_EN {
        RESTART_IFRTRIM_EN::from_bits(val)
    }
}
impl From<RESTART_IFRTRIM_EN> for u8 {
    #[inline(always)]
    fn from(val: RESTART_IFRTRIM_EN) -> u8 {
        RESTART_IFRTRIM_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIM_INIT_VAL_SEL {
    #[doc = "Mid-scale."]
    INIT_TRIM_FINE_MID = 0x0,
    #[doc = "IFR."]
    INIT_TRIM_FINE_IFR = 0x01,
}
impl TRIM_INIT_VAL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIM_INIT_VAL_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIM_INIT_VAL_SEL {
    #[inline(always)]
    fn from(val: u8) -> TRIM_INIT_VAL_SEL {
        TRIM_INIT_VAL_SEL::from_bits(val)
    }
}
impl From<TRIM_INIT_VAL_SEL> for u8 {
    #[inline(always)]
    fn from(val: TRIM_INIT_VAL_SEL) -> u8 {
        TRIM_INIT_VAL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UTMI_CLK_VLD {
    #[doc = "Not valid."]
    NOTVALID = 0x0,
    #[doc = "Valid."]
    VALID = 0x01,
}
impl UTMI_CLK_VLD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UTMI_CLK_VLD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UTMI_CLK_VLD {
    #[inline(always)]
    fn from(val: u8) -> UTMI_CLK_VLD {
        UTMI_CLK_VLD::from_bits(val)
    }
}
impl From<UTMI_CLK_VLD> for u8 {
    #[inline(always)]
    fn from(val: UTMI_CLK_VLD) -> u8 {
        UTMI_CLK_VLD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUS_SOURCE_SEL {
    #[doc = "vbus_valid."]
    VBUS_VALID = 0x0,
    #[doc = "sess_valid."]
    SESS_VALID_1 = 0x01,
    #[doc = "sess_valid."]
    SESS_VALID_2 = 0x02,
    #[doc = "sess_valid."]
    SESS_VALID_3 = 0x03,
}
impl VBUS_SOURCE_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUS_SOURCE_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUS_SOURCE_SEL {
    #[inline(always)]
    fn from(val: u8) -> VBUS_SOURCE_SEL {
        VBUS_SOURCE_SEL::from_bits(val)
    }
}
impl From<VBUS_SOURCE_SEL> for u8 {
    #[inline(always)]
    fn from(val: VBUS_SOURCE_SEL) -> u8 {
        VBUS_SOURCE_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIE {
    #[doc = "Disable."]
    INT_DIS = 0x0,
    #[doc = "Enable."]
    INT_EN = 0x01,
}
impl WIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIE {
    #[inline(always)]
    fn from(val: u8) -> WIE {
        WIE::from_bits(val)
    }
}
impl From<WIE> for u8 {
    #[inline(always)]
    fn from(val: WIE) -> u8 {
        WIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIR {
    #[doc = "Not received."]
    NO_WKUP_REQ = 0x0,
    #[doc = "Received."]
    WKUP_REQ = 0x01,
}
impl WIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIR {
    #[inline(always)]
    fn from(val: u8) -> WIR {
        WIR::from_bits(val)
    }
}
impl From<WIR> for u8 {
    #[inline(always)]
    fn from(val: WIR) -> u8 {
        WIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKUP_DPDM_EN {
    #[doc = "Disable only when VBUS is invalid."]
    DPDM_WKUP_DIS = 0x0,
    #[doc = "Enable (default)."]
    DPDM_WKUP_EN = 0x01,
}
impl WKUP_DPDM_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKUP_DPDM_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKUP_DPDM_EN {
    #[inline(always)]
    fn from(val: u8) -> WKUP_DPDM_EN {
        WKUP_DPDM_EN::from_bits(val)
    }
}
impl From<WKUP_DPDM_EN> for u8 {
    #[inline(always)]
    fn from(val: WKUP_DPDM_EN) -> u8 {
        WKUP_DPDM_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKUP_ID_EN {
    #[doc = "Disable."]
    WKUP_ID_DIS = 0x0,
    #[doc = "Enable."]
    WKUP_ID_EN = 0x01,
}
impl WKUP_ID_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKUP_ID_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKUP_ID_EN {
    #[inline(always)]
    fn from(val: u8) -> WKUP_ID_EN {
        WKUP_ID_EN::from_bits(val)
    }
}
impl From<WKUP_ID_EN> for u8 {
    #[inline(always)]
    fn from(val: WKUP_ID_EN) -> u8 {
        WKUP_ID_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKUP_SW {
    #[doc = "Inactive."]
    INACTIVE = 0x0,
    #[doc = "Force wake-up."]
    FORCE_WKUP = 0x01,
}
impl WKUP_SW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKUP_SW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKUP_SW {
    #[inline(always)]
    fn from(val: u8) -> WKUP_SW {
        WKUP_SW::from_bits(val)
    }
}
impl From<WKUP_SW> for u8 {
    #[inline(always)]
    fn from(val: WKUP_SW) -> u8 {
        WKUP_SW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKUP_SW_EN {
    #[doc = "Disable."]
    SW_WKUP_DIS = 0x0,
    #[doc = "Enable."]
    SW_WKUP_EN = 0x01,
}
impl WKUP_SW_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKUP_SW_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKUP_SW_EN {
    #[inline(always)]
    fn from(val: u8) -> WKUP_SW_EN {
        WKUP_SW_EN::from_bits(val)
    }
}
impl From<WKUP_SW_EN> for u8 {
    #[inline(always)]
    fn from(val: WKUP_SW_EN) -> u8 {
        WKUP_SW_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKUP_VBUS_EN {
    #[doc = "Disable."]
    WKUP_VBUS_DIS = 0x0,
    #[doc = "Enable."]
    WKUP_VBUS_EN = 0x01,
}
impl WKUP_VBUS_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKUP_VBUS_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKUP_VBUS_EN {
    #[inline(always)]
    fn from(val: u8) -> WKUP_VBUS_EN {
        WKUP_VBUS_EN::from_bits(val)
    }
}
impl From<WKUP_VBUS_EN> for u8 {
    #[inline(always)]
    fn from(val: WKUP_VBUS_EN) -> u8 {
        WKUP_VBUS_EN::to_bits(val)
    }
}
