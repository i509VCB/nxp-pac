#[doc = "GPC Interface control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntr(pub u32);
impl Cntr {
    #[doc = "MEGA domain (FlexRAM PDRAM1) power down request"]
    #[must_use]
    #[inline(always)]
    pub const fn mega_pdn_req(&self) -> super::vals::MegaPdnReq {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MegaPdnReq::from_bits(val as u8)
    }
    #[doc = "MEGA domain (FlexRAM PDRAM1) power down request"]
    #[inline(always)]
    pub const fn set_mega_pdn_req(&mut self, val: super::vals::MegaPdnReq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MEGA domain (FlexRAM PDRAM1) power up request"]
    #[must_use]
    #[inline(always)]
    pub const fn mega_pup_req(&self) -> super::vals::MegaPupReq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MegaPupReq::from_bits(val as u8)
    }
    #[doc = "MEGA domain (FlexRAM PDRAM1) power up request"]
    #[inline(always)]
    pub const fn set_mega_pup_req(&mut self, val: super::vals::MegaPupReq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FlexRAM PDRAM0 Power Gate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pdram0_pge(&self) -> super::vals::Pdram0Pge {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdram0Pge::from_bits(val as u8)
    }
    #[doc = "FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    pub const fn set_pdram0_pge(&mut self, val: super::vals::Pdram0Pge) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for Cntr {
    #[inline(always)]
    fn default() -> Cntr {
        Cntr(0)
    }
}
impl core::fmt::Debug for Cntr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntr")
            .field("mega_pdn_req", &self.mega_pdn_req())
            .field("mega_pup_req", &self.mega_pup_req())
            .field("pdram0_pge", &self.pdram0_pge())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cntr {{ mega_pdn_req: {:?}, mega_pup_req: {:?}, pdram0_pge: {:?} }}",
            self.mega_pdn_req(),
            self.mega_pup_req(),
            self.pdram0_pge()
        )
    }
}
#[doc = "IRQ masking register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr1(pub u32);
impl Imr1 {
    #[doc = "IRQ\\[31:0\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[must_use]
    #[inline(always)]
    pub const fn imr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[31:0\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub const fn set_imr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imr1 {
    #[inline(always)]
    fn default() -> Imr1 {
        Imr1(0)
    }
}
impl core::fmt::Debug for Imr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr1").field("imr1", &self.imr1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr1 {{ imr1: {=u32:?} }}", self.imr1())
    }
}
#[doc = "IRQ masking register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr2(pub u32);
impl Imr2 {
    #[doc = "IRQ\\[63:32\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[must_use]
    #[inline(always)]
    pub const fn imr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[63:32\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub const fn set_imr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imr2 {
    #[inline(always)]
    fn default() -> Imr2 {
        Imr2(0)
    }
}
impl core::fmt::Debug for Imr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr2").field("imr2", &self.imr2()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr2 {{ imr2: {=u32:?} }}", self.imr2())
    }
}
#[doc = "IRQ masking register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr3(pub u32);
impl Imr3 {
    #[doc = "IRQ\\[95:64\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[must_use]
    #[inline(always)]
    pub const fn imr3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[95:64\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub const fn set_imr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imr3 {
    #[inline(always)]
    fn default() -> Imr3 {
        Imr3(0)
    }
}
impl core::fmt::Debug for Imr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr3").field("imr3", &self.imr3()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr3 {{ imr3: {=u32:?} }}", self.imr3())
    }
}
#[doc = "IRQ masking register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr4(pub u32);
impl Imr4 {
    #[doc = "IRQ\\[127:96\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[must_use]
    #[inline(always)]
    pub const fn imr4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[127:96\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub const fn set_imr4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imr4 {
    #[inline(always)]
    fn default() -> Imr4 {
        Imr4(0)
    }
}
impl core::fmt::Debug for Imr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr4").field("imr4", &self.imr4()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr4 {{ imr4: {=u32:?} }}", self.imr4())
    }
}
#[doc = "IRQ masking register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr5(pub u32);
impl Imr5 {
    #[doc = "IRQ\\[159:128\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[must_use]
    #[inline(always)]
    pub const fn imr5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[159:128\\] masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub const fn set_imr5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imr5 {
    #[inline(always)]
    fn default() -> Imr5 {
        Imr5(0)
    }
}
impl core::fmt::Debug for Imr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr5").field("imr5", &self.imr5()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr5 {{ imr5: {=u32:?} }}", self.imr5())
    }
}
#[doc = "IRQ status resister 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr1(pub u32);
impl Isr1 {
    #[doc = "IRQ\\[31:0\\] status, read only"]
    #[must_use]
    #[inline(always)]
    pub const fn isr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[31:0\\] status, read only"]
    #[inline(always)]
    pub const fn set_isr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr1 {
    #[inline(always)]
    fn default() -> Isr1 {
        Isr1(0)
    }
}
impl core::fmt::Debug for Isr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr1").field("isr1", &self.isr1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr1 {{ isr1: {=u32:?} }}", self.isr1())
    }
}
#[doc = "IRQ status resister 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr2(pub u32);
impl Isr2 {
    #[doc = "IRQ\\[63:32\\] status, read only"]
    #[must_use]
    #[inline(always)]
    pub const fn isr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[63:32\\] status, read only"]
    #[inline(always)]
    pub const fn set_isr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr2 {
    #[inline(always)]
    fn default() -> Isr2 {
        Isr2(0)
    }
}
impl core::fmt::Debug for Isr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr2").field("isr2", &self.isr2()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr2 {{ isr2: {=u32:?} }}", self.isr2())
    }
}
#[doc = "IRQ status resister 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr3(pub u32);
impl Isr3 {
    #[doc = "IRQ\\[95:64\\] status, read only"]
    #[must_use]
    #[inline(always)]
    pub const fn isr3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[95:64\\] status, read only"]
    #[inline(always)]
    pub const fn set_isr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr3 {
    #[inline(always)]
    fn default() -> Isr3 {
        Isr3(0)
    }
}
impl core::fmt::Debug for Isr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr3").field("isr3", &self.isr3()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr3 {{ isr3: {=u32:?} }}", self.isr3())
    }
}
#[doc = "IRQ status resister 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr4(pub u32);
impl Isr4 {
    #[doc = "IRQ\\[127:96\\] status, read only"]
    #[must_use]
    #[inline(always)]
    pub const fn isr4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[127:96\\] status, read only"]
    #[inline(always)]
    pub const fn set_isr4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr4 {
    #[inline(always)]
    fn default() -> Isr4 {
        Isr4(0)
    }
}
impl core::fmt::Debug for Isr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr4").field("isr4", &self.isr4()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr4 {{ isr4: {=u32:?} }}", self.isr4())
    }
}
#[doc = "IRQ status resister 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr5(pub u32);
impl Isr5 {
    #[doc = "IRQ\\[159:128\\] status, read only"]
    #[must_use]
    #[inline(always)]
    pub const fn isr5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ\\[159:128\\] status, read only"]
    #[inline(always)]
    pub const fn set_isr5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr5 {
    #[inline(always)]
    fn default() -> Isr5 {
        Isr5(0)
    }
}
impl core::fmt::Debug for Isr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr5").field("isr5", &self.isr5()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr5 {{ isr5: {=u32:?} }}", self.isr5())
    }
}
