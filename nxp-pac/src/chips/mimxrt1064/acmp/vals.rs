#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cff {
    #[doc = "Falling-edge on COUT has not been detected."]
    Cff0 = 0x0,
    #[doc = "Falling-edge on COUT has occurred."]
    Cff1 = 0x01,
}
impl Cff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cff {
    #[inline(always)]
    fn from(val: u8) -> Cff {
        Cff::from_bits(val)
    }
}
impl From<Cff> for u8 {
    #[inline(always)]
    fn from(val: Cff) -> u8 {
        Cff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfr {
    #[doc = "Rising-edge on COUT has not been detected."]
    Cfr0 = 0x0,
    #[doc = "Rising-edge on COUT has occurred."]
    Cfr1 = 0x01,
}
impl Cfr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfr {
    #[inline(always)]
    fn from(val: u8) -> Cfr {
        Cfr::from_bits(val)
    }
}
impl From<Cfr> for u8 {
    #[inline(always)]
    fn from(val: Cfr) -> u8 {
        Cfr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cos {
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    Cos0 = 0x0,
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    Cos1 = 0x01,
}
impl Cos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cos {
    #[inline(always)]
    fn from(val: u8) -> Cos {
        Cos::from_bits(val)
    }
}
impl From<Cos> for u8 {
    #[inline(always)]
    fn from(val: Cos) -> u8 {
        Cos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dacen {
    #[doc = "DAC is disabled."]
    Dacen0 = 0x0,
    #[doc = "DAC is enabled."]
    Dacen1 = 0x01,
}
impl Dacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dacen {
    #[inline(always)]
    fn from(val: u8) -> Dacen {
        Dacen::from_bits(val)
    }
}
impl From<Dacen> for u8 {
    #[inline(always)]
    fn from(val: Dacen) -> u8 {
        Dacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "DMA is disabled."]
    Dmaen0 = 0x0,
    #[doc = "DMA is enabled."]
    Dmaen1 = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Analog Comparator is disabled."]
    En0 = 0x0,
    #[doc = "Analog Comparator is enabled."]
    En1 = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterCnt {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    FilterCnt0 = 0x0,
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    FilterCnt1 = 0x01,
    #[doc = "2 consecutive samples must agree."]
    FilterCnt2 = 0x02,
    #[doc = "3 consecutive samples must agree."]
    FilterCnt3 = 0x03,
    #[doc = "4 consecutive samples must agree."]
    FilterCnt4 = 0x04,
    #[doc = "5 consecutive samples must agree."]
    FilterCnt5 = 0x05,
    #[doc = "6 consecutive samples must agree."]
    FilterCnt6 = 0x06,
    #[doc = "7 consecutive samples must agree."]
    FilterCnt7 = 0x07,
}
impl FilterCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterCnt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterCnt {
    #[inline(always)]
    fn from(val: u8) -> FilterCnt {
        FilterCnt::from_bits(val)
    }
}
impl From<FilterCnt> for u8 {
    #[inline(always)]
    fn from(val: FilterCnt) -> u8 {
        FilterCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hystctr {
    #[doc = "Level 0."]
    Hystctr0 = 0x0,
    #[doc = "Level 1."]
    Hystctr1 = 0x01,
    #[doc = "Level 2."]
    Hystctr2 = 0x02,
    #[doc = "Level 3."]
    Hystctr3 = 0x03,
}
impl Hystctr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hystctr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hystctr {
    #[inline(always)]
    fn from(val: u8) -> Hystctr {
        Hystctr::from_bits(val)
    }
}
impl From<Hystctr> for u8 {
    #[inline(always)]
    fn from(val: Hystctr) -> u8 {
        Hystctr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ief {
    #[doc = "Interrupt is disabled."]
    Ief0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Ief1 = 0x01,
}
impl Ief {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ief {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ief {
    #[inline(always)]
    fn from(val: u8) -> Ief {
        Ief::from_bits(val)
    }
}
impl From<Ief> for u8 {
    #[inline(always)]
    fn from(val: Ief) -> u8 {
        Ief::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ier {
    #[doc = "Interrupt is disabled."]
    Ier0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Ier1 = 0x01,
}
impl Ier {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ier {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ier {
    #[inline(always)]
    fn from(val: u8) -> Ier {
        Ier::from_bits(val)
    }
}
impl From<Ier> for u8 {
    #[inline(always)]
    fn from(val: Ier) -> u8 {
        Ier::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inv {
    #[doc = "Does not invert the comparator output."]
    Inv0 = 0x0,
    #[doc = "Inverts the comparator output."]
    Inv1 = 0x01,
}
impl Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inv {
    #[inline(always)]
    fn from(val: u8) -> Inv {
        Inv::from_bits(val)
    }
}
impl From<Inv> for u8 {
    #[inline(always)]
    fn from(val: Inv) -> u8 {
        Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msel {
    #[doc = "IN0."]
    Msel0 = 0x0,
    #[doc = "IN1."]
    Msel1 = 0x01,
    #[doc = "IN2."]
    Msel2 = 0x02,
    #[doc = "IN3."]
    Msel3 = 0x03,
    #[doc = "IN4."]
    Msel4 = 0x04,
    #[doc = "IN5."]
    Msel5 = 0x05,
    #[doc = "IN6."]
    Msel6 = 0x06,
    #[doc = "IN7."]
    Msel7 = 0x07,
}
impl Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msel {
    #[inline(always)]
    fn from(val: u8) -> Msel {
        Msel::from_bits(val)
    }
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(val: Msel) -> u8 {
        Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ope {
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    Ope0 = 0x0,
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    Ope1 = 0x01,
}
impl Ope {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ope {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ope {
    #[inline(always)]
    fn from(val: u8) -> Ope {
        Ope::from_bits(val)
    }
}
impl From<Ope> for u8 {
    #[inline(always)]
    fn from(val: Ope) -> u8 {
        Ope::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmode {
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    Pmode0 = 0x0,
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    Pmode1 = 0x01,
}
impl Pmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmode {
    #[inline(always)]
    fn from(val: u8) -> Pmode {
        Pmode::from_bits(val)
    }
}
impl From<Pmode> for u8 {
    #[inline(always)]
    fn from(val: Pmode) -> u8 {
        Pmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "IN0."]
    Psel0 = 0x0,
    #[doc = "IN1."]
    Psel1 = 0x01,
    #[doc = "IN2."]
    Psel2 = 0x02,
    #[doc = "IN3."]
    Psel3 = 0x03,
    #[doc = "IN4."]
    Psel4 = 0x04,
    #[doc = "IN5."]
    Psel5 = 0x05,
    #[doc = "IN6."]
    Psel6 = 0x06,
    #[doc = "IN7."]
    Psel7 = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Se {
    #[doc = "Sampling mode is not selected."]
    Se0 = 0x0,
    #[doc = "Sampling mode is selected."]
    Se1 = 0x01,
}
impl Se {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Se {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Se {
    #[inline(always)]
    fn from(val: u8) -> Se {
        Se::from_bits(val)
    }
}
impl From<Se> for u8 {
    #[inline(always)]
    fn from(val: Se) -> u8 {
        Se::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrsel {
    #[doc = "Vin1 is selected as resistor ladder network supply reference."]
    Vrsel0 = 0x0,
    #[doc = "Vin2 is selected as resistor ladder network supply reference."]
    Vrsel1 = 0x01,
}
impl Vrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrsel {
    #[inline(always)]
    fn from(val: u8) -> Vrsel {
        Vrsel::from_bits(val)
    }
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(val: Vrsel) -> u8 {
        Vrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum We {
    #[doc = "Windowing mode is not selected."]
    We0 = 0x0,
    #[doc = "Windowing mode is selected."]
    We1 = 0x01,
}
impl We {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> We {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for We {
    #[inline(always)]
    fn from(val: u8) -> We {
        We::from_bits(val)
    }
}
impl From<We> for u8 {
    #[inline(always)]
    fn from(val: We) -> u8 {
        We::to_bits(val)
    }
}
