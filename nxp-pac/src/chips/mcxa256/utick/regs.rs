#[doc = "Capture"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "Captured Value for the Related Capture Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Captured Value for the Related Capture Event"]
    #[inline(always)]
    pub const fn set_cap_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Captured Value Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Captured Value Valid Flag"]
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
#[doc = "Capture Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capclr(pub u32);
impl Capclr {
    #[doc = "Clear Capture 0"]
    #[must_use]
    #[inline(always)]
    pub const fn capclr0(&self) -> super::vals::Capclr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Capclr0::from_bits(val as u8)
    }
    #[doc = "Clear Capture 0"]
    #[inline(always)]
    pub const fn set_capclr0(&mut self, val: super::vals::Capclr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear Capture 1"]
    #[must_use]
    #[inline(always)]
    pub const fn capclr1(&self) -> super::vals::Capclr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Capclr1::from_bits(val as u8)
    }
    #[doc = "Clear Capture 1"]
    #[inline(always)]
    pub const fn set_capclr1(&mut self, val: super::vals::Capclr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear Capture 2"]
    #[must_use]
    #[inline(always)]
    pub const fn capclr2(&self) -> super::vals::Capclr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Capclr2::from_bits(val as u8)
    }
    #[doc = "Clear Capture 2"]
    #[inline(always)]
    pub const fn set_capclr2(&mut self, val: super::vals::Capclr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Capture 3"]
    #[must_use]
    #[inline(always)]
    pub const fn capclr3(&self) -> super::vals::Capclr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Capclr3::from_bits(val as u8)
    }
    #[doc = "Clear Capture 3"]
    #[inline(always)]
    pub const fn set_capclr3(&mut self, val: super::vals::Capclr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
            "Capclr {{ capclr0: {:?}, capclr1: {:?}, capclr2: {:?}, capclr3: {:?} }}",
            self.capclr0(),
            self.capclr1(),
            self.capclr2(),
            self.capclr3()
        )
    }
}
#[doc = "Capture Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Enable Capture 0"]
    #[must_use]
    #[inline(always)]
    pub const fn capen0(&self) -> super::vals::Capen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Capen0::from_bits(val as u8)
    }
    #[doc = "Enable Capture 0"]
    #[inline(always)]
    pub const fn set_capen0(&mut self, val: super::vals::Capen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Capture 1"]
    #[must_use]
    #[inline(always)]
    pub const fn capen1(&self) -> super::vals::Capen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Capen1::from_bits(val as u8)
    }
    #[doc = "Enable Capture 1"]
    #[inline(always)]
    pub const fn set_capen1(&mut self, val: super::vals::Capen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Capture 2"]
    #[must_use]
    #[inline(always)]
    pub const fn capen2(&self) -> super::vals::Capen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Capen2::from_bits(val as u8)
    }
    #[doc = "Enable Capture 2"]
    #[inline(always)]
    pub const fn set_capen2(&mut self, val: super::vals::Capen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Capture 3"]
    #[must_use]
    #[inline(always)]
    pub const fn capen3(&self) -> super::vals::Capen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Capen3::from_bits(val as u8)
    }
    #[doc = "Enable Capture 3"]
    #[inline(always)]
    pub const fn set_capen3(&mut self, val: super::vals::Capen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture Polarity 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cappol0(&self) -> super::vals::Cappol0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cappol0::from_bits(val as u8)
    }
    #[doc = "Capture Polarity 0"]
    #[inline(always)]
    pub const fn set_cappol0(&mut self, val: super::vals::Cappol0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture-Polarity 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cappol1(&self) -> super::vals::Cappol1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cappol1::from_bits(val as u8)
    }
    #[doc = "Capture-Polarity 1"]
    #[inline(always)]
    pub const fn set_cappol1(&mut self, val: super::vals::Cappol1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Polarity 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cappol2(&self) -> super::vals::Cappol2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cappol2::from_bits(val as u8)
    }
    #[doc = "Capture Polarity 2"]
    #[inline(always)]
    pub const fn set_cappol2(&mut self, val: super::vals::Cappol2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture Polarity 3"]
    #[must_use]
    #[inline(always)]
    pub const fn cappol3(&self) -> super::vals::Cappol3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cappol3::from_bits(val as u8)
    }
    #[doc = "Capture Polarity 3"]
    #[inline(always)]
    pub const fn set_cappol3(&mut self, val: super::vals::Cappol3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
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
            "Cfg {{ capen0: {:?}, capen1: {:?}, capen2: {:?}, capen3: {:?}, cappol0: {:?}, cappol1: {:?}, cappol2: {:?}, cappol3: {:?} }}",
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
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Tick Interval"]
    #[must_use]
    #[inline(always)]
    pub const fn delayval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Tick Interval"]
    #[inline(always)]
    pub const fn set_delayval(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Repeat Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn repeat(&self) -> super::vals::Repeat {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Repeat::from_bits(val as u8)
    }
    #[doc = "Repeat Delay"]
    #[inline(always)]
    pub const fn set_repeat(&mut self, val: super::vals::Repeat) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Ctrl {{ delayval: {=u32:?}, repeat: {:?} }}",
            self.delayval(),
            self.repeat()
        )
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub const fn set_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Active Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "Timer Active Flag"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            "Stat {{ intr: {=bool:?}, active: {:?} }}",
            self.intr(),
            self.active()
        )
    }
}
