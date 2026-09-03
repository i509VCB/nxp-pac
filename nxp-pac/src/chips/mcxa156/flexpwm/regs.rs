#[doc = "PWM Source Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsrcsel(pub u16);
impl Dtsrcsel {
    #[doc = "Submodule 0 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel45(&self) -> super::vals::Sm0sel45 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm0sel45(&mut self, val: super::vals::Sm0sel45) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Submodule 0 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel23(&self) -> super::vals::Sm0sel23 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm0sel23(&mut self, val: super::vals::Sm0sel23) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Submodule 1 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel45(&self) -> super::vals::Sm1sel45 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm1sel45(&mut self, val: super::vals::Sm1sel45) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Submodule 1 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel23(&self) -> super::vals::Sm1sel23 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm1sel23(&mut self, val: super::vals::Sm1sel23) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Submodule 2 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel45(&self) -> super::vals::Sm2sel45 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm2sel45(&mut self, val: super::vals::Sm2sel45) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Submodule 2 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel23(&self) -> super::vals::Sm2sel23 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sm2sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm2sel23(&mut self, val: super::vals::Sm2sel23) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtsrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtsrcsel {{ sm0sel45: {:?}, sm0sel23: {:?}, sm1sel45: {:?}, sm1sel23: {:?}, sm2sel45: {:?}, sm2sel23: {:?} }}",
            self.sm0sel45(),
            self.sm0sel23(),
            self.sm1sel45(),
            self.sm1sel23(),
            self.sm2sel45(),
            self.sm2sel23()
        )
    }
}
#[doc = "Fault Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl0(pub u16);
impl Fctrl0 {
    #[doc = "Fault Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn fie(&self) -> super::vals::Fie {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fie::from_bits(val as u8)
    }
    #[doc = "Fault Interrupt Enables."]
    #[inline(always)]
    pub const fn set_fie(&mut self, val: super::vals::Fie) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Fault Safety Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fsafe(&self) -> super::vals::Fsafe {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Fsafe::from_bits(val as u8)
    }
    #[doc = "Fault Safety Mode."]
    #[inline(always)]
    pub const fn set_fsafe(&mut self, val: super::vals::Fsafe) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Automatic Fault Clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn fauto(&self) -> super::vals::Fauto {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Fauto::from_bits(val as u8)
    }
    #[doc = "Automatic Fault Clearing."]
    #[inline(always)]
    pub const fn set_fauto(&mut self, val: super::vals::Fauto) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Fault Level."]
    #[must_use]
    #[inline(always)]
    pub const fn flvl(&self) -> super::vals::Flvl {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Flvl::from_bits(val as u8)
    }
    #[doc = "Fault Level."]
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
#[doc = "Fault Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl20(pub u16);
impl Fctrl20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output."]
    #[must_use]
    #[inline(always)]
    pub const fn nocomb(&self) -> super::vals::Nocomb {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Nocomb::from_bits(val as u8)
    }
    #[doc = "No Combinational Path From Fault Input To PWM Output."]
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
#[doc = "Fault Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffilt0(pub u16);
impl Ffilt0 {
    #[doc = "Fault Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fault Filter Period."]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Fault Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Fault Filter Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Fault Glitch Stretch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gstr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Glitch Stretch Enable."]
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
#[doc = "Fault Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsts0(pub u16);
impl Fsts0 {
    #[doc = "Fault Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn fflag(&self) -> super::vals::Fflag {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fflag::from_bits(val as u8)
    }
    #[doc = "Fault Flags."]
    #[inline(always)]
    pub const fn set_fflag(&mut self, val: super::vals::Fflag) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Full Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn ffull(&self) -> super::vals::Ffull {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ffull::from_bits(val as u8)
    }
    #[doc = "Full Cycle."]
    #[inline(always)]
    pub const fn set_ffull(&mut self, val: super::vals::Ffull) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Filtered Fault Pins."]
    #[must_use]
    #[inline(always)]
    pub const fn ffpin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filtered Fault Pins."]
    #[inline(always)]
    pub const fn set_ffpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Half Cycle Fault Recovery."]
    #[must_use]
    #[inline(always)]
    pub const fn fhalf(&self) -> super::vals::Fhalf {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Fhalf::from_bits(val as u8)
    }
    #[doc = "Half Cycle Fault Recovery."]
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
#[doc = "Fault Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftst0(pub u16);
impl Ftst0 {
    #[doc = "Fault Test."]
    #[must_use]
    #[inline(always)]
    pub const fn ftest(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Test."]
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
#[doc = "Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u16);
impl Mask {
    #[doc = "PWM_X Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maskx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PWM_X Masks."]
    #[inline(always)]
    pub const fn set_maskx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "PWM_B Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maskb(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "PWM_B Masks."]
    #[inline(always)]
    pub const fn set_maskb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
    }
    #[doc = "PWM_A Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maska(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "PWM_A Masks."]
    #[inline(always)]
    pub const fn set_maska(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Update Mask Bits Immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn update_mask(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Update Mask Bits Immediately."]
    #[inline(always)]
    pub const fn set_update_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
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
#[doc = "Master Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u16);
impl Mctrl {
    #[doc = "Load Okay."]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> super::vals::Ldok {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay."]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: super::vals::Ldok) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Clear Load Okay."]
    #[must_use]
    #[inline(always)]
    pub const fn cldok(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Clear Load Okay."]
    #[inline(always)]
    pub const fn set_cldok(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
    }
    #[doc = "Run."]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> super::vals::Run {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Run::from_bits(val as u8)
    }
    #[doc = "Run."]
    #[inline(always)]
    pub const fn set_run(&mut self, val: super::vals::Run) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Current Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ipol(&self) -> super::vals::Ipol {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Ipol::from_bits(val as u8)
    }
    #[doc = "Current Polarity."]
    #[inline(always)]
    pub const fn set_ipol(&mut self, val: super::vals::Ipol) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u16) & 0x07) << 12usize);
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
#[doc = "Master Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl2(pub u16);
impl Mctrl2 {
    #[doc = "Write protect."]
    #[must_use]
    #[inline(always)]
    pub const fn wrprot(&self) -> super::vals::Wrprot {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Wrprot::from_bits(val as u8)
    }
    #[doc = "Write protect."]
    #[inline(always)]
    pub const fn set_wrprot(&mut self, val: super::vals::Wrprot) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch_cnt_prsc(&self) -> super::vals::StretchCntPrsc {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::StretchCntPrsc::from_bits(val as u8)
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig."]
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
#[doc = "Output Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outen(pub u16);
impl Outen {
    #[doc = "PWM_X Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PWM_X Output Enables."]
    #[inline(always)]
    pub const fn set_pwmx_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "PWM_B Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "PWM_B Output Enables."]
    #[inline(always)]
    pub const fn set_pwmb_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
    }
    #[doc = "PWM_A Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "PWM_A Output Enables."]
    #[inline(always)]
    pub const fn set_pwma_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
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
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompx(pub u16);
impl Sm0captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
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
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrlx(pub u16);
impl Sm0captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm0captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm0captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm0captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm0captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm0captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm0captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm0captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm0captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
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
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfiltx(pub u16);
impl Sm0captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
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
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cnt(pub u16);
impl Sm0cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
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
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl(pub u16);
impl Sm0ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm0ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm0ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm0ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm0ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm0ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm0ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm0ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm0ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm0ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm0ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm0ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
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
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl2(pub u16);
impl Sm0ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm0ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm0ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm0ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm0ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm0ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm0ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm0ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm0ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm0ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm0ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm0ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm0ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm0ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm0ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
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
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0(pub u16);
impl Sm0cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
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
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0cyc(pub u16);
impl Sm0cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
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
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1(pub u16);
impl Sm0cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
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
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1cyc(pub u16);
impl Sm0cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
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
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dismap0(pub u16);
impl Sm0dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
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
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dmaen(pub u16);
impl Sm0dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm0dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm0dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm0dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm0dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm0dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm0dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
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
            "Sm0dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dtcnt0(pub u16);
impl Sm0dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
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
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dtcnt1(pub u16);
impl Sm0dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
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
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0init(pub u16);
impl Sm0init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
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
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0inten(pub u16);
impl Sm0inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm0intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm0intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
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
            "Sm0inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0octrl(pub u16);
impl Sm0octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm0octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm0octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm0octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm0octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm0octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm0octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
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
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0sts(pub u16);
impl Sm0sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm0stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm0stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
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
            "Sm0sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0tctrl(pub u16);
impl Sm0tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm0tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm0tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm0tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm0tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm0tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm0tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm0tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm0tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm0tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm0tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
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
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val0(pub u16);
impl Sm0val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
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
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val1(pub u16);
impl Sm0val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
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
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val2(pub u16);
impl Sm0val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
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
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val3(pub u16);
impl Sm0val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
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
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val4(pub u16);
impl Sm0val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
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
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val5(pub u16);
impl Sm0val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
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
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompx(pub u16);
impl Sm1captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
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
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrlx(pub u16);
impl Sm1captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm1captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm1captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm1captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm1captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm1captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm1captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm1captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm1captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
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
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfiltx(pub u16);
impl Sm1captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
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
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cnt(pub u16);
impl Sm1cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
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
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl(pub u16);
impl Sm1ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm1ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm1ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm1ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm1ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm1ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm1ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm1ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm1ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm1ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm1ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
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
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl2(pub u16);
impl Sm1ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm1ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm1ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm1ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm1ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm1ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm1ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm1ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm1ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm1ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm1ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm1ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm1ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm1ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm1ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
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
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0(pub u16);
impl Sm1cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
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
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0cyc(pub u16);
impl Sm1cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
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
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1(pub u16);
impl Sm1cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
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
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1cyc(pub u16);
impl Sm1cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
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
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dismap0(pub u16);
impl Sm1dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
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
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dmaen(pub u16);
impl Sm1dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm1dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm1dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm1dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm1dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm1dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
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
            "Sm1dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dtcnt0(pub u16);
impl Sm1dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
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
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dtcnt1(pub u16);
impl Sm1dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
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
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1init(pub u16);
impl Sm1init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
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
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1inten(pub u16);
impl Sm1inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm1intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm1intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
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
            "Sm1inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1octrl(pub u16);
impl Sm1octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm1octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm1octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm1octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm1octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm1octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm1octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm1octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
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
#[doc = "Phase Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1phasedly(pub u16);
impl Sm1phasedly {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
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
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1sts(pub u16);
impl Sm1sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm1stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm1stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
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
            "Sm1sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1tctrl(pub u16);
impl Sm1tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm1tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm1tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm1tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm1tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm1tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm1tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm1tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm1tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm1tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm1tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
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
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val0(pub u16);
impl Sm1val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
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
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val1(pub u16);
impl Sm1val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
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
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val2(pub u16);
impl Sm1val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
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
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val3(pub u16);
impl Sm1val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
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
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val4(pub u16);
impl Sm1val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
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
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val5(pub u16);
impl Sm1val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
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
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompx(pub u16);
impl Sm2captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
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
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrlx(pub u16);
impl Sm2captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm2captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm2captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm2captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm2captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm2captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm2captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm2captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm2captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
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
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfiltx(pub u16);
impl Sm2captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
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
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cnt(pub u16);
impl Sm2cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
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
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl(pub u16);
impl Sm2ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm2ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm2ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm2ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm2ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm2ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm2ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm2ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm2ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm2ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm2ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm2ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
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
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl2(pub u16);
impl Sm2ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm2ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm2ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm2ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm2ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm2ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm2ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm2ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm2ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm2ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm2ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm2ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm2ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm2ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm2ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
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
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0(pub u16);
impl Sm2cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
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
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0cyc(pub u16);
impl Sm2cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
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
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1(pub u16);
impl Sm2cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
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
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1cyc(pub u16);
impl Sm2cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
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
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dismap0(pub u16);
impl Sm2dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
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
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dmaen(pub u16);
impl Sm2dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm2dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm2dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm2dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm2dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm2dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm2dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
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
            "Sm2dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dtcnt0(pub u16);
impl Sm2dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
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
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dtcnt1(pub u16);
impl Sm2dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
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
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2init(pub u16);
impl Sm2init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
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
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2inten(pub u16);
impl Sm2inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm2intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm2intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
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
            "Sm2inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2octrl(pub u16);
impl Sm2octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm2octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm2octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm2octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm2octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm2octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm2octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm2octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
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
#[doc = "Phase Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2phasedly(pub u16);
impl Sm2phasedly {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
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
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2sts(pub u16);
impl Sm2sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm2stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm2stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
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
            "Sm2sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2tctrl(pub u16);
impl Sm2tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm2tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm2tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm2tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm2tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm2tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm2tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm2tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm2tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm2tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm2tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
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
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val0(pub u16);
impl Sm2val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
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
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val1(pub u16);
impl Sm2val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
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
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val2(pub u16);
impl Sm2val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
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
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val3(pub u16);
impl Sm2val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
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
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val4(pub u16);
impl Sm2val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
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
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val5(pub u16);
impl Sm2val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
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
#[doc = "Software Controlled Output Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swcout(pub u16);
impl Swcout {
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out45(&self) -> super::vals::Sm0out45 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sm0out45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: super::vals::Sm0out45) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out23(&self) -> super::vals::Sm0out23 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0out23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: super::vals::Sm0out23) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out45(&self) -> super::vals::Sm1out45 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1out45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: super::vals::Sm1out45) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out23(&self) -> super::vals::Sm1out23 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sm1out23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: super::vals::Sm1out23) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out45(&self) -> super::vals::Sm2out45 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sm2out45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: super::vals::Sm2out45) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out23(&self) -> super::vals::Sm2out23 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sm2out23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: super::vals::Sm2out23) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swcout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swcout {{ sm0out45: {:?}, sm0out23: {:?}, sm1out45: {:?}, sm1out23: {:?}, sm2out45: {:?}, sm2out23: {:?} }}",
            self.sm0out45(),
            self.sm0out23(),
            self.sm1out45(),
            self.sm1out23(),
            self.sm2out45(),
            self.sm2out23()
        )
    }
}
