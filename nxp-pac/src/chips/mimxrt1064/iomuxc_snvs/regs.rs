#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxCtl(pub u32);
impl MuxCtl {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for MuxCtl {
    #[inline(always)]
    fn default() -> MuxCtl {
        MuxCtl(0)
    }
}
impl core::fmt::Debug for MuxCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MuxCtl")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MuxCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MuxCtl {{ mux_mode: {=u8:?}, sion: {=bool:?} }}",
            self.mux_mode(),
            self.sion()
        )
    }
}
#[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadOnoff(pub u32);
impl SwPadCtlPadOnoff {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadOnoffSre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadOnoffSre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadOnoffSre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadOnoffDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::SwPadCtlPadOnoffDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadOnoffDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::SwPadCtlPadOnoffSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SwPadCtlPadOnoffSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::SwPadCtlPadOnoffSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadOnoffOde {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SwPadCtlPadOnoffOde::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadOnoffOde) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> super::vals::SwPadCtlPadOnoffPke {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SwPadCtlPadOnoffPke::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: super::vals::SwPadCtlPadOnoffPke) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadOnoffPue {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SwPadCtlPadOnoffPue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadOnoffPue) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadOnoffPus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SwPadCtlPadOnoffPus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadOnoffPus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> super::vals::SwPadCtlPadOnoffHys {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SwPadCtlPadOnoffHys::from_bits(val as u8)
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: super::vals::SwPadCtlPadOnoffHys) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for SwPadCtlPadOnoff {
    #[inline(always)]
    fn default() -> SwPadCtlPadOnoff {
        SwPadCtlPadOnoff(0)
    }
}
impl core::fmt::Debug for SwPadCtlPadOnoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadOnoff")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadOnoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwPadCtlPadOnoff {{ sre: {:?}, dse: {:?}, speed: {:?}, ode: {:?}, pke: {:?}, pue: {:?}, pus: {:?}, hys: {:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadPmicOnReq(pub u32);
impl SwPadCtlPadPmicOnReq {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadPmicOnReqSre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadPmicOnReqSre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadPmicOnReqSre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadPmicOnReqDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::SwPadCtlPadPmicOnReqDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadPmicOnReqDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::SwPadCtlPadPmicOnReqSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SwPadCtlPadPmicOnReqSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::SwPadCtlPadPmicOnReqSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadPmicOnReqOde {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SwPadCtlPadPmicOnReqOde::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadPmicOnReqOde) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> super::vals::SwPadCtlPadPmicOnReqPke {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SwPadCtlPadPmicOnReqPke::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: super::vals::SwPadCtlPadPmicOnReqPke) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadPmicOnReqPue {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SwPadCtlPadPmicOnReqPue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadPmicOnReqPue) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadPmicOnReqPus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SwPadCtlPadPmicOnReqPus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadPmicOnReqPus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> super::vals::SwPadCtlPadPmicOnReqHys {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SwPadCtlPadPmicOnReqHys::from_bits(val as u8)
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: super::vals::SwPadCtlPadPmicOnReqHys) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for SwPadCtlPadPmicOnReq {
    #[inline(always)]
    fn default() -> SwPadCtlPadPmicOnReq {
        SwPadCtlPadPmicOnReq(0)
    }
}
impl core::fmt::Debug for SwPadCtlPadPmicOnReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadPmicOnReq")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadPmicOnReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwPadCtlPadPmicOnReq {{ sre: {:?}, dse: {:?}, speed: {:?}, ode: {:?}, pke: {:?}, pue: {:?}, pus: {:?}, hys: {:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadPmicStbyReq(pub u32);
impl SwPadCtlPadPmicStbyReq {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadPmicStbyReqSre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadPmicStbyReqSre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqSre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadPmicStbyReqDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::SwPadCtlPadPmicStbyReqDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::SwPadCtlPadPmicStbyReqSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SwPadCtlPadPmicStbyReqSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadPmicStbyReqOde {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SwPadCtlPadPmicStbyReqOde::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqOde) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> super::vals::SwPadCtlPadPmicStbyReqPke {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SwPadCtlPadPmicStbyReqPke::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqPke) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadPmicStbyReqPue {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SwPadCtlPadPmicStbyReqPue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqPue) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadPmicStbyReqPus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SwPadCtlPadPmicStbyReqPus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqPus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> super::vals::SwPadCtlPadPmicStbyReqHys {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SwPadCtlPadPmicStbyReqHys::from_bits(val as u8)
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: super::vals::SwPadCtlPadPmicStbyReqHys) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for SwPadCtlPadPmicStbyReq {
    #[inline(always)]
    fn default() -> SwPadCtlPadPmicStbyReq {
        SwPadCtlPadPmicStbyReq(0)
    }
}
impl core::fmt::Debug for SwPadCtlPadPmicStbyReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadPmicStbyReq")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadPmicStbyReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwPadCtlPadPmicStbyReq {{ sre: {:?}, dse: {:?}, speed: {:?}, ode: {:?}, pke: {:?}, pue: {:?}, pus: {:?}, hys: {:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadPorB(pub u32);
impl SwPadCtlPadPorB {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadPorBSre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadPorBSre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadPorBSre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadPorBDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::SwPadCtlPadPorBDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadPorBDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::SwPadCtlPadPorBSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SwPadCtlPadPorBSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::SwPadCtlPadPorBSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadPorBOde {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SwPadCtlPadPorBOde::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadPorBOde) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> super::vals::SwPadCtlPadPorBPke {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SwPadCtlPadPorBPke::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: super::vals::SwPadCtlPadPorBPke) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadPorBPue {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SwPadCtlPadPorBPue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadPorBPue) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadPorBPus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SwPadCtlPadPorBPus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadPorBPus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> super::vals::SwPadCtlPadPorBHys {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SwPadCtlPadPorBHys::from_bits(val as u8)
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: super::vals::SwPadCtlPadPorBHys) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for SwPadCtlPadPorB {
    #[inline(always)]
    fn default() -> SwPadCtlPadPorB {
        SwPadCtlPadPorB(0)
    }
}
impl core::fmt::Debug for SwPadCtlPadPorB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadPorB")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadPorB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwPadCtlPadPorB {{ sre: {:?}, dse: {:?}, speed: {:?}, ode: {:?}, pke: {:?}, pue: {:?}, pus: {:?}, hys: {:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadTestMode(pub u32);
impl SwPadCtlPadTestMode {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadTestModeSre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadTestModeSre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadTestModeSre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadTestModeDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::SwPadCtlPadTestModeDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadTestModeDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::SwPadCtlPadTestModeSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SwPadCtlPadTestModeSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::SwPadCtlPadTestModeSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadTestModeOde {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SwPadCtlPadTestModeOde::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadTestModeOde) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> super::vals::SwPadCtlPadTestModePke {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SwPadCtlPadTestModePke::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: super::vals::SwPadCtlPadTestModePke) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadTestModePue {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SwPadCtlPadTestModePue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadTestModePue) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadTestModePus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SwPadCtlPadTestModePus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadTestModePus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> super::vals::SwPadCtlPadTestModeHys {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SwPadCtlPadTestModeHys::from_bits(val as u8)
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: super::vals::SwPadCtlPadTestModeHys) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for SwPadCtlPadTestMode {
    #[inline(always)]
    fn default() -> SwPadCtlPadTestMode {
        SwPadCtlPadTestMode(0)
    }
}
impl core::fmt::Debug for SwPadCtlPadTestMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadTestMode")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadTestMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwPadCtlPadTestMode {{ sre: {:?}, dse: {:?}, speed: {:?}, ode: {:?}, pke: {:?}, pue: {:?}, pus: {:?}, hys: {:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadWakeup(pub u32);
impl SwPadCtlPadWakeup {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadWakeupSre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadWakeupSre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadWakeupSre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadWakeupDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::SwPadCtlPadWakeupDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadWakeupDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::SwPadCtlPadWakeupSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SwPadCtlPadWakeupSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::SwPadCtlPadWakeupSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadWakeupOde {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SwPadCtlPadWakeupOde::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadWakeupOde) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> super::vals::SwPadCtlPadWakeupPke {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SwPadCtlPadWakeupPke::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: super::vals::SwPadCtlPadWakeupPke) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadWakeupPue {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SwPadCtlPadWakeupPue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field Control signal to enable internal pull-up/down resistors or pad keeper functionality."]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadWakeupPue) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadWakeupPus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SwPadCtlPadWakeupPus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field Controls signals to select pull-up or pull-down internal resistance strength."]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadWakeupPus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> super::vals::SwPadCtlPadWakeupHys {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SwPadCtlPadWakeupHys::from_bits(val as u8)
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: super::vals::SwPadCtlPadWakeupHys) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for SwPadCtlPadWakeup {
    #[inline(always)]
    fn default() -> SwPadCtlPadWakeup {
        SwPadCtlPadWakeup(0)
    }
}
impl core::fmt::Debug for SwPadCtlPadWakeup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadWakeup")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadWakeup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwPadCtlPadWakeup {{ sre: {:?}, dse: {:?}, speed: {:?}, ode: {:?}, pke: {:?}, pue: {:?}, pus: {:?}, hys: {:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
