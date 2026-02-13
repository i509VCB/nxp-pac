#[doc = "Error Injection Channel Descriptor 0, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd0Word0(pub u32);
impl Eichd0Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd0Word0 {
    #[inline(always)]
    fn default() -> Eichd0Word0 {
        Eichd0Word0(0)
    }
}
impl core::fmt::Debug for Eichd0Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd0Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd0Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd0Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd0Word1(pub u32);
impl Eichd0Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd0Word1 {
    #[inline(always)]
    fn default() -> Eichd0Word1 {
        Eichd0Word1(0)
    }
}
impl core::fmt::Debug for Eichd0Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd0Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd0Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd0Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd1Word0(pub u32);
impl Eichd1Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd1Word0 {
    #[inline(always)]
    fn default() -> Eichd1Word0 {
        Eichd1Word0(0)
    }
}
impl core::fmt::Debug for Eichd1Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd1Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd1Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd1Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd1Word1(pub u32);
impl Eichd1Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd1Word1 {
    #[inline(always)]
    fn default() -> Eichd1Word1 {
        Eichd1Word1(0)
    }
}
impl core::fmt::Debug for Eichd1Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd1Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd1Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd1Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd2Word0(pub u32);
impl Eichd2Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd2Word0 {
    #[inline(always)]
    fn default() -> Eichd2Word0 {
        Eichd2Word0(0)
    }
}
impl core::fmt::Debug for Eichd2Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd2Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd2Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd2Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd2Word1(pub u32);
impl Eichd2Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd2Word1 {
    #[inline(always)]
    fn default() -> Eichd2Word1 {
        Eichd2Word1(0)
    }
}
impl core::fmt::Debug for Eichd2Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd2Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd2Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd2Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd3Word0(pub u32);
impl Eichd3Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd3Word0 {
    #[inline(always)]
    fn default() -> Eichd3Word0 {
        Eichd3Word0(0)
    }
}
impl core::fmt::Debug for Eichd3Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd3Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd3Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd3Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd3Word1(pub u32);
impl Eichd3Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd3Word1 {
    #[inline(always)]
    fn default() -> Eichd3Word1 {
        Eichd3Word1(0)
    }
}
impl core::fmt::Debug for Eichd3Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd3Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd3Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd3Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd4Word0(pub u32);
impl Eichd4Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd4Word0 {
    #[inline(always)]
    fn default() -> Eichd4Word0 {
        Eichd4Word0(0)
    }
}
impl core::fmt::Debug for Eichd4Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd4Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd4Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd4Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd4Word1(pub u32);
impl Eichd4Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd4Word1 {
    #[inline(always)]
    fn default() -> Eichd4Word1 {
        Eichd4Word1(0)
    }
}
impl core::fmt::Debug for Eichd4Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd4Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd4Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd4Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 5, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd5Word0(pub u32);
impl Eichd5Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd5Word0 {
    #[inline(always)]
    fn default() -> Eichd5Word0 {
        Eichd5Word0(0)
    }
}
impl core::fmt::Debug for Eichd5Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd5Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd5Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd5Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 5, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd5Word1(pub u32);
impl Eichd5Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd5Word1 {
    #[inline(always)]
    fn default() -> Eichd5Word1 {
        Eichd5Word1(0)
    }
}
impl core::fmt::Debug for Eichd5Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd5Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd5Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd5Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 6, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd6Word0(pub u32);
impl Eichd6Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd6Word0 {
    #[inline(always)]
    fn default() -> Eichd6Word0 {
        Eichd6Word0(0)
    }
}
impl core::fmt::Debug for Eichd6Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd6Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd6Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd6Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 6, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd6Word1(pub u32);
impl Eichd6Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd6Word1 {
    #[inline(always)]
    fn default() -> Eichd6Word1 {
        Eichd6Word1(0)
    }
}
impl core::fmt::Debug for Eichd6Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd6Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd6Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd6Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 7, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd7Word0(pub u32);
impl Eichd7Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eichd7Word0 {
    #[inline(always)]
    fn default() -> Eichd7Word0 {
        Eichd7Word0(0)
    }
}
impl core::fmt::Debug for Eichd7Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd7Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd7Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd7Word0 {{ chkbit_mask: {=bool:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 7, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd7Word1(pub u32);
impl Eichd7Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd7Word1 {
    #[inline(always)]
    fn default() -> Eichd7Word1 {
        Eichd7Word1(0)
    }
}
impl core::fmt::Debug for Eichd7Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd7Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd7Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd7Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 8, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd8Word0(pub u32);
impl Eichd8Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Eichd8Word0 {
    #[inline(always)]
    fn default() -> Eichd8Word0 {
        Eichd8Word0(0)
    }
}
impl core::fmt::Debug for Eichd8Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd8Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd8Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd8Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 8, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd8Word1(pub u32);
impl Eichd8Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd8Word1 {
    #[inline(always)]
    fn default() -> Eichd8Word1 {
        Eichd8Word1(0)
    }
}
impl core::fmt::Debug for Eichd8Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd8Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd8Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd8Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichen(pub u32);
impl Eichen {
    #[doc = "Error Injection Channel 8 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich8en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 8 Enable"]
    #[inline(always)]
    pub const fn set_eich8en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Error Injection Channel 7 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich7en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 7 Enable"]
    #[inline(always)]
    pub const fn set_eich7en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Error Injection Channel 6 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich6en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 6 Enable"]
    #[inline(always)]
    pub const fn set_eich6en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Error Injection Channel 5 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich5en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 5 Enable"]
    #[inline(always)]
    pub const fn set_eich5en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Error Injection Channel 4 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich4en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 4 Enable"]
    #[inline(always)]
    pub const fn set_eich4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Error Injection Channel 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich3en(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 3 Enable"]
    #[inline(always)]
    pub const fn set_eich3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Error Injection Channel 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich2en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 2 Enable"]
    #[inline(always)]
    pub const fn set_eich2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Error Injection Channel 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich1en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 1 Enable"]
    #[inline(always)]
    pub const fn set_eich1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Error Injection Channel 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich0en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 0 Enable"]
    #[inline(always)]
    pub const fn set_eich0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eichen {
    #[inline(always)]
    fn default() -> Eichen {
        Eichen(0)
    }
}
impl core::fmt::Debug for Eichen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichen")
            .field("eich8en", &self.eich8en())
            .field("eich7en", &self.eich7en())
            .field("eich6en", &self.eich6en())
            .field("eich5en", &self.eich5en())
            .field("eich4en", &self.eich4en())
            .field("eich3en", &self.eich3en())
            .field("eich2en", &self.eich2en())
            .field("eich1en", &self.eich1en())
            .field("eich0en", &self.eich0en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichen {{ eich8en: {=bool:?}, eich7en: {=bool:?}, eich6en: {=bool:?}, eich5en: {=bool:?}, eich4en: {=bool:?}, eich3en: {=bool:?}, eich2en: {=bool:?}, eich1en: {=bool:?}, eich0en: {=bool:?} }}",
            self.eich8en(),
            self.eich7en(),
            self.eich6en(),
            self.eich5en(),
            self.eich4en(),
            self.eich3en(),
            self.eich2en(),
            self.eich1en(),
            self.eich0en()
        )
    }
}
#[doc = "Error Injection Module Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eimcr(pub u32);
impl Eimcr {
    #[doc = "Global Error Injection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn geien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Global Error Injection Enable"]
    #[inline(always)]
    pub const fn set_geien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Eimcr {
    #[inline(always)]
    fn default() -> Eimcr {
        Eimcr(0)
    }
}
impl core::fmt::Debug for Eimcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eimcr")
            .field("geien", &self.geien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eimcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eimcr {{ geien: {=bool:?} }}", self.geien())
    }
}
