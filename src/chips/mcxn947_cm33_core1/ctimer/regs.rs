#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Rising Edge of Capture Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cap0re(&self) -> super::vals::Cap0re {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cap0re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 0"]
    #[inline(always)]
    pub const fn set_cap0re(&mut self, val: super::vals::Cap0re) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Falling Edge of Capture Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cap0fe(&self) -> super::vals::Cap0fe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cap0fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 0"]
    #[inline(always)]
    pub const fn set_cap0fe(&mut self, val: super::vals::Cap0fe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Generate Interrupt on Channel 0 Capture Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cap0i(&self) -> super::vals::Cap0i {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cap0i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 0 Capture Event"]
    #[inline(always)]
    pub const fn set_cap0i(&mut self, val: super::vals::Cap0i) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Rising Edge of Capture Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cap1re(&self) -> super::vals::Cap1re {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cap1re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 1"]
    #[inline(always)]
    pub const fn set_cap1re(&mut self, val: super::vals::Cap1re) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling Edge of Capture Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cap1fe(&self) -> super::vals::Cap1fe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cap1fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 1"]
    #[inline(always)]
    pub const fn set_cap1fe(&mut self, val: super::vals::Cap1fe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Generate Interrupt on Channel 1 Capture Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cap1i(&self) -> super::vals::Cap1i {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cap1i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 1 Capture Event"]
    #[inline(always)]
    pub const fn set_cap1i(&mut self, val: super::vals::Cap1i) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Rising Edge of Capture Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cap2re(&self) -> super::vals::Cap2re {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cap2re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 2"]
    #[inline(always)]
    pub const fn set_cap2re(&mut self, val: super::vals::Cap2re) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Falling Edge of Capture Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cap2fe(&self) -> super::vals::Cap2fe {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cap2fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 2"]
    #[inline(always)]
    pub const fn set_cap2fe(&mut self, val: super::vals::Cap2fe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Generate Interrupt on Channel 2 Capture Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cap2i(&self) -> super::vals::Cap2i {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cap2i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 2 Capture Event"]
    #[inline(always)]
    pub const fn set_cap2i(&mut self, val: super::vals::Cap2i) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising Edge of Capture Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn cap3re(&self) -> super::vals::Cap3re {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cap3re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 3"]
    #[inline(always)]
    pub const fn set_cap3re(&mut self, val: super::vals::Cap3re) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Falling Edge of Capture Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn cap3fe(&self) -> super::vals::Cap3fe {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cap3fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 3"]
    #[inline(always)]
    pub const fn set_cap3fe(&mut self, val: super::vals::Cap3fe) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Generate Interrupt on Channel 3 Capture Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cap3i(&self) -> super::vals::Cap3i {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cap3i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 3 Capture Event"]
    #[inline(always)]
    pub const fn set_cap3i(&mut self, val: super::vals::Cap3i) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("cap0re", &self.cap0re())
            .field("cap0fe", &self.cap0fe())
            .field("cap0i", &self.cap0i())
            .field("cap1re", &self.cap1re())
            .field("cap1fe", &self.cap1fe())
            .field("cap1i", &self.cap1i())
            .field("cap2re", &self.cap2re())
            .field("cap2fe", &self.cap2fe())
            .field("cap2i", &self.cap2i())
            .field("cap3re", &self.cap3re())
            .field("cap3fe", &self.cap3fe())
            .field("cap3i", &self.cap3i())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ cap0re: {:?}, cap0fe: {:?}, cap0i: {:?}, cap1re: {:?}, cap1fe: {:?}, cap1i: {:?}, cap2re: {:?}, cap2fe: {:?}, cap2i: {:?}, cap3re: {:?}, cap3fe: {:?}, cap3i: {:?} }}",
            self.cap0re(),
            self.cap0fe(),
            self.cap0i(),
            self.cap1re(),
            self.cap1fe(),
            self.cap1i(),
            self.cap2re(),
            self.cap2fe(),
            self.cap2i(),
            self.cap3re(),
            self.cap3fe(),
            self.cap3i()
        )
    }
}
#[doc = "Capture"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Timer Counter Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cap(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Capture Value"]
    #[inline(always)]
    pub const fn set_cap(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr").field("cap", &self.cap()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cr {{ cap: {=u32:?} }}", self.cap())
    }
}
#[doc = "Count Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctcr(pub u32);
impl Ctcr {
    #[doc = "Counter Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ctmode(&self) -> super::vals::Ctmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctmode::from_bits(val as u8)
    }
    #[doc = "Counter Timer Mode"]
    #[inline(always)]
    pub const fn set_ctmode(&mut self, val: super::vals::Ctmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Count Input Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cinsel(&self) -> super::vals::Cinsel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Cinsel::from_bits(val as u8)
    }
    #[doc = "Count Input Select"]
    #[inline(always)]
    pub const fn set_cinsel(&mut self, val: super::vals::Cinsel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Capture Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn encc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Channel Enable"]
    #[inline(always)]
    pub const fn set_encc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn selcc(&self) -> super::vals::Selcc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Selcc::from_bits(val as u8)
    }
    #[doc = "Edge Select"]
    #[inline(always)]
    pub const fn set_selcc(&mut self, val: super::vals::Selcc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for Ctcr {
    #[inline(always)]
    fn default() -> Ctcr {
        Ctcr(0)
    }
}
impl core::fmt::Debug for Ctcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctcr")
            .field("ctmode", &self.ctmode())
            .field("cinsel", &self.cinsel())
            .field("encc", &self.encc())
            .field("selcc", &self.selcc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctcr {{ ctmode: {:?}, cinsel: {:?}, encc: {=bool:?}, selcc: {:?} }}",
            self.ctmode(),
            self.cinsel(),
            self.encc(),
            self.selcc()
        )
    }
}
#[doc = "External Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emr(pub u32);
impl Emr {
    #[doc = "External Match 0"]
    #[must_use]
    #[inline(always)]
    pub const fn em0(&self) -> super::vals::Em0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Em0::from_bits(val as u8)
    }
    #[doc = "External Match 0"]
    #[inline(always)]
    pub const fn set_em0(&mut self, val: super::vals::Em0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "External Match 1"]
    #[must_use]
    #[inline(always)]
    pub const fn em1(&self) -> super::vals::Em1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Em1::from_bits(val as u8)
    }
    #[doc = "External Match 1"]
    #[inline(always)]
    pub const fn set_em1(&mut self, val: super::vals::Em1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "External Match 2"]
    #[must_use]
    #[inline(always)]
    pub const fn em2(&self) -> super::vals::Em2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Em2::from_bits(val as u8)
    }
    #[doc = "External Match 2"]
    #[inline(always)]
    pub const fn set_em2(&mut self, val: super::vals::Em2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "External Match 3"]
    #[must_use]
    #[inline(always)]
    pub const fn em3(&self) -> super::vals::Em3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Em3::from_bits(val as u8)
    }
    #[doc = "External Match 3"]
    #[inline(always)]
    pub const fn set_em3(&mut self, val: super::vals::Em3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "External Match Control 0"]
    #[must_use]
    #[inline(always)]
    pub const fn emc0(&self) -> super::vals::Emc0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Emc0::from_bits(val as u8)
    }
    #[doc = "External Match Control 0"]
    #[inline(always)]
    pub const fn set_emc0(&mut self, val: super::vals::Emc0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "External Match Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn emc1(&self) -> super::vals::Emc1 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Emc1::from_bits(val as u8)
    }
    #[doc = "External Match Control 1"]
    #[inline(always)]
    pub const fn set_emc1(&mut self, val: super::vals::Emc1) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "External Match Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn emc2(&self) -> super::vals::Emc2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Emc2::from_bits(val as u8)
    }
    #[doc = "External Match Control 2"]
    #[inline(always)]
    pub const fn set_emc2(&mut self, val: super::vals::Emc2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "External Match Control 3"]
    #[must_use]
    #[inline(always)]
    pub const fn emc3(&self) -> super::vals::Emc3 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Emc3::from_bits(val as u8)
    }
    #[doc = "External Match Control 3"]
    #[inline(always)]
    pub const fn set_emc3(&mut self, val: super::vals::Emc3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for Emr {
    #[inline(always)]
    fn default() -> Emr {
        Emr(0)
    }
}
impl core::fmt::Debug for Emr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emr")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("emc0", &self.emc0())
            .field("emc1", &self.emc1())
            .field("emc2", &self.emc2())
            .field("emc3", &self.emc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emr {{ em0: {:?}, em1: {:?}, em2: {:?}, em3: {:?}, emc0: {:?}, emc1: {:?}, emc2: {:?}, emc3: {:?} }}",
            self.em0(),
            self.em1(),
            self.em2(),
            self.em3(),
            self.emc0(),
            self.emc1(),
            self.emc2(),
            self.emc3()
        )
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "Interrupt Flag for Match Channel 0 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn mr0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 0 Event"]
    #[inline(always)]
    pub const fn set_mr0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Flag for Match Channel 1 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn mr1int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 1 Event"]
    #[inline(always)]
    pub const fn set_mr1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Flag for Match Channel 2 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn mr2int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 2 Event"]
    #[inline(always)]
    pub const fn set_mr2int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Flag for Match Channel 3 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn mr3int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 3 Event"]
    #[inline(always)]
    pub const fn set_mr3int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 0 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cr0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 0 Event"]
    #[inline(always)]
    pub const fn set_cr0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 1 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cr1int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 1 Event"]
    #[inline(always)]
    pub const fn set_cr1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 2 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cr2int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 2 Event"]
    #[inline(always)]
    pub const fn set_cr2int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 3 Event"]
    #[must_use]
    #[inline(always)]
    pub const fn cr3int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 3 Event"]
    #[inline(always)]
    pub const fn set_cr3int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
impl core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ir")
            .field("mr0int", &self.mr0int())
            .field("mr1int", &self.mr1int())
            .field("mr2int", &self.mr2int())
            .field("mr3int", &self.mr3int())
            .field("cr0int", &self.cr0int())
            .field("cr1int", &self.cr1int())
            .field("cr2int", &self.cr2int())
            .field("cr3int", &self.cr3int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ir {{ mr0int: {=bool:?}, mr1int: {=bool:?}, mr2int: {=bool:?}, mr3int: {=bool:?}, cr0int: {=bool:?}, cr1int: {=bool:?}, cr2int: {=bool:?}, cr3int: {=bool:?} }}",
            self.mr0int(),
            self.mr1int(),
            self.mr2int(),
            self.mr3int(),
            self.cr0int(),
            self.cr1int(),
            self.cr2int(),
            self.cr3int()
        )
    }
}
#[doc = "Match Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Interrupt on MR0"]
    #[must_use]
    #[inline(always)]
    pub const fn mr0i(&self) -> super::vals::Mr0i {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mr0i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR0"]
    #[inline(always)]
    pub const fn set_mr0i(&mut self, val: super::vals::Mr0i) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset on MR0"]
    #[must_use]
    #[inline(always)]
    pub const fn mr0r(&self) -> super::vals::Mr0r {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mr0r::from_bits(val as u8)
    }
    #[doc = "Reset on MR0"]
    #[inline(always)]
    pub const fn set_mr0r(&mut self, val: super::vals::Mr0r) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop on MR0"]
    #[must_use]
    #[inline(always)]
    pub const fn mr0s(&self) -> super::vals::Mr0s {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mr0s::from_bits(val as u8)
    }
    #[doc = "Stop on MR0"]
    #[inline(always)]
    pub const fn set_mr0s(&mut self, val: super::vals::Mr0s) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt on MR1"]
    #[must_use]
    #[inline(always)]
    pub const fn mr1i(&self) -> super::vals::Mr1i {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mr1i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR1"]
    #[inline(always)]
    pub const fn set_mr1i(&mut self, val: super::vals::Mr1i) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset on MR1"]
    #[must_use]
    #[inline(always)]
    pub const fn mr1r(&self) -> super::vals::Mr1r {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mr1r::from_bits(val as u8)
    }
    #[doc = "Reset on MR1"]
    #[inline(always)]
    pub const fn set_mr1r(&mut self, val: super::vals::Mr1r) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Stop on MR1"]
    #[must_use]
    #[inline(always)]
    pub const fn mr1s(&self) -> super::vals::Mr1s {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mr1s::from_bits(val as u8)
    }
    #[doc = "Stop on MR1"]
    #[inline(always)]
    pub const fn set_mr1s(&mut self, val: super::vals::Mr1s) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on MR2"]
    #[must_use]
    #[inline(always)]
    pub const fn mr2i(&self) -> super::vals::Mr2i {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mr2i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR2"]
    #[inline(always)]
    pub const fn set_mr2i(&mut self, val: super::vals::Mr2i) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset on MR2"]
    #[must_use]
    #[inline(always)]
    pub const fn mr2r(&self) -> super::vals::Mr2r {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mr2r::from_bits(val as u8)
    }
    #[doc = "Reset on MR2"]
    #[inline(always)]
    pub const fn set_mr2r(&mut self, val: super::vals::Mr2r) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Stop on MR2"]
    #[must_use]
    #[inline(always)]
    pub const fn mr2s(&self) -> super::vals::Mr2s {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mr2s::from_bits(val as u8)
    }
    #[doc = "Stop on MR2"]
    #[inline(always)]
    pub const fn set_mr2s(&mut self, val: super::vals::Mr2s) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt on MR3"]
    #[must_use]
    #[inline(always)]
    pub const fn mr3i(&self) -> super::vals::Mr3i {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Mr3i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR3"]
    #[inline(always)]
    pub const fn set_mr3i(&mut self, val: super::vals::Mr3i) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset on MR3"]
    #[must_use]
    #[inline(always)]
    pub const fn mr3r(&self) -> super::vals::Mr3r {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Mr3r::from_bits(val as u8)
    }
    #[doc = "Reset on MR3"]
    #[inline(always)]
    pub const fn set_mr3r(&mut self, val: super::vals::Mr3r) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Stop on MR3"]
    #[must_use]
    #[inline(always)]
    pub const fn mr3s(&self) -> super::vals::Mr3s {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mr3s::from_bits(val as u8)
    }
    #[doc = "Stop on MR3"]
    #[inline(always)]
    pub const fn set_mr3s(&mut self, val: super::vals::Mr3s) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Reload MR"]
    #[must_use]
    #[inline(always)]
    pub const fn mr0rl(&self) -> super::vals::Mr0rl {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mr0rl::from_bits(val as u8)
    }
    #[doc = "Reload MR"]
    #[inline(always)]
    pub const fn set_mr0rl(&mut self, val: super::vals::Mr0rl) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Reload MR"]
    #[must_use]
    #[inline(always)]
    pub const fn mr1rl(&self) -> super::vals::Mr1rl {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mr1rl::from_bits(val as u8)
    }
    #[doc = "Reload MR"]
    #[inline(always)]
    pub const fn set_mr1rl(&mut self, val: super::vals::Mr1rl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Reload MR"]
    #[must_use]
    #[inline(always)]
    pub const fn mr2rl(&self) -> super::vals::Mr2rl {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Mr2rl::from_bits(val as u8)
    }
    #[doc = "Reload MR"]
    #[inline(always)]
    pub const fn set_mr2rl(&mut self, val: super::vals::Mr2rl) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Reload MR"]
    #[must_use]
    #[inline(always)]
    pub const fn mr3rl(&self) -> super::vals::Mr3rl {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mr3rl::from_bits(val as u8)
    }
    #[doc = "Reload MR"]
    #[inline(always)]
    pub const fn set_mr3rl(&mut self, val: super::vals::Mr3rl) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("mr0i", &self.mr0i())
            .field("mr0r", &self.mr0r())
            .field("mr0s", &self.mr0s())
            .field("mr1i", &self.mr1i())
            .field("mr1r", &self.mr1r())
            .field("mr1s", &self.mr1s())
            .field("mr2i", &self.mr2i())
            .field("mr2r", &self.mr2r())
            .field("mr2s", &self.mr2s())
            .field("mr3i", &self.mr3i())
            .field("mr3r", &self.mr3r())
            .field("mr3s", &self.mr3s())
            .field("mr0rl", &self.mr0rl())
            .field("mr1rl", &self.mr1rl())
            .field("mr2rl", &self.mr2rl())
            .field("mr3rl", &self.mr3rl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ mr0i: {:?}, mr0r: {:?}, mr0s: {:?}, mr1i: {:?}, mr1r: {:?}, mr1s: {:?}, mr2i: {:?}, mr2r: {:?}, mr2s: {:?}, mr3i: {:?}, mr3r: {:?}, mr3s: {:?}, mr0rl: {:?}, mr1rl: {:?}, mr2rl: {:?}, mr3rl: {:?} }}",
            self.mr0i(),
            self.mr0r(),
            self.mr0s(),
            self.mr1i(),
            self.mr1r(),
            self.mr1s(),
            self.mr2i(),
            self.mr2r(),
            self.mr2s(),
            self.mr3i(),
            self.mr3r(),
            self.mr3s(),
            self.mr0rl(),
            self.mr1rl(),
            self.mr2rl(),
            self.mr3rl()
        )
    }
}
#[doc = "Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mr(pub u32);
impl Mr {
    #[doc = "Timer Counter Match Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Match Value"]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mr {
    #[inline(always)]
    fn default() -> Mr {
        Mr(0)
    }
}
impl core::fmt::Debug for Mr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mr")
            .field("match_", &self.match_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mr {{ match_: {=u32:?} }}", self.match_())
    }
}
#[doc = "Match Shadow"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Timer Counter Match Shadow Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match_shadow(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Match Shadow Value"]
    #[inline(always)]
    pub const fn set_match_shadow(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0)
    }
}
impl core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msr")
            .field("match_shadow", &self.match_shadow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Msr {{ match_shadow: {=u32:?} }}", self.match_shadow())
    }
}
#[doc = "Prescale Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc = "Prescale Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pcval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale Counter Value"]
    #[inline(always)]
    pub const fn set_pcval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        Pc(0)
    }
}
impl core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pc").field("pcval", &self.pcval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pc {{ pcval: {=u32:?} }}", self.pcval())
    }
}
#[doc = "Prescale"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescale Reload Value"]
    #[must_use]
    #[inline(always)]
    pub const fn prval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale Reload Value"]
    #[inline(always)]
    pub const fn set_prval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr").field("prval", &self.prval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pr {{ prval: {=u32:?} }}", self.prval())
    }
}
#[doc = "PWM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmc(pub u32);
impl Pwmc {
    #[doc = "PWM Mode Enable for Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen0(&self) -> super::vals::Pwmen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pwmen0::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 0"]
    #[inline(always)]
    pub const fn set_pwmen0(&mut self, val: super::vals::Pwmen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PWM Mode Enable for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen1(&self) -> super::vals::Pwmen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pwmen1::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 1"]
    #[inline(always)]
    pub const fn set_pwmen1(&mut self, val: super::vals::Pwmen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PWM Mode Enable for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen2(&self) -> super::vals::Pwmen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pwmen2::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 2"]
    #[inline(always)]
    pub const fn set_pwmen2(&mut self, val: super::vals::Pwmen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "PWM Mode Enable for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen3(&self) -> super::vals::Pwmen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pwmen3::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 3"]
    #[inline(always)]
    pub const fn set_pwmen3(&mut self, val: super::vals::Pwmen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwmc {
    #[inline(always)]
    fn default() -> Pwmc {
        Pwmc(0)
    }
}
impl core::fmt::Debug for Pwmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwmc")
            .field("pwmen0", &self.pwmen0())
            .field("pwmen1", &self.pwmen1())
            .field("pwmen2", &self.pwmen2())
            .field("pwmen3", &self.pwmen3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwmc {{ pwmen0: {:?}, pwmen1: {:?}, pwmen2: {:?}, pwmen3: {:?} }}",
            self.pwmen0(),
            self.pwmen1(),
            self.pwmen2(),
            self.pwmen3()
        )
    }
}
#[doc = "Timer Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tc(pub u32);
impl Tc {
    #[doc = "Timer Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tcval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Value"]
    #[inline(always)]
    pub const fn set_tcval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tc {
    #[inline(always)]
    fn default() -> Tc {
        Tc(0)
    }
}
impl core::fmt::Debug for Tc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tc").field("tcval", &self.tcval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tc {{ tcval: {=u32:?} }}", self.tcval())
    }
}
#[doc = "Timer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Enable"]
    #[inline(always)]
    pub const fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Reset Enable"]
    #[inline(always)]
    pub const fn set_crst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow Global Count Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn agcen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Global Count Enable"]
    #[inline(always)]
    pub const fn set_agcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Allow Trigger Count Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn atcen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Trigger Count Enable"]
    #[inline(always)]
    pub const fn set_atcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("cen", &self.cen())
            .field("crst", &self.crst())
            .field("agcen", &self.agcen())
            .field("atcen", &self.atcen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ cen: {=bool:?}, crst: {=bool:?}, agcen: {=bool:?}, atcen: {=bool:?} }}",
            self.cen(),
            self.crst(),
            self.agcen(),
            self.atcen()
        )
    }
}
