#[doc = "Capture register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP(pub u32);
impl CAP {
    #[doc = "Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP_VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub const fn set_CAP_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAP {
    #[inline(always)]
    fn default() -> CAP {
        CAP(0)
    }
}
impl core::fmt::Debug for CAP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP")
            .field("CAP_VALUE", &self.CAP_VALUE())
            .field("VALID", &self.VALID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP {{ CAP_VALUE: {=u32:?}, VALID: {=bool:?} }}",
            self.CAP_VALUE(),
            self.VALID()
        )
    }
}
#[doc = "Capture clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCLR(pub u32);
impl CAPCLR {
    #[doc = "Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCLR0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    pub const fn set_CAPCLR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCLR1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    pub const fn set_CAPCLR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCLR2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    pub const fn set_CAPCLR2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCLR3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    pub const fn set_CAPCLR3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for CAPCLR {
    #[inline(always)]
    fn default() -> CAPCLR {
        CAPCLR(0)
    }
}
impl core::fmt::Debug for CAPCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCLR")
            .field("CAPCLR0", &self.CAPCLR0())
            .field("CAPCLR1", &self.CAPCLR1())
            .field("CAPCLR2", &self.CAPCLR2())
            .field("CAPCLR3", &self.CAPCLR3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCLR {{ CAPCLR0: {=bool:?}, CAPCLR1: {=bool:?}, CAPCLR2: {=bool:?}, CAPCLR3: {=bool:?} }}",
            self.CAPCLR0(),
            self.CAPCLR1(),
            self.CAPCLR2(),
            self.CAPCLR3()
        )
    }
}
#[doc = "Capture configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPEN0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_CAPEN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPEN1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_CAPEN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPEN2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_CAPEN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPEN3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_CAPEN3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPPOL0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_CAPPOL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPPOL1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_CAPPOL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPPOL2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_CAPPOL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPPOL3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_CAPPOL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            .field("CAPEN0", &self.CAPEN0())
            .field("CAPEN1", &self.CAPEN1())
            .field("CAPEN2", &self.CAPEN2())
            .field("CAPEN3", &self.CAPEN3())
            .field("CAPPOL0", &self.CAPPOL0())
            .field("CAPPOL1", &self.CAPPOL1())
            .field("CAPPOL2", &self.CAPPOL2())
            .field("CAPPOL3", &self.CAPPOL3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ CAPEN0: {=bool:?}, CAPEN1: {=bool:?}, CAPEN2: {=bool:?}, CAPEN3: {=bool:?}, CAPPOL0: {=bool:?}, CAPPOL1: {=bool:?}, CAPPOL2: {=bool:?}, CAPPOL3: {=bool:?} }}",
            self.CAPEN0(),
            self.CAPEN1(),
            self.CAPEN2(),
            self.CAPEN3(),
            self.CAPPOL0(),
            self.CAPPOL1(),
            self.CAPPOL2(),
            self.CAPPOL3()
        )
    }
}
#[doc = "Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAYVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub const fn set_DELAYVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[must_use]
    #[inline(always)]
    pub const fn REPEAT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub const fn set_REPEAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("DELAYVAL", &self.DELAYVAL())
            .field("REPEAT", &self.REPEAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ DELAYVAL: {=u32:?}, REPEAT: {=bool:?} }}",
            self.DELAYVAL(),
            self.REPEAT()
        )
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[must_use]
    #[inline(always)]
    pub const fn INTR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[inline(always)]
    pub const fn set_INTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("INTR", &self.INTR())
            .field("ACTIVE", &self.ACTIVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ INTR: {=bool:?}, ACTIVE: {=bool:?} }}",
            self.INTR(),
            self.ACTIVE()
        )
    }
}
