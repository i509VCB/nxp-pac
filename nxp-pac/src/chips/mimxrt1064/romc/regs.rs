#[doc = "ROMC Address Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rompatcha(pub u32);
impl Rompatcha {
    #[doc = "THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch"]
    #[must_use]
    #[inline(always)]
    pub const fn thumbx(&self) -> super::vals::Thumbx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Thumbx::from_bits(val as u8)
    }
    #[doc = "THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch"]
    #[inline(always)]
    pub const fn set_thumbx(&mut self, val: super::vals::Thumbx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Address Comparator Registers - Indicates the memory address to be watched"]
    #[must_use]
    #[inline(always)]
    pub const fn addrx(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Address Comparator Registers - Indicates the memory address to be watched"]
    #[inline(always)]
    pub const fn set_addrx(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 1usize)) | (((val as u32) & 0x003f_ffff) << 1usize);
    }
}
impl Default for Rompatcha {
    #[inline(always)]
    fn default() -> Rompatcha {
        Rompatcha(0)
    }
}
impl core::fmt::Debug for Rompatcha {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rompatcha")
            .field("thumbx", &self.thumbx())
            .field("addrx", &self.addrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rompatcha {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rompatcha {{ thumbx: {:?}, addrx: {=u32:?} }}",
            self.thumbx(),
            self.addrx()
        )
    }
}
#[doc = "ROMC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rompatchcntl(pub u32);
impl Rompatchcntl {
    #[doc = "Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    #[must_use]
    #[inline(always)]
    pub const fn datafix(&self) -> super::vals::Datafix {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Datafix::from_bits(val as u8)
    }
    #[doc = "Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    #[inline(always)]
    pub const fn set_datafix(&mut self, val: super::vals::Datafix) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "ROMC Disable -- This bit, when set, disables all ROMC operations"]
    #[must_use]
    #[inline(always)]
    pub const fn dis(&self) -> super::vals::Dis {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dis::from_bits(val as u8)
    }
    #[doc = "ROMC Disable -- This bit, when set, disables all ROMC operations"]
    #[inline(always)]
    pub const fn set_dis(&mut self, val: super::vals::Dis) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Rompatchcntl {
    #[inline(always)]
    fn default() -> Rompatchcntl {
        Rompatchcntl(0)
    }
}
impl core::fmt::Debug for Rompatchcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rompatchcntl")
            .field("datafix", &self.datafix())
            .field("dis", &self.dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rompatchcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rompatchcntl {{ datafix: {:?}, dis: {:?} }}",
            self.datafix(),
            self.dis()
        )
    }
}
#[doc = "ROMC Data Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rompatchd(pub u32);
impl Rompatchd {
    #[doc = "Data Fix Registers - Stores the data used for 1-word data fix operations"]
    #[must_use]
    #[inline(always)]
    pub const fn datax(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Fix Registers - Stores the data used for 1-word data fix operations"]
    #[inline(always)]
    pub const fn set_datax(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rompatchd {
    #[inline(always)]
    fn default() -> Rompatchd {
        Rompatchd(0)
    }
}
impl core::fmt::Debug for Rompatchd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rompatchd")
            .field("datax", &self.datax())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rompatchd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rompatchd {{ datax: {=u32:?} }}", self.datax())
    }
}
#[doc = "ROMC Enable Register Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rompatchenl(pub u32);
impl Rompatchenl {
    #[doc = "Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Enable::from_bits(val as u16)
    }
    #[doc = "Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rompatchenl {
    #[inline(always)]
    fn default() -> Rompatchenl {
        Rompatchenl(0)
    }
}
impl core::fmt::Debug for Rompatchenl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rompatchenl")
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rompatchenl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rompatchenl {{ enable: {:?} }}", self.enable())
    }
}
#[doc = "ROMC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rompatchsr(pub u32);
impl Rompatchsr {
    #[doc = "ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB"]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> super::vals::Source {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Source::from_bits(val as u8)
    }
    #[doc = "ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB"]
    #[inline(always)]
    pub const fn set_source(&mut self, val: super::vals::Source) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> super::vals::Sw {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Sw::from_bits(val as u8)
    }
    #[doc = "ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: super::vals::Sw) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for Rompatchsr {
    #[inline(always)]
    fn default() -> Rompatchsr {
        Rompatchsr(0)
    }
}
impl core::fmt::Debug for Rompatchsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rompatchsr")
            .field("source", &self.source())
            .field("sw", &self.sw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rompatchsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rompatchsr {{ source: {:?}, sw: {:?} }}",
            self.source(),
            self.sw()
        )
    }
}
