#[doc = "Timer Channel Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capt0(pub u16);
impl Capt0 {
    #[doc = "Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn set_capture(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Capt0 {
    #[inline(always)]
    fn default() -> Capt0 {
        Capt0(0)
    }
}
impl core::fmt::Debug for Capt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capt0")
            .field("capture", &self.capture())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Capt0 {{ capture: {=u16:?} }}", self.capture())
    }
}
#[doc = "Timer Channel Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capt1(pub u16);
impl Capt1 {
    #[doc = "Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn set_capture(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Capt1 {
    #[inline(always)]
    fn default() -> Capt1 {
        Capt1(0)
    }
}
impl core::fmt::Debug for Capt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capt1")
            .field("capture", &self.capture())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Capt1 {{ capture: {=u16:?} }}", self.capture())
    }
}
#[doc = "Timer Channel Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capt2(pub u16);
impl Capt2 {
    #[doc = "Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn set_capture(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Capt2 {
    #[inline(always)]
    fn default() -> Capt2 {
        Capt2(0)
    }
}
impl core::fmt::Debug for Capt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capt2")
            .field("capture", &self.capture())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Capt2 {{ capture: {=u16:?} }}", self.capture())
    }
}
#[doc = "Timer Channel Capture Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capt3(pub u16);
impl Capt3 {
    #[doc = "Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn set_capture(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Capt3 {
    #[inline(always)]
    fn default() -> Capt3 {
        Capt3(0)
    }
}
impl core::fmt::Debug for Capt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capt3")
            .field("capture", &self.capture())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Capt3 {{ capture: {=u16:?} }}", self.capture())
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld10(pub u16);
impl Cmpld10 {
    #[doc = "COMPARATOR_LOAD_1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_1"]
    #[inline(always)]
    pub const fn set_comparator_load_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld10 {
    #[inline(always)]
    fn default() -> Cmpld10 {
        Cmpld10(0)
    }
}
impl core::fmt::Debug for Cmpld10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld10")
            .field("comparator_load_1", &self.comparator_load_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld10 {{ comparator_load_1: {=u16:?} }}",
            self.comparator_load_1()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld11(pub u16);
impl Cmpld11 {
    #[doc = "COMPARATOR_LOAD_1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_1"]
    #[inline(always)]
    pub const fn set_comparator_load_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld11 {
    #[inline(always)]
    fn default() -> Cmpld11 {
        Cmpld11(0)
    }
}
impl core::fmt::Debug for Cmpld11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld11")
            .field("comparator_load_1", &self.comparator_load_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld11 {{ comparator_load_1: {=u16:?} }}",
            self.comparator_load_1()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld12(pub u16);
impl Cmpld12 {
    #[doc = "COMPARATOR_LOAD_1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_1"]
    #[inline(always)]
    pub const fn set_comparator_load_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld12 {
    #[inline(always)]
    fn default() -> Cmpld12 {
        Cmpld12(0)
    }
}
impl core::fmt::Debug for Cmpld12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld12")
            .field("comparator_load_1", &self.comparator_load_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld12 {{ comparator_load_1: {=u16:?} }}",
            self.comparator_load_1()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld13(pub u16);
impl Cmpld13 {
    #[doc = "COMPARATOR_LOAD_1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_1"]
    #[inline(always)]
    pub const fn set_comparator_load_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld13 {
    #[inline(always)]
    fn default() -> Cmpld13 {
        Cmpld13(0)
    }
}
impl core::fmt::Debug for Cmpld13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld13")
            .field("comparator_load_1", &self.comparator_load_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld13 {{ comparator_load_1: {=u16:?} }}",
            self.comparator_load_1()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld20(pub u16);
impl Cmpld20 {
    #[doc = "COMPARATOR_LOAD_2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_2"]
    #[inline(always)]
    pub const fn set_comparator_load_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld20 {
    #[inline(always)]
    fn default() -> Cmpld20 {
        Cmpld20(0)
    }
}
impl core::fmt::Debug for Cmpld20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld20")
            .field("comparator_load_2", &self.comparator_load_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld20 {{ comparator_load_2: {=u16:?} }}",
            self.comparator_load_2()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld21(pub u16);
impl Cmpld21 {
    #[doc = "COMPARATOR_LOAD_2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_2"]
    #[inline(always)]
    pub const fn set_comparator_load_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld21 {
    #[inline(always)]
    fn default() -> Cmpld21 {
        Cmpld21(0)
    }
}
impl core::fmt::Debug for Cmpld21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld21")
            .field("comparator_load_2", &self.comparator_load_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld21 {{ comparator_load_2: {=u16:?} }}",
            self.comparator_load_2()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld22(pub u16);
impl Cmpld22 {
    #[doc = "COMPARATOR_LOAD_2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_2"]
    #[inline(always)]
    pub const fn set_comparator_load_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld22 {
    #[inline(always)]
    fn default() -> Cmpld22 {
        Cmpld22(0)
    }
}
impl core::fmt::Debug for Cmpld22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld22")
            .field("comparator_load_2", &self.comparator_load_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld22 {{ comparator_load_2: {=u16:?} }}",
            self.comparator_load_2()
        )
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpld23(pub u16);
impl Cmpld23 {
    #[doc = "COMPARATOR_LOAD_2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparator_load_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COMPARATOR_LOAD_2"]
    #[inline(always)]
    pub const fn set_comparator_load_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cmpld23 {
    #[inline(always)]
    fn default() -> Cmpld23 {
        Cmpld23(0)
    }
}
impl core::fmt::Debug for Cmpld23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpld23")
            .field("comparator_load_2", &self.comparator_load_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpld23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpld23 {{ comparator_load_2: {=u16:?} }}",
            self.comparator_load_2()
        )
    }
}
#[doc = "Timer Channel Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntr0(pub u16);
impl Cntr0 {
    #[doc = "COUNTER"]
    #[must_use]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COUNTER"]
    #[inline(always)]
    pub const fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cntr0 {
    #[inline(always)]
    fn default() -> Cntr0 {
        Cntr0(0)
    }
}
impl core::fmt::Debug for Cntr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntr0")
            .field("counter", &self.counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cntr0 {{ counter: {=u16:?} }}", self.counter())
    }
}
#[doc = "Timer Channel Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntr1(pub u16);
impl Cntr1 {
    #[doc = "COUNTER"]
    #[must_use]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COUNTER"]
    #[inline(always)]
    pub const fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cntr1 {
    #[inline(always)]
    fn default() -> Cntr1 {
        Cntr1(0)
    }
}
impl core::fmt::Debug for Cntr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntr1")
            .field("counter", &self.counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cntr1 {{ counter: {=u16:?} }}", self.counter())
    }
}
#[doc = "Timer Channel Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntr2(pub u16);
impl Cntr2 {
    #[doc = "COUNTER"]
    #[must_use]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COUNTER"]
    #[inline(always)]
    pub const fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cntr2 {
    #[inline(always)]
    fn default() -> Cntr2 {
        Cntr2(0)
    }
}
impl core::fmt::Debug for Cntr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntr2")
            .field("counter", &self.counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cntr2 {{ counter: {=u16:?} }}", self.counter())
    }
}
#[doc = "Timer Channel Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntr3(pub u16);
impl Cntr3 {
    #[doc = "COUNTER"]
    #[must_use]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "COUNTER"]
    #[inline(always)]
    pub const fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Cntr3 {
    #[inline(always)]
    fn default() -> Cntr3 {
        Cntr3(0)
    }
}
impl core::fmt::Debug for Cntr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntr3")
            .field("counter", &self.counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cntr3 {{ counter: {=u16:?} }}", self.counter())
    }
}
#[doc = "Timer Channel Compare Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp10(pub u16);
impl Comp10 {
    #[doc = "Comparison Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 1"]
    #[inline(always)]
    pub const fn set_comparison_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp10 {
    #[inline(always)]
    fn default() -> Comp10 {
        Comp10(0)
    }
}
impl core::fmt::Debug for Comp10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp10")
            .field("comparison_1", &self.comparison_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp10 {{ comparison_1: {=u16:?} }}",
            self.comparison_1()
        )
    }
}
#[doc = "Timer Channel Compare Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp11(pub u16);
impl Comp11 {
    #[doc = "Comparison Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 1"]
    #[inline(always)]
    pub const fn set_comparison_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp11 {
    #[inline(always)]
    fn default() -> Comp11 {
        Comp11(0)
    }
}
impl core::fmt::Debug for Comp11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp11")
            .field("comparison_1", &self.comparison_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp11 {{ comparison_1: {=u16:?} }}",
            self.comparison_1()
        )
    }
}
#[doc = "Timer Channel Compare Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp12(pub u16);
impl Comp12 {
    #[doc = "Comparison Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 1"]
    #[inline(always)]
    pub const fn set_comparison_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp12 {
    #[inline(always)]
    fn default() -> Comp12 {
        Comp12(0)
    }
}
impl core::fmt::Debug for Comp12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp12")
            .field("comparison_1", &self.comparison_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp12 {{ comparison_1: {=u16:?} }}",
            self.comparison_1()
        )
    }
}
#[doc = "Timer Channel Compare Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp13(pub u16);
impl Comp13 {
    #[doc = "Comparison Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 1"]
    #[inline(always)]
    pub const fn set_comparison_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp13 {
    #[inline(always)]
    fn default() -> Comp13 {
        Comp13(0)
    }
}
impl core::fmt::Debug for Comp13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp13")
            .field("comparison_1", &self.comparison_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp13 {{ comparison_1: {=u16:?} }}",
            self.comparison_1()
        )
    }
}
#[doc = "Timer Channel Compare Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp20(pub u16);
impl Comp20 {
    #[doc = "Comparison Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 2"]
    #[inline(always)]
    pub const fn set_comparison_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp20 {
    #[inline(always)]
    fn default() -> Comp20 {
        Comp20(0)
    }
}
impl core::fmt::Debug for Comp20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp20")
            .field("comparison_2", &self.comparison_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp20 {{ comparison_2: {=u16:?} }}",
            self.comparison_2()
        )
    }
}
#[doc = "Timer Channel Compare Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp21(pub u16);
impl Comp21 {
    #[doc = "Comparison Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 2"]
    #[inline(always)]
    pub const fn set_comparison_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp21 {
    #[inline(always)]
    fn default() -> Comp21 {
        Comp21(0)
    }
}
impl core::fmt::Debug for Comp21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp21")
            .field("comparison_2", &self.comparison_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp21 {{ comparison_2: {=u16:?} }}",
            self.comparison_2()
        )
    }
}
#[doc = "Timer Channel Compare Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp22(pub u16);
impl Comp22 {
    #[doc = "Comparison Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 2"]
    #[inline(always)]
    pub const fn set_comparison_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp22 {
    #[inline(always)]
    fn default() -> Comp22 {
        Comp22(0)
    }
}
impl core::fmt::Debug for Comp22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp22")
            .field("comparison_2", &self.comparison_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp22 {{ comparison_2: {=u16:?} }}",
            self.comparison_2()
        )
    }
}
#[doc = "Timer Channel Compare Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp23(pub u16);
impl Comp23 {
    #[doc = "Comparison Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn comparison_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Comparison Value 2"]
    #[inline(always)]
    pub const fn set_comparison_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Comp23 {
    #[inline(always)]
    fn default() -> Comp23 {
        Comp23(0)
    }
}
impl core::fmt::Debug for Comp23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp23")
            .field("comparison_2", &self.comparison_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp23 {{ comparison_2: {=u16:?} }}",
            self.comparison_2()
        )
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl0(pub u16);
impl Csctrl0 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl0Cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl0Cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl0Cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl0Cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl0Cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl0Cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> super::vals::Csctrl0Up {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Csctrl0Up::from_bits(val as u8)
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: super::vals::Csctrl0Up) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> super::vals::Csctrl0Tci {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Csctrl0Tci::from_bits(val as u8)
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: super::vals::Csctrl0Tci) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl0DbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl0DbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl0DbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl0 {
    #[inline(always)]
    fn default() -> Csctrl0 {
        Csctrl0(0)
    }
}
impl core::fmt::Debug for Csctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl0")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csctrl0 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {:?}, tci: {:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}",
            self.cl1(),
            self.cl2(),
            self.tcf1(),
            self.tcf2(),
            self.tcf1en(),
            self.tcf2en(),
            self.up(),
            self.tci(),
            self.roc(),
            self.alt_load(),
            self.fault(),
            self.dbg_en()
        )
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl1(pub u16);
impl Csctrl1 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl1Cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl1Cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl1Cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl1Cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl1Cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl1Cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> super::vals::Csctrl1Up {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Csctrl1Up::from_bits(val as u8)
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: super::vals::Csctrl1Up) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> super::vals::Csctrl1Tci {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Csctrl1Tci::from_bits(val as u8)
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: super::vals::Csctrl1Tci) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl1DbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl1DbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl1DbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl1 {
    #[inline(always)]
    fn default() -> Csctrl1 {
        Csctrl1(0)
    }
}
impl core::fmt::Debug for Csctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl1")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csctrl1 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {:?}, tci: {:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}",
            self.cl1(),
            self.cl2(),
            self.tcf1(),
            self.tcf2(),
            self.tcf1en(),
            self.tcf2en(),
            self.up(),
            self.tci(),
            self.roc(),
            self.alt_load(),
            self.fault(),
            self.dbg_en()
        )
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl2(pub u16);
impl Csctrl2 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl2Cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl2Cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl2Cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl2Cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl2Cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl2Cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> super::vals::Csctrl2Up {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Csctrl2Up::from_bits(val as u8)
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: super::vals::Csctrl2Up) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> super::vals::Csctrl2Tci {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Csctrl2Tci::from_bits(val as u8)
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: super::vals::Csctrl2Tci) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl2DbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl2DbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl2DbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl2 {
    #[inline(always)]
    fn default() -> Csctrl2 {
        Csctrl2(0)
    }
}
impl core::fmt::Debug for Csctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl2")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csctrl2 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {:?}, tci: {:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}",
            self.cl1(),
            self.cl2(),
            self.tcf1(),
            self.tcf2(),
            self.tcf1en(),
            self.tcf2en(),
            self.up(),
            self.tci(),
            self.roc(),
            self.alt_load(),
            self.fault(),
            self.dbg_en()
        )
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl3(pub u16);
impl Csctrl3 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl3Cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl3Cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl3Cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl3Cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl3Cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl3Cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> super::vals::Csctrl3Up {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Csctrl3Up::from_bits(val as u8)
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: super::vals::Csctrl3Up) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> super::vals::Csctrl3Tci {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Csctrl3Tci::from_bits(val as u8)
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: super::vals::Csctrl3Tci) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl3DbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl3DbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl3DbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl3 {
    #[inline(always)]
    fn default() -> Csctrl3 {
        Csctrl3(0)
    }
}
impl core::fmt::Debug for Csctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl3")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csctrl3 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {:?}, tci: {:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}",
            self.cl1(),
            self.cl2(),
            self.tcf1(),
            self.tcf2(),
            self.tcf1en(),
            self.tcf2en(),
            self.up(),
            self.tci(),
            self.roc(),
            self.alt_load(),
            self.fault(),
            self.dbg_en()
        )
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u16);
impl Ctrl0 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl0Outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl0Outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl0Outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Ctrl0Dir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctrl0Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Ctrl0Dir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> super::vals::Ctrl0Length {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ctrl0Length::from_bits(val as u8)
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: super::vals::Ctrl0Length) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> super::vals::Ctrl0Once {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ctrl0Once::from_bits(val as u8)
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: super::vals::Ctrl0Once) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl0Scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl0Scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl0Scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl0Pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl0Pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl0Pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl0Cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl0Cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl0Cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl0 {{ outmode: {:?}, coinit: {=bool:?}, dir: {:?}, length: {:?}, once: {:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}",
            self.outmode(),
            self.coinit(),
            self.dir(),
            self.length(),
            self.once(),
            self.scs(),
            self.pcs(),
            self.cm()
        )
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u16);
impl Ctrl1 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl1Outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl1Outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl1Outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Ctrl1Dir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctrl1Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Ctrl1Dir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> super::vals::Ctrl1Length {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ctrl1Length::from_bits(val as u8)
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: super::vals::Ctrl1Length) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> super::vals::Ctrl1Once {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ctrl1Once::from_bits(val as u8)
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: super::vals::Ctrl1Once) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl1Scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl1Scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl1Scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl1Pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl1Pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl1Pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl1Cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl1Cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl1Cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ outmode: {:?}, coinit: {=bool:?}, dir: {:?}, length: {:?}, once: {:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}",
            self.outmode(),
            self.coinit(),
            self.dir(),
            self.length(),
            self.once(),
            self.scs(),
            self.pcs(),
            self.cm()
        )
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u16);
impl Ctrl2 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl2Outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl2Outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl2Outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Ctrl2Dir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctrl2Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Ctrl2Dir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> super::vals::Ctrl2Length {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ctrl2Length::from_bits(val as u8)
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: super::vals::Ctrl2Length) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> super::vals::Ctrl2Once {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ctrl2Once::from_bits(val as u8)
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: super::vals::Ctrl2Once) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl2Scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl2Scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl2Scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl2Pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl2Pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl2Pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl2Cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl2Cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl2Cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ outmode: {:?}, coinit: {=bool:?}, dir: {:?}, length: {:?}, once: {:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}",
            self.outmode(),
            self.coinit(),
            self.dir(),
            self.length(),
            self.once(),
            self.scs(),
            self.pcs(),
            self.cm()
        )
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3(pub u16);
impl Ctrl3 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl3Outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl3Outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl3Outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Ctrl3Dir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctrl3Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Ctrl3Dir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> super::vals::Ctrl3Length {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ctrl3Length::from_bits(val as u8)
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: super::vals::Ctrl3Length) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> super::vals::Ctrl3Once {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ctrl3Once::from_bits(val as u8)
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: super::vals::Ctrl3Once) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl3Scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl3Scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl3Scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl3Pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl3Pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl3Pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl3Cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl3Cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl3Cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl3 {
    #[inline(always)]
    fn default() -> Ctrl3 {
        Ctrl3(0)
    }
}
impl core::fmt::Debug for Ctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl3 {{ outmode: {:?}, coinit: {=bool:?}, dir: {:?}, length: {:?}, once: {:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}",
            self.outmode(),
            self.coinit(),
            self.dir(),
            self.length(),
            self.once(),
            self.scs(),
            self.pcs(),
            self.cm()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0(pub u16);
impl Dma0 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma0 {
    #[inline(always)]
    fn default() -> Dma0 {
        Dma0(0)
    }
}
impl core::fmt::Debug for Dma0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1(pub u16);
impl Dma1 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma1 {
    #[inline(always)]
    fn default() -> Dma1 {
        Dma1(0)
    }
}
impl core::fmt::Debug for Dma1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2(pub u16);
impl Dma2 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma2 {
    #[inline(always)]
    fn default() -> Dma2 {
        Dma2(0)
    }
}
impl core::fmt::Debug for Dma2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma2")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma2 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3(pub u16);
impl Dma3 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma3 {
    #[inline(always)]
    fn default() -> Dma3 {
        Dma3(0)
    }
}
impl core::fmt::Debug for Dma3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma3")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma3 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enbl(pub u16);
impl Enbl {
    #[doc = "Timer Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enbl(&self) -> super::vals::Enbl {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enbl::from_bits(val as u8)
    }
    #[doc = "Timer Channel Enable"]
    #[inline(always)]
    pub const fn set_enbl(&mut self, val: super::vals::Enbl) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Enbl {
    #[inline(always)]
    fn default() -> Enbl {
        Enbl(0)
    }
}
impl core::fmt::Debug for Enbl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enbl").field("enbl", &self.enbl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enbl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enbl {{ enbl: {:?} }}", self.enbl())
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt0(pub u16);
impl Filt0 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt0 {
    #[inline(always)]
    fn default() -> Filt0 {
        Filt0(0)
    }
}
impl core::fmt::Debug for Filt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt0")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt0 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt1(pub u16);
impl Filt1 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt1 {
    #[inline(always)]
    fn default() -> Filt1 {
        Filt1(0)
    }
}
impl core::fmt::Debug for Filt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt1")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt1 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt2(pub u16);
impl Filt2 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt2 {
    #[inline(always)]
    fn default() -> Filt2 {
        Filt2(0)
    }
}
impl core::fmt::Debug for Filt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt2")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt2 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt3(pub u16);
impl Filt3 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt3 {
    #[inline(always)]
    fn default() -> Filt3 {
        Filt3(0)
    }
}
impl core::fmt::Debug for Filt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt3")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt3 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hold0(pub u16);
impl Hold0 {
    #[doc = "HOLD"]
    #[must_use]
    #[inline(always)]
    pub const fn hold(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HOLD"]
    #[inline(always)]
    pub const fn set_hold(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Hold0 {
    #[inline(always)]
    fn default() -> Hold0 {
        Hold0(0)
    }
}
impl core::fmt::Debug for Hold0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hold0").field("hold", &self.hold()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hold0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hold0 {{ hold: {=u16:?} }}", self.hold())
    }
}
#[doc = "Timer Channel Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hold1(pub u16);
impl Hold1 {
    #[doc = "HOLD"]
    #[must_use]
    #[inline(always)]
    pub const fn hold(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HOLD"]
    #[inline(always)]
    pub const fn set_hold(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Hold1 {
    #[inline(always)]
    fn default() -> Hold1 {
        Hold1(0)
    }
}
impl core::fmt::Debug for Hold1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hold1").field("hold", &self.hold()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hold1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hold1 {{ hold: {=u16:?} }}", self.hold())
    }
}
#[doc = "Timer Channel Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hold2(pub u16);
impl Hold2 {
    #[doc = "HOLD"]
    #[must_use]
    #[inline(always)]
    pub const fn hold(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HOLD"]
    #[inline(always)]
    pub const fn set_hold(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Hold2 {
    #[inline(always)]
    fn default() -> Hold2 {
        Hold2(0)
    }
}
impl core::fmt::Debug for Hold2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hold2").field("hold", &self.hold()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hold2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hold2 {{ hold: {=u16:?} }}", self.hold())
    }
}
#[doc = "Timer Channel Hold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hold3(pub u16);
impl Hold3 {
    #[doc = "HOLD"]
    #[must_use]
    #[inline(always)]
    pub const fn hold(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HOLD"]
    #[inline(always)]
    pub const fn set_hold(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Hold3 {
    #[inline(always)]
    fn default() -> Hold3 {
        Hold3(0)
    }
}
impl core::fmt::Debug for Hold3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hold3").field("hold", &self.hold()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hold3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hold3 {{ hold: {=u16:?} }}", self.hold())
    }
}
#[doc = "Timer Channel Load Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load0(pub u16);
impl Load0 {
    #[doc = "Timer Load Register"]
    #[must_use]
    #[inline(always)]
    pub const fn load(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Load Register"]
    #[inline(always)]
    pub const fn set_load(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Load0 {
    #[inline(always)]
    fn default() -> Load0 {
        Load0(0)
    }
}
impl core::fmt::Debug for Load0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Load0").field("load", &self.load()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Load0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Load0 {{ load: {=u16:?} }}", self.load())
    }
}
#[doc = "Timer Channel Load Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load1(pub u16);
impl Load1 {
    #[doc = "Timer Load Register"]
    #[must_use]
    #[inline(always)]
    pub const fn load(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Load Register"]
    #[inline(always)]
    pub const fn set_load(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Load1 {
    #[inline(always)]
    fn default() -> Load1 {
        Load1(0)
    }
}
impl core::fmt::Debug for Load1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Load1").field("load", &self.load()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Load1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Load1 {{ load: {=u16:?} }}", self.load())
    }
}
#[doc = "Timer Channel Load Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load2(pub u16);
impl Load2 {
    #[doc = "Timer Load Register"]
    #[must_use]
    #[inline(always)]
    pub const fn load(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Load Register"]
    #[inline(always)]
    pub const fn set_load(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Load2 {
    #[inline(always)]
    fn default() -> Load2 {
        Load2(0)
    }
}
impl core::fmt::Debug for Load2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Load2").field("load", &self.load()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Load2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Load2 {{ load: {=u16:?} }}", self.load())
    }
}
#[doc = "Timer Channel Load Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load3(pub u16);
impl Load3 {
    #[doc = "Timer Load Register"]
    #[must_use]
    #[inline(always)]
    pub const fn load(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Load Register"]
    #[inline(always)]
    pub const fn set_load(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Load3 {
    #[inline(always)]
    fn default() -> Load3 {
        Load3(0)
    }
}
impl core::fmt::Debug for Load3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Load3").field("load", &self.load()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Load3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Load3 {{ load: {=u16:?} }}", self.load())
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl0(pub u16);
impl Sctrl0 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> super::vals::Sctrl0Oen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sctrl0Oen::from_bits(val as u8)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: super::vals::Sctrl0Oen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> super::vals::Sctrl0Ops {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sctrl0Ops::from_bits(val as u8)
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: super::vals::Sctrl0Ops) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl0CaptureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl0CaptureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl0CaptureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl0 {
    #[inline(always)]
    fn default() -> Sctrl0 {
        Sctrl0(0)
    }
}
impl core::fmt::Debug for Sctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl0")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctrl0 {{ oen: {:?}, ops: {:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}",
            self.oen(),
            self.ops(),
            self.force(),
            self.val(),
            self.eeof(),
            self.mstr(),
            self.capture_mode(),
            self.input(),
            self.ips(),
            self.iefie(),
            self.ief(),
            self.tofie(),
            self.tof(),
            self.tcfie(),
            self.tcf()
        )
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl1(pub u16);
impl Sctrl1 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> super::vals::Sctrl1Oen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sctrl1Oen::from_bits(val as u8)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: super::vals::Sctrl1Oen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> super::vals::Sctrl1Ops {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sctrl1Ops::from_bits(val as u8)
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: super::vals::Sctrl1Ops) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl1CaptureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl1CaptureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl1CaptureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl1 {
    #[inline(always)]
    fn default() -> Sctrl1 {
        Sctrl1(0)
    }
}
impl core::fmt::Debug for Sctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl1")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctrl1 {{ oen: {:?}, ops: {:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}",
            self.oen(),
            self.ops(),
            self.force(),
            self.val(),
            self.eeof(),
            self.mstr(),
            self.capture_mode(),
            self.input(),
            self.ips(),
            self.iefie(),
            self.ief(),
            self.tofie(),
            self.tof(),
            self.tcfie(),
            self.tcf()
        )
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl2(pub u16);
impl Sctrl2 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> super::vals::Sctrl2Oen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sctrl2Oen::from_bits(val as u8)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: super::vals::Sctrl2Oen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> super::vals::Sctrl2Ops {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sctrl2Ops::from_bits(val as u8)
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: super::vals::Sctrl2Ops) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl2CaptureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl2CaptureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl2CaptureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl2 {
    #[inline(always)]
    fn default() -> Sctrl2 {
        Sctrl2(0)
    }
}
impl core::fmt::Debug for Sctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl2")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctrl2 {{ oen: {:?}, ops: {:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}",
            self.oen(),
            self.ops(),
            self.force(),
            self.val(),
            self.eeof(),
            self.mstr(),
            self.capture_mode(),
            self.input(),
            self.ips(),
            self.iefie(),
            self.ief(),
            self.tofie(),
            self.tof(),
            self.tcfie(),
            self.tcf()
        )
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl3(pub u16);
impl Sctrl3 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> super::vals::Sctrl3Oen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sctrl3Oen::from_bits(val as u8)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: super::vals::Sctrl3Oen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> super::vals::Sctrl3Ops {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sctrl3Ops::from_bits(val as u8)
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: super::vals::Sctrl3Ops) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl3CaptureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl3CaptureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl3CaptureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl3 {
    #[inline(always)]
    fn default() -> Sctrl3 {
        Sctrl3(0)
    }
}
impl core::fmt::Debug for Sctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl3")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctrl3 {{ oen: {:?}, ops: {:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}",
            self.oen(),
            self.ops(),
            self.force(),
            self.val(),
            self.eeof(),
            self.mstr(),
            self.capture_mode(),
            self.input(),
            self.ips(),
            self.iefie(),
            self.ief(),
            self.tofie(),
            self.tof(),
            self.tcfie(),
            self.tcf()
        )
    }
}
