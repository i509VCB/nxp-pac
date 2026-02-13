#[doc = "Byte pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B(pub u8);
impl B {
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
impl Default for B {
    #[inline(always)]
    fn default() -> B {
        B(0)
    }
}
impl core::fmt::Debug for B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B").field("pbyte", &self.pbyte()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "B {{ pbyte: {=bool:?} }}", self.pbyte())
    }
}
#[doc = "Clear port for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clr(pub u32);
impl Clr {
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
impl Default for Clr {
    #[inline(always)]
    fn default() -> Clr {
        Clr(0)
    }
}
impl core::fmt::Debug for Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clr").field("clrp", &self.clrp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clr {{ clrp: {=u32:?} }}", self.clrp())
    }
}
#[doc = "Direction registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir(pub u32);
impl Dir {
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
impl Default for Dir {
    #[inline(always)]
    fn default() -> Dir {
        Dir(0)
    }
}
impl core::fmt::Debug for Dir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir").field("dirp", &self.dirp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dir {{ dirp: {=u32:?} }}", self.dirp())
    }
}
#[doc = "Clear pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirclr(pub u32);
impl Dirclr {
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
impl Default for Dirclr {
    #[inline(always)]
    fn default() -> Dirclr {
        Dirclr(0)
    }
}
impl core::fmt::Debug for Dirclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dirclr")
            .field("dirclrp", &self.dirclrp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dirclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dirclr {{ dirclrp: {=u32:?} }}", self.dirclrp())
    }
}
#[doc = "Toggle pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirnot(pub u32);
impl Dirnot {
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
impl Default for Dirnot {
    #[inline(always)]
    fn default() -> Dirnot {
        Dirnot(0)
    }
}
impl core::fmt::Debug for Dirnot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dirnot")
            .field("dirnotp", &self.dirnotp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dirnot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dirnot {{ dirnotp: {=u32:?} }}", self.dirnotp())
    }
}
#[doc = "Set pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirset(pub u32);
impl Dirset {
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
impl Default for Dirset {
    #[inline(always)]
    fn default() -> Dirset {
        Dirset(0)
    }
}
impl core::fmt::Debug for Dirset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dirset")
            .field("dirsetp", &self.dirsetp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dirset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dirset {{ dirsetp: {=u32:?} }}", self.dirsetp())
    }
}
#[doc = "Mask register for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u32);
impl Mask {
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
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
impl core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask")
            .field("maskp", &self.maskp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mask {{ maskp: {=u32:?} }}", self.maskp())
    }
}
#[doc = "Masked port register for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpin(pub u32);
impl Mpin {
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
impl Default for Mpin {
    #[inline(always)]
    fn default() -> Mpin {
        Mpin(0)
    }
}
impl core::fmt::Debug for Mpin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mpin")
            .field("mportp", &self.mportp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mpin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mpin {{ mportp: {=u32:?} }}", self.mportp())
    }
}
#[doc = "Toggle port for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Not(pub u32);
impl Not {
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
impl Default for Not {
    #[inline(always)]
    fn default() -> Not {
        Not(0)
    }
}
impl core::fmt::Debug for Not {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Not").field("notp", &self.notp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Not {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Not {{ notp: {=u32:?} }}", self.notp())
    }
}
#[doc = "Port pin register for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pin(pub u32);
impl Pin {
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
impl Default for Pin {
    #[inline(always)]
    fn default() -> Pin {
        Pin(0)
    }
}
impl core::fmt::Debug for Pin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pin").field("port", &self.port()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pin {{ port: {=u32:?} }}", self.port())
    }
}
#[doc = "Write: Set register for port. Read: output bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Set(pub u32);
impl Set {
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
impl Default for Set {
    #[inline(always)]
    fn default() -> Set {
        Set(0)
    }
}
impl core::fmt::Debug for Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Set").field("setp", &self.setp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Set {{ setp: {=u32:?} }}", self.setp())
    }
}
#[doc = "Word pin registers for all port GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W(pub u32);
impl W {
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
impl Default for W {
    #[inline(always)]
    fn default() -> W {
        W(0)
    }
}
impl core::fmt::Debug for W {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W").field("pword", &self.pword()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for W {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "W {{ pword: {=u32:?} }}", self.pword())
    }
}
