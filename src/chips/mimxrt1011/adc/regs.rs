#[doc = "Calibration value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cal(pub u32);
impl Cal {
    #[doc = "Calibration Result Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_code(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Calibration Result Value"]
    #[inline(always)]
    pub const fn set_cal_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Cal {
    #[inline(always)]
    fn default() -> Cal {
        Cal(0)
    }
}
impl core::fmt::Debug for Cal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cal")
            .field("cal_code", &self.cal_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cal {{ cal_code: {=u8:?} }}", self.cal_code())
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Input Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adiclk(&self) -> super::vals::Adiclk {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Adiclk::from_bits(val as u8)
    }
    #[doc = "Input Clock Select"]
    #[inline(always)]
    pub const fn set_adiclk(&mut self, val: super::vals::Adiclk) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Conversion Mode Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Conversion Mode Selection"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Long Sample Time Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn adlsmp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Long Sample Time Configuration"]
    #[inline(always)]
    pub const fn set_adlsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clock Divide Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adiv(&self) -> super::vals::Adiv {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Adiv::from_bits(val as u8)
    }
    #[doc = "Clock Divide Select"]
    #[inline(always)]
    pub const fn set_adiv(&mut self, val: super::vals::Adiv) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Low-Power Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn adlpc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Configuration"]
    #[inline(always)]
    pub const fn set_adlpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Defines the total sample time duration in number of full cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn adsts(&self) -> super::vals::Adsts {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Adsts::from_bits(val as u8)
    }
    #[doc = "Defines the total sample time duration in number of full cycles"]
    #[inline(always)]
    pub const fn set_adsts(&mut self, val: super::vals::Adsts) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Speed Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn adhsc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Configuration"]
    #[inline(always)]
    pub const fn set_adhsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Voltage Reference Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Voltage Reference Selection"]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "Conversion Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adtrg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Trigger Select"]
    #[inline(always)]
    pub const fn set_adtrg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Hardware Average select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Avgs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::Avgs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Data Overwrite Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovwren(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Data Overwrite Enable"]
    #[inline(always)]
    pub const fn set_ovwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("adiclk", &self.adiclk())
            .field("mode", &self.mode())
            .field("adlsmp", &self.adlsmp())
            .field("adiv", &self.adiv())
            .field("adlpc", &self.adlpc())
            .field("adsts", &self.adsts())
            .field("adhsc", &self.adhsc())
            .field("refsel", &self.refsel())
            .field("adtrg", &self.adtrg())
            .field("avgs", &self.avgs())
            .field("ovwren", &self.ovwren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ adiclk: {:?}, mode: {:?}, adlsmp: {=bool:?}, adiv: {:?}, adlpc: {=bool:?}, adsts: {:?}, adhsc: {=bool:?}, refsel: {:?}, adtrg: {=bool:?}, avgs: {:?}, ovwren: {=bool:?} }}",
            self.adiclk(),
            self.mode(),
            self.adlsmp(),
            self.adiv(),
            self.adlpc(),
            self.adsts(),
            self.adhsc(),
            self.refsel(),
            self.adtrg(),
            self.avgs(),
            self.ovwren()
        )
    }
}
#[doc = "Compare value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc = "Compare Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cv1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Compare Value 1"]
    #[inline(always)]
    pub const fn set_cv1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Compare Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cv2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Compare Value 2"]
    #[inline(always)]
    pub const fn set_cv2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Cv {
    #[inline(always)]
    fn default() -> Cv {
        Cv(0)
    }
}
impl core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cv")
            .field("cv1", &self.cv1())
            .field("cv2", &self.cv2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cv {{ cv1: {=u16:?}, cv2: {=u16:?} }}",
            self.cv1(),
            self.cv2()
        )
    }
}
#[doc = "General control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gc(pub u32);
impl Gc {
    #[doc = "Asynchronous clock output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adacken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous clock output enable"]
    #[inline(always)]
    pub const fn set_adacken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Compare Function Range Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn acren(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Function Range Enable"]
    #[inline(always)]
    pub const fn set_acren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare Function Greater Than Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn acfgt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Function Greater Than Enable"]
    #[inline(always)]
    pub const fn set_acfgt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn acfe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_acfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Hardware average enable"]
    #[must_use]
    #[inline(always)]
    pub const fn avge(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware average enable"]
    #[inline(always)]
    pub const fn set_avge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Continuous Conversion Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adco(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Conversion Enable"]
    #[inline(always)]
    pub const fn set_adco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Calibration"]
    #[must_use]
    #[inline(always)]
    pub const fn cal(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration"]
    #[inline(always)]
    pub const fn set_cal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Gc {
    #[inline(always)]
    fn default() -> Gc {
        Gc(0)
    }
}
impl core::fmt::Debug for Gc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gc")
            .field("adacken", &self.adacken())
            .field("dmaen", &self.dmaen())
            .field("acren", &self.acren())
            .field("acfgt", &self.acfgt())
            .field("acfe", &self.acfe())
            .field("avge", &self.avge())
            .field("adco", &self.adco())
            .field("cal", &self.cal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gc {{ adacken: {=bool:?}, dmaen: {=bool:?}, acren: {=bool:?}, acfgt: {=bool:?}, acfe: {=bool:?}, avge: {=bool:?}, adco: {=bool:?}, cal: {=bool:?} }}",
            self.adacken(),
            self.dmaen(),
            self.acren(),
            self.acfgt(),
            self.acfe(),
            self.avge(),
            self.adco(),
            self.cal()
        )
    }
}
#[doc = "General status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gs(pub u32);
impl Gs {
    #[doc = "Conversion Active"]
    #[must_use]
    #[inline(always)]
    pub const fn adact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Active"]
    #[inline(always)]
    pub const fn set_adact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Calibration Failed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn calf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Failed Flag"]
    #[inline(always)]
    pub const fn set_calf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Asynchronous wakeup interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn awkst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous wakeup interrupt status"]
    #[inline(always)]
    pub const fn set_awkst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Gs {
    #[inline(always)]
    fn default() -> Gs {
        Gs(0)
    }
}
impl core::fmt::Debug for Gs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gs")
            .field("adact", &self.adact())
            .field("calf", &self.calf())
            .field("awkst", &self.awkst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gs {{ adact: {=bool:?}, calf: {=bool:?}, awkst: {=bool:?} }}",
            self.adact(),
            self.calf(),
            self.awkst()
        )
    }
}
#[doc = "Control register for hardware triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc(pub u32);
impl Hc {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::HcAdch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::HcAdch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::HcAdch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Complete Interrupt Enable/Disable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn aien(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Complete Interrupt Enable/Disable Control"]
    #[inline(always)]
    pub const fn set_aien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Hc {
    #[inline(always)]
    fn default() -> Hc {
        Hc(0)
    }
}
impl core::fmt::Debug for Hc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc")
            .field("adch", &self.adch())
            .field("aien", &self.aien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hc {{ adch: {:?}, aien: {=bool:?} }}",
            self.adch(),
            self.aien()
        )
    }
}
#[doc = "Control register for hardware triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hc0(pub u32);
impl Hc0 {
    #[doc = "Input Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Hc0Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Hc0Adch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::Hc0Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Complete Interrupt Enable/Disable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn aien(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Conversion Complete Interrupt Enable/Disable Control"]
    #[inline(always)]
    pub const fn set_aien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Hc0 {
    #[inline(always)]
    fn default() -> Hc0 {
        Hc0(0)
    }
}
impl core::fmt::Debug for Hc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hc0")
            .field("adch", &self.adch())
            .field("aien", &self.aien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hc0 {{ adch: {:?}, aien: {=bool:?} }}",
            self.adch(),
            self.aien()
        )
    }
}
#[doc = "Status register for HW triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hs(pub u32);
impl Hs {
    #[doc = "Conversion Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn coco(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Conversion Complete Flag"]
    #[inline(always)]
    pub const fn set_coco(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Hs {
    #[inline(always)]
    fn default() -> Hs {
        Hs(0)
    }
}
impl core::fmt::Debug for Hs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hs")
            .field("coco[0]", &self.coco(0usize))
            .field("coco[1]", &self.coco(1usize))
            .field("coco[2]", &self.coco(2usize))
            .field("coco[3]", &self.coco(3usize))
            .field("coco[4]", &self.coco(4usize))
            .field("coco[5]", &self.coco(5usize))
            .field("coco[6]", &self.coco(6usize))
            .field("coco[7]", &self.coco(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hs {{ coco[0]: {=bool:?}, coco[1]: {=bool:?}, coco[2]: {=bool:?}, coco[3]: {=bool:?}, coco[4]: {=bool:?}, coco[5]: {=bool:?}, coco[6]: {=bool:?}, coco[7]: {=bool:?} }}",
            self.coco(0usize),
            self.coco(1usize),
            self.coco(2usize),
            self.coco(3usize),
            self.coco(4usize),
            self.coco(5usize),
            self.coco(6usize),
            self.coco(7usize)
        )
    }
}
#[doc = "Offset correction value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofs(pub u32);
impl Ofs {
    #[doc = "Offset value"]
    #[must_use]
    #[inline(always)]
    pub const fn ofs(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset value"]
    #[inline(always)]
    pub const fn set_ofs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Sign bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sign(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Sign bit"]
    #[inline(always)]
    pub const fn set_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Ofs {
    #[inline(always)]
    fn default() -> Ofs {
        Ofs(0)
    }
}
impl core::fmt::Debug for Ofs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ofs")
            .field("ofs", &self.ofs())
            .field("sign", &self.sign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ofs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ofs {{ ofs: {=u16:?}, sign: {=bool:?} }}",
            self.ofs(),
            self.sign()
        )
    }
}
#[doc = "Data result register for HW triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R(pub u32);
impl R {
    #[doc = "Data (result of an ADC conversion)"]
    #[must_use]
    #[inline(always)]
    pub const fn cdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Data (result of an ADC conversion)"]
    #[inline(always)]
    pub const fn set_cdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for R {
    #[inline(always)]
    fn default() -> R {
        R(0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R").field("cdata", &self.cdata()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "R {{ cdata: {=u16:?} }}", self.cdata())
    }
}
#[doc = "Data result register for HW triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R0(pub u32);
impl R0 {
    #[doc = "Data (result of an ADC conversion)"]
    #[must_use]
    #[inline(always)]
    pub const fn cdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Data (result of an ADC conversion)"]
    #[inline(always)]
    pub const fn set_cdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for R0 {
    #[inline(always)]
    fn default() -> R0 {
        R0(0)
    }
}
impl core::fmt::Debug for R0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R0").field("cdata", &self.cdata()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "R0 {{ cdata: {=u16:?} }}", self.cdata())
    }
}
