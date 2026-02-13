#[doc = "PWM Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsrcsel(pub u16);
impl Dtsrcsel {
    #[doc = "Submodule 0 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel45(&self) -> super::vals::Sm0sel45 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm0sel45(&mut self, val: super::vals::Sm0sel45) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel23(&self) -> super::vals::Sm0sel23 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm0sel23(&mut self, val: super::vals::Sm0sel23) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel45(&self) -> super::vals::Sm1sel45 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm1sel45(&mut self, val: super::vals::Sm1sel45) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel23(&self) -> super::vals::Sm1sel23 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm1sel23(&mut self, val: super::vals::Sm1sel23) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel45(&self) -> super::vals::Sm2sel45 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm2sel45(&mut self, val: super::vals::Sm2sel45) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel23(&self) -> super::vals::Sm2sel23 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sm2sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm2sel23(&mut self, val: super::vals::Sm2sel23) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel45(&self) -> super::vals::Sm3sel45 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Sm3sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm3sel45(&mut self, val: super::vals::Sm3sel45) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel23(&self) -> super::vals::Sm3sel23 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sm3sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm3sel23(&mut self, val: super::vals::Sm3sel23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Dtsrcsel {
    #[inline(always)]
    fn default() -> Dtsrcsel {
        Dtsrcsel(0)
    }
}
impl core::fmt::Debug for Dtsrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtsrcsel")
            .field("sm0sel45", &self.sm0sel45())
            .field("sm0sel23", &self.sm0sel23())
            .field("sm1sel45", &self.sm1sel45())
            .field("sm1sel23", &self.sm1sel23())
            .field("sm2sel45", &self.sm2sel45())
            .field("sm2sel23", &self.sm2sel23())
            .field("sm3sel45", &self.sm3sel45())
            .field("sm3sel23", &self.sm3sel23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtsrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtsrcsel {{ sm0sel45: {:?}, sm0sel23: {:?}, sm1sel45: {:?}, sm1sel23: {:?}, sm2sel45: {:?}, sm2sel23: {:?}, sm3sel45: {:?}, sm3sel23: {:?} }}",
            self.sm0sel45(),
            self.sm0sel23(),
            self.sm1sel45(),
            self.sm1sel23(),
            self.sm2sel45(),
            self.sm2sel23(),
            self.sm3sel45(),
            self.sm3sel23()
        )
    }
}
#[doc = "Fault Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl0(pub u16);
impl Fctrl0 {
    #[doc = "Fault Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn fie(&self) -> super::vals::Fie {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fie::from_bits(val as u8)
    }
    #[doc = "Fault Interrupt Enables"]
    #[inline(always)]
    pub const fn set_fie(&mut self, val: super::vals::Fie) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Fault Safety Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fsafe(&self) -> super::vals::Fsafe {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Fsafe::from_bits(val as u8)
    }
    #[doc = "Fault Safety Mode"]
    #[inline(always)]
    pub const fn set_fsafe(&mut self, val: super::vals::Fsafe) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Automatic Fault Clearing"]
    #[must_use]
    #[inline(always)]
    pub const fn fauto(&self) -> super::vals::Fauto {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Fauto::from_bits(val as u8)
    }
    #[doc = "Automatic Fault Clearing"]
    #[inline(always)]
    pub const fn set_fauto(&mut self, val: super::vals::Fauto) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Fault Level"]
    #[must_use]
    #[inline(always)]
    pub const fn flvl(&self) -> super::vals::Flvl {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Flvl::from_bits(val as u8)
    }
    #[doc = "Fault Level"]
    #[inline(always)]
    pub const fn set_flvl(&mut self, val: super::vals::Flvl) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fctrl0 {
    #[inline(always)]
    fn default() -> Fctrl0 {
        Fctrl0(0)
    }
}
impl core::fmt::Debug for Fctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl0")
            .field("fie", &self.fie())
            .field("fsafe", &self.fsafe())
            .field("fauto", &self.fauto())
            .field("flvl", &self.flvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl0 {{ fie: {:?}, fsafe: {:?}, fauto: {:?}, flvl: {:?} }}",
            self.fie(),
            self.fsafe(),
            self.fauto(),
            self.flvl()
        )
    }
}
#[doc = "Fault Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl20(pub u16);
impl Fctrl20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    #[must_use]
    #[inline(always)]
    pub const fn nocomb(&self) -> super::vals::Nocomb {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Nocomb::from_bits(val as u8)
    }
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub const fn set_nocomb(&mut self, val: super::vals::Nocomb) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Fctrl20 {
    #[inline(always)]
    fn default() -> Fctrl20 {
        Fctrl20(0)
    }
}
impl core::fmt::Debug for Fctrl20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl20")
            .field("nocomb", &self.nocomb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fctrl20 {{ nocomb: {:?} }}", self.nocomb())
    }
}
#[doc = "Fault Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffilt0(pub u16);
impl Ffilt0 {
    #[doc = "Fault Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fault Filter Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Fault Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Fault Filter Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Fault Glitch Stretch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gstr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub const fn set_gstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Ffilt0 {
    #[inline(always)]
    fn default() -> Ffilt0 {
        Ffilt0(0)
    }
}
impl core::fmt::Debug for Ffilt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ffilt0")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .field("gstr", &self.gstr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ffilt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ffilt0 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?}, gstr: {=bool:?} }}",
            self.filt_per(),
            self.filt_cnt(),
            self.gstr()
        )
    }
}
#[doc = "Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsts0(pub u16);
impl Fsts0 {
    #[doc = "Fault Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn fflag(&self) -> super::vals::Fflag {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fflag::from_bits(val as u8)
    }
    #[doc = "Fault Flags"]
    #[inline(always)]
    pub const fn set_fflag(&mut self, val: super::vals::Fflag) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Full Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn ffull(&self) -> super::vals::Ffull {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ffull::from_bits(val as u8)
    }
    #[doc = "Full Cycle"]
    #[inline(always)]
    pub const fn set_ffull(&mut self, val: super::vals::Ffull) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Filtered Fault Pins"]
    #[must_use]
    #[inline(always)]
    pub const fn ffpin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filtered Fault Pins"]
    #[inline(always)]
    pub const fn set_ffpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Half Cycle Fault Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn fhalf(&self) -> super::vals::Fhalf {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Fhalf::from_bits(val as u8)
    }
    #[doc = "Half Cycle Fault Recovery"]
    #[inline(always)]
    pub const fn set_fhalf(&mut self, val: super::vals::Fhalf) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fsts0 {
    #[inline(always)]
    fn default() -> Fsts0 {
        Fsts0(0)
    }
}
impl core::fmt::Debug for Fsts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsts0")
            .field("fflag", &self.fflag())
            .field("ffull", &self.ffull())
            .field("ffpin", &self.ffpin())
            .field("fhalf", &self.fhalf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fsts0 {{ fflag: {:?}, ffull: {:?}, ffpin: {=u8:?}, fhalf: {:?} }}",
            self.fflag(),
            self.ffull(),
            self.ffpin(),
            self.fhalf()
        )
    }
}
#[doc = "Fault Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftst0(pub u16);
impl Ftst0 {
    #[doc = "Fault Test"]
    #[must_use]
    #[inline(always)]
    pub const fn ftest(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Test"]
    #[inline(always)]
    pub const fn set_ftest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
}
impl Default for Ftst0 {
    #[inline(always)]
    fn default() -> Ftst0 {
        Ftst0(0)
    }
}
impl core::fmt::Debug for Ftst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftst0")
            .field("ftest", &self.ftest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ftst0 {{ ftest: {=bool:?} }}", self.ftest())
    }
}
#[doc = "Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u16);
impl Mask {
    #[doc = "PWM_X Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn maskx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Masks"]
    #[inline(always)]
    pub const fn set_maskx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn maskb(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Masks"]
    #[inline(always)]
    pub const fn set_maskb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn maska(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Masks"]
    #[inline(always)]
    pub const fn set_maska(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Update Mask Bits Immediately"]
    #[must_use]
    #[inline(always)]
    pub const fn update_mask(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Update Mask Bits Immediately"]
    #[inline(always)]
    pub const fn set_update_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
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
            .field("maskx", &self.maskx())
            .field("maskb", &self.maskb())
            .field("maska", &self.maska())
            .field("update_mask", &self.update_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mask {{ maskx: {=u8:?}, maskb: {=u8:?}, maska: {=u8:?}, update_mask: {=u8:?} }}",
            self.maskx(),
            self.maskb(),
            self.maska(),
            self.update_mask()
        )
    }
}
#[doc = "Master Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u16);
impl Mctrl {
    #[doc = "Load Okay"]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> super::vals::Ldok {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay"]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: super::vals::Ldok) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Clear Load Okay"]
    #[must_use]
    #[inline(always)]
    pub const fn cldok(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Load Okay"]
    #[inline(always)]
    pub const fn set_cldok(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "Run"]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> super::vals::Run {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Run::from_bits(val as u8)
    }
    #[doc = "Run"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: super::vals::Run) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Current Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn ipol(&self) -> super::vals::Ipol {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Ipol::from_bits(val as u8)
    }
    #[doc = "Current Polarity"]
    #[inline(always)]
    pub const fn set_ipol(&mut self, val: super::vals::Ipol) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("ldok", &self.ldok())
            .field("cldok", &self.cldok())
            .field("run", &self.run())
            .field("ipol", &self.ipol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ ldok: {:?}, cldok: {=u8:?}, run: {:?}, ipol: {:?} }}",
            self.ldok(),
            self.cldok(),
            self.run(),
            self.ipol()
        )
    }
}
#[doc = "Master Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl2(pub u16);
impl Mctrl2 {
    #[doc = "Write protect"]
    #[must_use]
    #[inline(always)]
    pub const fn wrprot(&self) -> super::vals::Wrprot {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Wrprot::from_bits(val as u8)
    }
    #[doc = "Write protect"]
    #[inline(always)]
    pub const fn set_wrprot(&mut self, val: super::vals::Wrprot) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig"]
    #[must_use]
    #[inline(always)]
    pub const fn stretch_cnt_prsc(&self) -> super::vals::StretchCntPrsc {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::StretchCntPrsc::from_bits(val as u8)
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig"]
    #[inline(always)]
    pub const fn set_stretch_cnt_prsc(&mut self, val: super::vals::StretchCntPrsc) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
}
impl Default for Mctrl2 {
    #[inline(always)]
    fn default() -> Mctrl2 {
        Mctrl2(0)
    }
}
impl core::fmt::Debug for Mctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl2")
            .field("wrprot", &self.wrprot())
            .field("stretch_cnt_prsc", &self.stretch_cnt_prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl2 {{ wrprot: {:?}, stretch_cnt_prsc: {:?} }}",
            self.wrprot(),
            self.stretch_cnt_prsc()
        )
    }
}
#[doc = "Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outen(pub u16);
impl Outen {
    #[doc = "PWM_X Output Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Output Enables"]
    #[inline(always)]
    pub const fn set_pwmx_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Output Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Output Enables"]
    #[inline(always)]
    pub const fn set_pwmb_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Output Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Output Enables"]
    #[inline(always)]
    pub const fn set_pwma_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Outen {
    #[inline(always)]
    fn default() -> Outen {
        Outen(0)
    }
}
impl core::fmt::Debug for Outen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outen")
            .field("pwmx_en", &self.pwmx_en())
            .field("pwmb_en", &self.pwmb_en())
            .field("pwma_en", &self.pwma_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outen {{ pwmx_en: {=u8:?}, pwmb_en: {=u8:?}, pwma_en: {=u8:?} }}",
            self.pwmx_en(),
            self.pwmb_en(),
            self.pwma_en()
        )
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompa(pub u16);
impl Sm0captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompa {
    #[inline(always)]
    fn default() -> Sm0captcompa {
        Sm0captcompa(0)
    }
}
impl core::fmt::Debug for Sm0captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captcompa {{ edgcmpa: {=u8:?}, edgcnta: {=u8:?} }}",
            self.edgcmpa(),
            self.edgcnta()
        )
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompb(pub u16);
impl Sm0captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompb {
    #[inline(always)]
    fn default() -> Sm0captcompb {
        Sm0captcompb(0)
    }
}
impl core::fmt::Debug for Sm0captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captcompb {{ edgcmpb: {=u8:?}, edgcntb: {=u8:?} }}",
            self.edgcmpb(),
            self.edgcntb()
        )
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompx(pub u16);
impl Sm0captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompx {
    #[inline(always)]
    fn default() -> Sm0captcompx {
        Sm0captcompx(0)
    }
}
impl core::fmt::Debug for Sm0captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrla(pub u16);
impl Sm0captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm0captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm0captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm0captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm0captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm0captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm0captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm0captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm0captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrla {
    #[inline(always)]
    fn default() -> Sm0captctrla {
        Sm0captctrla(0)
    }
}
impl core::fmt::Debug for Sm0captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captctrla {{ arma: {=bool:?}, oneshota: {:?}, edga0: {:?}, edga1: {:?}, inp_sela: {:?}, edgcnta_en: {=bool:?}, cfawm: {=u8:?}, ca0cnt: {=u8:?}, ca1cnt: {=u8:?} }}",
            self.arma(),
            self.oneshota(),
            self.edga0(),
            self.edga1(),
            self.inp_sela(),
            self.edgcnta_en(),
            self.cfawm(),
            self.ca0cnt(),
            self.ca1cnt()
        )
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrlb(pub u16);
impl Sm0captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm0captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm0captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm0captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm0captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm0captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm0captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm0captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm0captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrlb {
    #[inline(always)]
    fn default() -> Sm0captctrlb {
        Sm0captctrlb(0)
    }
}
impl core::fmt::Debug for Sm0captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captctrlb {{ armb: {=bool:?}, oneshotb: {:?}, edgb0: {:?}, edgb1: {:?}, inp_selb: {:?}, edgcntb_en: {=bool:?}, cfbwm: {=u8:?}, cb0cnt: {=u8:?}, cb1cnt: {=u8:?} }}",
            self.armb(),
            self.oneshotb(),
            self.edgb0(),
            self.edgb1(),
            self.inp_selb(),
            self.edgcntb_en(),
            self.cfbwm(),
            self.cb0cnt(),
            self.cb1cnt()
        )
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrlx(pub u16);
impl Sm0captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm0captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm0captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm0captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm0captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm0captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm0captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm0captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm0captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrlx {
    #[inline(always)]
    fn default() -> Sm0captctrlx {
        Sm0captctrlx(0)
    }
}
impl core::fmt::Debug for Sm0captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfilta(pub u16);
impl Sm0captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfilta {
    #[inline(always)]
    fn default() -> Sm0captfilta {
        Sm0captfilta(0)
    }
}
impl core::fmt::Debug for Sm0captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfilta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captfilta {{ capta_filt_per: {=u8:?}, capta_filt_cnt: {=u8:?} }}",
            self.capta_filt_per(),
            self.capta_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfiltb(pub u16);
impl Sm0captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfiltb {
    #[inline(always)]
    fn default() -> Sm0captfiltb {
        Sm0captfiltb(0)
    }
}
impl core::fmt::Debug for Sm0captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfiltb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captfiltb {{ captb_filt_per: {=u8:?}, captb_filt_cnt: {=u8:?} }}",
            self.captb_filt_per(),
            self.captb_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfiltx(pub u16);
impl Sm0captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfiltx {
    #[inline(always)]
    fn default() -> Sm0captfiltx {
        Sm0captfiltx(0)
    }
}
impl core::fmt::Debug for Sm0captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cnt(pub u16);
impl Sm0cnt {
    #[doc = "Counter Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cnt {
    #[inline(always)]
    fn default() -> Sm0cnt {
        Sm0cnt(0)
    }
}
impl core::fmt::Debug for Sm0cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl(pub u16);
impl Sm0ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm0ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm0ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm0ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm0ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm0ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm0ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm0ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm0ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm0ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm0ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm0ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm0ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm0ctrl {
    #[inline(always)]
    fn default() -> Sm0ctrl {
        Sm0ctrl(0)
    }
}
impl core::fmt::Debug for Sm0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl2(pub u16);
impl Sm0ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm0ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm0ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm0ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm0ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm0ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm0ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm0ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm0ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm0ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm0ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm0ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm0ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm0ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm0ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0ctrl2 {
    #[inline(always)]
    fn default() -> Sm0ctrl2 {
        Sm0ctrl2(0)
    }
}
impl core::fmt::Debug for Sm0ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0(pub u16);
impl Sm0cval0 {
    #[doc = "Capture Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0"]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval0 {
    #[inline(always)]
    fn default() -> Sm0cval0 {
        Sm0cval0(0)
    }
}
impl core::fmt::Debug for Sm0cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0cyc(pub u16);
impl Sm0cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval0cyc {
    #[inline(always)]
    fn default() -> Sm0cval0cyc {
        Sm0cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1(pub u16);
impl Sm0cval1 {
    #[doc = "Capture Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1"]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval1 {
    #[inline(always)]
    fn default() -> Sm0cval1 {
        Sm0cval1(0)
    }
}
impl core::fmt::Debug for Sm0cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1cyc(pub u16);
impl Sm0cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval1cyc {
    #[inline(always)]
    fn default() -> Sm0cval1cyc {
        Sm0cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Capture Value 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval2(pub u16);
impl Sm0cval2 {
    #[doc = "Capture Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn captval2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 2"]
    #[inline(always)]
    pub const fn set_captval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval2 {
    #[inline(always)]
    fn default() -> Sm0cval2 {
        Sm0cval2(0)
    }
}
impl core::fmt::Debug for Sm0cval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval2")
            .field("captval2", &self.captval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval2 {{ captval2: {=u16:?} }}", self.captval2())
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval2cyc(pub u16);
impl Sm0cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval2cyc {
    #[inline(always)]
    fn default() -> Sm0cval2cyc {
        Sm0cval2cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval2cyc {{ cval2cyc: {=u8:?} }}", self.cval2cyc())
    }
}
#[doc = "Capture Value 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval3(pub u16);
impl Sm0cval3 {
    #[doc = "Capture Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn captval3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 3"]
    #[inline(always)]
    pub const fn set_captval3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval3 {
    #[inline(always)]
    fn default() -> Sm0cval3 {
        Sm0cval3(0)
    }
}
impl core::fmt::Debug for Sm0cval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval3")
            .field("captval3", &self.captval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval3 {{ captval3: {=u16:?} }}", self.captval3())
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval3cyc(pub u16);
impl Sm0cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval3cyc {
    #[inline(always)]
    fn default() -> Sm0cval3cyc {
        Sm0cval3cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval3cyc {{ cval3cyc: {=u8:?} }}", self.cval3cyc())
    }
}
#[doc = "Capture Value 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval4(pub u16);
impl Sm0cval4 {
    #[doc = "Capture Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captval4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 4"]
    #[inline(always)]
    pub const fn set_captval4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval4 {
    #[inline(always)]
    fn default() -> Sm0cval4 {
        Sm0cval4(0)
    }
}
impl core::fmt::Debug for Sm0cval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval4")
            .field("captval4", &self.captval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval4 {{ captval4: {=u16:?} }}", self.captval4())
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval4cyc(pub u16);
impl Sm0cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval4cyc {
    #[inline(always)]
    fn default() -> Sm0cval4cyc {
        Sm0cval4cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval4cyc {{ cval4cyc: {=u8:?} }}", self.cval4cyc())
    }
}
#[doc = "Capture Value 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval5(pub u16);
impl Sm0cval5 {
    #[doc = "Capture Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn captval5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 5"]
    #[inline(always)]
    pub const fn set_captval5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval5 {
    #[inline(always)]
    fn default() -> Sm0cval5 {
        Sm0cval5(0)
    }
}
impl core::fmt::Debug for Sm0cval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval5")
            .field("captval5", &self.captval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval5 {{ captval5: {=u16:?} }}", self.captval5())
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval5cyc(pub u16);
impl Sm0cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval5cyc {
    #[inline(always)]
    fn default() -> Sm0cval5cyc {
        Sm0cval5cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval5cyc {{ cval5cyc: {=u8:?} }}", self.cval5cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dismap0(pub u16);
impl Sm0dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm0dismap0 {
    #[inline(always)]
    fn default() -> Sm0dismap0 {
        Sm0dismap0(0)
    }
}
impl core::fmt::Debug for Sm0dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dmaen(pub u16);
impl Sm0dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm0dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm0dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm0dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm0dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm0dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm0dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm0dmaen {
    #[inline(always)]
    fn default() -> Sm0dmaen {
        Sm0dmaen(0)
    }
}
impl core::fmt::Debug for Sm0dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, cb0de: {=bool:?}, cb1de: {=bool:?}, ca0de: {=bool:?}, ca1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.cb0de(),
            self.cb1de(),
            self.ca0de(),
            self.ca1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dtcnt0(pub u16);
impl Sm0dtcnt0 {
    #[doc = "Deadtime Count Register 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm0dtcnt0 {
    #[inline(always)]
    fn default() -> Sm0dtcnt0 {
        Sm0dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm0dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dtcnt1(pub u16);
impl Sm0dtcnt1 {
    #[doc = "Deadtime Count Register 1"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm0dtcnt1 {
    #[inline(always)]
    fn default() -> Sm0dtcnt1 {
        Sm0dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm0dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval1(pub u16);
impl Sm0fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval1 {
    #[inline(always)]
    fn default() -> Sm0fracval1 {
        Sm0fracval1(0)
    }
}
impl core::fmt::Debug for Sm0fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0fracval1 {{ fracval1: {=u8:?} }}", self.fracval1())
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval2(pub u16);
impl Sm0fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval2 {
    #[inline(always)]
    fn default() -> Sm0fracval2 {
        Sm0fracval2(0)
    }
}
impl core::fmt::Debug for Sm0fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0fracval2 {{ fracval2: {=u8:?} }}", self.fracval2())
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval3(pub u16);
impl Sm0fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval3 {
    #[inline(always)]
    fn default() -> Sm0fracval3 {
        Sm0fracval3(0)
    }
}
impl core::fmt::Debug for Sm0fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0fracval3 {{ fracval3: {=u8:?} }}", self.fracval3())
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval4(pub u16);
impl Sm0fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval4 {
    #[inline(always)]
    fn default() -> Sm0fracval4 {
        Sm0fracval4(0)
    }
}
impl core::fmt::Debug for Sm0fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0fracval4 {{ fracval4: {=u8:?} }}", self.fracval4())
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval5(pub u16);
impl Sm0fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval5 {
    #[inline(always)]
    fn default() -> Sm0fracval5 {
        Sm0fracval5(0)
    }
}
impl core::fmt::Debug for Sm0fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0fracval5 {{ fracval5: {=u8:?} }}", self.fracval5())
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0frctrl(pub u16);
impl Sm0frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0frctrl {
    #[inline(always)]
    fn default() -> Sm0frctrl {
        Sm0frctrl(0)
    }
}
impl core::fmt::Debug for Sm0frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0frctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0frctrl {{ frac1_en: {=bool:?}, frac23_en: {=bool:?}, frac45_en: {=bool:?}, test: {=bool:?} }}",
            self.frac1_en(),
            self.frac23_en(),
            self.frac45_en(),
            self.test()
        )
    }
}
#[doc = "Initial Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0init(pub u16);
impl Sm0init {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0init {
    #[inline(always)]
    fn default() -> Sm0init {
        Sm0init(0)
    }
}
impl core::fmt::Debug for Sm0init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0inten(pub u16);
impl Sm0inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm0intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm0intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm0inten {
    #[inline(always)]
    fn default() -> Sm0inten {
        Sm0inten(0)
    }
}
impl core::fmt::Debug for Sm0inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, cb0ie: {=bool:?}, cb1ie: {=bool:?}, ca0ie: {=bool:?}, ca1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.cb0ie(),
            self.cb1ie(),
            self.ca0ie(),
            self.ca1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0octrl(pub u16);
impl Sm0octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm0octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm0octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm0octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm0octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm0octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm0octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0octrl {
    #[inline(always)]
    fn default() -> Sm0octrl {
        Sm0octrl(0)
    }
}
impl core::fmt::Debug for Sm0octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0sts(pub u16);
impl Sm0sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm0stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm0stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm0sts {
    #[inline(always)]
    fn default() -> Sm0sts {
        Sm0sts(0)
    }
}
impl core::fmt::Debug for Sm0sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, cfb0: {=bool:?}, cfb1: {=bool:?}, cfa0: {=bool:?}, cfa1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.cfb0(),
            self.cfb1(),
            self.cfa0(),
            self.cfa1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0tctrl(pub u16);
impl Sm0tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm0tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm0tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm0tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm0tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm0tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm0tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm0tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm0tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm0tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm0tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm0tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0tctrl {
    #[inline(always)]
    fn default() -> Sm0tctrl {
        Sm0tctrl(0)
    }
}
impl core::fmt::Debug for Sm0tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val0(pub u16);
impl Sm0val0 {
    #[doc = "Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0"]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val0 {
    #[inline(always)]
    fn default() -> Sm0val0 {
        Sm0val0(0)
    }
}
impl core::fmt::Debug for Sm0val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val1(pub u16);
impl Sm0val1 {
    #[doc = "Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val1 {
    #[inline(always)]
    fn default() -> Sm0val1 {
        Sm0val1(0)
    }
}
impl core::fmt::Debug for Sm0val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val2(pub u16);
impl Sm0val2 {
    #[doc = "Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val2 {
    #[inline(always)]
    fn default() -> Sm0val2 {
        Sm0val2(0)
    }
}
impl core::fmt::Debug for Sm0val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val3(pub u16);
impl Sm0val3 {
    #[doc = "Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3"]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val3 {
    #[inline(always)]
    fn default() -> Sm0val3 {
        Sm0val3(0)
    }
}
impl core::fmt::Debug for Sm0val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val4(pub u16);
impl Sm0val4 {
    #[doc = "Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val4 {
    #[inline(always)]
    fn default() -> Sm0val4 {
        Sm0val4(0)
    }
}
impl core::fmt::Debug for Sm0val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val5(pub u16);
impl Sm0val5 {
    #[doc = "Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5"]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val5 {
    #[inline(always)]
    fn default() -> Sm0val5 {
        Sm0val5(0)
    }
}
impl core::fmt::Debug for Sm0val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompa(pub u16);
impl Sm1captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompa {
    #[inline(always)]
    fn default() -> Sm1captcompa {
        Sm1captcompa(0)
    }
}
impl core::fmt::Debug for Sm1captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captcompa {{ edgcmpa: {=u8:?}, edgcnta: {=u8:?} }}",
            self.edgcmpa(),
            self.edgcnta()
        )
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompb(pub u16);
impl Sm1captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompb {
    #[inline(always)]
    fn default() -> Sm1captcompb {
        Sm1captcompb(0)
    }
}
impl core::fmt::Debug for Sm1captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captcompb {{ edgcmpb: {=u8:?}, edgcntb: {=u8:?} }}",
            self.edgcmpb(),
            self.edgcntb()
        )
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompx(pub u16);
impl Sm1captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompx {
    #[inline(always)]
    fn default() -> Sm1captcompx {
        Sm1captcompx(0)
    }
}
impl core::fmt::Debug for Sm1captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrla(pub u16);
impl Sm1captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm1captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm1captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm1captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm1captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm1captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm1captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm1captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm1captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrla {
    #[inline(always)]
    fn default() -> Sm1captctrla {
        Sm1captctrla(0)
    }
}
impl core::fmt::Debug for Sm1captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captctrla {{ arma: {=bool:?}, oneshota: {:?}, edga0: {:?}, edga1: {:?}, inp_sela: {:?}, edgcnta_en: {=bool:?}, cfawm: {=u8:?}, ca0cnt: {=u8:?}, ca1cnt: {=u8:?} }}",
            self.arma(),
            self.oneshota(),
            self.edga0(),
            self.edga1(),
            self.inp_sela(),
            self.edgcnta_en(),
            self.cfawm(),
            self.ca0cnt(),
            self.ca1cnt()
        )
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrlb(pub u16);
impl Sm1captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm1captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm1captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm1captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm1captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm1captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm1captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm1captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm1captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrlb {
    #[inline(always)]
    fn default() -> Sm1captctrlb {
        Sm1captctrlb(0)
    }
}
impl core::fmt::Debug for Sm1captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captctrlb {{ armb: {=bool:?}, oneshotb: {:?}, edgb0: {:?}, edgb1: {:?}, inp_selb: {:?}, edgcntb_en: {=bool:?}, cfbwm: {=u8:?}, cb0cnt: {=u8:?}, cb1cnt: {=u8:?} }}",
            self.armb(),
            self.oneshotb(),
            self.edgb0(),
            self.edgb1(),
            self.inp_selb(),
            self.edgcntb_en(),
            self.cfbwm(),
            self.cb0cnt(),
            self.cb1cnt()
        )
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrlx(pub u16);
impl Sm1captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm1captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm1captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm1captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm1captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm1captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm1captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm1captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm1captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrlx {
    #[inline(always)]
    fn default() -> Sm1captctrlx {
        Sm1captctrlx(0)
    }
}
impl core::fmt::Debug for Sm1captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfilta(pub u16);
impl Sm1captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfilta {
    #[inline(always)]
    fn default() -> Sm1captfilta {
        Sm1captfilta(0)
    }
}
impl core::fmt::Debug for Sm1captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfilta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captfilta {{ capta_filt_per: {=u8:?}, capta_filt_cnt: {=u8:?} }}",
            self.capta_filt_per(),
            self.capta_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfiltb(pub u16);
impl Sm1captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfiltb {
    #[inline(always)]
    fn default() -> Sm1captfiltb {
        Sm1captfiltb(0)
    }
}
impl core::fmt::Debug for Sm1captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfiltb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captfiltb {{ captb_filt_per: {=u8:?}, captb_filt_cnt: {=u8:?} }}",
            self.captb_filt_per(),
            self.captb_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfiltx(pub u16);
impl Sm1captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfiltx {
    #[inline(always)]
    fn default() -> Sm1captfiltx {
        Sm1captfiltx(0)
    }
}
impl core::fmt::Debug for Sm1captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cnt(pub u16);
impl Sm1cnt {
    #[doc = "Counter Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cnt {
    #[inline(always)]
    fn default() -> Sm1cnt {
        Sm1cnt(0)
    }
}
impl core::fmt::Debug for Sm1cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl(pub u16);
impl Sm1ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm1ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm1ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm1ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm1ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm1ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm1ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm1ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm1ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm1ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm1ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm1ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm1ctrl {
    #[inline(always)]
    fn default() -> Sm1ctrl {
        Sm1ctrl(0)
    }
}
impl core::fmt::Debug for Sm1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl2(pub u16);
impl Sm1ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm1ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm1ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm1ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm1ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm1ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm1ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm1ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm1ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm1ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm1ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm1ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm1ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm1ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm1ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1ctrl2 {
    #[inline(always)]
    fn default() -> Sm1ctrl2 {
        Sm1ctrl2(0)
    }
}
impl core::fmt::Debug for Sm1ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0(pub u16);
impl Sm1cval0 {
    #[doc = "Capture Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0"]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval0 {
    #[inline(always)]
    fn default() -> Sm1cval0 {
        Sm1cval0(0)
    }
}
impl core::fmt::Debug for Sm1cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0cyc(pub u16);
impl Sm1cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval0cyc {
    #[inline(always)]
    fn default() -> Sm1cval0cyc {
        Sm1cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1(pub u16);
impl Sm1cval1 {
    #[doc = "Capture Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1"]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval1 {
    #[inline(always)]
    fn default() -> Sm1cval1 {
        Sm1cval1(0)
    }
}
impl core::fmt::Debug for Sm1cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1cyc(pub u16);
impl Sm1cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval1cyc {
    #[inline(always)]
    fn default() -> Sm1cval1cyc {
        Sm1cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Capture Value 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval2(pub u16);
impl Sm1cval2 {
    #[doc = "Capture Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn captval2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 2"]
    #[inline(always)]
    pub const fn set_captval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval2 {
    #[inline(always)]
    fn default() -> Sm1cval2 {
        Sm1cval2(0)
    }
}
impl core::fmt::Debug for Sm1cval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval2")
            .field("captval2", &self.captval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval2 {{ captval2: {=u16:?} }}", self.captval2())
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval2cyc(pub u16);
impl Sm1cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval2cyc {
    #[inline(always)]
    fn default() -> Sm1cval2cyc {
        Sm1cval2cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval2cyc {{ cval2cyc: {=u8:?} }}", self.cval2cyc())
    }
}
#[doc = "Capture Value 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval3(pub u16);
impl Sm1cval3 {
    #[doc = "Capture Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn captval3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 3"]
    #[inline(always)]
    pub const fn set_captval3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval3 {
    #[inline(always)]
    fn default() -> Sm1cval3 {
        Sm1cval3(0)
    }
}
impl core::fmt::Debug for Sm1cval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval3")
            .field("captval3", &self.captval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval3 {{ captval3: {=u16:?} }}", self.captval3())
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval3cyc(pub u16);
impl Sm1cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval3cyc {
    #[inline(always)]
    fn default() -> Sm1cval3cyc {
        Sm1cval3cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval3cyc {{ cval3cyc: {=u8:?} }}", self.cval3cyc())
    }
}
#[doc = "Capture Value 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval4(pub u16);
impl Sm1cval4 {
    #[doc = "Capture Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captval4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 4"]
    #[inline(always)]
    pub const fn set_captval4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval4 {
    #[inline(always)]
    fn default() -> Sm1cval4 {
        Sm1cval4(0)
    }
}
impl core::fmt::Debug for Sm1cval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval4")
            .field("captval4", &self.captval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval4 {{ captval4: {=u16:?} }}", self.captval4())
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval4cyc(pub u16);
impl Sm1cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval4cyc {
    #[inline(always)]
    fn default() -> Sm1cval4cyc {
        Sm1cval4cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval4cyc {{ cval4cyc: {=u8:?} }}", self.cval4cyc())
    }
}
#[doc = "Capture Value 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval5(pub u16);
impl Sm1cval5 {
    #[doc = "Capture Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn captval5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 5"]
    #[inline(always)]
    pub const fn set_captval5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval5 {
    #[inline(always)]
    fn default() -> Sm1cval5 {
        Sm1cval5(0)
    }
}
impl core::fmt::Debug for Sm1cval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval5")
            .field("captval5", &self.captval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval5 {{ captval5: {=u16:?} }}", self.captval5())
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval5cyc(pub u16);
impl Sm1cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval5cyc {
    #[inline(always)]
    fn default() -> Sm1cval5cyc {
        Sm1cval5cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval5cyc {{ cval5cyc: {=u8:?} }}", self.cval5cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dismap0(pub u16);
impl Sm1dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm1dismap0 {
    #[inline(always)]
    fn default() -> Sm1dismap0 {
        Sm1dismap0(0)
    }
}
impl core::fmt::Debug for Sm1dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dmaen(pub u16);
impl Sm1dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm1dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm1dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm1dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm1dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm1dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm1dmaen {
    #[inline(always)]
    fn default() -> Sm1dmaen {
        Sm1dmaen(0)
    }
}
impl core::fmt::Debug for Sm1dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, cb0de: {=bool:?}, cb1de: {=bool:?}, ca0de: {=bool:?}, ca1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.cb0de(),
            self.cb1de(),
            self.ca0de(),
            self.ca1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dtcnt0(pub u16);
impl Sm1dtcnt0 {
    #[doc = "Deadtime Count Register 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm1dtcnt0 {
    #[inline(always)]
    fn default() -> Sm1dtcnt0 {
        Sm1dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm1dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dtcnt1(pub u16);
impl Sm1dtcnt1 {
    #[doc = "Deadtime Count Register 1"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm1dtcnt1 {
    #[inline(always)]
    fn default() -> Sm1dtcnt1 {
        Sm1dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm1dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval1(pub u16);
impl Sm1fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval1 {
    #[inline(always)]
    fn default() -> Sm1fracval1 {
        Sm1fracval1(0)
    }
}
impl core::fmt::Debug for Sm1fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1fracval1 {{ fracval1: {=u8:?} }}", self.fracval1())
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval2(pub u16);
impl Sm1fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval2 {
    #[inline(always)]
    fn default() -> Sm1fracval2 {
        Sm1fracval2(0)
    }
}
impl core::fmt::Debug for Sm1fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1fracval2 {{ fracval2: {=u8:?} }}", self.fracval2())
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval3(pub u16);
impl Sm1fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval3 {
    #[inline(always)]
    fn default() -> Sm1fracval3 {
        Sm1fracval3(0)
    }
}
impl core::fmt::Debug for Sm1fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1fracval3 {{ fracval3: {=u8:?} }}", self.fracval3())
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval4(pub u16);
impl Sm1fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval4 {
    #[inline(always)]
    fn default() -> Sm1fracval4 {
        Sm1fracval4(0)
    }
}
impl core::fmt::Debug for Sm1fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1fracval4 {{ fracval4: {=u8:?} }}", self.fracval4())
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval5(pub u16);
impl Sm1fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval5 {
    #[inline(always)]
    fn default() -> Sm1fracval5 {
        Sm1fracval5(0)
    }
}
impl core::fmt::Debug for Sm1fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1fracval5 {{ fracval5: {=u8:?} }}", self.fracval5())
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1frctrl(pub u16);
impl Sm1frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1frctrl {
    #[inline(always)]
    fn default() -> Sm1frctrl {
        Sm1frctrl(0)
    }
}
impl core::fmt::Debug for Sm1frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1frctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1frctrl {{ frac1_en: {=bool:?}, frac23_en: {=bool:?}, frac45_en: {=bool:?}, test: {=bool:?} }}",
            self.frac1_en(),
            self.frac23_en(),
            self.frac45_en(),
            self.test()
        )
    }
}
#[doc = "Initial Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1init(pub u16);
impl Sm1init {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1init {
    #[inline(always)]
    fn default() -> Sm1init {
        Sm1init(0)
    }
}
impl core::fmt::Debug for Sm1init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1inten(pub u16);
impl Sm1inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm1intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm1intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm1inten {
    #[inline(always)]
    fn default() -> Sm1inten {
        Sm1inten(0)
    }
}
impl core::fmt::Debug for Sm1inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, cb0ie: {=bool:?}, cb1ie: {=bool:?}, ca0ie: {=bool:?}, ca1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.cb0ie(),
            self.cb1ie(),
            self.ca0ie(),
            self.ca1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1octrl(pub u16);
impl Sm1octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm1octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm1octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm1octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm1octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm1octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm1octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm1octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1octrl {
    #[inline(always)]
    fn default() -> Sm1octrl {
        Sm1octrl(0)
    }
}
impl core::fmt::Debug for Sm1octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1phasedly(pub u16);
impl Sm1phasedly {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1phasedly {
    #[inline(always)]
    fn default() -> Sm1phasedly {
        Sm1phasedly(0)
    }
}
impl core::fmt::Debug for Sm1phasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1phasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1phasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1phasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1sts(pub u16);
impl Sm1sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm1stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm1stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm1sts {
    #[inline(always)]
    fn default() -> Sm1sts {
        Sm1sts(0)
    }
}
impl core::fmt::Debug for Sm1sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, cfb0: {=bool:?}, cfb1: {=bool:?}, cfa0: {=bool:?}, cfa1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.cfb0(),
            self.cfb1(),
            self.cfa0(),
            self.cfa1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1tctrl(pub u16);
impl Sm1tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm1tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm1tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm1tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm1tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm1tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm1tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm1tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm1tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm1tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm1tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm1tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1tctrl {
    #[inline(always)]
    fn default() -> Sm1tctrl {
        Sm1tctrl(0)
    }
}
impl core::fmt::Debug for Sm1tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val0(pub u16);
impl Sm1val0 {
    #[doc = "Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0"]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val0 {
    #[inline(always)]
    fn default() -> Sm1val0 {
        Sm1val0(0)
    }
}
impl core::fmt::Debug for Sm1val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val1(pub u16);
impl Sm1val1 {
    #[doc = "Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val1 {
    #[inline(always)]
    fn default() -> Sm1val1 {
        Sm1val1(0)
    }
}
impl core::fmt::Debug for Sm1val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val2(pub u16);
impl Sm1val2 {
    #[doc = "Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val2 {
    #[inline(always)]
    fn default() -> Sm1val2 {
        Sm1val2(0)
    }
}
impl core::fmt::Debug for Sm1val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val3(pub u16);
impl Sm1val3 {
    #[doc = "Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3"]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val3 {
    #[inline(always)]
    fn default() -> Sm1val3 {
        Sm1val3(0)
    }
}
impl core::fmt::Debug for Sm1val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val4(pub u16);
impl Sm1val4 {
    #[doc = "Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val4 {
    #[inline(always)]
    fn default() -> Sm1val4 {
        Sm1val4(0)
    }
}
impl core::fmt::Debug for Sm1val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val5(pub u16);
impl Sm1val5 {
    #[doc = "Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5"]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val5 {
    #[inline(always)]
    fn default() -> Sm1val5 {
        Sm1val5(0)
    }
}
impl core::fmt::Debug for Sm1val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompa(pub u16);
impl Sm2captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompa {
    #[inline(always)]
    fn default() -> Sm2captcompa {
        Sm2captcompa(0)
    }
}
impl core::fmt::Debug for Sm2captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captcompa {{ edgcmpa: {=u8:?}, edgcnta: {=u8:?} }}",
            self.edgcmpa(),
            self.edgcnta()
        )
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompb(pub u16);
impl Sm2captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompb {
    #[inline(always)]
    fn default() -> Sm2captcompb {
        Sm2captcompb(0)
    }
}
impl core::fmt::Debug for Sm2captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captcompb {{ edgcmpb: {=u8:?}, edgcntb: {=u8:?} }}",
            self.edgcmpb(),
            self.edgcntb()
        )
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompx(pub u16);
impl Sm2captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompx {
    #[inline(always)]
    fn default() -> Sm2captcompx {
        Sm2captcompx(0)
    }
}
impl core::fmt::Debug for Sm2captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrla(pub u16);
impl Sm2captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm2captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm2captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm2captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm2captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm2captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm2captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm2captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm2captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrla {
    #[inline(always)]
    fn default() -> Sm2captctrla {
        Sm2captctrla(0)
    }
}
impl core::fmt::Debug for Sm2captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captctrla {{ arma: {=bool:?}, oneshota: {:?}, edga0: {:?}, edga1: {:?}, inp_sela: {:?}, edgcnta_en: {=bool:?}, cfawm: {=u8:?}, ca0cnt: {=u8:?}, ca1cnt: {=u8:?} }}",
            self.arma(),
            self.oneshota(),
            self.edga0(),
            self.edga1(),
            self.inp_sela(),
            self.edgcnta_en(),
            self.cfawm(),
            self.ca0cnt(),
            self.ca1cnt()
        )
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrlb(pub u16);
impl Sm2captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm2captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm2captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm2captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm2captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm2captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm2captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm2captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm2captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrlb {
    #[inline(always)]
    fn default() -> Sm2captctrlb {
        Sm2captctrlb(0)
    }
}
impl core::fmt::Debug for Sm2captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captctrlb {{ armb: {=bool:?}, oneshotb: {:?}, edgb0: {:?}, edgb1: {:?}, inp_selb: {:?}, edgcntb_en: {=bool:?}, cfbwm: {=u8:?}, cb0cnt: {=u8:?}, cb1cnt: {=u8:?} }}",
            self.armb(),
            self.oneshotb(),
            self.edgb0(),
            self.edgb1(),
            self.inp_selb(),
            self.edgcntb_en(),
            self.cfbwm(),
            self.cb0cnt(),
            self.cb1cnt()
        )
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrlx(pub u16);
impl Sm2captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm2captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm2captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm2captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm2captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm2captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm2captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm2captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm2captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrlx {
    #[inline(always)]
    fn default() -> Sm2captctrlx {
        Sm2captctrlx(0)
    }
}
impl core::fmt::Debug for Sm2captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfilta(pub u16);
impl Sm2captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfilta {
    #[inline(always)]
    fn default() -> Sm2captfilta {
        Sm2captfilta(0)
    }
}
impl core::fmt::Debug for Sm2captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfilta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captfilta {{ capta_filt_per: {=u8:?}, capta_filt_cnt: {=u8:?} }}",
            self.capta_filt_per(),
            self.capta_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfiltb(pub u16);
impl Sm2captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfiltb {
    #[inline(always)]
    fn default() -> Sm2captfiltb {
        Sm2captfiltb(0)
    }
}
impl core::fmt::Debug for Sm2captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfiltb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captfiltb {{ captb_filt_per: {=u8:?}, captb_filt_cnt: {=u8:?} }}",
            self.captb_filt_per(),
            self.captb_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfiltx(pub u16);
impl Sm2captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfiltx {
    #[inline(always)]
    fn default() -> Sm2captfiltx {
        Sm2captfiltx(0)
    }
}
impl core::fmt::Debug for Sm2captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cnt(pub u16);
impl Sm2cnt {
    #[doc = "Counter Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cnt {
    #[inline(always)]
    fn default() -> Sm2cnt {
        Sm2cnt(0)
    }
}
impl core::fmt::Debug for Sm2cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl(pub u16);
impl Sm2ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm2ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm2ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm2ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm2ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm2ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm2ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm2ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm2ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm2ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm2ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm2ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm2ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm2ctrl {
    #[inline(always)]
    fn default() -> Sm2ctrl {
        Sm2ctrl(0)
    }
}
impl core::fmt::Debug for Sm2ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl2(pub u16);
impl Sm2ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm2ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm2ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm2ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm2ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm2ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm2ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm2ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm2ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm2ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm2ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm2ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm2ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm2ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm2ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2ctrl2 {
    #[inline(always)]
    fn default() -> Sm2ctrl2 {
        Sm2ctrl2(0)
    }
}
impl core::fmt::Debug for Sm2ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0(pub u16);
impl Sm2cval0 {
    #[doc = "Capture Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0"]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval0 {
    #[inline(always)]
    fn default() -> Sm2cval0 {
        Sm2cval0(0)
    }
}
impl core::fmt::Debug for Sm2cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0cyc(pub u16);
impl Sm2cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval0cyc {
    #[inline(always)]
    fn default() -> Sm2cval0cyc {
        Sm2cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1(pub u16);
impl Sm2cval1 {
    #[doc = "Capture Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1"]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval1 {
    #[inline(always)]
    fn default() -> Sm2cval1 {
        Sm2cval1(0)
    }
}
impl core::fmt::Debug for Sm2cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1cyc(pub u16);
impl Sm2cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval1cyc {
    #[inline(always)]
    fn default() -> Sm2cval1cyc {
        Sm2cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Capture Value 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval2(pub u16);
impl Sm2cval2 {
    #[doc = "Capture Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn captval2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 2"]
    #[inline(always)]
    pub const fn set_captval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval2 {
    #[inline(always)]
    fn default() -> Sm2cval2 {
        Sm2cval2(0)
    }
}
impl core::fmt::Debug for Sm2cval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval2")
            .field("captval2", &self.captval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval2 {{ captval2: {=u16:?} }}", self.captval2())
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval2cyc(pub u16);
impl Sm2cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval2cyc {
    #[inline(always)]
    fn default() -> Sm2cval2cyc {
        Sm2cval2cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval2cyc {{ cval2cyc: {=u8:?} }}", self.cval2cyc())
    }
}
#[doc = "Capture Value 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval3(pub u16);
impl Sm2cval3 {
    #[doc = "Capture Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn captval3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 3"]
    #[inline(always)]
    pub const fn set_captval3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval3 {
    #[inline(always)]
    fn default() -> Sm2cval3 {
        Sm2cval3(0)
    }
}
impl core::fmt::Debug for Sm2cval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval3")
            .field("captval3", &self.captval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval3 {{ captval3: {=u16:?} }}", self.captval3())
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval3cyc(pub u16);
impl Sm2cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval3cyc {
    #[inline(always)]
    fn default() -> Sm2cval3cyc {
        Sm2cval3cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval3cyc {{ cval3cyc: {=u8:?} }}", self.cval3cyc())
    }
}
#[doc = "Capture Value 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval4(pub u16);
impl Sm2cval4 {
    #[doc = "Capture Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captval4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 4"]
    #[inline(always)]
    pub const fn set_captval4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval4 {
    #[inline(always)]
    fn default() -> Sm2cval4 {
        Sm2cval4(0)
    }
}
impl core::fmt::Debug for Sm2cval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval4")
            .field("captval4", &self.captval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval4 {{ captval4: {=u16:?} }}", self.captval4())
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval4cyc(pub u16);
impl Sm2cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval4cyc {
    #[inline(always)]
    fn default() -> Sm2cval4cyc {
        Sm2cval4cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval4cyc {{ cval4cyc: {=u8:?} }}", self.cval4cyc())
    }
}
#[doc = "Capture Value 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval5(pub u16);
impl Sm2cval5 {
    #[doc = "Capture Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn captval5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 5"]
    #[inline(always)]
    pub const fn set_captval5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval5 {
    #[inline(always)]
    fn default() -> Sm2cval5 {
        Sm2cval5(0)
    }
}
impl core::fmt::Debug for Sm2cval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval5")
            .field("captval5", &self.captval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval5 {{ captval5: {=u16:?} }}", self.captval5())
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval5cyc(pub u16);
impl Sm2cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval5cyc {
    #[inline(always)]
    fn default() -> Sm2cval5cyc {
        Sm2cval5cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval5cyc {{ cval5cyc: {=u8:?} }}", self.cval5cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dismap0(pub u16);
impl Sm2dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm2dismap0 {
    #[inline(always)]
    fn default() -> Sm2dismap0 {
        Sm2dismap0(0)
    }
}
impl core::fmt::Debug for Sm2dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dmaen(pub u16);
impl Sm2dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm2dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm2dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm2dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm2dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm2dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm2dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm2dmaen {
    #[inline(always)]
    fn default() -> Sm2dmaen {
        Sm2dmaen(0)
    }
}
impl core::fmt::Debug for Sm2dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, cb0de: {=bool:?}, cb1de: {=bool:?}, ca0de: {=bool:?}, ca1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.cb0de(),
            self.cb1de(),
            self.ca0de(),
            self.ca1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dtcnt0(pub u16);
impl Sm2dtcnt0 {
    #[doc = "Deadtime Count Register 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm2dtcnt0 {
    #[inline(always)]
    fn default() -> Sm2dtcnt0 {
        Sm2dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm2dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dtcnt1(pub u16);
impl Sm2dtcnt1 {
    #[doc = "Deadtime Count Register 1"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm2dtcnt1 {
    #[inline(always)]
    fn default() -> Sm2dtcnt1 {
        Sm2dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm2dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval1(pub u16);
impl Sm2fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval1 {
    #[inline(always)]
    fn default() -> Sm2fracval1 {
        Sm2fracval1(0)
    }
}
impl core::fmt::Debug for Sm2fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2fracval1 {{ fracval1: {=u8:?} }}", self.fracval1())
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval2(pub u16);
impl Sm2fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval2 {
    #[inline(always)]
    fn default() -> Sm2fracval2 {
        Sm2fracval2(0)
    }
}
impl core::fmt::Debug for Sm2fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2fracval2 {{ fracval2: {=u8:?} }}", self.fracval2())
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval3(pub u16);
impl Sm2fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval3 {
    #[inline(always)]
    fn default() -> Sm2fracval3 {
        Sm2fracval3(0)
    }
}
impl core::fmt::Debug for Sm2fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2fracval3 {{ fracval3: {=u8:?} }}", self.fracval3())
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval4(pub u16);
impl Sm2fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval4 {
    #[inline(always)]
    fn default() -> Sm2fracval4 {
        Sm2fracval4(0)
    }
}
impl core::fmt::Debug for Sm2fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2fracval4 {{ fracval4: {=u8:?} }}", self.fracval4())
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval5(pub u16);
impl Sm2fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval5 {
    #[inline(always)]
    fn default() -> Sm2fracval5 {
        Sm2fracval5(0)
    }
}
impl core::fmt::Debug for Sm2fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2fracval5 {{ fracval5: {=u8:?} }}", self.fracval5())
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2frctrl(pub u16);
impl Sm2frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2frctrl {
    #[inline(always)]
    fn default() -> Sm2frctrl {
        Sm2frctrl(0)
    }
}
impl core::fmt::Debug for Sm2frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2frctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2frctrl {{ frac1_en: {=bool:?}, frac23_en: {=bool:?}, frac45_en: {=bool:?}, test: {=bool:?} }}",
            self.frac1_en(),
            self.frac23_en(),
            self.frac45_en(),
            self.test()
        )
    }
}
#[doc = "Initial Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2init(pub u16);
impl Sm2init {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2init {
    #[inline(always)]
    fn default() -> Sm2init {
        Sm2init(0)
    }
}
impl core::fmt::Debug for Sm2init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2inten(pub u16);
impl Sm2inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm2intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm2intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm2inten {
    #[inline(always)]
    fn default() -> Sm2inten {
        Sm2inten(0)
    }
}
impl core::fmt::Debug for Sm2inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, cb0ie: {=bool:?}, cb1ie: {=bool:?}, ca0ie: {=bool:?}, ca1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.cb0ie(),
            self.cb1ie(),
            self.ca0ie(),
            self.ca1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2octrl(pub u16);
impl Sm2octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm2octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm2octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm2octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm2octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm2octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm2octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm2octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2octrl {
    #[inline(always)]
    fn default() -> Sm2octrl {
        Sm2octrl(0)
    }
}
impl core::fmt::Debug for Sm2octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2phasedly(pub u16);
impl Sm2phasedly {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2phasedly {
    #[inline(always)]
    fn default() -> Sm2phasedly {
        Sm2phasedly(0)
    }
}
impl core::fmt::Debug for Sm2phasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2phasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2phasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2phasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2sts(pub u16);
impl Sm2sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm2stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm2stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm2sts {
    #[inline(always)]
    fn default() -> Sm2sts {
        Sm2sts(0)
    }
}
impl core::fmt::Debug for Sm2sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, cfb0: {=bool:?}, cfb1: {=bool:?}, cfa0: {=bool:?}, cfa1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.cfb0(),
            self.cfb1(),
            self.cfa0(),
            self.cfa1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2tctrl(pub u16);
impl Sm2tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm2tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm2tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm2tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm2tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm2tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm2tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm2tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm2tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm2tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm2tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm2tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2tctrl {
    #[inline(always)]
    fn default() -> Sm2tctrl {
        Sm2tctrl(0)
    }
}
impl core::fmt::Debug for Sm2tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val0(pub u16);
impl Sm2val0 {
    #[doc = "Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0"]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val0 {
    #[inline(always)]
    fn default() -> Sm2val0 {
        Sm2val0(0)
    }
}
impl core::fmt::Debug for Sm2val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val1(pub u16);
impl Sm2val1 {
    #[doc = "Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val1 {
    #[inline(always)]
    fn default() -> Sm2val1 {
        Sm2val1(0)
    }
}
impl core::fmt::Debug for Sm2val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val2(pub u16);
impl Sm2val2 {
    #[doc = "Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val2 {
    #[inline(always)]
    fn default() -> Sm2val2 {
        Sm2val2(0)
    }
}
impl core::fmt::Debug for Sm2val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val3(pub u16);
impl Sm2val3 {
    #[doc = "Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3"]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val3 {
    #[inline(always)]
    fn default() -> Sm2val3 {
        Sm2val3(0)
    }
}
impl core::fmt::Debug for Sm2val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val4(pub u16);
impl Sm2val4 {
    #[doc = "Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val4 {
    #[inline(always)]
    fn default() -> Sm2val4 {
        Sm2val4(0)
    }
}
impl core::fmt::Debug for Sm2val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val5(pub u16);
impl Sm2val5 {
    #[doc = "Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5"]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val5 {
    #[inline(always)]
    fn default() -> Sm2val5 {
        Sm2val5(0)
    }
}
impl core::fmt::Debug for Sm2val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompa(pub u16);
impl Sm3captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompa {
    #[inline(always)]
    fn default() -> Sm3captcompa {
        Sm3captcompa(0)
    }
}
impl core::fmt::Debug for Sm3captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captcompa {{ edgcmpa: {=u8:?}, edgcnta: {=u8:?} }}",
            self.edgcmpa(),
            self.edgcnta()
        )
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompb(pub u16);
impl Sm3captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompb {
    #[inline(always)]
    fn default() -> Sm3captcompb {
        Sm3captcompb(0)
    }
}
impl core::fmt::Debug for Sm3captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captcompb {{ edgcmpb: {=u8:?}, edgcntb: {=u8:?} }}",
            self.edgcmpb(),
            self.edgcntb()
        )
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompx(pub u16);
impl Sm3captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompx {
    #[inline(always)]
    fn default() -> Sm3captcompx {
        Sm3captcompx(0)
    }
}
impl core::fmt::Debug for Sm3captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrla(pub u16);
impl Sm3captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm3captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm3captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm3captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm3captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm3captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm3captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm3captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm3captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm3captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrla {
    #[inline(always)]
    fn default() -> Sm3captctrla {
        Sm3captctrla(0)
    }
}
impl core::fmt::Debug for Sm3captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captctrla {{ arma: {=bool:?}, oneshota: {:?}, edga0: {:?}, edga1: {:?}, inp_sela: {:?}, edgcnta_en: {=bool:?}, cfawm: {=u8:?}, ca0cnt: {=u8:?}, ca1cnt: {=u8:?} }}",
            self.arma(),
            self.oneshota(),
            self.edga0(),
            self.edga1(),
            self.inp_sela(),
            self.edgcnta_en(),
            self.cfawm(),
            self.ca0cnt(),
            self.ca1cnt()
        )
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrlb(pub u16);
impl Sm3captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm3captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm3captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm3captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm3captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm3captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm3captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm3captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm3captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm3captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrlb {
    #[inline(always)]
    fn default() -> Sm3captctrlb {
        Sm3captctrlb(0)
    }
}
impl core::fmt::Debug for Sm3captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captctrlb {{ armb: {=bool:?}, oneshotb: {:?}, edgb0: {:?}, edgb1: {:?}, inp_selb: {:?}, edgcntb_en: {=bool:?}, cfbwm: {=u8:?}, cb0cnt: {=u8:?}, cb1cnt: {=u8:?} }}",
            self.armb(),
            self.oneshotb(),
            self.edgb0(),
            self.edgb1(),
            self.inp_selb(),
            self.edgcntb_en(),
            self.cfbwm(),
            self.cb0cnt(),
            self.cb1cnt()
        )
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrlx(pub u16);
impl Sm3captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm3captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm3captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm3captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm3captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm3captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm3captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm3captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm3captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm3captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrlx {
    #[inline(always)]
    fn default() -> Sm3captctrlx {
        Sm3captctrlx(0)
    }
}
impl core::fmt::Debug for Sm3captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfilta(pub u16);
impl Sm3captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfilta {
    #[inline(always)]
    fn default() -> Sm3captfilta {
        Sm3captfilta(0)
    }
}
impl core::fmt::Debug for Sm3captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfilta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captfilta {{ capta_filt_per: {=u8:?}, capta_filt_cnt: {=u8:?} }}",
            self.capta_filt_per(),
            self.capta_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfiltb(pub u16);
impl Sm3captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfiltb {
    #[inline(always)]
    fn default() -> Sm3captfiltb {
        Sm3captfiltb(0)
    }
}
impl core::fmt::Debug for Sm3captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfiltb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captfiltb {{ captb_filt_per: {=u8:?}, captb_filt_cnt: {=u8:?} }}",
            self.captb_filt_per(),
            self.captb_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfiltx(pub u16);
impl Sm3captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfiltx {
    #[inline(always)]
    fn default() -> Sm3captfiltx {
        Sm3captfiltx(0)
    }
}
impl core::fmt::Debug for Sm3captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cnt(pub u16);
impl Sm3cnt {
    #[doc = "Counter Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cnt {
    #[inline(always)]
    fn default() -> Sm3cnt {
        Sm3cnt(0)
    }
}
impl core::fmt::Debug for Sm3cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3ctrl(pub u16);
impl Sm3ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm3ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm3ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm3ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm3ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm3ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm3ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm3ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm3ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm3ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm3ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm3ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm3ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm3ctrl {
    #[inline(always)]
    fn default() -> Sm3ctrl {
        Sm3ctrl(0)
    }
}
impl core::fmt::Debug for Sm3ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3ctrl2(pub u16);
impl Sm3ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm3ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm3ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm3ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm3ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm3ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm3ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm3ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm3ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm3ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm3ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm3ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm3ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm3ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm3ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm3ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3ctrl2 {
    #[inline(always)]
    fn default() -> Sm3ctrl2 {
        Sm3ctrl2(0)
    }
}
impl core::fmt::Debug for Sm3ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval0(pub u16);
impl Sm3cval0 {
    #[doc = "Capture Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0"]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval0 {
    #[inline(always)]
    fn default() -> Sm3cval0 {
        Sm3cval0(0)
    }
}
impl core::fmt::Debug for Sm3cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval0cyc(pub u16);
impl Sm3cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval0cyc {
    #[inline(always)]
    fn default() -> Sm3cval0cyc {
        Sm3cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval1(pub u16);
impl Sm3cval1 {
    #[doc = "Capture Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1"]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval1 {
    #[inline(always)]
    fn default() -> Sm3cval1 {
        Sm3cval1(0)
    }
}
impl core::fmt::Debug for Sm3cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval1cyc(pub u16);
impl Sm3cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval1cyc {
    #[inline(always)]
    fn default() -> Sm3cval1cyc {
        Sm3cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Capture Value 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval2(pub u16);
impl Sm3cval2 {
    #[doc = "Capture Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn captval2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 2"]
    #[inline(always)]
    pub const fn set_captval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval2 {
    #[inline(always)]
    fn default() -> Sm3cval2 {
        Sm3cval2(0)
    }
}
impl core::fmt::Debug for Sm3cval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval2")
            .field("captval2", &self.captval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval2 {{ captval2: {=u16:?} }}", self.captval2())
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval2cyc(pub u16);
impl Sm3cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval2cyc {
    #[inline(always)]
    fn default() -> Sm3cval2cyc {
        Sm3cval2cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval2cyc {{ cval2cyc: {=u8:?} }}", self.cval2cyc())
    }
}
#[doc = "Capture Value 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval3(pub u16);
impl Sm3cval3 {
    #[doc = "Capture Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn captval3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 3"]
    #[inline(always)]
    pub const fn set_captval3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval3 {
    #[inline(always)]
    fn default() -> Sm3cval3 {
        Sm3cval3(0)
    }
}
impl core::fmt::Debug for Sm3cval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval3")
            .field("captval3", &self.captval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval3 {{ captval3: {=u16:?} }}", self.captval3())
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval3cyc(pub u16);
impl Sm3cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval3cyc {
    #[inline(always)]
    fn default() -> Sm3cval3cyc {
        Sm3cval3cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval3cyc {{ cval3cyc: {=u8:?} }}", self.cval3cyc())
    }
}
#[doc = "Capture Value 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval4(pub u16);
impl Sm3cval4 {
    #[doc = "Capture Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captval4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 4"]
    #[inline(always)]
    pub const fn set_captval4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval4 {
    #[inline(always)]
    fn default() -> Sm3cval4 {
        Sm3cval4(0)
    }
}
impl core::fmt::Debug for Sm3cval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval4")
            .field("captval4", &self.captval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval4 {{ captval4: {=u16:?} }}", self.captval4())
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval4cyc(pub u16);
impl Sm3cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval4cyc {
    #[inline(always)]
    fn default() -> Sm3cval4cyc {
        Sm3cval4cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval4cyc {{ cval4cyc: {=u8:?} }}", self.cval4cyc())
    }
}
#[doc = "Capture Value 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval5(pub u16);
impl Sm3cval5 {
    #[doc = "Capture Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn captval5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 5"]
    #[inline(always)]
    pub const fn set_captval5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval5 {
    #[inline(always)]
    fn default() -> Sm3cval5 {
        Sm3cval5(0)
    }
}
impl core::fmt::Debug for Sm3cval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval5")
            .field("captval5", &self.captval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval5 {{ captval5: {=u16:?} }}", self.captval5())
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval5cyc(pub u16);
impl Sm3cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval5cyc {
    #[inline(always)]
    fn default() -> Sm3cval5cyc {
        Sm3cval5cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval5cyc {{ cval5cyc: {=u8:?} }}", self.cval5cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dismap0(pub u16);
impl Sm3dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm3dismap0 {
    #[inline(always)]
    fn default() -> Sm3dismap0 {
        Sm3dismap0(0)
    }
}
impl core::fmt::Debug for Sm3dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dmaen(pub u16);
impl Sm3dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm3dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm3dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm3dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm3dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm3dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm3dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm3dmaen {
    #[inline(always)]
    fn default() -> Sm3dmaen {
        Sm3dmaen(0)
    }
}
impl core::fmt::Debug for Sm3dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, cb0de: {=bool:?}, cb1de: {=bool:?}, ca0de: {=bool:?}, ca1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.cb0de(),
            self.cb1de(),
            self.ca0de(),
            self.ca1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dtcnt0(pub u16);
impl Sm3dtcnt0 {
    #[doc = "Deadtime Count Register 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm3dtcnt0 {
    #[inline(always)]
    fn default() -> Sm3dtcnt0 {
        Sm3dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm3dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dtcnt1(pub u16);
impl Sm3dtcnt1 {
    #[doc = "Deadtime Count Register 1"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm3dtcnt1 {
    #[inline(always)]
    fn default() -> Sm3dtcnt1 {
        Sm3dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm3dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval1(pub u16);
impl Sm3fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval1 {
    #[inline(always)]
    fn default() -> Sm3fracval1 {
        Sm3fracval1(0)
    }
}
impl core::fmt::Debug for Sm3fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3fracval1 {{ fracval1: {=u8:?} }}", self.fracval1())
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval2(pub u16);
impl Sm3fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval2 {
    #[inline(always)]
    fn default() -> Sm3fracval2 {
        Sm3fracval2(0)
    }
}
impl core::fmt::Debug for Sm3fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3fracval2 {{ fracval2: {=u8:?} }}", self.fracval2())
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval3(pub u16);
impl Sm3fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval3 {
    #[inline(always)]
    fn default() -> Sm3fracval3 {
        Sm3fracval3(0)
    }
}
impl core::fmt::Debug for Sm3fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3fracval3 {{ fracval3: {=u8:?} }}", self.fracval3())
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval4(pub u16);
impl Sm3fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval4 {
    #[inline(always)]
    fn default() -> Sm3fracval4 {
        Sm3fracval4(0)
    }
}
impl core::fmt::Debug for Sm3fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3fracval4 {{ fracval4: {=u8:?} }}", self.fracval4())
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval5(pub u16);
impl Sm3fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval5 {
    #[inline(always)]
    fn default() -> Sm3fracval5 {
        Sm3fracval5(0)
    }
}
impl core::fmt::Debug for Sm3fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3fracval5 {{ fracval5: {=u8:?} }}", self.fracval5())
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3frctrl(pub u16);
impl Sm3frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3frctrl {
    #[inline(always)]
    fn default() -> Sm3frctrl {
        Sm3frctrl(0)
    }
}
impl core::fmt::Debug for Sm3frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3frctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3frctrl {{ frac1_en: {=bool:?}, frac23_en: {=bool:?}, frac45_en: {=bool:?}, test: {=bool:?} }}",
            self.frac1_en(),
            self.frac23_en(),
            self.frac45_en(),
            self.test()
        )
    }
}
#[doc = "Initial Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3init(pub u16);
impl Sm3init {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3init {
    #[inline(always)]
    fn default() -> Sm3init {
        Sm3init(0)
    }
}
impl core::fmt::Debug for Sm3init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3inten(pub u16);
impl Sm3inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm3intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm3intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm3intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm3inten {
    #[inline(always)]
    fn default() -> Sm3inten {
        Sm3inten(0)
    }
}
impl core::fmt::Debug for Sm3inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, cb0ie: {=bool:?}, cb1ie: {=bool:?}, ca0ie: {=bool:?}, ca1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.cb0ie(),
            self.cb1ie(),
            self.ca0ie(),
            self.ca1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3octrl(pub u16);
impl Sm3octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm3octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm3octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm3octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm3octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm3octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm3octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm3octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3octrl {
    #[inline(always)]
    fn default() -> Sm3octrl {
        Sm3octrl(0)
    }
}
impl core::fmt::Debug for Sm3octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3phasedly(pub u16);
impl Sm3phasedly {
    #[doc = "Initial Count Register Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3phasedly {
    #[inline(always)]
    fn default() -> Sm3phasedly {
        Sm3phasedly(0)
    }
}
impl core::fmt::Debug for Sm3phasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3phasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3phasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3phasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3sts(pub u16);
impl Sm3sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm3stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm3stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm3stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm3sts {
    #[inline(always)]
    fn default() -> Sm3sts {
        Sm3sts(0)
    }
}
impl core::fmt::Debug for Sm3sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, cfb0: {=bool:?}, cfb1: {=bool:?}, cfa0: {=bool:?}, cfa1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.cfb0(),
            self.cfb1(),
            self.cfa0(),
            self.cfa1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3tctrl(pub u16);
impl Sm3tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm3tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm3tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm3tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm3tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm3tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm3tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm3tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm3tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm3tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm3tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm3tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm3tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3tctrl {
    #[inline(always)]
    fn default() -> Sm3tctrl {
        Sm3tctrl(0)
    }
}
impl core::fmt::Debug for Sm3tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val0(pub u16);
impl Sm3val0 {
    #[doc = "Value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0"]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val0 {
    #[inline(always)]
    fn default() -> Sm3val0 {
        Sm3val0(0)
    }
}
impl core::fmt::Debug for Sm3val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val1(pub u16);
impl Sm3val1 {
    #[doc = "Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1"]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val1 {
    #[inline(always)]
    fn default() -> Sm3val1 {
        Sm3val1(0)
    }
}
impl core::fmt::Debug for Sm3val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val2(pub u16);
impl Sm3val2 {
    #[doc = "Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2"]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val2 {
    #[inline(always)]
    fn default() -> Sm3val2 {
        Sm3val2(0)
    }
}
impl core::fmt::Debug for Sm3val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val3(pub u16);
impl Sm3val3 {
    #[doc = "Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3"]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val3 {
    #[inline(always)]
    fn default() -> Sm3val3 {
        Sm3val3(0)
    }
}
impl core::fmt::Debug for Sm3val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val4(pub u16);
impl Sm3val4 {
    #[doc = "Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4"]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val4 {
    #[inline(always)]
    fn default() -> Sm3val4 {
        Sm3val4(0)
    }
}
impl core::fmt::Debug for Sm3val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val5(pub u16);
impl Sm3val5 {
    #[doc = "Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5"]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val5 {
    #[inline(always)]
    fn default() -> Sm3val5 {
        Sm3val5(0)
    }
}
impl core::fmt::Debug for Sm3val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Software Controlled Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swcout(pub u16);
impl Swcout {
    #[doc = "Submodule 0 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out45(&self) -> super::vals::Sm0out45 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sm0out45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: super::vals::Sm0out45) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out23(&self) -> super::vals::Sm0out23 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0out23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: super::vals::Sm0out23) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out45(&self) -> super::vals::Sm1out45 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1out45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: super::vals::Sm1out45) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out23(&self) -> super::vals::Sm1out23 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sm1out23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: super::vals::Sm1out23) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out45(&self) -> super::vals::Sm2out45 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sm2out45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: super::vals::Sm2out45) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out23(&self) -> super::vals::Sm2out23 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sm2out23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: super::vals::Sm2out23) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out45(&self) -> super::vals::Sm3out45 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3out45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm3out45(&mut self, val: super::vals::Sm3out45) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out23(&self) -> super::vals::Sm3out23 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm3out23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm3out23(&mut self, val: super::vals::Sm3out23) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
}
impl Default for Swcout {
    #[inline(always)]
    fn default() -> Swcout {
        Swcout(0)
    }
}
impl core::fmt::Debug for Swcout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swcout")
            .field("sm0out45", &self.sm0out45())
            .field("sm0out23", &self.sm0out23())
            .field("sm1out45", &self.sm1out45())
            .field("sm1out23", &self.sm1out23())
            .field("sm2out45", &self.sm2out45())
            .field("sm2out23", &self.sm2out23())
            .field("sm3out45", &self.sm3out45())
            .field("sm3out23", &self.sm3out23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swcout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swcout {{ sm0out45: {:?}, sm0out23: {:?}, sm1out45: {:?}, sm1out23: {:?}, sm2out45: {:?}, sm2out23: {:?}, sm3out45: {:?}, sm3out23: {:?} }}",
            self.sm0out45(),
            self.sm0out23(),
            self.sm1out45(),
            self.sm1out23(),
            self.sm2out45(),
            self.sm2out23(),
            self.sm3out45(),
            self.sm3out23()
        )
    }
}
