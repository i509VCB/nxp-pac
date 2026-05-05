#[doc = "Byte pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B_(pub u8);
impl B_ {
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
impl Default for B_ {
    #[inline(always)]
    fn default() -> B_ {
        B_(0)
    }
}
impl core::fmt::Debug for B_ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_").field("PBYTE", &self.PBYTE()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B_ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B_ {{ PBYTE: {=bool:?} }}", self.PBYTE())
    }
}
#[doc = "Clear port for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLR(pub u32);
impl CLR {
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
impl Default for CLR {
    #[inline(always)]
    fn default() -> CLR {
        CLR(0)
    }
}
impl core::fmt::Debug for CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR").field("CLRP", &self.CLRP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLR {{ CLRP: {=u32:?} }}", self.CLRP())
    }
}
#[doc = "Direction registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIR(pub u32);
impl DIR {
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
impl Default for DIR {
    #[inline(always)]
    fn default() -> DIR {
        DIR(0)
    }
}
impl core::fmt::Debug for DIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR").field("DIRP", &self.DIRP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIR {{ DIRP: {=u32:?} }}", self.DIRP())
    }
}
#[doc = "Clear pin direction bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIRCLR(pub u32);
impl DIRCLR {
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
impl Default for DIRCLR {
    #[inline(always)]
    fn default() -> DIRCLR {
        DIRCLR(0)
    }
}
impl core::fmt::Debug for DIRCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIRCLR")
            .field("DIRCLRP", &self.DIRCLRP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIRCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIRCLR {{ DIRCLRP: {=u32:?} }}", self.DIRCLRP())
    }
}
#[doc = "Toggle pin direction bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIRNOT(pub u32);
impl DIRNOT {
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
impl Default for DIRNOT {
    #[inline(always)]
    fn default() -> DIRNOT {
        DIRNOT(0)
    }
}
impl core::fmt::Debug for DIRNOT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIRNOT")
            .field("DIRNOTP", &self.DIRNOTP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIRNOT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIRNOT {{ DIRNOTP: {=u32:?} }}", self.DIRNOTP())
    }
}
#[doc = "Set pin direction bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIRSET(pub u32);
impl DIRSET {
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
impl Default for DIRSET {
    #[inline(always)]
    fn default() -> DIRSET {
        DIRSET(0)
    }
}
impl core::fmt::Debug for DIRSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIRSET")
            .field("DIRSETP", &self.DIRSETP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIRSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIRSET {{ DIRSETP: {=u32:?} }}", self.DIRSETP())
    }
}
#[doc = "Mask register for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK(pub u32);
impl MASK {
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
impl Default for MASK {
    #[inline(always)]
    fn default() -> MASK {
        MASK(0)
    }
}
impl core::fmt::Debug for MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK")
            .field("MASKP", &self.MASKP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASK {{ MASKP: {=u32:?} }}", self.MASKP())
    }
}
#[doc = "Masked port register for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MPIN(pub u32);
impl MPIN {
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
impl Default for MPIN {
    #[inline(always)]
    fn default() -> MPIN {
        MPIN(0)
    }
}
impl core::fmt::Debug for MPIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPIN")
            .field("MPORTP", &self.MPORTP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MPIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MPIN {{ MPORTP: {=u32:?} }}", self.MPORTP())
    }
}
#[doc = "Toggle port for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NOT(pub u32);
impl NOT {
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
impl Default for NOT {
    #[inline(always)]
    fn default() -> NOT {
        NOT(0)
    }
}
impl core::fmt::Debug for NOT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NOT").field("NOTP", &self.NOTP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NOT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NOT {{ NOTP: {=u32:?} }}", self.NOTP())
    }
}
#[doc = "Port pin register for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIN(pub u32);
impl PIN {
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
impl Default for PIN {
    #[inline(always)]
    fn default() -> PIN {
        PIN(0)
    }
}
impl core::fmt::Debug for PIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN").field("PORT", &self.PORT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PIN {{ PORT: {=u32:?} }}", self.PORT())
    }
}
#[doc = "Write: Set register for port. Read: output bits for port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET(pub u32);
impl SET {
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
impl Default for SET {
    #[inline(always)]
    fn default() -> SET {
        SET(0)
    }
}
impl core::fmt::Debug for SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET").field("SETP", &self.SETP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET {{ SETP: {=u32:?} }}", self.SETP())
    }
}
#[doc = "Word pin registers for all port GPIO pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W_(pub u32);
impl W_ {
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
impl Default for W_ {
    #[inline(always)]
    fn default() -> W_ {
        W_(0)
    }
}
impl core::fmt::Debug for W_ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W_").field("PWORD", &self.PWORD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W_ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W_ {{ PWORD: {=u32:?} }}", self.PWORD())
    }
}
