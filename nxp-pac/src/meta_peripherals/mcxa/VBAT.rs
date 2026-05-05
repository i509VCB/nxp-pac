#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "VBAT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbat {
    ptr: *mut u8,
}
unsafe impl Send for Vbat {}
unsafe impl Sync for Vbat {}
impl Vbat {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status A."]
    #[inline(always)]
    pub const fn statusa(self) -> crate::pac::common::Reg<Statusa, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Enable A."]
    #[inline(always)]
    pub const fn irqena(self) -> crate::pac::common::Reg<Irqena, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Wake-up Enable A."]
    #[inline(always)]
    pub const fn wakena(self) -> crate::pac::common::Reg<Wakena, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Wake-up Configuration."]
    #[inline(always)]
    pub const fn wakecfg(self) -> crate::pac::common::Reg<Wakecfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Oscillator Control A."]
    #[inline(always)]
    pub const fn oscctla(self) -> crate::pac::common::Reg<Oscctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Oscillator Configuration A."]
    #[inline(always)]
    pub const fn osccfga(self) -> crate::pac::common::Reg<Osccfga, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Oscillator Test A."]
    #[inline(always)]
    pub const fn osctsta(self) -> crate::pac::common::Reg<Osctsta, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Oscillator Lock A."]
    #[inline(always)]
    pub const fn osclcka(self) -> crate::pac::common::Reg<Osclcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Oscillator Clock Enable."]
    #[inline(always)]
    pub const fn oscclke(self) -> crate::pac::common::Reg<Oscclke, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "FRO16K Control A."]
    #[inline(always)]
    pub const fn froctla(self) -> crate::pac::common::Reg<Froctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FRO16K Configuration A."]
    #[inline(always)]
    pub const fn frocfga(self) -> crate::pac::common::Reg<Frocfga, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "FRO16K Test A."]
    #[inline(always)]
    pub const fn frotsta(self) -> crate::pac::common::Reg<Frotsta, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "FRO16K Lock A."]
    #[inline(always)]
    pub const fn frolcka(self) -> crate::pac::common::Reg<Frolcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FRO16K Clock Enable."]
    #[inline(always)]
    pub const fn froclke(self) -> crate::pac::common::Reg<Froclke, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "LDO_RAM Control A."]
    #[inline(always)]
    pub const fn ldoctla(self) -> crate::pac::common::Reg<Ldoctla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "LDO_RAM Configuration A."]
    #[inline(always)]
    pub const fn ldocfga(self) -> crate::pac::common::Reg<Ldocfga, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "LDO_RAM Test A."]
    #[inline(always)]
    pub const fn ldotsta(self) -> crate::pac::common::Reg<Ldotsta, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "LDO_RAM Lock A."]
    #[inline(always)]
    pub const fn ldolcka(self) -> crate::pac::common::Reg<Ldolcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "RAM Control."]
    #[inline(always)]
    pub const fn ldoramc(self) -> crate::pac::common::Reg<Ldoramc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Bandgap Timer 0."]
    #[inline(always)]
    pub const fn ldotimer0(self) -> crate::pac::common::Reg<Ldotimer0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Bandgap Timer 1."]
    #[inline(always)]
    pub const fn ldotimer1(self) -> crate::pac::common::Reg<Ldotimer1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Switch Control A."]
    #[inline(always)]
    pub const fn swictla(self) -> crate::pac::common::Reg<Swictla, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Switch Lock A."]
    #[inline(always)]
    pub const fn swilcka(self) -> crate::pac::common::Reg<Swilcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Array of registers: WAKEUPA."]
    #[inline(always)]
    pub const fn wakeup(self, n: usize) -> Wakeup {
        assert!(n < 2usize);
        unsafe { Wakeup::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 8usize) as _) }
    }
    #[doc = "Wakeup Lock A."]
    #[inline(always)]
    pub const fn waklcka(self) -> crate::pac::common::Reg<Waklcka, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
}
#[doc = "Array of registers: WAKEUPA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeup {
    ptr: *mut u8,
}
unsafe impl Send for Wakeup {}
unsafe impl Sync for Wakeup {}
impl Wakeup {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Wakeup 0 Register A."]
    #[inline(always)]
    pub const fn wakeupa(self) -> crate::pac::common::Reg<Wakeupa, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "FRO16K Configuration A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frocfga(pub u32);
impl Frocfga {
    #[doc = "Frequency Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency Trim."]
    #[inline(always)]
    pub const fn set_freq_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Temperature Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn temp_trim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Temperature Trim."]
    #[inline(always)]
    pub const fn set_temp_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Frocfga {
    #[inline(always)]
    fn default() -> Frocfga {
        Frocfga(0)
    }
}
impl core::fmt::Debug for Frocfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frocfga")
            .field("freq_trim", &self.freq_trim())
            .field("temp_trim", &self.temp_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frocfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frocfga {{ freq_trim: {=u8:?}, temp_trim: {=u8:?} }}",
            self.freq_trim(),
            self.temp_trim()
        )
    }
}
#[doc = "FRO16K Clock Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froclke(pub u32);
impl Froclke {
    #[doc = "Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock Enable."]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Froclke {
    #[inline(always)]
    fn default() -> Froclke {
        Froclke(0)
    }
}
impl core::fmt::Debug for Froclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "FRO16K Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctla(pub u32);
impl Froctla {
    #[doc = "FRO16K Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fro_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FRO16K Enable."]
    #[inline(always)]
    pub const fn set_fro_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Froctla {
    #[inline(always)]
    fn default() -> Froctla {
        Froctla(0)
    }
}
impl core::fmt::Debug for Froctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froctla")
            .field("fro_en", &self.fro_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froctla {{ fro_en: {=bool:?} }}", self.fro_en())
    }
}
#[doc = "FRO16K Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolcka(pub u32);
impl Frolcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Frolcka {
    #[inline(always)]
    fn default() -> Frolcka {
        Frolcka(0)
    }
}
impl core::fmt::Debug for Frolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "FRO16K Test A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frotsta(pub u32);
impl Frotsta {
    #[doc = "Test Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn tstmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Test Mode."]
    #[inline(always)]
    pub const fn set_tstmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Frotsta {
    #[inline(always)]
    fn default() -> Frotsta {
        Frotsta(0)
    }
}
impl core::fmt::Debug for Frotsta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frotsta")
            .field("tstmode", &self.tstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frotsta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frotsta {{ tstmode: {=bool:?} }}", self.tstmode())
    }
}
#[doc = "Interrupt Enable A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqena(pub u32);
impl Irqena {
    #[doc = "POR Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> IrqenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        IrqenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag."]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: IrqenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0."]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2."]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready."]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32k Ready."]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Irqena {
    #[inline(always)]
    fn default() -> Irqena {
        Irqena(0)
    }
}
impl core::fmt::Debug for Irqena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqena")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy()
        )
    }
}
#[doc = "LDO_RAM Configuration A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldocfga(pub u32);
impl Ldocfga {
    #[doc = "LDO Voltage Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "LDO Voltage Trim."]
    #[inline(always)]
    pub const fn set_ldo_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Bandgap Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn bg_2x2_trim(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Bandgap Trim."]
    #[inline(always)]
    pub const fn set_bg_2x2_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Bandgap Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn bg_5x5_trim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap Trim."]
    #[inline(always)]
    pub const fn set_bg_5x5_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Refresh Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn refresh_trim(&self) -> RefreshTrim {
        let val = (self.0 >> 10usize) & 0x03;
        RefreshTrim::from_bits(val as u8)
    }
    #[doc = "Refresh Trim."]
    #[inline(always)]
    pub const fn set_refresh_trim(&mut self, val: RefreshTrim) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for Ldocfga {
    #[inline(always)]
    fn default() -> Ldocfga {
        Ldocfga(0)
    }
}
impl core::fmt::Debug for Ldocfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldocfga")
            .field("ldo_trim", &self.ldo_trim())
            .field("bg_2x2_trim", &self.bg_2x2_trim())
            .field("bg_5x5_trim", &self.bg_5x5_trim())
            .field("refresh_trim", &self.refresh_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldocfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldocfga {{ ldo_trim: {=u8:?}, bg_2x2_trim: {=u8:?}, bg_5x5_trim: {=u8:?}, refresh_trim: {:?} }}",
            self.ldo_trim(),
            self.bg_2x2_trim(),
            self.bg_5x5_trim(),
            self.refresh_trim()
        )
    }
}
#[doc = "LDO_RAM Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoctla(pub u32);
impl Ldoctla {
    #[doc = "Bandgap Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Enable."]
    #[inline(always)]
    pub const fn set_bg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable."]
    #[inline(always)]
    pub const fn set_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refresh Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn refresh_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refresh Enable."]
    #[inline(always)]
    pub const fn set_refresh_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ldoctla {
    #[inline(always)]
    fn default() -> Ldoctla {
        Ldoctla(0)
    }
}
impl core::fmt::Debug for Ldoctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoctla")
            .field("bg_en", &self.bg_en())
            .field("ldo_en", &self.ldo_en())
            .field("refresh_en", &self.refresh_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldoctla {{ bg_en: {=bool:?}, ldo_en: {=bool:?}, refresh_en: {=bool:?} }}",
            self.bg_en(),
            self.ldo_en(),
            self.refresh_en()
        )
    }
}
#[doc = "LDO_RAM Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldolcka(pub u32);
impl Ldolcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ldolcka {
    #[inline(always)]
    fn default() -> Ldolcka {
        Ldolcka(0)
    }
}
impl core::fmt::Debug for Ldolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "RAM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoramc(pub u32);
impl Ldoramc {
    #[doc = "Isolate SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Isolate SRAM."]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Switch SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn swi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Switch SRAM."]
    #[inline(always)]
    pub const fn set_swi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retention."]
    #[must_use]
    #[inline(always)]
    pub const fn ret0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Retention."]
    #[inline(always)]
    pub const fn set_ret0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Retention."]
    #[must_use]
    #[inline(always)]
    pub const fn ret1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Retention."]
    #[inline(always)]
    pub const fn set_ret1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Retention."]
    #[must_use]
    #[inline(always)]
    pub const fn ret2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Retention."]
    #[inline(always)]
    pub const fn set_ret2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Retention."]
    #[must_use]
    #[inline(always)]
    pub const fn ret3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Retention."]
    #[inline(always)]
    pub const fn set_ret3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Ldoramc {
    #[inline(always)]
    fn default() -> Ldoramc {
        Ldoramc(0)
    }
}
impl core::fmt::Debug for Ldoramc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoramc")
            .field("iso", &self.iso())
            .field("swi", &self.swi())
            .field("ret0", &self.ret0())
            .field("ret1", &self.ret1())
            .field("ret2", &self.ret2())
            .field("ret3", &self.ret3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoramc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldoramc {{ iso: {=bool:?}, swi: {=bool:?}, ret0: {=bool:?}, ret1: {=bool:?}, ret2: {=bool:?}, ret3: {=bool:?} }}",
            self.iso(),
            self.swi(),
            self.ret0(),
            self.ret1(),
            self.ret2(),
            self.ret3()
        )
    }
}
#[doc = "Bandgap Timer 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer0(pub u32);
impl Ldotimer0 {
    #[doc = "Timeout Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> Timcfg {
        let val = (self.0 >> 0usize) & 0x07;
        Timcfg::from_bits(val as u8)
    }
    #[doc = "Timeout Configuration."]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: Timcfg) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[inline(always)]
    pub const fn set_timen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldotimer0 {
    #[inline(always)]
    fn default() -> Ldotimer0 {
        Ldotimer0(0)
    }
}
impl core::fmt::Debug for Ldotimer0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotimer0")
            .field("timcfg", &self.timcfg())
            .field("timen", &self.timen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotimer0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotimer0 {{ timcfg: {:?}, timen: {=bool:?} }}",
            self.timcfg(),
            self.timen()
        )
    }
}
#[doc = "Bandgap Timer 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer1(pub u32);
impl Ldotimer1 {
    #[doc = "Timeout Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timeout Configuration."]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable."]
    #[inline(always)]
    pub const fn set_timen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldotimer1 {
    #[inline(always)]
    fn default() -> Ldotimer1 {
        Ldotimer1(0)
    }
}
impl core::fmt::Debug for Ldotimer1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotimer1")
            .field("timcfg", &self.timcfg())
            .field("timen", &self.timen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotimer1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotimer1 {{ timcfg: {=u32:?}, timen: {=bool:?} }}",
            self.timcfg(),
            self.timen()
        )
    }
}
#[doc = "LDO_RAM Test A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotsta(pub u32);
impl Ldotsta {
    #[doc = "Test Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn tstmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Test Mode."]
    #[inline(always)]
    pub const fn set_tstmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Spare Control."]
    #[must_use]
    #[inline(always)]
    pub const fn control(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Spare Control."]
    #[inline(always)]
    pub const fn set_control(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Discharge Switch."]
    #[must_use]
    #[inline(always)]
    pub const fn dischg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Discharge Switch."]
    #[inline(always)]
    pub const fn set_dischg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Spare Status."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Spare Status."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ldotsta {
    #[inline(always)]
    fn default() -> Ldotsta {
        Ldotsta(0)
    }
}
impl core::fmt::Debug for Ldotsta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotsta")
            .field("tstmode", &self.tstmode())
            .field("control", &self.control())
            .field("dischg", &self.dischg())
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotsta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotsta {{ tstmode: {=u8:?}, control: {=u8:?}, dischg: {=bool:?}, status: {=u8:?} }}",
            self.tstmode(),
            self.control(),
            self.dischg(),
            self.status()
        )
    }
}
#[doc = "Oscillator Configuration A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccfga(pub u32);
impl Osccfga {
    #[doc = "Comparator Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_trim(&self) -> CmpTrim {
        let val = (self.0 >> 0usize) & 0x03;
        CmpTrim::from_bits(val as u8)
    }
    #[doc = "Comparator Trim."]
    #[inline(always)]
    pub const fn set_cmp_trim(&mut self, val: CmpTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAP2_TRIM."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2_trim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CAP2_TRIM."]
    #[inline(always)]
    pub const fn set_cap2_trim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Delay Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn dly_trim(&self) -> DlyTrim {
        let val = (self.0 >> 3usize) & 0x0f;
        DlyTrim::from_bits(val as u8)
    }
    #[doc = "Delay Trim."]
    #[inline(always)]
    pub const fn set_dly_trim(&mut self, val: DlyTrim) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "Capacitor Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_trim(&self) -> CapTrim {
        let val = (self.0 >> 7usize) & 0x03;
        CapTrim::from_bits(val as u8)
    }
    #[doc = "Capacitor Trim."]
    #[inline(always)]
    pub const fn set_cap_trim(&mut self, val: CapTrim) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "Initialization Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn init_trim(&self) -> InitTrim {
        let val = (self.0 >> 9usize) & 0x07;
        InitTrim::from_bits(val as u8)
    }
    #[doc = "Initialization Trim."]
    #[inline(always)]
    pub const fn set_init_trim(&mut self, val: InitTrim) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
}
impl Default for Osccfga {
    #[inline(always)]
    fn default() -> Osccfga {
        Osccfga(0)
    }
}
impl core::fmt::Debug for Osccfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osccfga")
            .field("cmp_trim", &self.cmp_trim())
            .field("cap2_trim", &self.cap2_trim())
            .field("dly_trim", &self.dly_trim())
            .field("cap_trim", &self.cap_trim())
            .field("init_trim", &self.init_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osccfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osccfga {{ cmp_trim: {:?}, cap2_trim: {=bool:?}, dly_trim: {:?}, cap_trim: {:?}, init_trim: {:?} }}",
            self.cmp_trim(),
            self.cap2_trim(),
            self.dly_trim(),
            self.cap_trim(),
            self.init_trim()
        )
    }
}
#[doc = "Oscillator Clock Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscclke(pub u32);
impl Oscclke {
    #[doc = "Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock Enable."]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Oscclke {
    #[inline(always)]
    fn default() -> Oscclke {
        Oscclke(0)
    }
}
impl core::fmt::Debug for Oscclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oscclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "Oscillator Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscctla(pub u32);
impl Oscctla {
    #[doc = "Crystal Oscillator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Enable."]
    #[inline(always)]
    pub const fn set_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Crystal Oscillator Bypass Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_byp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Bypass Enable."]
    #[inline(always)]
    pub const fn set_osc_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Amplifier Gain Coarse Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_amp_gain(&self) -> CoarseAmpGain {
        let val = (self.0 >> 2usize) & 0x03;
        CoarseAmpGain::from_bits(val as u8)
    }
    #[doc = "Amplifier Gain Coarse Adjustment."]
    #[inline(always)]
    pub const fn set_coarse_amp_gain(&mut self, val: CoarseAmpGain) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Amplifier Gain Fine Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn fine_amp_gain(&self) -> FineAmpGain {
        let val = (self.0 >> 4usize) & 0x03;
        FineAmpGain::from_bits(val as u8)
    }
    #[doc = "Amplifier Gain Fine Adjustment."]
    #[inline(always)]
    pub const fn set_fine_amp_gain(&mut self, val: FineAmpGain) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Output Hysteresis Select."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_sel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Output Hysteresis Select."]
    #[inline(always)]
    pub const fn set_hyst_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Crystal Load Capacitance Selection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_sel_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Load Capacitance Selection Enable."]
    #[inline(always)]
    pub const fn set_cap_sel_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn extal_cap_sel(&self) -> ExtalCapSel {
        let val = (self.0 >> 8usize) & 0x0f;
        ExtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[inline(always)]
    pub const fn set_extal_cap_sel(&mut self, val: ExtalCapSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_cap_sel(&self) -> XtalCapSel {
        let val = (self.0 >> 12usize) & 0x0f;
        XtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection."]
    #[inline(always)]
    pub const fn set_xtal_cap_sel(&mut self, val: XtalCapSel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mode_en(&self) -> ModeEn {
        let val = (self.0 >> 16usize) & 0x03;
        ModeEn::from_bits(val as u8)
    }
    #[doc = "Mode Enable."]
    #[inline(always)]
    pub const fn set_mode_en(&mut self, val: ModeEn) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Supply Detector Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn supply_det(&self) -> SupplyDet {
        let val = (self.0 >> 18usize) & 0x03;
        SupplyDet::from_bits(val as u8)
    }
    #[doc = "Supply Detector Trim."]
    #[inline(always)]
    pub const fn set_supply_det(&mut self, val: SupplyDet) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
}
impl Default for Oscctla {
    #[inline(always)]
    fn default() -> Oscctla {
        Oscctla(0)
    }
}
impl core::fmt::Debug for Oscctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscctla")
            .field("osc_en", &self.osc_en())
            .field("osc_byp_en", &self.osc_byp_en())
            .field("coarse_amp_gain", &self.coarse_amp_gain())
            .field("fine_amp_gain", &self.fine_amp_gain())
            .field("hyst_sel", &self.hyst_sel())
            .field("cap_sel_en", &self.cap_sel_en())
            .field("extal_cap_sel", &self.extal_cap_sel())
            .field("xtal_cap_sel", &self.xtal_cap_sel())
            .field("mode_en", &self.mode_en())
            .field("supply_det", &self.supply_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Oscctla {{ osc_en: {=bool:?}, osc_byp_en: {=bool:?}, coarse_amp_gain: {:?}, fine_amp_gain: {:?}, hyst_sel: {=bool:?}, cap_sel_en: {=bool:?}, extal_cap_sel: {:?}, xtal_cap_sel: {:?}, mode_en: {:?}, supply_det: {:?} }}",
            self.osc_en(),
            self.osc_byp_en(),
            self.coarse_amp_gain(),
            self.fine_amp_gain(),
            self.hyst_sel(),
            self.cap_sel_en(),
            self.extal_cap_sel(),
            self.xtal_cap_sel(),
            self.mode_en(),
            self.supply_det()
        )
    }
}
#[doc = "Oscillator Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osclcka(pub u32);
impl Osclcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Osclcka {
    #[inline(always)]
    fn default() -> Osclcka {
        Osclcka(0)
    }
}
impl core::fmt::Debug for Osclcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osclcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osclcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osclcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Oscillator Test A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osctsta(pub u32);
impl Osctsta {
    #[doc = "Test Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn tstmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Test Mode."]
    #[inline(always)]
    pub const fn set_tstmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Spare Control."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "Spare Control."]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "Test Output."]
    #[must_use]
    #[inline(always)]
    pub const fn tstout(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Test Output."]
    #[inline(always)]
    pub const fn set_tstout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Osctsta {
    #[inline(always)]
    fn default() -> Osctsta {
        Osctsta(0)
    }
}
impl core::fmt::Debug for Osctsta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osctsta")
            .field("tstmode", &self.tstmode())
            .field("spare", &self.spare())
            .field("tstout", &self.tstout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osctsta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osctsta {{ tstmode: {=u8:?}, spare: {=u8:?}, tstout: {=bool:?} }}",
            self.tstmode(),
            self.spare(),
            self.tstout()
        )
    }
}
#[doc = "Status A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusa(pub u32);
impl Statusa {
    #[doc = "POR Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> StatusaPorDet {
        let val = (self.0 >> 0usize) & 0x01;
        StatusaPorDet::from_bits(val as u8)
    }
    #[doc = "POR Detect Flag."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: StatusaPorDet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> StatusaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        StatusaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag."]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: StatusaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> StatusaTimer0Flag {
        let val = (self.0 >> 2usize) & 0x01;
        StatusaTimer0Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 0 Flag."]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: StatusaTimer0Flag) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 1 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> StatusaTimer1Flag {
        let val = (self.0 >> 3usize) & 0x01;
        StatusaTimer1Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 1 Flag."]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: StatusaTimer1Flag) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> StatusaLdoRdy {
        let val = (self.0 >> 4usize) & 0x01;
        StatusaLdoRdy::from_bits(val as u8)
    }
    #[doc = "LDO Ready."]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: StatusaLdoRdy) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> StatusaOscRdy {
        let val = (self.0 >> 5usize) & 0x01;
        StatusaOscRdy::from_bits(val as u8)
    }
    #[doc = "OSC32k Ready."]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: StatusaOscRdy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Statusa {
    #[inline(always)]
    fn default() -> Statusa {
        Statusa(0)
    }
}
impl core::fmt::Debug for Statusa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statusa")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Statusa {{ por_det: {:?}, wakeup_flag: {:?}, timer0_flag: {:?}, timer1_flag: {:?}, ldo_rdy: {:?}, osc_rdy: {:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy()
        )
    }
}
#[doc = "Switch Control A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swictla(pub u32);
impl Swictla {
    #[doc = "Switch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swi_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Enable."]
    #[inline(always)]
    pub const fn set_swi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Power Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Enable."]
    #[inline(always)]
    pub const fn set_lp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Swictla {
    #[inline(always)]
    fn default() -> Swictla {
        Swictla(0)
    }
}
impl core::fmt::Debug for Swictla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swictla")
            .field("swi_en", &self.swi_en())
            .field("lp_en", &self.lp_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swictla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swictla {{ swi_en: {=bool:?}, lp_en: {=bool:?} }}",
            self.swi_en(),
            self.lp_en()
        )
    }
}
#[doc = "Switch Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swilcka(pub u32);
impl Swilcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Swilcka {
    #[inline(always)]
    fn default() -> Swilcka {
        Swilcka(0)
    }
}
impl core::fmt::Debug for Swilcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swilcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swilcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swilcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
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
#[doc = "Wake-up Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakecfg(pub u32);
impl Wakecfg {
    #[doc = "Output."]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> Out {
        let val = (self.0 >> 0usize) & 0x01;
        Out::from_bits(val as u8)
    }
    #[doc = "Output."]
    #[inline(always)]
    pub const fn set_out(&mut self, val: Out) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Wakecfg {
    #[inline(always)]
    fn default() -> Wakecfg {
        Wakecfg(0)
    }
}
impl core::fmt::Debug for Wakecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakecfg").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakecfg {{ out: {:?} }}", self.out())
    }
}
#[doc = "Wake-up Enable A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakena(pub u32);
impl Wakena {
    #[doc = "POR Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect."]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wake-up Pin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> WakenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        WakenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Flag."]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: WakenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0."]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2."]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready."]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32K Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32K Ready."]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Wakena {
    #[inline(always)]
    fn default() -> Wakena {
        Wakena(0)
    }
}
impl core::fmt::Debug for Wakena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakena")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wakena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy()
        )
    }
}
#[doc = "Wakeup 0 Register A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupa(pub u32);
impl Wakeupa {
    #[doc = "Register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register."]
    #[inline(always)]
    pub const fn set_reg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wakeupa {
    #[inline(always)]
    fn default() -> Wakeupa {
        Wakeupa(0)
    }
}
impl core::fmt::Debug for Wakeupa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupa").field("reg", &self.reg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeupa {{ reg: {=u32:?} }}", self.reg())
    }
}
#[doc = "Wakeup Lock A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklcka(pub u32);
impl Waklcka {
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Waklcka {
    #[inline(always)]
    fn default() -> Waklcka {
        Waklcka(0)
    }
}
impl core::fmt::Debug for Waklcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Waklcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Waklcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Waklcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapTrim {
    #[doc = "Default (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 00 )."]
    Val0 = 0x0,
    #[doc = "-1us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 01)."]
    Val1 = 0x01,
    #[doc = "-2us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 10) or or +3.5us (when CAP2_TRIM = 1 and CAP_TRIM\\[1:0\\] = 10)."]
    Val2 = 0x02,
    #[doc = "-2.5us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 11) or +1us (when CAP2_TRIM = 1 and CAP_TRIM\\[1:0\\] = 11)."]
    Val3 = 0x03,
}
impl CapTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapTrim {
    #[inline(always)]
    fn from(val: u8) -> CapTrim {
        CapTrim::from_bits(val)
    }
}
impl From<CapTrim> for u8 {
    #[inline(always)]
    fn from(val: CapTrim) -> u8 {
        CapTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpTrim {
    #[doc = "760 mV."]
    Cmp760 = 0x0,
    #[doc = "770 mV."]
    Cmp770 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "740 mV."]
    Cmp740 = 0x03,
}
impl CmpTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpTrim {
    #[inline(always)]
    fn from(val: u8) -> CmpTrim {
        CmpTrim::from_bits(val)
    }
}
impl From<CmpTrim> for u8 {
    #[inline(always)]
    fn from(val: CmpTrim) -> u8 {
        CmpTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoarseAmpGain {
    #[doc = "ESR Range 0."]
    Gain05 = 0x0,
    #[doc = "ESR Range 1."]
    Gain10 = 0x01,
    #[doc = "ESR Range 2."]
    Gain18 = 0x02,
    #[doc = "ESR Range 3."]
    Gain33 = 0x03,
}
impl CoarseAmpGain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoarseAmpGain {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoarseAmpGain {
    #[inline(always)]
    fn from(val: u8) -> CoarseAmpGain {
        CoarseAmpGain::from_bits(val)
    }
}
impl From<CoarseAmpGain> for u8 {
    #[inline(always)]
    fn from(val: CoarseAmpGain) -> u8 {
        CoarseAmpGain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DlyTrim {
    #[doc = "P current 9(nA) and N Current 6(nA)."]
    Dly96 = 0x0,
    #[doc = "P current 13(nA) and N Current 6(nA)."]
    Dly136 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "P current 4(nA) and N Current 6(nA)."]
    Dly46 = 0x03,
    #[doc = "P current 9(nA) and N Current 4(nA)."]
    Dly94 = 0x04,
    #[doc = "P current 13(nA) and N Current 4(nA)."]
    Dly134 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "P current 4(nA) and N Current 4(nA)."]
    Dly44 = 0x07,
    #[doc = "P current 9(nA) and N Current 2(nA)."]
    Dly92 = 0x08,
    #[doc = "P current 13(nA) and N Current 2(nA)."]
    Dly132 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "P current 4(nA) and N Current 2(nA)."]
    Dly42 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DlyTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DlyTrim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DlyTrim {
    #[inline(always)]
    fn from(val: u8) -> DlyTrim {
        DlyTrim::from_bits(val)
    }
}
impl From<DlyTrim> for u8 {
    #[inline(always)]
    fn from(val: DlyTrim) -> u8 {
        DlyTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtalCapSel {
    #[doc = "0 pF."]
    Sel0 = 0x0,
    #[doc = "2 pF."]
    Sel2 = 0x01,
    #[doc = "4 pF."]
    Sel4 = 0x02,
    #[doc = "6 pF."]
    Sel6 = 0x03,
    #[doc = "8 pF."]
    Sel8 = 0x04,
    #[doc = "10 pF."]
    Sel10 = 0x05,
    #[doc = "12 pF."]
    Sel12 = 0x06,
    #[doc = "14 pF."]
    Sel14 = 0x07,
    #[doc = "16 pF."]
    Sel16 = 0x08,
    #[doc = "18 pF."]
    Sel18 = 0x09,
    #[doc = "20 pF."]
    Sel20 = 0x0a,
    #[doc = "22 pF."]
    Sel22 = 0x0b,
    #[doc = "24 pF."]
    Sel24 = 0x0c,
    #[doc = "26 pF."]
    Sel26 = 0x0d,
    #[doc = "28 pF."]
    Sel28 = 0x0e,
    #[doc = "30 pF."]
    Sel30 = 0x0f,
}
impl ExtalCapSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtalCapSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtalCapSel {
    #[inline(always)]
    fn from(val: u8) -> ExtalCapSel {
        ExtalCapSel::from_bits(val)
    }
}
impl From<ExtalCapSel> for u8 {
    #[inline(always)]
    fn from(val: ExtalCapSel) -> u8 {
        ExtalCapSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FineAmpGain {
    #[doc = "200 mV."]
    Agc200 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl FineAmpGain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FineAmpGain {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FineAmpGain {
    #[inline(always)]
    fn from(val: u8) -> FineAmpGain {
        FineAmpGain::from_bits(val)
    }
}
impl From<FineAmpGain> for u8 {
    #[inline(always)]
    fn from(val: FineAmpGain) -> u8 {
        FineAmpGain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitTrim {
    #[doc = "8 s."]
    Sel0 = 0x0,
    #[doc = "4 s."]
    Sel1 = 0x01,
    #[doc = "2 s."]
    Sel2 = 0x02,
    #[doc = "1 s."]
    Sel3 = 0x03,
    #[doc = "0.5 s."]
    Sel4 = 0x04,
    #[doc = "0.25 s."]
    Sel5 = 0x05,
    #[doc = "0.125 s."]
    Sel6 = 0x06,
    #[doc = "0.5 ms."]
    Sel7 = 0x07,
}
impl InitTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitTrim {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitTrim {
    #[inline(always)]
    fn from(val: u8) -> InitTrim {
        InitTrim::from_bits(val)
    }
}
impl From<InitTrim> for u8 {
    #[inline(always)]
    fn from(val: InitTrim) -> u8 {
        InitTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqenaWakeupFlag {
    #[doc = "Disable."]
    Clr = 0x0,
    #[doc = "Enable."]
    Set = 0x01,
}
impl IrqenaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqenaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqenaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> IrqenaWakeupFlag {
        IrqenaWakeupFlag::from_bits(val)
    }
}
impl From<IrqenaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: IrqenaWakeupFlag) -> u8 {
        IrqenaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModeEn {
    #[doc = "Normal mode."]
    Hp = 0x0,
    #[doc = "Startup mode."]
    Lp = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Low power mode."]
    Sw = 0x03,
}
impl ModeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModeEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModeEn {
    #[inline(always)]
    fn from(val: u8) -> ModeEn {
        ModeEn::from_bits(val)
    }
}
impl From<ModeEn> for u8 {
    #[inline(always)]
    fn from(val: ModeEn) -> u8 {
        ModeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out {
    #[doc = "Logic zero (asserted)."]
    On = 0x0,
    #[doc = "Logic one."]
    Off = 0x01,
}
impl Out {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out {
    #[inline(always)]
    fn from(val: u8) -> Out {
        Out::from_bits(val)
    }
}
impl From<Out> for u8 {
    #[inline(always)]
    fn from(val: Out) -> u8 {
        Out::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefreshTrim {
    #[doc = "Refresh every 7.8125 ms."]
    Sel7 = 0x0,
    #[doc = "Refresh every 15.625 ms."]
    Sel15 = 0x01,
    #[doc = "Refresh every 31.25 ms."]
    Sel31 = 0x02,
    #[doc = "Refresh every 62.5 ms."]
    Sel62 = 0x03,
}
impl RefreshTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefreshTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefreshTrim {
    #[inline(always)]
    fn from(val: u8) -> RefreshTrim {
        RefreshTrim::from_bits(val)
    }
}
impl From<RefreshTrim> for u8 {
    #[inline(always)]
    fn from(val: RefreshTrim) -> u8 {
        RefreshTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaLdoRdy {
    #[doc = "Disabled (not ready)."]
    Clr = 0x0,
    #[doc = "Enabled (ready)."]
    Set = 0x01,
}
impl StatusaLdoRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaLdoRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaLdoRdy {
    #[inline(always)]
    fn from(val: u8) -> StatusaLdoRdy {
        StatusaLdoRdy::from_bits(val)
    }
}
impl From<StatusaLdoRdy> for u8 {
    #[inline(always)]
    fn from(val: StatusaLdoRdy) -> u8 {
        StatusaLdoRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaOscRdy {
    #[doc = "Disabled (clock not ready)."]
    Clr = 0x0,
    #[doc = "Enabled (clock ready)."]
    Set = 0x01,
}
impl StatusaOscRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaOscRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaOscRdy {
    #[inline(always)]
    fn from(val: u8) -> StatusaOscRdy {
        StatusaOscRdy::from_bits(val)
    }
}
impl From<StatusaOscRdy> for u8 {
    #[inline(always)]
    fn from(val: StatusaOscRdy) -> u8 {
        StatusaOscRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaPorDet {
    #[doc = "Not reset."]
    Clr = 0x0,
    #[doc = "Reset."]
    Set = 0x01,
}
impl StatusaPorDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaPorDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaPorDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaPorDet {
        StatusaPorDet::from_bits(val)
    }
}
impl From<StatusaPorDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaPorDet) -> u8 {
        StatusaPorDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTimer0Flag {
    #[doc = "Not reached."]
    Clr = 0x0,
    #[doc = "Reached."]
    Set = 0x01,
}
impl StatusaTimer0Flag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTimer0Flag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTimer0Flag {
    #[inline(always)]
    fn from(val: u8) -> StatusaTimer0Flag {
        StatusaTimer0Flag::from_bits(val)
    }
}
impl From<StatusaTimer0Flag> for u8 {
    #[inline(always)]
    fn from(val: StatusaTimer0Flag) -> u8 {
        StatusaTimer0Flag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTimer1Flag {
    #[doc = "Not reached."]
    Clr = 0x0,
    #[doc = "Reached."]
    Set = 0x01,
}
impl StatusaTimer1Flag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTimer1Flag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTimer1Flag {
    #[inline(always)]
    fn from(val: u8) -> StatusaTimer1Flag {
        StatusaTimer1Flag::from_bits(val)
    }
}
impl From<StatusaTimer1Flag> for u8 {
    #[inline(always)]
    fn from(val: StatusaTimer1Flag) -> u8 {
        StatusaTimer1Flag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaWakeupFlag {
    #[doc = "Not asserted."]
    Clr = 0x0,
    #[doc = "Asserted."]
    Set = 0x01,
}
impl StatusaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> StatusaWakeupFlag {
        StatusaWakeupFlag::from_bits(val)
    }
}
impl From<StatusaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: StatusaWakeupFlag) -> u8 {
        StatusaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SupplyDet {
    #[doc = "VBAT supply is less than 3V."]
    L3vsupply = 0x0,
    #[doc = "VBAT supply is greater than 3V."]
    G3vsupply = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SupplyDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SupplyDet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SupplyDet {
    #[inline(always)]
    fn from(val: u8) -> SupplyDet {
        SupplyDet::from_bits(val)
    }
}
impl From<SupplyDet> for u8 {
    #[inline(always)]
    fn from(val: SupplyDet) -> u8 {
        SupplyDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timcfg {
    #[doc = "1 s."]
    Cfg1000 = 0x0,
    #[doc = "500 ms."]
    Cfg500 = 0x01,
    #[doc = "250 ms."]
    Cfg250 = 0x02,
    #[doc = "125 ms."]
    Cfg125 = 0x03,
    #[doc = "62.5 ms."]
    Cfg62 = 0x04,
    #[doc = "31.25 ms."]
    Cfg31 = 0x05,
    #[doc = "15.625 ms."]
    Cfg15 = 0x06,
    #[doc = "7.8125 ms."]
    Cfg7 = 0x07,
}
impl Timcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timcfg {
    #[inline(always)]
    fn from(val: u8) -> Timcfg {
        Timcfg::from_bits(val)
    }
}
impl From<Timcfg> for u8 {
    #[inline(always)]
    fn from(val: Timcfg) -> u8 {
        Timcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakenaWakeupFlag {
    #[doc = "Disable."]
    Clr = 0x0,
    #[doc = "Enable."]
    Set = 0x01,
}
impl WakenaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakenaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakenaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> WakenaWakeupFlag {
        WakenaWakeupFlag::from_bits(val)
    }
}
impl From<WakenaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: WakenaWakeupFlag) -> u8 {
        WakenaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XtalCapSel {
    #[doc = "0 pF."]
    Sel0 = 0x0,
    #[doc = "2 pF."]
    Sel2 = 0x01,
    #[doc = "4 pF."]
    Sel4 = 0x02,
    #[doc = "6 pF."]
    Sel6 = 0x03,
    #[doc = "8 pF."]
    Sel8 = 0x04,
    #[doc = "10 pF."]
    Sel10 = 0x05,
    #[doc = "12 pF."]
    Sel12 = 0x06,
    #[doc = "14 pF."]
    Sel14 = 0x07,
    #[doc = "16 pF."]
    Sel16 = 0x08,
    #[doc = "18 pF."]
    Sel18 = 0x09,
    #[doc = "20 pF."]
    Sel20 = 0x0a,
    #[doc = "22 pF."]
    Sel22 = 0x0b,
    #[doc = "24 pF."]
    Sel24 = 0x0c,
    #[doc = "26 pF."]
    Sel26 = 0x0d,
    #[doc = "28 pF."]
    Sel28 = 0x0e,
    #[doc = "30 pF."]
    Sel30 = 0x0f,
}
impl XtalCapSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XtalCapSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XtalCapSel {
    #[inline(always)]
    fn from(val: u8) -> XtalCapSel {
        XtalCapSel::from_bits(val)
    }
}
impl From<XtalCapSel> for u8 {
    #[inline(always)]
    fn from(val: XtalCapSel) -> u8 {
        XtalCapSel::to_bits(val)
    }
}
