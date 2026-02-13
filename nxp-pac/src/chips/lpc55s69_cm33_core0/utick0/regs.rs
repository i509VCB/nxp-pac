#[doc = "Capture register ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub const fn set_cap_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cap {
    #[inline(always)]
    fn default() -> Cap {
        Cap(0)
    }
}
impl core::fmt::Debug for Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap")
            .field("cap_value", &self.cap_value())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap {{ cap_value: {=u32:?}, valid: {=bool:?} }}",
            self.cap_value(),
            self.valid()
        )
    }
}
#[doc = "Capture clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capclr(pub u32);
impl Capclr {
    #[doc = "Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    pub const fn set_capclr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    pub const fn set_capclr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    pub const fn set_capclr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    pub const fn set_capclr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Capclr {
    #[inline(always)]
    fn default() -> Capclr {
        Capclr(0)
    }
}
impl core::fmt::Debug for Capclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capclr")
            .field("capclr0", &self.capclr0())
            .field("capclr1", &self.capclr1())
            .field("capclr2", &self.capclr2())
            .field("capclr3", &self.capclr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capclr {{ capclr0: {=bool:?}, capclr1: {=bool:?}, capclr2: {=bool:?}, capclr3: {=bool:?} }}",
            self.capclr0(),
            self.capclr1(),
            self.capclr2(),
            self.capclr3()
        )
    }
}
#[doc = "Capture configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn capen0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_capen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn capen1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_capen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn capen2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_capen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn capen3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn set_capen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_cappol0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_cappol1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_cappol2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn set_cappol3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("capen0", &self.capen0())
            .field("capen1", &self.capen1())
            .field("capen2", &self.capen2())
            .field("capen3", &self.capen3())
            .field("cappol0", &self.cappol0())
            .field("cappol1", &self.cappol1())
            .field("cappol2", &self.cappol2())
            .field("cappol3", &self.cappol3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ capen0: {=bool:?}, capen1: {=bool:?}, capen2: {=bool:?}, capen3: {=bool:?}, cappol0: {=bool:?}, cappol1: {=bool:?}, cappol2: {=bool:?}, cappol3: {=bool:?} }}",
            self.capen0(),
            self.capen1(),
            self.capen2(),
            self.capen3(),
            self.cappol0(),
            self.cappol1(),
            self.cappol2(),
            self.cappol3()
        )
    }
}
#[doc = "Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[must_use]
    #[inline(always)]
    pub const fn delayval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub const fn set_delayval(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[must_use]
    #[inline(always)]
    pub const fn repeat(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub const fn set_repeat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("delayval", &self.delayval())
            .field("repeat", &self.repeat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ delayval: {=u32:?}, repeat: {=bool:?} }}",
            self.delayval(),
            self.repeat()
        )
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[must_use]
    #[inline(always)]
    pub const fn intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[inline(always)]
    pub const fn set_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("intr", &self.intr())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ intr: {=bool:?}, active: {=bool:?} }}",
            self.intr(),
            self.active()
        )
    }
}
