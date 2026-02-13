#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B00(pub u8);
impl B00 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B00 {
    #[inline(always)]
    fn default() -> B00 {
        B00(0)
    }
}
impl core::fmt::Debug for B00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B00").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B00 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B01(pub u8);
impl B01 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B01 {
    #[inline(always)]
    fn default() -> B01 {
        B01(0)
    }
}
impl core::fmt::Debug for B01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B01").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B01 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B010(pub u8);
impl B010 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B010 {
    #[inline(always)]
    fn default() -> B010 {
        B010(0)
    }
}
impl core::fmt::Debug for B010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B010")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B010 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B010 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B011(pub u8);
impl B011 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B011 {
    #[inline(always)]
    fn default() -> B011 {
        B011(0)
    }
}
impl core::fmt::Debug for B011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B011")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B011 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B011 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B012(pub u8);
impl B012 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B012 {
    #[inline(always)]
    fn default() -> B012 {
        B012(0)
    }
}
impl core::fmt::Debug for B012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B012")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B012 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B012 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B013(pub u8);
impl B013 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B013 {
    #[inline(always)]
    fn default() -> B013 {
        B013(0)
    }
}
impl core::fmt::Debug for B013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B013")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B013 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B013 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B014(pub u8);
impl B014 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B014 {
    #[inline(always)]
    fn default() -> B014 {
        B014(0)
    }
}
impl core::fmt::Debug for B014 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B014")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B014 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B014 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B015(pub u8);
impl B015 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B015 {
    #[inline(always)]
    fn default() -> B015 {
        B015(0)
    }
}
impl core::fmt::Debug for B015 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B015")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B015 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B015 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B016(pub u8);
impl B016 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B016 {
    #[inline(always)]
    fn default() -> B016 {
        B016(0)
    }
}
impl core::fmt::Debug for B016 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B016")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B016 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B016 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B017(pub u8);
impl B017 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B017 {
    #[inline(always)]
    fn default() -> B017 {
        B017(0)
    }
}
impl core::fmt::Debug for B017 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B017")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B017 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B017 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B018(pub u8);
impl B018 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B018 {
    #[inline(always)]
    fn default() -> B018 {
        B018(0)
    }
}
impl core::fmt::Debug for B018 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B018")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B018 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B018 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B019(pub u8);
impl B019 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B019 {
    #[inline(always)]
    fn default() -> B019 {
        B019(0)
    }
}
impl core::fmt::Debug for B019 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B019")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B019 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B019 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B02(pub u8);
impl B02 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B02 {
    #[inline(always)]
    fn default() -> B02 {
        B02(0)
    }
}
impl core::fmt::Debug for B02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B02").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B02 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B020(pub u8);
impl B020 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B020 {
    #[inline(always)]
    fn default() -> B020 {
        B020(0)
    }
}
impl core::fmt::Debug for B020 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B020")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B020 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B020 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B021(pub u8);
impl B021 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B021 {
    #[inline(always)]
    fn default() -> B021 {
        B021(0)
    }
}
impl core::fmt::Debug for B021 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B021")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B021 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B021 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B022(pub u8);
impl B022 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B022 {
    #[inline(always)]
    fn default() -> B022 {
        B022(0)
    }
}
impl core::fmt::Debug for B022 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B022")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B022 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B022 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B023(pub u8);
impl B023 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B023 {
    #[inline(always)]
    fn default() -> B023 {
        B023(0)
    }
}
impl core::fmt::Debug for B023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B023")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B023 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B024(pub u8);
impl B024 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B024 {
    #[inline(always)]
    fn default() -> B024 {
        B024(0)
    }
}
impl core::fmt::Debug for B024 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B024")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B024 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B024 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B025(pub u8);
impl B025 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B025 {
    #[inline(always)]
    fn default() -> B025 {
        B025(0)
    }
}
impl core::fmt::Debug for B025 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B025")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B025 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B025 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B026(pub u8);
impl B026 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B026 {
    #[inline(always)]
    fn default() -> B026 {
        B026(0)
    }
}
impl core::fmt::Debug for B026 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B026")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B026 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B026 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B027(pub u8);
impl B027 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B027 {
    #[inline(always)]
    fn default() -> B027 {
        B027(0)
    }
}
impl core::fmt::Debug for B027 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B027")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B027 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B027 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B028(pub u8);
impl B028 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B028 {
    #[inline(always)]
    fn default() -> B028 {
        B028(0)
    }
}
impl core::fmt::Debug for B028 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B028")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B028 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B028 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B029(pub u8);
impl B029 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B029 {
    #[inline(always)]
    fn default() -> B029 {
        B029(0)
    }
}
impl core::fmt::Debug for B029 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B029")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B029 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B029 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B03(pub u8);
impl B03 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B03 {
    #[inline(always)]
    fn default() -> B03 {
        B03(0)
    }
}
impl core::fmt::Debug for B03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B03").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B03 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B030(pub u8);
impl B030 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B030 {
    #[inline(always)]
    fn default() -> B030 {
        B030(0)
    }
}
impl core::fmt::Debug for B030 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B030")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B030 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B030 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B031(pub u8);
impl B031 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B031 {
    #[inline(always)]
    fn default() -> B031 {
        B031(0)
    }
}
impl core::fmt::Debug for B031 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B031")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B031 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B031 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B04(pub u8);
impl B04 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B04 {
    #[inline(always)]
    fn default() -> B04 {
        B04(0)
    }
}
impl core::fmt::Debug for B04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B04").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B04 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B04 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B05(pub u8);
impl B05 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B05 {
    #[inline(always)]
    fn default() -> B05 {
        B05(0)
    }
}
impl core::fmt::Debug for B05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B05").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B05 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B05 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B06(pub u8);
impl B06 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B06 {
    #[inline(always)]
    fn default() -> B06 {
        B06(0)
    }
}
impl core::fmt::Debug for B06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B06").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B06 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B06 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B07(pub u8);
impl B07 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B07 {
    #[inline(always)]
    fn default() -> B07 {
        B07(0)
    }
}
impl core::fmt::Debug for B07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B07").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B07 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B07 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B08(pub u8);
impl B08 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B08 {
    #[inline(always)]
    fn default() -> B08 {
        B08(0)
    }
}
impl core::fmt::Debug for B08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B08").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B08 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B08 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B09(pub u8);
impl B09 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B09 {
    #[inline(always)]
    fn default() -> B09 {
        B09(0)
    }
}
impl core::fmt::Debug for B09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B09").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B09 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B09 {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Clear port for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clr0(pub u32);
impl Clr0 {
    #[doc = "Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn clrp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub const fn set_clrp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clr0 {
    #[inline(always)]
    fn default() -> Clr0 {
        Clr0(0)
    }
}
impl core::fmt::Debug for Clr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clr0").field("clrp", &self.clrp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clr0 {{ clrp: {=u32:?} }}", self.clrp())
    }
}
#[doc = "Direction registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir0(pub u32);
impl Dir0 {
    #[doc = "Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[must_use]
    #[inline(always)]
    pub const fn dirp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub const fn set_dirp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dir0 {
    #[inline(always)]
    fn default() -> Dir0 {
        Dir0(0)
    }
}
impl core::fmt::Debug for Dir0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir0").field("dirp", &self.dirp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dir0 {{ dirp: {=u32:?} }}", self.dirp())
    }
}
#[doc = "Clear pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirclr0(pub u32);
impl Dirclr0 {
    #[doc = "Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dirclrp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub const fn set_dirclrp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dirclr0 {
    #[inline(always)]
    fn default() -> Dirclr0 {
        Dirclr0(0)
    }
}
impl core::fmt::Debug for Dirclr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dirclr0")
            .field("dirclrp", &self.dirclrp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dirclr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dirclr0 {{ dirclrp: {=u32:?} }}", self.dirclrp())
    }
}
#[doc = "Toggle pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirnot0(pub u32);
impl Dirnot0 {
    #[doc = "Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dirnotp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    pub const fn set_dirnotp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dirnot0 {
    #[inline(always)]
    fn default() -> Dirnot0 {
        Dirnot0(0)
    }
}
impl core::fmt::Debug for Dirnot0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dirnot0")
            .field("dirnotp", &self.dirnotp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dirnot0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dirnot0 {{ dirnotp: {=u32:?} }}", self.dirnotp())
    }
}
#[doc = "Set pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirset0(pub u32);
impl Dirset0 {
    #[doc = "Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dirsetp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub const fn set_dirsetp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dirset0 {
    #[inline(always)]
    fn default() -> Dirset0 {
        Dirset0(0)
    }
}
impl core::fmt::Debug for Dirset0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dirset0")
            .field("dirsetp", &self.dirsetp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dirset0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dirset0 {{ dirsetp: {=u32:?} }}", self.dirsetp())
    }
}
#[doc = "Mask register for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask0(pub u32);
impl Mask0 {
    #[doc = "Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[must_use]
    #[inline(always)]
    pub const fn maskp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub const fn set_maskp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask0 {
    #[inline(always)]
    fn default() -> Mask0 {
        Mask0(0)
    }
}
impl core::fmt::Debug for Mask0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask0")
            .field("maskp", &self.maskp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mask0 {{ maskp: {=u32:?} }}", self.maskp())
    }
}
#[doc = "Masked port register for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpin0(pub u32);
impl Mpin0 {
    #[doc = "Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mportp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub const fn set_mportp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mpin0 {
    #[inline(always)]
    fn default() -> Mpin0 {
        Mpin0(0)
    }
}
impl core::fmt::Debug for Mpin0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mpin0")
            .field("mportp", &self.mportp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mpin0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mpin0 {{ mportp: {=u32:?} }}", self.mportp())
    }
}
#[doc = "Toggle port for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Not0(pub u32);
impl Not0 {
    #[doc = "Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn notp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub const fn set_notp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Not0 {
    #[inline(always)]
    fn default() -> Not0 {
        Not0(0)
    }
}
impl core::fmt::Debug for Not0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Not0").field("notp", &self.notp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Not0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Not0 {{ notp: {=u32:?} }}", self.notp())
    }
}
#[doc = "Port pin register for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pin0(pub u32);
impl Pin0 {
    #[doc = "Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub const fn set_port(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pin0 {
    #[inline(always)]
    fn default() -> Pin0 {
        Pin0(0)
    }
}
impl core::fmt::Debug for Pin0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pin0").field("port", &self.port()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pin0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pin0 {{ port: {=u32:?} }}", self.port())
    }
}
#[doc = "Write: Set register for port. Read: output bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Set0(pub u32);
impl Set0 {
    #[doc = "Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn setp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub const fn set_setp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Set0 {
    #[inline(always)]
    fn default() -> Set0 {
        Set0(0)
    }
}
impl core::fmt::Debug for Set0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Set0").field("setp", &self.setp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Set0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Set0 {{ setp: {=u32:?} }}", self.setp())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W00(pub u32);
impl W00 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W00 {
    #[inline(always)]
    fn default() -> W00 {
        W00(0)
    }
}
impl core::fmt::Debug for W00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W00").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W00 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W01(pub u32);
impl W01 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W01 {
    #[inline(always)]
    fn default() -> W01 {
        W01(0)
    }
}
impl core::fmt::Debug for W01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W01").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W01 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W010(pub u32);
impl W010 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W010 {
    #[inline(always)]
    fn default() -> W010 {
        W010(0)
    }
}
impl core::fmt::Debug for W010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W010")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W010 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W010 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W011(pub u32);
impl W011 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W011 {
    #[inline(always)]
    fn default() -> W011 {
        W011(0)
    }
}
impl core::fmt::Debug for W011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W011")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W011 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W011 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W012(pub u32);
impl W012 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W012 {
    #[inline(always)]
    fn default() -> W012 {
        W012(0)
    }
}
impl core::fmt::Debug for W012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W012")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W012 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W012 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W013(pub u32);
impl W013 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W013 {
    #[inline(always)]
    fn default() -> W013 {
        W013(0)
    }
}
impl core::fmt::Debug for W013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W013")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W013 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W013 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W014(pub u32);
impl W014 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W014 {
    #[inline(always)]
    fn default() -> W014 {
        W014(0)
    }
}
impl core::fmt::Debug for W014 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W014")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W014 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W014 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W015(pub u32);
impl W015 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W015 {
    #[inline(always)]
    fn default() -> W015 {
        W015(0)
    }
}
impl core::fmt::Debug for W015 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W015")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W015 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W015 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W016(pub u32);
impl W016 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W016 {
    #[inline(always)]
    fn default() -> W016 {
        W016(0)
    }
}
impl core::fmt::Debug for W016 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W016")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W016 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W016 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W017(pub u32);
impl W017 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W017 {
    #[inline(always)]
    fn default() -> W017 {
        W017(0)
    }
}
impl core::fmt::Debug for W017 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W017")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W017 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W017 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W018(pub u32);
impl W018 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W018 {
    #[inline(always)]
    fn default() -> W018 {
        W018(0)
    }
}
impl core::fmt::Debug for W018 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W018")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W018 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W018 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W019(pub u32);
impl W019 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W019 {
    #[inline(always)]
    fn default() -> W019 {
        W019(0)
    }
}
impl core::fmt::Debug for W019 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W019")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W019 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W019 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W02(pub u32);
impl W02 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W02 {
    #[inline(always)]
    fn default() -> W02 {
        W02(0)
    }
}
impl core::fmt::Debug for W02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W02").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W02 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W020(pub u32);
impl W020 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W020 {
    #[inline(always)]
    fn default() -> W020 {
        W020(0)
    }
}
impl core::fmt::Debug for W020 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W020")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W020 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W020 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W021(pub u32);
impl W021 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W021 {
    #[inline(always)]
    fn default() -> W021 {
        W021(0)
    }
}
impl core::fmt::Debug for W021 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W021")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W021 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W021 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W022(pub u32);
impl W022 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W022 {
    #[inline(always)]
    fn default() -> W022 {
        W022(0)
    }
}
impl core::fmt::Debug for W022 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W022")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W022 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W022 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W023(pub u32);
impl W023 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W023 {
    #[inline(always)]
    fn default() -> W023 {
        W023(0)
    }
}
impl core::fmt::Debug for W023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W023")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W023 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W024(pub u32);
impl W024 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W024 {
    #[inline(always)]
    fn default() -> W024 {
        W024(0)
    }
}
impl core::fmt::Debug for W024 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W024")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W024 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W024 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W025(pub u32);
impl W025 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W025 {
    #[inline(always)]
    fn default() -> W025 {
        W025(0)
    }
}
impl core::fmt::Debug for W025 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W025")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W025 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W025 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W026(pub u32);
impl W026 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W026 {
    #[inline(always)]
    fn default() -> W026 {
        W026(0)
    }
}
impl core::fmt::Debug for W026 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W026")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W026 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W026 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W027(pub u32);
impl W027 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W027 {
    #[inline(always)]
    fn default() -> W027 {
        W027(0)
    }
}
impl core::fmt::Debug for W027 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W027")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W027 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W027 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W028(pub u32);
impl W028 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W028 {
    #[inline(always)]
    fn default() -> W028 {
        W028(0)
    }
}
impl core::fmt::Debug for W028 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W028")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W028 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W028 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W029(pub u32);
impl W029 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W029 {
    #[inline(always)]
    fn default() -> W029 {
        W029(0)
    }
}
impl core::fmt::Debug for W029 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W029")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W029 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W029 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W03(pub u32);
impl W03 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W03 {
    #[inline(always)]
    fn default() -> W03 {
        W03(0)
    }
}
impl core::fmt::Debug for W03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W03").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W03 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W030(pub u32);
impl W030 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W030 {
    #[inline(always)]
    fn default() -> W030 {
        W030(0)
    }
}
impl core::fmt::Debug for W030 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W030")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W030 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W030 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W031(pub u32);
impl W031 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W031 {
    #[inline(always)]
    fn default() -> W031 {
        W031(0)
    }
}
impl core::fmt::Debug for W031 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W031")
            .field("pword", &self.pword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W031 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W031 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W04(pub u32);
impl W04 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W04 {
    #[inline(always)]
    fn default() -> W04 {
        W04(0)
    }
}
impl core::fmt::Debug for W04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W04").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W04 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W04 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W05(pub u32);
impl W05 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W05 {
    #[inline(always)]
    fn default() -> W05 {
        W05(0)
    }
}
impl core::fmt::Debug for W05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W05").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W05 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W05 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W06(pub u32);
impl W06 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W06 {
    #[inline(always)]
    fn default() -> W06 {
        W06(0)
    }
}
impl core::fmt::Debug for W06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W06").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W06 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W06 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W07(pub u32);
impl W07 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W07 {
    #[inline(always)]
    fn default() -> W07 {
        W07(0)
    }
}
impl core::fmt::Debug for W07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W07").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W07 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W07 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W08(pub u32);
impl W08 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W08 {
    #[inline(always)]
    fn default() -> W08 {
        W08(0)
    }
}
impl core::fmt::Debug for W08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W08").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W08 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W08 {{ pword: {=u32:?} }}", self.pword())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W09(pub u32);
impl W09 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W09 {
    #[inline(always)]
    fn default() -> W09 {
        W09(0)
    }
}
impl core::fmt::Debug for W09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W09").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W09 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W09 {{ pword: {=u32:?} }}", self.pword())
    }
}
