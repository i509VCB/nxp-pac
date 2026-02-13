#[doc = "FRO16K Clock Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froclke(pub u32);
impl Froclke {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
#[doc = "FRO16K Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctla(pub u32);
impl Froctla {
    #[doc = "FRO16K Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fro_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FRO16K Enable"]
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
#[doc = "FRO16K Control B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctlb(pub u32);
impl Froctlb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Froctlb {
    #[inline(always)]
    fn default() -> Froctlb {
        Froctlb(0)
    }
}
impl core::fmt::Debug for Froctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froctlb {{ inverse: {=bool:?} }}", self.inverse())
    }
}
#[doc = "FRO16K Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolcka(pub u32);
impl Frolcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
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
#[doc = "FRO16K Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolckb(pub u32);
impl Frolckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::FrolckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FrolckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::FrolckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Frolckb {
    #[inline(always)]
    fn default() -> Frolckb {
        Frolckb(0)
    }
}
impl core::fmt::Debug for Frolckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frolckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Interrupt Enable A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqena(pub u32);
impl Irqena {
    #[doc = "POR Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> super::vals::IrqenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IrqenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag"]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: super::vals::IrqenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0"]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2"]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready"]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32k Ready"]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> super::vals::IrqenaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IrqenaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect"]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: super::vals::IrqenaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect"]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect"]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect"]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 Detect"]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq0_det(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 0 Detect"]
    #[inline(always)]
    pub const fn set_irq0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt 1 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq1_det(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 1 Detect"]
    #[inline(always)]
    pub const fn set_irq1_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt 2 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq2_det(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 2 Detect"]
    #[inline(always)]
    pub const fn set_irq2_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt 3 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq3_det(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 3 Detect"]
    #[inline(always)]
    pub const fn set_irq3_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq0_det", &self.irq0_det())
            .field("irq1_det", &self.irq1_det())
            .field("irq2_det", &self.irq2_det())
            .field("irq3_det", &self.irq3_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?}, clock_det: {=bool:?}, config_det: {:?}, volt_det: {=bool:?}, temp_det: {=bool:?}, light_det: {=bool:?}, sec0_det: {=bool:?}, irq0_det: {=bool:?}, irq1_det: {=bool:?}, irq2_det: {=bool:?}, irq3_det: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det(),
            self.irq0_det(),
            self.irq1_det(),
            self.irq2_det(),
            self.irq3_det()
        )
    }
}
#[doc = "Interrupt Enable B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqenb(pub u32);
impl Irqenb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Irqenb {
    #[inline(always)]
    fn default() -> Irqenb {
        Irqenb(0)
    }
}
impl core::fmt::Debug for Irqenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqenb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irqenb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "LDO_RAM Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoctla(pub u32);
impl Ldoctla {
    #[doc = "Bandgap Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Enable"]
    #[inline(always)]
    pub const fn set_bg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable"]
    #[inline(always)]
    pub const fn set_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refresh Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn refresh_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refresh Enable"]
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
#[doc = "LDO_RAM Control B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoctlb(pub u32);
impl Ldoctlb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Ldoctlb {
    #[inline(always)]
    fn default() -> Ldoctlb {
        Ldoctlb(0)
    }
}
impl core::fmt::Debug for Ldoctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldoctlb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "LDO_RAM Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldolcka(pub u32);
impl Ldolcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
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
#[doc = "LDO_RAM Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldolckb(pub u32);
impl Ldolckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::LdolckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LdolckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::LdolckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ldolckb {
    #[inline(always)]
    fn default() -> Ldolckb {
        Ldolckb(0)
    }
}
impl core::fmt::Debug for Ldolckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldolckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldolckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldolckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "RAM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoramc(pub u32);
impl Ldoramc {
    #[doc = "Isolate SRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Isolate SRAM"]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Switch SRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn swi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Switch SRAM"]
    #[inline(always)]
    pub const fn set_swi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
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
#[doc = "Bandgap Timer 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer0(pub u32);
impl Ldotimer0 {
    #[doc = "Timeout Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> super::vals::Timcfg {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Timcfg::from_bits(val as u8)
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: super::vals::Timcfg) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable"]
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
#[doc = "Bandgap Timer 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer1(pub u32);
impl Ldotimer1 {
    #[doc = "Timeout Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable"]
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
#[doc = "Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locka(pub u32);
impl Locka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Locka {
    #[inline(always)]
    fn default() -> Locka {
        Locka(0)
    }
}
impl core::fmt::Debug for Locka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Locka").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Locka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Locka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lockb(pub u32);
impl Lockb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::LockbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::LockbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lockb {
    #[inline(always)]
    fn default() -> Lockb {
        Lockb(0)
    }
}
impl core::fmt::Debug for Lockb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lockb").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lockb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lockb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "CLKMON Configuration A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moncfga(pub u32);
impl Moncfga {
    #[doc = "Frequency Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn freq_trim(&self) -> super::vals::FreqTrim {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FreqTrim::from_bits(val as u8)
    }
    #[doc = "Frequency Trim"]
    #[inline(always)]
    pub const fn set_freq_trim(&mut self, val: super::vals::FreqTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Divide Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn divide_trim(&self) -> super::vals::DivideTrim {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DivideTrim::from_bits(val as u8)
    }
    #[doc = "Divide Trim"]
    #[inline(always)]
    pub const fn set_divide_trim(&mut self, val: super::vals::DivideTrim) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd_trim(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved Trim"]
    #[inline(always)]
    pub const fn set_rsvd_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
}
impl Default for Moncfga {
    #[inline(always)]
    fn default() -> Moncfga {
        Moncfga(0)
    }
}
impl core::fmt::Debug for Moncfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moncfga")
            .field("freq_trim", &self.freq_trim())
            .field("divide_trim", &self.divide_trim())
            .field("rsvd_trim", &self.rsvd_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moncfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Moncfga {{ freq_trim: {:?}, divide_trim: {:?}, rsvd_trim: {=u8:?} }}",
            self.freq_trim(),
            self.divide_trim(),
            self.rsvd_trim()
        )
    }
}
#[doc = "CLKMON Configuration B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moncfgb(pub u32);
impl Moncfgb {
    #[doc = "Inverse value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Inverse value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Moncfgb {
    #[inline(always)]
    fn default() -> Moncfgb {
        Moncfgb(0)
    }
}
impl core::fmt::Debug for Moncfgb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moncfgb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moncfgb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Moncfgb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "CLKMON Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monctla(pub u32);
impl Monctla {
    #[doc = "CLKMON Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CLKMON Enable"]
    #[inline(always)]
    pub const fn set_mon_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monctla {
    #[inline(always)]
    fn default() -> Monctla {
        Monctla(0)
    }
}
impl core::fmt::Debug for Monctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monctla")
            .field("mon_en", &self.mon_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monctla {{ mon_en: {=bool:?} }}", self.mon_en())
    }
}
#[doc = "CLKMON Control B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monctlb(pub u32);
impl Monctlb {
    #[doc = "Inverse value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monctlb {
    #[inline(always)]
    fn default() -> Monctlb {
        Monctlb(0)
    }
}
impl core::fmt::Debug for Monctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monctlb {{ inverse: {=bool:?} }}", self.inverse())
    }
}
#[doc = "CLKMON Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monlcka(pub u32);
impl Monlcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monlcka {
    #[inline(always)]
    fn default() -> Monlcka {
        Monlcka(0)
    }
}
impl core::fmt::Debug for Monlcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monlcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monlcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monlcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "CLKMON Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monlckb(pub u32);
impl Monlckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::MonlckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MonlckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::MonlckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Monlckb {
    #[inline(always)]
    fn default() -> Monlckb {
        Monlckb(0)
    }
}
impl core::fmt::Debug for Monlckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monlckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monlckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monlckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Oscillator Configuration A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccfga(pub u32);
impl Osccfga {
    #[doc = "Comparator Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_trim(&self) -> super::vals::CmpTrim {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CmpTrim::from_bits(val as u8)
    }
    #[doc = "Comparator Trim"]
    #[inline(always)]
    pub const fn set_cmp_trim(&mut self, val: super::vals::CmpTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAP2_TRIM"]
    #[must_use]
    #[inline(always)]
    pub const fn cap2_trim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CAP2_TRIM"]
    #[inline(always)]
    pub const fn set_cap2_trim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Delay Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_trim(&self) -> super::vals::DlyTrim {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::DlyTrim::from_bits(val as u8)
    }
    #[doc = "Delay Trim"]
    #[inline(always)]
    pub const fn set_dly_trim(&mut self, val: super::vals::DlyTrim) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "Capacitor Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_trim(&self) -> super::vals::CapTrim {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::CapTrim::from_bits(val as u8)
    }
    #[doc = "Capacitor Trim"]
    #[inline(always)]
    pub const fn set_cap_trim(&mut self, val: super::vals::CapTrim) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "Initialization Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn init_trim(&self) -> super::vals::InitTrim {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::InitTrim::from_bits(val as u8)
    }
    #[doc = "Initialization Trim"]
    #[inline(always)]
    pub const fn set_init_trim(&mut self, val: super::vals::InitTrim) {
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
#[doc = "Oscillator Configuration B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccfgb(pub u32);
impl Osccfgb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Osccfgb {
    #[inline(always)]
    fn default() -> Osccfgb {
        Osccfgb(0)
    }
}
impl core::fmt::Debug for Osccfgb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osccfgb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osccfgb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osccfgb {{ inverse: {=u16:?} }}", self.inverse())
    }
}
#[doc = "Oscillator Clock Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscclke(pub u32);
impl Oscclke {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
#[doc = "Oscillator Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscctla(pub u32);
impl Oscctla {
    #[doc = "Crystal Oscillator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Enable"]
    #[inline(always)]
    pub const fn set_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Crystal Oscillator Bypass Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_byp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Bypass Enable"]
    #[inline(always)]
    pub const fn set_osc_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Amplifier gain adjustment bits to allow the use of a wide range of external crystal ESR values See the device datasheet for the ranges supported by this device"]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_amp_gain(&self) -> super::vals::CoarseAmpGain {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CoarseAmpGain::from_bits(val as u8)
    }
    #[doc = "Amplifier gain adjustment bits to allow the use of a wide range of external crystal ESR values See the device datasheet for the ranges supported by this device"]
    #[inline(always)]
    pub const fn set_coarse_amp_gain(&mut self, val: super::vals::CoarseAmpGain) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Crystal Load Capacitance Selection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_sel_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Load Capacitance Selection Enable"]
    #[inline(always)]
    pub const fn set_cap_sel_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn extal_cap_sel(&self) -> super::vals::ExtalCapSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::ExtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[inline(always)]
    pub const fn set_extal_cap_sel(&mut self, val: super::vals::ExtalCapSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_cap_sel(&self) -> super::vals::XtalCapSel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::XtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[inline(always)]
    pub const fn set_xtal_cap_sel(&mut self, val: super::vals::XtalCapSel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mode_en(&self) -> super::vals::ModeEn {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::ModeEn::from_bits(val as u8)
    }
    #[doc = "Mode Enable"]
    #[inline(always)]
    pub const fn set_mode_en(&mut self, val: super::vals::ModeEn) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Supply Detector Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn supply_det(&self) -> super::vals::SupplyDet {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::SupplyDet::from_bits(val as u8)
    }
    #[doc = "Supply Detector Trim"]
    #[inline(always)]
    pub const fn set_supply_det(&mut self, val: super::vals::SupplyDet) {
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
            "Oscctla {{ osc_en: {=bool:?}, osc_byp_en: {=bool:?}, coarse_amp_gain: {:?}, cap_sel_en: {=bool:?}, extal_cap_sel: {:?}, xtal_cap_sel: {:?}, mode_en: {:?}, supply_det: {:?} }}",
            self.osc_en(),
            self.osc_byp_en(),
            self.coarse_amp_gain(),
            self.cap_sel_en(),
            self.extal_cap_sel(),
            self.xtal_cap_sel(),
            self.mode_en(),
            self.supply_det()
        )
    }
}
#[doc = "Oscillator Control B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscctlb(pub u32);
impl Oscctlb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Oscctlb {
    #[inline(always)]
    fn default() -> Oscctlb {
        Oscctlb(0)
    }
}
impl core::fmt::Debug for Oscctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oscctlb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Oscillator Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osclcka(pub u32);
impl Osclcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
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
#[doc = "Oscillator Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osclckb(pub u32);
impl Osclckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::OsclckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::OsclckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::OsclckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Osclckb {
    #[inline(always)]
    fn default() -> Osclckb {
        Osclckb(0)
    }
}
impl core::fmt::Debug for Osclckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osclckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osclckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osclckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Status A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusa(pub u32);
impl Statusa {
    #[doc = "POR Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> super::vals::StatusaPorDet {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatusaPorDet::from_bits(val as u8)
    }
    #[doc = "POR Detect Flag"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: super::vals::StatusaPorDet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> super::vals::StatusaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StatusaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag"]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: super::vals::StatusaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> super::vals::StatusaTimer0Flag {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StatusaTimer0Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 0 Flag"]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: super::vals::StatusaTimer0Flag) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 1 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> super::vals::StatusaTimer1Flag {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StatusaTimer1Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 1 Flag"]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: super::vals::StatusaTimer1Flag) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> super::vals::StatusaLdoRdy {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StatusaLdoRdy::from_bits(val as u8)
    }
    #[doc = "LDO Ready"]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: super::vals::StatusaLdoRdy) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> super::vals::StatusaOscRdy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::StatusaOscRdy::from_bits(val as u8)
    }
    #[doc = "OSC32k Ready"]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: super::vals::StatusaOscRdy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> super::vals::StatusaClockDet {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::StatusaClockDet::from_bits(val as u8)
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: super::vals::StatusaClockDet) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> super::vals::StatusaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::StatusaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect Flag"]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: super::vals::StatusaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> super::vals::StatusaVoltDet {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StatusaVoltDet::from_bits(val as u8)
    }
    #[doc = "Voltage Detect"]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: super::vals::StatusaVoltDet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> super::vals::StatusaTempDet {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::StatusaTempDet::from_bits(val as u8)
    }
    #[doc = "Temperature Detect"]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: super::vals::StatusaTempDet) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> super::vals::StatusaLightDet {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::StatusaLightDet::from_bits(val as u8)
    }
    #[doc = "Light Detect"]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: super::vals::StatusaLightDet) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> super::vals::StatusaSec0Det {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::StatusaSec0Det::from_bits(val as u8)
    }
    #[doc = "Input 0 Detect"]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: super::vals::StatusaSec0Det) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq0_det(&self) -> super::vals::StatusaIrq0Det {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::StatusaIrq0Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 0 Detect"]
    #[inline(always)]
    pub const fn set_irq0_det(&mut self, val: super::vals::StatusaIrq0Det) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt 1 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq1_det(&self) -> super::vals::StatusaIrq1Det {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::StatusaIrq1Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 1 Detect"]
    #[inline(always)]
    pub const fn set_irq1_det(&mut self, val: super::vals::StatusaIrq1Det) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt 2 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq2_det(&self) -> super::vals::StatusaIrq2Det {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::StatusaIrq2Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 2 Detect"]
    #[inline(always)]
    pub const fn set_irq2_det(&mut self, val: super::vals::StatusaIrq2Det) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt 3 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq3_det(&self) -> super::vals::StatusaIrq3Det {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::StatusaIrq3Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 3 Detect"]
    #[inline(always)]
    pub const fn set_irq3_det(&mut self, val: super::vals::StatusaIrq3Det) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
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
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq0_det", &self.irq0_det())
            .field("irq1_det", &self.irq1_det())
            .field("irq2_det", &self.irq2_det())
            .field("irq3_det", &self.irq3_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Statusa {{ por_det: {:?}, wakeup_flag: {:?}, timer0_flag: {:?}, timer1_flag: {:?}, ldo_rdy: {:?}, osc_rdy: {:?}, clock_det: {:?}, config_det: {:?}, volt_det: {:?}, temp_det: {:?}, light_det: {:?}, sec0_det: {:?}, irq0_det: {:?}, irq1_det: {:?}, irq2_det: {:?}, irq3_det: {:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det(),
            self.irq0_det(),
            self.irq1_det(),
            self.irq2_det(),
            self.irq3_det()
        )
    }
}
#[doc = "Status B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusb(pub u32);
impl Statusb {
    #[doc = "Inverse value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Statusb {
    #[inline(always)]
    fn default() -> Statusb {
        Statusb(0)
    }
}
impl core::fmt::Debug for Statusb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statusb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Statusb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Switch Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swictla(pub u32);
impl Swictla {
    #[doc = "Switch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn swi_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Enable"]
    #[inline(always)]
    pub const fn set_swi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Power Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Enable"]
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
#[doc = "Switch Control B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swictlb(pub u32);
impl Swictlb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Swictlb {
    #[inline(always)]
    fn default() -> Swictlb {
        Swictlb(0)
    }
}
impl core::fmt::Debug for Swictlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swictlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swictlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swictlb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "Switch Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swilcka(pub u32);
impl Swilcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
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
#[doc = "Switch Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swilckb(pub u32);
impl Swilckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::SwilckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwilckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::SwilckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Swilckb {
    #[inline(always)]
    fn default() -> Swilckb {
        Swilckb(0)
    }
}
impl core::fmt::Debug for Swilckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swilckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swilckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swilckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "TAMPER Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamctla(pub u32);
impl Tamctla {
    #[doc = "Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn volt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_volt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Temperature Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn temp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect Enable"]
    #[inline(always)]
    pub const fn set_temp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Light Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn light_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect Enable"]
    #[inline(always)]
    pub const fn set_light_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Tamctla {
    #[inline(always)]
    fn default() -> Tamctla {
        Tamctla(0)
    }
}
impl core::fmt::Debug for Tamctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamctla")
            .field("volt_en", &self.volt_en())
            .field("temp_en", &self.temp_en())
            .field("light_en", &self.light_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tamctla {{ volt_en: {=bool:?}, temp_en: {=bool:?}, light_en: {=bool:?} }}",
            self.volt_en(),
            self.temp_en(),
            self.light_en()
        )
    }
}
#[doc = "TAMPER Control B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamctlb(pub u32);
impl Tamctlb {
    #[doc = "Inverse value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Inverse value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Tamctlb {
    #[inline(always)]
    fn default() -> Tamctlb {
        Tamctlb(0)
    }
}
impl core::fmt::Debug for Tamctlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamctlb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamctlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamctlb {{ inverse: {=u8:?} }}", self.inverse())
    }
}
#[doc = "TAMPER Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamlcka(pub u32);
impl Tamlcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Tamlcka {
    #[inline(always)]
    fn default() -> Tamlcka {
        Tamlcka(0)
    }
}
impl core::fmt::Debug for Tamlcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamlcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamlcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamlcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "TAMPER Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamlckb(pub u32);
impl Tamlckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::TamlckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TamlckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::TamlckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Tamlckb {
    #[inline(always)]
    fn default() -> Tamlckb {
        Tamlckb(0)
    }
}
impl core::fmt::Debug for Tamlckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamlckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamlckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamlckb {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Tamper Enable A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tampera(pub u32);
impl Tampera {
    #[doc = "POR Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> super::vals::TamperaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TamperaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect"]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: super::vals::TamperaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect"]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect"]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect"]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 Detect"]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Tampera {
    #[inline(always)]
    fn default() -> Tampera {
        Tampera(0)
    }
}
impl core::fmt::Debug for Tampera {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tampera")
            .field("por_det", &self.por_det())
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tampera {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tampera {{ por_det: {=bool:?}, clock_det: {=bool:?}, config_det: {:?}, volt_det: {=bool:?}, temp_det: {=bool:?}, light_det: {=bool:?}, sec0_det: {=bool:?} }}",
            self.por_det(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det()
        )
    }
}
#[doc = "Tamper Enable B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamperb(pub u32);
impl Tamperb {
    #[doc = "Inverse value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Inverse value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tamperb {
    #[inline(always)]
    fn default() -> Tamperb {
        Tamperb(0)
    }
}
impl core::fmt::Debug for Tamperb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tamperb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tamperb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tamperb {{ inverse: {=u16:?} }}", self.inverse())
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
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
#[doc = "Wake-up Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakecfg(pub u32);
impl Wakecfg {
    #[doc = "Output"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> super::vals::Out {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Out::from_bits(val as u8)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: super::vals::Out) {
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
#[doc = "Wake-up Enable A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakena(pub u32);
impl Wakena {
    #[doc = "POR Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wake-up Pin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> super::vals::WakenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::WakenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Flag"]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: super::vals::WakenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0"]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2"]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready"]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32K Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32K Ready"]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configuration Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn config_det(&self) -> super::vals::WakenaConfigDet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::WakenaConfigDet::from_bits(val as u8)
    }
    #[doc = "Configuration Detect"]
    #[inline(always)]
    pub const fn set_config_det(&mut self, val: super::vals::WakenaConfigDet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn volt_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect"]
    #[inline(always)]
    pub const fn set_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Temperature Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn temp_det(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Detect"]
    #[inline(always)]
    pub const fn set_temp_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Light Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn light_det(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Light Detect"]
    #[inline(always)]
    pub const fn set_light_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 Detect"]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq0_det(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 0 Detect"]
    #[inline(always)]
    pub const fn set_irq0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt 1 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq1_det(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 1 Detect"]
    #[inline(always)]
    pub const fn set_irq1_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt 2 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq2_det(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 2 Detect"]
    #[inline(always)]
    pub const fn set_irq2_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt 3 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq3_det(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 3 Detect"]
    #[inline(always)]
    pub const fn set_irq3_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("clock_det", &self.clock_det())
            .field("config_det", &self.config_det())
            .field("volt_det", &self.volt_det())
            .field("temp_det", &self.temp_det())
            .field("light_det", &self.light_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq0_det", &self.irq0_det())
            .field("irq1_det", &self.irq1_det())
            .field("irq2_det", &self.irq2_det())
            .field("irq3_det", &self.irq3_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wakena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?}, clock_det: {=bool:?}, config_det: {:?}, volt_det: {=bool:?}, temp_det: {=bool:?}, light_det: {=bool:?}, sec0_det: {=bool:?}, irq0_det: {=bool:?}, irq1_det: {=bool:?}, irq2_det: {=bool:?}, irq3_det: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.config_det(),
            self.volt_det(),
            self.temp_det(),
            self.light_det(),
            self.sec0_det(),
            self.irq0_det(),
            self.irq1_det(),
            self.irq2_det(),
            self.irq3_det()
        )
    }
}
#[doc = "Wake-up Enable B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakenb(pub u32);
impl Wakenb {
    #[doc = "Inverse Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Inverse Value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Wakenb {
    #[inline(always)]
    fn default() -> Wakenb {
        Wakenb(0)
    }
}
impl core::fmt::Debug for Wakenb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakenb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakenb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakenb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Wakeup 0 Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupa(pub u32);
impl Wakeupa {
    #[doc = "Register"]
    #[must_use]
    #[inline(always)]
    pub const fn reg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register"]
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
#[doc = "Wakeup 0 Register B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupb(pub u32);
impl Wakeupb {
    #[doc = "Inverse value"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Inverse value"]
    #[inline(always)]
    pub const fn set_inverse(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wakeupb {
    #[inline(always)]
    fn default() -> Wakeupb {
        Wakeupb(0)
    }
}
impl core::fmt::Debug for Wakeupb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupb")
            .field("inverse", &self.inverse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeupb {{ inverse: {=u32:?} }}", self.inverse())
    }
}
#[doc = "Wakeup Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklcka(pub u32);
impl Waklcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
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
#[doc = "Wakeup Lock B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklckb(pub u32);
impl Waklckb {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::WaklckbLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::WaklckbLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::WaklckbLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Waklckb {
    #[inline(always)]
    fn default() -> Waklckb {
        Waklckb(0)
    }
}
impl core::fmt::Debug for Waklckb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Waklckb")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Waklckb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Waklckb {{ lock: {:?} }}", self.lock())
    }
}
