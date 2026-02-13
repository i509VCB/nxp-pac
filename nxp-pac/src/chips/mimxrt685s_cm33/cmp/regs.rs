#[doc = "CMP Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C0(pub u32);
impl C0 {
    #[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[must_use]
    #[inline(always)]
    pub const fn hystctr(&self) -> super::vals::Hystctr {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Hystctr::from_bits(val as u8)
    }
    #[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline(always)]
    pub const fn set_hystctr(&mut self, val: super::vals::Hystctr) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_cnt(&self) -> super::vals::FilterCnt {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::FilterCnt::from_bits(val as u8)
    }
    #[doc = "Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filter_cnt(&mut self, val: super::vals::FilterCnt) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Comparator Module Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::En {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Comparator Module Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Comparator Output Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ope(&self) -> super::vals::Ope {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ope::from_bits(val as u8)
    }
    #[doc = "Comparator Output Pin Enable"]
    #[inline(always)]
    pub const fn set_ope(&mut self, val: super::vals::Ope) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Comparator Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cos(&self) -> super::vals::Cos {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cos::from_bits(val as u8)
    }
    #[doc = "Comparator Output Select"]
    #[inline(always)]
    pub const fn set_cos(&mut self, val: super::vals::Cos) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Comparator invert"]
    #[must_use]
    #[inline(always)]
    pub const fn invt(&self) -> super::vals::Invt {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Invt::from_bits(val as u8)
    }
    #[doc = "Comparator invert"]
    #[inline(always)]
    pub const fn set_invt(&mut self, val: super::vals::Invt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pmode(&self) -> super::vals::Pmode {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pmode::from_bits(val as u8)
    }
    #[doc = "Power Mode Select"]
    #[inline(always)]
    pub const fn set_pmode(&mut self, val: super::vals::Pmode) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Windowing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> super::vals::We {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::We::from_bits(val as u8)
    }
    #[doc = "Windowing Enable"]
    #[inline(always)]
    pub const fn set_we(&mut self, val: super::vals::We) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Sample Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn se(&self) -> super::vals::Se {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Se::from_bits(val as u8)
    }
    #[doc = "Sample Enable"]
    #[inline(always)]
    pub const fn set_se(&mut self, val: super::vals::Se) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn fpr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Filter Sample Period"]
    #[inline(always)]
    pub const fn set_fpr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Analog Comparator Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cout(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Output"]
    #[inline(always)]
    pub const fn set_cout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Analog Comparator Flag Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn cff(&self) -> super::vals::Cff {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Cff::from_bits(val as u8)
    }
    #[doc = "Analog Comparator Flag Falling"]
    #[inline(always)]
    pub const fn set_cff(&mut self, val: super::vals::Cff) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Analog Comparator Flag Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> super::vals::Cfr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Cfr::from_bits(val as u8)
    }
    #[doc = "Analog Comparator Flag Rising"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: super::vals::Cfr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> super::vals::Ief {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ief::from_bits(val as u8)
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: super::vals::Ief) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn ier(&self) -> super::vals::Ier {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ier::from_bits(val as u8)
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub const fn set_ier(&mut self, val: super::vals::Ier) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "CMP to DAC link enable."]
    #[must_use]
    #[inline(always)]
    pub const fn linken(&self) -> super::vals::Linken {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Linken::from_bits(val as u8)
    }
    #[doc = "CMP to DAC link enable."]
    #[inline(always)]
    pub const fn set_linken(&mut self, val: super::vals::Linken) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for C0 {
    #[inline(always)]
    fn default() -> C0 {
        C0(0)
    }
}
impl core::fmt::Debug for C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C0")
            .field("hystctr", &self.hystctr())
            .field("filter_cnt", &self.filter_cnt())
            .field("en", &self.en())
            .field("ope", &self.ope())
            .field("cos", &self.cos())
            .field("invt", &self.invt())
            .field("pmode", &self.pmode())
            .field("we", &self.we())
            .field("se", &self.se())
            .field("fpr", &self.fpr())
            .field("cout", &self.cout())
            .field("cff", &self.cff())
            .field("cfr", &self.cfr())
            .field("ief", &self.ief())
            .field("ier", &self.ier())
            .field("dmaen", &self.dmaen())
            .field("linken", &self.linken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for C0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "C0 {{ hystctr: {:?}, filter_cnt: {:?}, en: {:?}, ope: {:?}, cos: {:?}, invt: {:?}, pmode: {:?}, we: {:?}, se: {:?}, fpr: {=u8:?}, cout: {=bool:?}, cff: {:?}, cfr: {:?}, ief: {:?}, ier: {:?}, dmaen: {:?}, linken: {:?} }}",
            self.hystctr(),
            self.filter_cnt(),
            self.en(),
            self.ope(),
            self.cos(),
            self.invt(),
            self.pmode(),
            self.we(),
            self.se(),
            self.fpr(),
            self.cout(),
            self.cff(),
            self.cfr(),
            self.ief(),
            self.ier(),
            self.dmaen(),
            self.linken()
        )
    }
}
#[doc = "CMP Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C1(pub u32);
impl C1 {
    #[doc = "DAC Output Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vosel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DAC Output Voltage Select"]
    #[inline(always)]
    pub const fn set_vosel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DAC Mode Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dmode(&self) -> super::vals::Dmode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmode::from_bits(val as u8)
    }
    #[doc = "DAC Mode Selection"]
    #[inline(always)]
    pub const fn set_dmode(&mut self, val: super::vals::Dmode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Supply Voltage Reference Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vrsel(&self) -> super::vals::Vrsel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Vrsel::from_bits(val as u8)
    }
    #[doc = "Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub const fn set_vrsel(&mut self, val: super::vals::Vrsel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DAC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dacen(&self) -> super::vals::Dacen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dacen::from_bits(val as u8)
    }
    #[doc = "DAC Enable"]
    #[inline(always)]
    pub const fn set_dacen(&mut self, val: super::vals::Dacen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 0 input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chn0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 input enable"]
    #[inline(always)]
    pub const fn set_chn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 1 input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chn1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 input enable"]
    #[inline(always)]
    pub const fn set_chn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 2 input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chn2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 input enable"]
    #[inline(always)]
    pub const fn set_chn2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 3 input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chn3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 input enable"]
    #[inline(always)]
    pub const fn set_chn3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Channel 4 input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chn4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 input enable"]
    #[inline(always)]
    pub const fn set_chn4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel 5 input enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chn5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 input enable"]
    #[inline(always)]
    pub const fn set_chn5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Minus Input MUX Control"]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Msel {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Msel::from_bits(val as u8)
    }
    #[doc = "Minus Input MUX Control"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Msel) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Plus Input MUX Control"]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Psel::from_bits(val as u8)
    }
    #[doc = "Plus Input MUX Control"]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for C1 {
    #[inline(always)]
    fn default() -> C1 {
        C1(0)
    }
}
impl core::fmt::Debug for C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1")
            .field("vosel", &self.vosel())
            .field("dmode", &self.dmode())
            .field("vrsel", &self.vrsel())
            .field("dacen", &self.dacen())
            .field("chn0", &self.chn0())
            .field("chn1", &self.chn1())
            .field("chn2", &self.chn2())
            .field("chn3", &self.chn3())
            .field("chn4", &self.chn4())
            .field("chn5", &self.chn5())
            .field("msel", &self.msel())
            .field("psel", &self.psel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for C1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "C1 {{ vosel: {=u8:?}, dmode: {:?}, vrsel: {:?}, dacen: {:?}, chn0: {=bool:?}, chn1: {=bool:?}, chn2: {=bool:?}, chn3: {=bool:?}, chn4: {=bool:?}, chn5: {=bool:?}, msel: {:?}, psel: {:?} }}",
            self.vosel(),
            self.dmode(),
            self.vrsel(),
            self.dacen(),
            self.chn0(),
            self.chn1(),
            self.chn2(),
            self.chn3(),
            self.chn4(),
            self.chn5(),
            self.msel(),
            self.psel()
        )
    }
}
#[doc = "CMP Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C2(pub u32);
impl C2 {
    #[doc = "ACOn"]
    #[must_use]
    #[inline(always)]
    pub const fn acon(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ACOn"]
    #[inline(always)]
    pub const fn set_acon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Comparator and DAC initialization delay modulus."]
    #[must_use]
    #[inline(always)]
    pub const fn initmod(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub const fn set_initmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Number of sample clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn nsam(&self) -> super::vals::Nsam {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Nsam::from_bits(val as u8)
    }
    #[doc = "Number of sample clocks"]
    #[inline(always)]
    pub const fn set_nsam(&mut self, val: super::vals::Nsam) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CH0F"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0f(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CH0F"]
    #[inline(always)]
    pub const fn set_ch0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CH1F"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1f(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CH1F"]
    #[inline(always)]
    pub const fn set_ch1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CH2F"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2f(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CH2F"]
    #[inline(always)]
    pub const fn set_ch2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CH3F"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3f(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "CH3F"]
    #[inline(always)]
    pub const fn set_ch3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CH4F"]
    #[must_use]
    #[inline(always)]
    pub const fn ch4f(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CH4F"]
    #[inline(always)]
    pub const fn set_ch4f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CH5F"]
    #[must_use]
    #[inline(always)]
    pub const fn ch5f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CH5F"]
    #[inline(always)]
    pub const fn set_ch5f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Fixed channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fxmxch(&self) -> super::vals::Fxmxch {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Fxmxch::from_bits(val as u8)
    }
    #[doc = "Fixed channel selection"]
    #[inline(always)]
    pub const fn set_fxmxch(&mut self, val: super::vals::Fxmxch) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
    #[doc = "Fixed MUX Port"]
    #[must_use]
    #[inline(always)]
    pub const fn fxmp(&self) -> super::vals::Fxmp {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Fxmp::from_bits(val as u8)
    }
    #[doc = "Fixed MUX Port"]
    #[inline(always)]
    pub const fn set_fxmp(&mut self, val: super::vals::Fxmp) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Round-Robin interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rrie(&self) -> super::vals::Rrie {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Rrie::from_bits(val as u8)
    }
    #[doc = "Round-Robin interrupt enable"]
    #[inline(always)]
    pub const fn set_rrie(&mut self, val: super::vals::Rrie) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for C2 {
    #[inline(always)]
    fn default() -> C2 {
        C2(0)
    }
}
impl core::fmt::Debug for C2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2")
            .field("acon", &self.acon())
            .field("initmod", &self.initmod())
            .field("nsam", &self.nsam())
            .field("ch0f", &self.ch0f())
            .field("ch1f", &self.ch1f())
            .field("ch2f", &self.ch2f())
            .field("ch3f", &self.ch3f())
            .field("ch4f", &self.ch4f())
            .field("ch5f", &self.ch5f())
            .field("fxmxch", &self.fxmxch())
            .field("fxmp", &self.fxmp())
            .field("rrie", &self.rrie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for C2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "C2 {{ acon: {=u8:?}, initmod: {=u8:?}, nsam: {:?}, ch0f: {=bool:?}, ch1f: {=bool:?}, ch2f: {=bool:?}, ch3f: {=bool:?}, ch4f: {=bool:?}, ch5f: {=bool:?}, fxmxch: {:?}, fxmp: {:?}, rrie: {:?} }}",
            self.acon(),
            self.initmod(),
            self.nsam(),
            self.ch0f(),
            self.ch1f(),
            self.ch2f(),
            self.ch3f(),
            self.ch4f(),
            self.ch5f(),
            self.fxmxch(),
            self.fxmp(),
            self.rrie()
        )
    }
}
#[doc = "CMP Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C3(pub u32);
impl C3 {
    #[doc = "Analog Comparator Phase2 Timing Control."]
    #[must_use]
    #[inline(always)]
    pub const fn acph2tc(&self) -> super::vals::Acph2tc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Acph2tc::from_bits(val as u8)
    }
    #[doc = "Analog Comparator Phase2 Timing Control."]
    #[inline(always)]
    pub const fn set_acph2tc(&mut self, val: super::vals::Acph2tc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Analog Comparator Phase1 Timing Control."]
    #[must_use]
    #[inline(always)]
    pub const fn acph1tc(&self) -> super::vals::Acph1tc {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Acph1tc::from_bits(val as u8)
    }
    #[doc = "Analog Comparator Phase1 Timing Control."]
    #[inline(always)]
    pub const fn set_acph1tc(&mut self, val: super::vals::Acph1tc) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Analog Comparator Sampling Time control."]
    #[must_use]
    #[inline(always)]
    pub const fn acsat(&self) -> super::vals::Acsat {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Acsat::from_bits(val as u8)
    }
    #[doc = "Analog Comparator Sampling Time control."]
    #[inline(always)]
    pub const fn set_acsat(&mut self, val: super::vals::Acsat) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Discrete Mode Clock Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dmcs(&self) -> super::vals::Dmcs {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmcs::from_bits(val as u8)
    }
    #[doc = "Discrete Mode Clock Selection"]
    #[inline(always)]
    pub const fn set_dmcs(&mut self, val: super::vals::Dmcs) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Resistor Divider Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdive(&self) -> super::vals::Rdive {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Rdive::from_bits(val as u8)
    }
    #[doc = "Resistor Divider Enable"]
    #[inline(always)]
    pub const fn set_rdive(&mut self, val: super::vals::Rdive) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Negative Channel Continuous Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nchcten(&self) -> super::vals::Nchcten {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Nchcten::from_bits(val as u8)
    }
    #[doc = "Negative Channel Continuous Mode Enable."]
    #[inline(always)]
    pub const fn set_nchcten(&mut self, val: super::vals::Nchcten) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Positive Channel Continuous Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pchcten(&self) -> super::vals::Pchcten {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pchcten::from_bits(val as u8)
    }
    #[doc = "Positive Channel Continuous Mode Enable."]
    #[inline(always)]
    pub const fn set_pchcten(&mut self, val: super::vals::Pchcten) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for C3 {
    #[inline(always)]
    fn default() -> C3 {
        C3(0)
    }
}
impl core::fmt::Debug for C3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3")
            .field("acph2tc", &self.acph2tc())
            .field("acph1tc", &self.acph1tc())
            .field("acsat", &self.acsat())
            .field("dmcs", &self.dmcs())
            .field("rdive", &self.rdive())
            .field("nchcten", &self.nchcten())
            .field("pchcten", &self.pchcten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for C3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "C3 {{ acph2tc: {:?}, acph1tc: {:?}, acsat: {:?}, dmcs: {:?}, rdive: {:?}, nchcten: {:?}, pchcten: {:?} }}",
            self.acph2tc(),
            self.acph1tc(),
            self.acsat(),
            self.dmcs(),
            self.rdive(),
            self.nchcten(),
            self.pchcten()
        )
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
    #[must_use]
    #[inline(always)]
    pub const fn param(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
    #[inline(always)]
    pub const fn set_param(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("param", &self.param())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ param: {=u32:?} }}", self.param())
    }
}
#[doc = "Round-Robin Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RrTimerCr(pub u32);
impl RrTimerCr {
    #[doc = "This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_timer_reload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
    #[inline(always)]
    pub const fn set_rr_timer_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "RR_TIMER enable. When low, rr_timer count will be held at zero. When set, timer will commence continuous, repetitive counting beginning with the 1st or 2nd rising edge of the 32 KHz rr_clock.1"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_timer_ena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RR_TIMER enable. When low, rr_timer count will be held at zero. When set, timer will commence continuous, repetitive counting beginning with the 1st or 2nd rising edge of the 32 KHz rr_clock.1"]
    #[inline(always)]
    pub const fn set_rr_timer_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RrTimerCr {
    #[inline(always)]
    fn default() -> RrTimerCr {
        RrTimerCr(0)
    }
}
impl core::fmt::Debug for RrTimerCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RrTimerCr")
            .field("rr_timer_reload", &self.rr_timer_reload())
            .field("rr_timer_ena", &self.rr_timer_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RrTimerCr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RrTimerCr {{ rr_timer_reload: {=u32:?}, rr_timer_ena: {=bool:?} }}",
            self.rr_timer_reload(),
            self.rr_timer_ena()
        )
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number. This read only filed returns the feature set number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number. This read only filed returns the feature set number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number. This read only field returns the minor version number for the module specification."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number. This read only field returns the minor version number for the module specification."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number. This read only field returns the major version number for the module specification."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number. This read only field returns the major version number for the module specification."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
