#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "PUF SRAM Controller activation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller activation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF SRAM Clock Gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ckgating(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Clock Gating control."]
    #[inline(always)]
    pub const fn set_ckgating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("enable", &self.enable())
            .field("ckgating", &self.ckgating())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ enable: {=bool:?}, ckgating: {=bool:?} }}",
            self.enable(),
            self.ckgating()
        )
    }
}
#[doc = "Interrupt Enable Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_CLR_ENABLE(pub u32);
impl INT_CLR_ENABLE {
    #[doc = "READY Interrupt Enable clear."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable clear."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable clear."]
    #[must_use]
    #[inline(always)]
    pub const fn APB_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable clear."]
    #[inline(always)]
    pub const fn set_APB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for INT_CLR_ENABLE {
    #[inline(always)]
    fn default() -> INT_CLR_ENABLE {
        INT_CLR_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_CLR_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR_ENABLE")
            .field("READY", &self.READY())
            .field("APB_ERR", &self.APB_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_CLR_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_CLR_ENABLE {{ READY: {=bool:?}, APB_ERR: {=bool:?} }}",
            self.READY(),
            self.APB_ERR()
        )
    }
}
#[doc = "Interrupt Status Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_CLR_STATUS(pub u32);
impl INT_CLR_STATUS {
    #[doc = "READY Interrupt Status clear."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status clear."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn APB_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status Clear."]
    #[inline(always)]
    pub const fn set_APB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for INT_CLR_STATUS {
    #[inline(always)]
    fn default() -> INT_CLR_STATUS {
        INT_CLR_STATUS(0)
    }
}
impl core::fmt::Debug for INT_CLR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR_STATUS")
            .field("READY", &self.READY())
            .field("APB_ERR", &self.APB_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_CLR_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_CLR_STATUS {{ READY: {=bool:?}, APB_ERR: {=bool:?} }}",
            self.READY(),
            self.APB_ERR()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_ENABLE(pub u32);
impl INT_ENABLE {
    #[doc = "READY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn APB_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_APB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for INT_ENABLE {
    #[inline(always)]
    fn default() -> INT_ENABLE {
        INT_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENABLE")
            .field("READY", &self.READY())
            .field("APB_ERR", &self.APB_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_ENABLE {{ READY: {=bool:?}, APB_ERR: {=bool:?} }}",
            self.READY(),
            self.APB_ERR()
        )
    }
}
#[doc = "Interrupt Enable Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_SET_ENABLE(pub u32);
impl INT_SET_ENABLE {
    #[doc = "READY Interrupt Enable set."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable set."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable set."]
    #[must_use]
    #[inline(always)]
    pub const fn APB_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable set."]
    #[inline(always)]
    pub const fn set_APB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for INT_SET_ENABLE {
    #[inline(always)]
    fn default() -> INT_SET_ENABLE {
        INT_SET_ENABLE(0)
    }
}
impl core::fmt::Debug for INT_SET_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_SET_ENABLE")
            .field("READY", &self.READY())
            .field("APB_ERR", &self.APB_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_SET_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_SET_ENABLE {{ READY: {=bool:?}, APB_ERR: {=bool:?} }}",
            self.READY(),
            self.APB_ERR()
        )
    }
}
#[doc = "Interrupt Status set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_SET_STATUS(pub u32);
impl INT_SET_STATUS {
    #[doc = "READY Interrupt Status set."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status set."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status Set."]
    #[must_use]
    #[inline(always)]
    pub const fn APB_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status Set."]
    #[inline(always)]
    pub const fn set_APB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for INT_SET_STATUS {
    #[inline(always)]
    fn default() -> INT_SET_STATUS {
        INT_SET_STATUS(0)
    }
}
impl core::fmt::Debug for INT_SET_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_SET_STATUS")
            .field("READY", &self.READY())
            .field("APB_ERR", &self.APB_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_SET_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_SET_STATUS {{ READY: {=bool:?}, APB_ERR: {=bool:?} }}",
            self.READY(),
            self.APB_ERR()
        )
    }
}
#[doc = "Interrupt Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_STATUS(pub u32);
impl INT_STATUS {
    #[doc = "READY Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn APB_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status."]
    #[inline(always)]
    pub const fn set_APB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for INT_STATUS {
    #[inline(always)]
    fn default() -> INT_STATUS {
        INT_STATUS(0)
    }
}
impl core::fmt::Debug for INT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("READY", &self.READY())
            .field("APB_ERR", &self.APB_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_STATUS {{ READY: {=bool:?}, APB_ERR: {=bool:?} }}",
            self.READY(),
            self.APB_ERR()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "PUF SRAM Controller State."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller State."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("READY", &self.READY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STATUS {{ READY: {=bool:?} }}", self.READY())
    }
}
