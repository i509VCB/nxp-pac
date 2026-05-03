#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_0(pub u8);
impl B0_0 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_0 {
    #[inline(always)]
    fn default() -> B0_0 {
        B0_0(0)
    }
}
impl core::fmt::Debug for B0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_0")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_0 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_1(pub u8);
impl B0_1 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_1 {
    #[inline(always)]
    fn default() -> B0_1 {
        B0_1(0)
    }
}
impl core::fmt::Debug for B0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_1")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_1 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_10(pub u8);
impl B0_10 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_10 {
    #[inline(always)]
    fn default() -> B0_10 {
        B0_10(0)
    }
}
impl core::fmt::Debug for B0_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_10")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_10 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_11(pub u8);
impl B0_11 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_11 {
    #[inline(always)]
    fn default() -> B0_11 {
        B0_11(0)
    }
}
impl core::fmt::Debug for B0_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_11")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_11 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_12(pub u8);
impl B0_12 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_12 {
    #[inline(always)]
    fn default() -> B0_12 {
        B0_12(0)
    }
}
impl core::fmt::Debug for B0_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_12")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_12 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_13(pub u8);
impl B0_13 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_13 {
    #[inline(always)]
    fn default() -> B0_13 {
        B0_13(0)
    }
}
impl core::fmt::Debug for B0_13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_13")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_13 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_14(pub u8);
impl B0_14 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_14 {
    #[inline(always)]
    fn default() -> B0_14 {
        B0_14(0)
    }
}
impl core::fmt::Debug for B0_14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_14")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_14 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_15(pub u8);
impl B0_15 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_15 {
    #[inline(always)]
    fn default() -> B0_15 {
        B0_15(0)
    }
}
impl core::fmt::Debug for B0_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_15")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_15 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_16(pub u8);
impl B0_16 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_16 {
    #[inline(always)]
    fn default() -> B0_16 {
        B0_16(0)
    }
}
impl core::fmt::Debug for B0_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_16")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_16 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_17(pub u8);
impl B0_17 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_17 {
    #[inline(always)]
    fn default() -> B0_17 {
        B0_17(0)
    }
}
impl core::fmt::Debug for B0_17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_17")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_17 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_18(pub u8);
impl B0_18 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_18 {
    #[inline(always)]
    fn default() -> B0_18 {
        B0_18(0)
    }
}
impl core::fmt::Debug for B0_18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_18")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_18 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_19(pub u8);
impl B0_19 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_19 {
    #[inline(always)]
    fn default() -> B0_19 {
        B0_19(0)
    }
}
impl core::fmt::Debug for B0_19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_19")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_19 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_2(pub u8);
impl B0_2 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_2 {
    #[inline(always)]
    fn default() -> B0_2 {
        B0_2(0)
    }
}
impl core::fmt::Debug for B0_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_2")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_2 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_20(pub u8);
impl B0_20 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_20 {
    #[inline(always)]
    fn default() -> B0_20 {
        B0_20(0)
    }
}
impl core::fmt::Debug for B0_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_20")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_20 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_21(pub u8);
impl B0_21 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_21 {
    #[inline(always)]
    fn default() -> B0_21 {
        B0_21(0)
    }
}
impl core::fmt::Debug for B0_21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_21")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_21 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_22(pub u8);
impl B0_22 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_22 {
    #[inline(always)]
    fn default() -> B0_22 {
        B0_22(0)
    }
}
impl core::fmt::Debug for B0_22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_22")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_22 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_23(pub u8);
impl B0_23 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_23 {
    #[inline(always)]
    fn default() -> B0_23 {
        B0_23(0)
    }
}
impl core::fmt::Debug for B0_23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_23")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_23 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_24(pub u8);
impl B0_24 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_24 {
    #[inline(always)]
    fn default() -> B0_24 {
        B0_24(0)
    }
}
impl core::fmt::Debug for B0_24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_24")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_24 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_25(pub u8);
impl B0_25 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_25 {
    #[inline(always)]
    fn default() -> B0_25 {
        B0_25(0)
    }
}
impl core::fmt::Debug for B0_25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_25")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_25 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_26(pub u8);
impl B0_26 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_26 {
    #[inline(always)]
    fn default() -> B0_26 {
        B0_26(0)
    }
}
impl core::fmt::Debug for B0_26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_26")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_26 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_27(pub u8);
impl B0_27 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_27 {
    #[inline(always)]
    fn default() -> B0_27 {
        B0_27(0)
    }
}
impl core::fmt::Debug for B0_27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_27")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_27 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_28(pub u8);
impl B0_28 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_28 {
    #[inline(always)]
    fn default() -> B0_28 {
        B0_28(0)
    }
}
impl core::fmt::Debug for B0_28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_28")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_28 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_29(pub u8);
impl B0_29 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_29 {
    #[inline(always)]
    fn default() -> B0_29 {
        B0_29(0)
    }
}
impl core::fmt::Debug for B0_29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_29")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_29 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_3(pub u8);
impl B0_3 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_3 {
    #[inline(always)]
    fn default() -> B0_3 {
        B0_3(0)
    }
}
impl core::fmt::Debug for B0_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_3")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_3 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_30(pub u8);
impl B0_30 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_30 {
    #[inline(always)]
    fn default() -> B0_30 {
        B0_30(0)
    }
}
impl core::fmt::Debug for B0_30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_30")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_30 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_31(pub u8);
impl B0_31 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_31 {
    #[inline(always)]
    fn default() -> B0_31 {
        B0_31(0)
    }
}
impl core::fmt::Debug for B0_31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_31")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_31 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_4(pub u8);
impl B0_4 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_4 {
    #[inline(always)]
    fn default() -> B0_4 {
        B0_4(0)
    }
}
impl core::fmt::Debug for B0_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_4")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_4 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_5(pub u8);
impl B0_5 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_5 {
    #[inline(always)]
    fn default() -> B0_5 {
        B0_5(0)
    }
}
impl core::fmt::Debug for B0_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_5")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_5 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_6(pub u8);
impl B0_6 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_6 {
    #[inline(always)]
    fn default() -> B0_6 {
        B0_6(0)
    }
}
impl core::fmt::Debug for B0_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_6")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_6 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_7(pub u8);
impl B0_7 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_7 {
    #[inline(always)]
    fn default() -> B0_7 {
        B0_7(0)
    }
}
impl core::fmt::Debug for B0_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_7")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_7 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_8(pub u8);
impl B0_8 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_8 {
    #[inline(always)]
    fn default() -> B0_8 {
        B0_8(0)
    }
}
impl core::fmt::Debug for B0_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_8")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_8 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0_9(pub u8);
impl B0_9 {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PBYTE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B0_9 {
    #[inline(always)]
    fn default() -> B0_9 {
        B0_9(0)
    }
}
impl core::fmt::Debug for B0_9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_9")
            .field("PBYTE", &self.PBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B0_9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B0_9 {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Clear port for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLR0(pub u32);
impl CLR0 {
    #[doc = "Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub const fn set_CLRP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLR0 {
    #[inline(always)]
    fn default() -> CLR0 {
        CLR0(0)
    }
}
impl core::fmt::Debug for CLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR0").field("CLRP", &self.CLRP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLR0 {{ CLRP: {=u32:?} }}", self.CLRP())
    }
}
#[doc = "Direction registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIR0(pub u32);
impl DIR0 {
    #[doc = "Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[must_use]
    #[inline(always)]
    pub const fn DIRP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub const fn set_DIRP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DIR0 {
    #[inline(always)]
    fn default() -> DIR0 {
        DIR0(0)
    }
}
impl core::fmt::Debug for DIR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR0").field("DIRP", &self.DIRP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIR0 {{ DIRP: {=u32:?} }}", self.DIRP())
    }
}
#[doc = "Clear pin direction bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIRCLR0(pub u32);
impl DIRCLR0 {
    #[doc = "Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[must_use]
    #[inline(always)]
    pub const fn DIRCLRP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub const fn set_DIRCLRP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DIRCLR0 {
    #[inline(always)]
    fn default() -> DIRCLR0 {
        DIRCLR0(0)
    }
}
impl core::fmt::Debug for DIRCLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIRCLR0")
            .field("DIRCLRP", &self.DIRCLRP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIRCLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIRCLR0 {{ DIRCLRP: {=u32:?} }}", self.DIRCLRP())
    }
}
#[doc = "Toggle pin direction bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIRNOT0(pub u32);
impl DIRNOT0 {
    #[doc = "Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[must_use]
    #[inline(always)]
    pub const fn DIRNOTP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    pub const fn set_DIRNOTP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DIRNOT0 {
    #[inline(always)]
    fn default() -> DIRNOT0 {
        DIRNOT0(0)
    }
}
impl core::fmt::Debug for DIRNOT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIRNOT0")
            .field("DIRNOTP", &self.DIRNOTP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIRNOT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIRNOT0 {{ DIRNOTP: {=u32:?} }}", self.DIRNOTP())
    }
}
#[doc = "Set pin direction bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIRSET0(pub u32);
impl DIRSET0 {
    #[doc = "Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[must_use]
    #[inline(always)]
    pub const fn DIRSETP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub const fn set_DIRSETP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DIRSET0 {
    #[inline(always)]
    fn default() -> DIRSET0 {
        DIRSET0(0)
    }
}
impl core::fmt::Debug for DIRSET0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIRSET0")
            .field("DIRSETP", &self.DIRSETP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIRSET0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIRSET0 {{ DIRSETP: {=u32:?} }}", self.DIRSETP())
    }
}
#[doc = "Mask register for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK0(pub u32);
impl MASK0 {
    #[doc = "Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[must_use]
    #[inline(always)]
    pub const fn MASKP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub const fn set_MASKP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MASK0 {
    #[inline(always)]
    fn default() -> MASK0 {
        MASK0(0)
    }
}
impl core::fmt::Debug for MASK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK0")
            .field("MASKP", &self.MASKP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASK0 {{ MASKP: {=u32:?} }}", self.MASKP())
    }
}
#[doc = "Masked port register for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MPIN0(pub u32);
impl MPIN0 {
    #[doc = "Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn MPORTP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub const fn set_MPORTP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MPIN0 {
    #[inline(always)]
    fn default() -> MPIN0 {
        MPIN0(0)
    }
}
impl core::fmt::Debug for MPIN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPIN0")
            .field("MPORTP", &self.MPORTP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MPIN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MPIN0 {{ MPORTP: {=u32:?} }}", self.MPORTP())
    }
}
#[doc = "Toggle port for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NOT0(pub u32);
impl NOT0 {
    #[doc = "Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn NOTP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub const fn set_NOTP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NOT0 {
    #[inline(always)]
    fn default() -> NOT0 {
        NOT0(0)
    }
}
impl core::fmt::Debug for NOT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NOT0").field("NOTP", &self.NOTP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NOT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NOT0 {{ NOTP: {=u32:?} }}", self.NOTP())
    }
}
#[doc = "Port pin register for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIN0(pub u32);
impl PIN0 {
    #[doc = "Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub const fn set_PORT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PIN0 {
    #[inline(always)]
    fn default() -> PIN0 {
        PIN0(0)
    }
}
impl core::fmt::Debug for PIN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN0").field("PORT", &self.PORT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PIN0 {{ PORT: {=u32:?} }}", self.PORT())
    }
}
#[doc = "Write: Set register for port. Read: output bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET0(pub u32);
impl SET0 {
    #[doc = "Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[must_use]
    #[inline(always)]
    pub const fn SETP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub const fn set_SETP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SET0 {
    #[inline(always)]
    fn default() -> SET0 {
        SET0(0)
    }
}
impl core::fmt::Debug for SET0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET0").field("SETP", &self.SETP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET0 {{ SETP: {=u32:?} }}", self.SETP())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_0(pub u32);
impl W0_0 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_0 {
    #[inline(always)]
    fn default() -> W0_0 {
        W0_0(0)
    }
}
impl core::fmt::Debug for W0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_0")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_0 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_1(pub u32);
impl W0_1 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_1 {
    #[inline(always)]
    fn default() -> W0_1 {
        W0_1(0)
    }
}
impl core::fmt::Debug for W0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_1")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_1 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_10(pub u32);
impl W0_10 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_10 {
    #[inline(always)]
    fn default() -> W0_10 {
        W0_10(0)
    }
}
impl core::fmt::Debug for W0_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_10")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_10 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_11(pub u32);
impl W0_11 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_11 {
    #[inline(always)]
    fn default() -> W0_11 {
        W0_11(0)
    }
}
impl core::fmt::Debug for W0_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_11")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_11 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_12(pub u32);
impl W0_12 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_12 {
    #[inline(always)]
    fn default() -> W0_12 {
        W0_12(0)
    }
}
impl core::fmt::Debug for W0_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_12")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_12 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_13(pub u32);
impl W0_13 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_13 {
    #[inline(always)]
    fn default() -> W0_13 {
        W0_13(0)
    }
}
impl core::fmt::Debug for W0_13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_13")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_13 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_14(pub u32);
impl W0_14 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_14 {
    #[inline(always)]
    fn default() -> W0_14 {
        W0_14(0)
    }
}
impl core::fmt::Debug for W0_14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_14")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_14 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_15(pub u32);
impl W0_15 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_15 {
    #[inline(always)]
    fn default() -> W0_15 {
        W0_15(0)
    }
}
impl core::fmt::Debug for W0_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_15")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_15 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_16(pub u32);
impl W0_16 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_16 {
    #[inline(always)]
    fn default() -> W0_16 {
        W0_16(0)
    }
}
impl core::fmt::Debug for W0_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_16")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_16 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_17(pub u32);
impl W0_17 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_17 {
    #[inline(always)]
    fn default() -> W0_17 {
        W0_17(0)
    }
}
impl core::fmt::Debug for W0_17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_17")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_17 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_18(pub u32);
impl W0_18 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_18 {
    #[inline(always)]
    fn default() -> W0_18 {
        W0_18(0)
    }
}
impl core::fmt::Debug for W0_18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_18")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_18 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_19(pub u32);
impl W0_19 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_19 {
    #[inline(always)]
    fn default() -> W0_19 {
        W0_19(0)
    }
}
impl core::fmt::Debug for W0_19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_19")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_19 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_2(pub u32);
impl W0_2 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_2 {
    #[inline(always)]
    fn default() -> W0_2 {
        W0_2(0)
    }
}
impl core::fmt::Debug for W0_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_2")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_2 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_20(pub u32);
impl W0_20 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_20 {
    #[inline(always)]
    fn default() -> W0_20 {
        W0_20(0)
    }
}
impl core::fmt::Debug for W0_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_20")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_20 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_21(pub u32);
impl W0_21 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_21 {
    #[inline(always)]
    fn default() -> W0_21 {
        W0_21(0)
    }
}
impl core::fmt::Debug for W0_21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_21")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_21 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_22(pub u32);
impl W0_22 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_22 {
    #[inline(always)]
    fn default() -> W0_22 {
        W0_22(0)
    }
}
impl core::fmt::Debug for W0_22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_22")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_22 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_23(pub u32);
impl W0_23 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_23 {
    #[inline(always)]
    fn default() -> W0_23 {
        W0_23(0)
    }
}
impl core::fmt::Debug for W0_23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_23")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_23 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_24(pub u32);
impl W0_24 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_24 {
    #[inline(always)]
    fn default() -> W0_24 {
        W0_24(0)
    }
}
impl core::fmt::Debug for W0_24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_24")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_24 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_25(pub u32);
impl W0_25 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_25 {
    #[inline(always)]
    fn default() -> W0_25 {
        W0_25(0)
    }
}
impl core::fmt::Debug for W0_25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_25")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_25 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_26(pub u32);
impl W0_26 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_26 {
    #[inline(always)]
    fn default() -> W0_26 {
        W0_26(0)
    }
}
impl core::fmt::Debug for W0_26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_26")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_26 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_27(pub u32);
impl W0_27 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_27 {
    #[inline(always)]
    fn default() -> W0_27 {
        W0_27(0)
    }
}
impl core::fmt::Debug for W0_27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_27")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_27 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_28(pub u32);
impl W0_28 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_28 {
    #[inline(always)]
    fn default() -> W0_28 {
        W0_28(0)
    }
}
impl core::fmt::Debug for W0_28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_28")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_28 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_29(pub u32);
impl W0_29 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_29 {
    #[inline(always)]
    fn default() -> W0_29 {
        W0_29(0)
    }
}
impl core::fmt::Debug for W0_29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_29")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_29 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_3(pub u32);
impl W0_3 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_3 {
    #[inline(always)]
    fn default() -> W0_3 {
        W0_3(0)
    }
}
impl core::fmt::Debug for W0_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_3")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_3 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_30(pub u32);
impl W0_30 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_30 {
    #[inline(always)]
    fn default() -> W0_30 {
        W0_30(0)
    }
}
impl core::fmt::Debug for W0_30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_30")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_30 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_31(pub u32);
impl W0_31 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_31 {
    #[inline(always)]
    fn default() -> W0_31 {
        W0_31(0)
    }
}
impl core::fmt::Debug for W0_31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_31")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_31 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_4(pub u32);
impl W0_4 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_4 {
    #[inline(always)]
    fn default() -> W0_4 {
        W0_4(0)
    }
}
impl core::fmt::Debug for W0_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_4")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_4 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_5(pub u32);
impl W0_5 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_5 {
    #[inline(always)]
    fn default() -> W0_5 {
        W0_5(0)
    }
}
impl core::fmt::Debug for W0_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_5")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_5 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_6(pub u32);
impl W0_6 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_6 {
    #[inline(always)]
    fn default() -> W0_6 {
        W0_6(0)
    }
}
impl core::fmt::Debug for W0_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_6")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_6 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_7(pub u32);
impl W0_7 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_7 {
    #[inline(always)]
    fn default() -> W0_7 {
        W0_7(0)
    }
}
impl core::fmt::Debug for W0_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_7")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_7 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_8(pub u32);
impl W0_8 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_8 {
    #[inline(always)]
    fn default() -> W0_8 {
        W0_8(0)
    }
}
impl core::fmt::Debug for W0_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_8")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_8 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W0_9(pub u32);
impl W0_9 {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[must_use]
    #[inline(always)]
    pub const fn PWORD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn set_PWORD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W0_9 {
    #[inline(always)]
    fn default() -> W0_9 {
        W0_9(0)
    }
}
impl core::fmt::Debug for W0_9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_9")
            .field("PWORD", &self.PWORD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W0_9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W0_9 {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
