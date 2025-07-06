#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LCsi {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1 = 0x01,
}
impl Hp0LCsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LCsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LCsi {
    #[inline(always)]
    fn from(val: u8) -> Hp0LCsi {
        Hp0LCsi::from_bits(val)
    }
}
impl From<Hp0LCsi> for u8 {
    #[inline(always)]
    fn from(val: Hp0LCsi) -> u8 {
        Hp0LCsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LDcp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit cannot be written by the software."]
    L_DCP_1 = 0x01,
}
impl Hp0LDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LDcp {
    #[inline(always)]
    fn from(val: u8) -> Hp0LDcp {
        Hp0LDcp::from_bits(val)
    }
}
impl From<Hp0LDcp> for u8 {
    #[inline(always)]
    fn from(val: Hp0LDcp) -> u8 {
        Hp0LDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LDma {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1 = 0x01,
}
impl Hp0LDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LDma {
    #[inline(always)]
    fn from(val: u8) -> Hp0LDma {
        Hp0LDma::from_bits(val)
    }
}
impl From<Hp0LDma> for u8 {
    #[inline(always)]
    fn from(val: Hp0LDma) -> u8 {
        Hp0LDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LEnet {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1 = 0x01,
}
impl Hp0LEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LEnet {
    #[inline(always)]
    fn from(val: u8) -> Hp0LEnet {
        Hp0LEnet::from_bits(val)
    }
}
impl From<Hp0LEnet> for u8 {
    #[inline(always)]
    fn from(val: Hp0LEnet) -> u8 {
        Hp0LEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LEnet2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET2_1 = 0x01,
}
impl Hp0LEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LEnet2 {
    #[inline(always)]
    fn from(val: u8) -> Hp0LEnet2 {
        Hp0LEnet2::from_bits(val)
    }
}
impl From<Hp0LEnet2> for u8 {
    #[inline(always)]
    fn from(val: Hp0LEnet2) -> u8 {
        Hp0LEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LLcdif {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1 = 0x01,
}
impl Hp0LLcdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LLcdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LLcdif {
    #[inline(always)]
    fn from(val: u8) -> Hp0LLcdif {
        Hp0LLcdif::from_bits(val)
    }
}
impl From<Hp0LLcdif> for u8 {
    #[inline(always)]
    fn from(val: Hp0LLcdif) -> u8 {
        Hp0LLcdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LPxp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1 = 0x01,
}
impl Hp0LPxp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LPxp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LPxp {
    #[inline(always)]
    fn from(val: u8) -> Hp0LPxp {
        Hp0LPxp::from_bits(val)
    }
}
impl From<Hp0LPxp> for u8 {
    #[inline(always)]
    fn from(val: Hp0LPxp) -> u8 {
        Hp0LPxp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LTpsmp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1 = 0x01,
}
impl Hp0LTpsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LTpsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LTpsmp {
    #[inline(always)]
    fn from(val: u8) -> Hp0LTpsmp {
        Hp0LTpsmp::from_bits(val)
    }
}
impl From<Hp0LTpsmp> for u8 {
    #[inline(always)]
    fn from(val: Hp0LTpsmp) -> u8 {
        Hp0LTpsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LUsb {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1 = 0x01,
}
impl Hp0LUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LUsb {
    #[inline(always)]
    fn from(val: u8) -> Hp0LUsb {
        Hp0LUsb::from_bits(val)
    }
}
impl From<Hp0LUsb> for u8 {
    #[inline(always)]
    fn from(val: Hp0LUsb) -> u8 {
        Hp0LUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LUsdhc1 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1 = 0x01,
}
impl Hp0LUsdhc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LUsdhc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LUsdhc1 {
    #[inline(always)]
    fn from(val: u8) -> Hp0LUsdhc1 {
        Hp0LUsdhc1::from_bits(val)
    }
}
impl From<Hp0LUsdhc1> for u8 {
    #[inline(always)]
    fn from(val: Hp0LUsdhc1) -> u8 {
        Hp0LUsdhc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LUsdhc2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1 = 0x01,
}
impl Hp0LUsdhc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LUsdhc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LUsdhc2 {
    #[inline(always)]
    fn from(val: u8) -> Hp0LUsdhc2 {
        Hp0LUsdhc2::from_bits(val)
    }
}
impl From<Hp0LUsdhc2> for u8 {
    #[inline(always)]
    fn from(val: Hp0LUsdhc2) -> u8 {
        Hp0LUsdhc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpCsi {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_1 = 0x01,
}
impl HpCsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpCsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpCsi {
    #[inline(always)]
    fn from(val: u8) -> HpCsi {
        HpCsi::from_bits(val)
    }
}
impl From<HpCsi> for u8 {
    #[inline(always)]
    fn from(val: HpCsi) -> u8 {
        HpCsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpDcp {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_1 = 0x01,
}
impl HpDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpDcp {
    #[inline(always)]
    fn from(val: u8) -> HpDcp {
        HpDcp::from_bits(val)
    }
}
impl From<HpDcp> for u8 {
    #[inline(always)]
    fn from(val: HpDcp) -> u8 {
        HpDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpDma {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_1 = 0x01,
}
impl HpDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpDma {
    #[inline(always)]
    fn from(val: u8) -> HpDma {
        HpDma::from_bits(val)
    }
}
impl From<HpDma> for u8 {
    #[inline(always)]
    fn from(val: HpDma) -> u8 {
        HpDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpEnet {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_1 = 0x01,
}
impl HpEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpEnet {
    #[inline(always)]
    fn from(val: u8) -> HpEnet {
        HpEnet::from_bits(val)
    }
}
impl From<HpEnet> for u8 {
    #[inline(always)]
    fn from(val: HpEnet) -> u8 {
        HpEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpEnet2 {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET2_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET2_1 = 0x01,
}
impl HpEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpEnet2 {
    #[inline(always)]
    fn from(val: u8) -> HpEnet2 {
        HpEnet2::from_bits(val)
    }
}
impl From<HpEnet2> for u8 {
    #[inline(always)]
    fn from(val: HpEnet2) -> u8 {
        HpEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpLcdif {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_1 = 0x01,
}
impl HpLcdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpLcdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpLcdif {
    #[inline(always)]
    fn from(val: u8) -> HpLcdif {
        HpLcdif::from_bits(val)
    }
}
impl From<HpLcdif> for u8 {
    #[inline(always)]
    fn from(val: HpLcdif) -> u8 {
        HpLcdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpPxp {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_1 = 0x01,
}
impl HpPxp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpPxp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpPxp {
    #[inline(always)]
    fn from(val: u8) -> HpPxp {
        HpPxp::from_bits(val)
    }
}
impl From<HpPxp> for u8 {
    #[inline(always)]
    fn from(val: HpPxp) -> u8 {
        HpPxp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpTpsmp {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_1 = 0x01,
}
impl HpTpsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpTpsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpTpsmp {
    #[inline(always)]
    fn from(val: u8) -> HpTpsmp {
        HpTpsmp::from_bits(val)
    }
}
impl From<HpTpsmp> for u8 {
    #[inline(always)]
    fn from(val: HpTpsmp) -> u8 {
        HpTpsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpUsb {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_1 = 0x01,
}
impl HpUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpUsb {
    #[inline(always)]
    fn from(val: u8) -> HpUsb {
        HpUsb::from_bits(val)
    }
}
impl From<HpUsb> for u8 {
    #[inline(always)]
    fn from(val: HpUsb) -> u8 {
        HpUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpUsdhc1 {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_1 = 0x01,
}
impl HpUsdhc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpUsdhc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpUsdhc1 {
    #[inline(always)]
    fn from(val: u8) -> HpUsdhc1 {
        HpUsdhc1::from_bits(val)
    }
}
impl From<HpUsdhc1> for u8 {
    #[inline(always)]
    fn from(val: HpUsdhc1) -> u8 {
        HpUsdhc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpUsdhc2 {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_1 = 0x01,
}
impl HpUsdhc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpUsdhc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpUsdhc2 {
    #[inline(always)]
    fn from(val: u8) -> HpUsdhc2 {
        HpUsdhc2::from_bits(val)
    }
}
impl From<HpUsdhc2> for u8 {
    #[inline(always)]
    fn from(val: HpUsdhc2) -> u8 {
        HpUsdhc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcCsi {
    #[doc = "User mode for the corresponding master"]
    HPC_CSI_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_CSI_1 = 0x01,
}
impl HpcCsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcCsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcCsi {
    #[inline(always)]
    fn from(val: u8) -> HpcCsi {
        HpcCsi::from_bits(val)
    }
}
impl From<HpcCsi> for u8 {
    #[inline(always)]
    fn from(val: HpcCsi) -> u8 {
        HpcCsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcDcp {
    #[doc = "User mode for the corresponding master"]
    HPC_DCP_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_DCP_1 = 0x01,
}
impl HpcDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcDcp {
    #[inline(always)]
    fn from(val: u8) -> HpcDcp {
        HpcDcp::from_bits(val)
    }
}
impl From<HpcDcp> for u8 {
    #[inline(always)]
    fn from(val: HpcDcp) -> u8 {
        HpcDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcDma {
    #[doc = "User mode for the corresponding master"]
    HPC_DMA_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_DMA_1 = 0x01,
}
impl HpcDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcDma {
    #[inline(always)]
    fn from(val: u8) -> HpcDma {
        HpcDma::from_bits(val)
    }
}
impl From<HpcDma> for u8 {
    #[inline(always)]
    fn from(val: HpcDma) -> u8 {
        HpcDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcEnet {
    #[doc = "User mode for the corresponding master"]
    HPC_ENET_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_ENET_1 = 0x01,
}
impl HpcEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcEnet {
    #[inline(always)]
    fn from(val: u8) -> HpcEnet {
        HpcEnet::from_bits(val)
    }
}
impl From<HpcEnet> for u8 {
    #[inline(always)]
    fn from(val: HpcEnet) -> u8 {
        HpcEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcEnet2 {
    #[doc = "User mode for the corresponding master"]
    HPC_ENET2_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_ENET2_1 = 0x01,
}
impl HpcEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcEnet2 {
    #[inline(always)]
    fn from(val: u8) -> HpcEnet2 {
        HpcEnet2::from_bits(val)
    }
}
impl From<HpcEnet2> for u8 {
    #[inline(always)]
    fn from(val: HpcEnet2) -> u8 {
        HpcEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcLcdif {
    #[doc = "User mode for the corresponding master"]
    HPC_LCDIF_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_LCDIF_1 = 0x01,
}
impl HpcLcdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcLcdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcLcdif {
    #[inline(always)]
    fn from(val: u8) -> HpcLcdif {
        HpcLcdif::from_bits(val)
    }
}
impl From<HpcLcdif> for u8 {
    #[inline(always)]
    fn from(val: HpcLcdif) -> u8 {
        HpcLcdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcPxp {
    #[doc = "User mode for the corresponding master"]
    HPC_PXP_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_PXP_1 = 0x01,
}
impl HpcPxp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcPxp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcPxp {
    #[inline(always)]
    fn from(val: u8) -> HpcPxp {
        HpcPxp::from_bits(val)
    }
}
impl From<HpcPxp> for u8 {
    #[inline(always)]
    fn from(val: HpcPxp) -> u8 {
        HpcPxp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcTpsmp {
    #[doc = "User mode for the corresponding master"]
    HPC_TPSMP_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_TPSMP_1 = 0x01,
}
impl HpcTpsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcTpsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcTpsmp {
    #[inline(always)]
    fn from(val: u8) -> HpcTpsmp {
        HpcTpsmp::from_bits(val)
    }
}
impl From<HpcTpsmp> for u8 {
    #[inline(always)]
    fn from(val: HpcTpsmp) -> u8 {
        HpcTpsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcUsb {
    #[doc = "User mode for the corresponding master"]
    HPC_USB_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USB_1 = 0x01,
}
impl HpcUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcUsb {
    #[inline(always)]
    fn from(val: u8) -> HpcUsb {
        HpcUsb::from_bits(val)
    }
}
impl From<HpcUsb> for u8 {
    #[inline(always)]
    fn from(val: HpcUsb) -> u8 {
        HpcUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcUsdhc1 {
    #[doc = "User mode for the corresponding master"]
    HPC_USDHC1_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USDHC1_1 = 0x01,
}
impl HpcUsdhc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcUsdhc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcUsdhc1 {
    #[inline(always)]
    fn from(val: u8) -> HpcUsdhc1 {
        HpcUsdhc1::from_bits(val)
    }
}
impl From<HpcUsdhc1> for u8 {
    #[inline(always)]
    fn from(val: HpcUsdhc1) -> u8 {
        HpcUsdhc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcUsdhc2 {
    #[doc = "User mode for the corresponding master"]
    HPC_USDHC2_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USDHC2_1 = 0x01,
}
impl HpcUsdhc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcUsdhc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcUsdhc2 {
    #[inline(always)]
    fn from(val: u8) -> HpcUsdhc2 {
        HpcUsdhc2::from_bits(val)
    }
}
impl From<HpcUsdhc2> for u8 {
    #[inline(always)]
    fn from(val: HpcUsdhc2) -> u8 {
        HpcUsdhc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LCsi {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1 = 0x01,
}
impl Hpcontrol0LCsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LCsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LCsi {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LCsi {
        Hpcontrol0LCsi::from_bits(val)
    }
}
impl From<Hpcontrol0LCsi> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LCsi) -> u8 {
        Hpcontrol0LCsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LDcp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DCP_1 = 0x01,
}
impl Hpcontrol0LDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LDcp {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LDcp {
        Hpcontrol0LDcp::from_bits(val)
    }
}
impl From<Hpcontrol0LDcp> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LDcp) -> u8 {
        Hpcontrol0LDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LDma {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1 = 0x01,
}
impl Hpcontrol0LDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LDma {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LDma {
        Hpcontrol0LDma::from_bits(val)
    }
}
impl From<Hpcontrol0LDma> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LDma) -> u8 {
        Hpcontrol0LDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LEnet {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1 = 0x01,
}
impl Hpcontrol0LEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LEnet {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LEnet {
        Hpcontrol0LEnet::from_bits(val)
    }
}
impl From<Hpcontrol0LEnet> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LEnet) -> u8 {
        Hpcontrol0LEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LEnet2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET2_1 = 0x01,
}
impl Hpcontrol0LEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LEnet2 {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LEnet2 {
        Hpcontrol0LEnet2::from_bits(val)
    }
}
impl From<Hpcontrol0LEnet2> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LEnet2) -> u8 {
        Hpcontrol0LEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LLcdif {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1 = 0x01,
}
impl Hpcontrol0LLcdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LLcdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LLcdif {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LLcdif {
        Hpcontrol0LLcdif::from_bits(val)
    }
}
impl From<Hpcontrol0LLcdif> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LLcdif) -> u8 {
        Hpcontrol0LLcdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LPxp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1 = 0x01,
}
impl Hpcontrol0LPxp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LPxp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LPxp {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LPxp {
        Hpcontrol0LPxp::from_bits(val)
    }
}
impl From<Hpcontrol0LPxp> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LPxp) -> u8 {
        Hpcontrol0LPxp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LTpsmp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1 = 0x01,
}
impl Hpcontrol0LTpsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LTpsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LTpsmp {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LTpsmp {
        Hpcontrol0LTpsmp::from_bits(val)
    }
}
impl From<Hpcontrol0LTpsmp> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LTpsmp) -> u8 {
        Hpcontrol0LTpsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LUsb {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1 = 0x01,
}
impl Hpcontrol0LUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LUsb {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LUsb {
        Hpcontrol0LUsb::from_bits(val)
    }
}
impl From<Hpcontrol0LUsb> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LUsb) -> u8 {
        Hpcontrol0LUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LUsdhc1 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1 = 0x01,
}
impl Hpcontrol0LUsdhc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LUsdhc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LUsdhc1 {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LUsdhc1 {
        Hpcontrol0LUsdhc1::from_bits(val)
    }
}
impl From<Hpcontrol0LUsdhc1> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LUsdhc1) -> u8 {
        Hpcontrol0LUsdhc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LUsdhc2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1 = 0x01,
}
impl Hpcontrol0LUsdhc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LUsdhc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LUsdhc2 {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LUsdhc2 {
        Hpcontrol0LUsdhc2::from_bits(val)
    }
}
impl From<Hpcontrol0LUsdhc2> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LUsdhc2) -> u8 {
        Hpcontrol0LUsdhc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockS1 {
    #[doc = "Not locked. The bits 16-23 can be written by the software."]
    LOCK_S1_0 = 0x0,
    #[doc = "The bits 16-23 are locked and can't be written by the software."]
    LOCK_S1_1 = 0x01,
}
impl LockS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockS1 {
    #[inline(always)]
    fn from(val: u8) -> LockS1 {
        LockS1::from_bits(val)
    }
}
impl From<LockS1> for u8 {
    #[inline(always)]
    fn from(val: LockS1) -> u8 {
        LockS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockS2 {
    #[doc = "Not locked. Bits 7-0 can be written by the software."]
    LOCK_S2_0 = 0x0,
    #[doc = "Bits 7-0 are locked and cannot be written by the software"]
    LOCK_S2_1 = 0x01,
}
impl LockS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockS2 {
    #[inline(always)]
    fn from(val: u8) -> LockS2 {
        LockS2::from_bits(val)
    }
}
impl From<LockS2> for u8 {
    #[inline(always)]
    fn from(val: LockS2) -> u8 {
        LockS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaCsi {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_CSI_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_CSI_1 = 0x01,
}
impl NsaCsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaCsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaCsi {
    #[inline(always)]
    fn from(val: u8) -> NsaCsi {
        NsaCsi::from_bits(val)
    }
}
impl From<NsaCsi> for u8 {
    #[inline(always)]
    fn from(val: NsaCsi) -> u8 {
        NsaCsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaDcp {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_DCP_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_DCP_1 = 0x01,
}
impl NsaDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaDcp {
    #[inline(always)]
    fn from(val: u8) -> NsaDcp {
        NsaDcp::from_bits(val)
    }
}
impl From<NsaDcp> for u8 {
    #[inline(always)]
    fn from(val: NsaDcp) -> u8 {
        NsaDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaDma {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_DMA_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_DMA_1 = 0x01,
}
impl NsaDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaDma {
    #[inline(always)]
    fn from(val: u8) -> NsaDma {
        NsaDma::from_bits(val)
    }
}
impl From<NsaDma> for u8 {
    #[inline(always)]
    fn from(val: NsaDma) -> u8 {
        NsaDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaEnet {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_ENET_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_ENET_1 = 0x01,
}
impl NsaEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaEnet {
    #[inline(always)]
    fn from(val: u8) -> NsaEnet {
        NsaEnet::from_bits(val)
    }
}
impl From<NsaEnet> for u8 {
    #[inline(always)]
    fn from(val: NsaEnet) -> u8 {
        NsaEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaEnet2 {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_ENET2_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_ENET2_1 = 0x01,
}
impl NsaEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaEnet2 {
    #[inline(always)]
    fn from(val: u8) -> NsaEnet2 {
        NsaEnet2::from_bits(val)
    }
}
impl From<NsaEnet2> for u8 {
    #[inline(always)]
    fn from(val: NsaEnet2) -> u8 {
        NsaEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaLcdif {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_LCDIF_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_LCDIF_1 = 0x01,
}
impl NsaLcdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaLcdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaLcdif {
    #[inline(always)]
    fn from(val: u8) -> NsaLcdif {
        NsaLcdif::from_bits(val)
    }
}
impl From<NsaLcdif> for u8 {
    #[inline(always)]
    fn from(val: NsaLcdif) -> u8 {
        NsaLcdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaPxp {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_PXP_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_PXP_1 = 0x01,
}
impl NsaPxp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaPxp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaPxp {
    #[inline(always)]
    fn from(val: u8) -> NsaPxp {
        NsaPxp::from_bits(val)
    }
}
impl From<NsaPxp> for u8 {
    #[inline(always)]
    fn from(val: NsaPxp) -> u8 {
        NsaPxp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaTpsmp {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_TPSMP_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_TPSMP_1 = 0x01,
}
impl NsaTpsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaTpsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaTpsmp {
    #[inline(always)]
    fn from(val: u8) -> NsaTpsmp {
        NsaTpsmp::from_bits(val)
    }
}
impl From<NsaTpsmp> for u8 {
    #[inline(always)]
    fn from(val: NsaTpsmp) -> u8 {
        NsaTpsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaUsb {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USB_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USB_1 = 0x01,
}
impl NsaUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaUsb {
    #[inline(always)]
    fn from(val: u8) -> NsaUsb {
        NsaUsb::from_bits(val)
    }
}
impl From<NsaUsb> for u8 {
    #[inline(always)]
    fn from(val: NsaUsb) -> u8 {
        NsaUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaUsdhc1 {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USDHC1_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USDHC1_1 = 0x01,
}
impl NsaUsdhc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaUsdhc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaUsdhc1 {
    #[inline(always)]
    fn from(val: u8) -> NsaUsdhc1 {
        NsaUsdhc1::from_bits(val)
    }
}
impl From<NsaUsdhc1> for u8 {
    #[inline(always)]
    fn from(val: NsaUsdhc1) -> u8 {
        NsaUsdhc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaUsdhc2 {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USDHC2_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USDHC2_1 = 0x01,
}
impl NsaUsdhc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaUsdhc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaUsdhc2 {
    #[inline(always)]
    fn from(val: u8) -> NsaUsdhc2 {
        NsaUsdhc2::from_bits(val)
    }
}
impl From<NsaUsdhc2> for u8 {
    #[inline(always)]
    fn from(val: NsaUsdhc2) -> u8 {
        NsaUsdhc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsrS1 {
    #[doc = "The non-secure supervisor read access is disabled for the first slave."]
    NSR_S1_0 = 0x0,
    #[doc = "The non-secure supervisor read access is enabled for the first slave."]
    NSR_S1_1 = 0x01,
}
impl NsrS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsrS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsrS1 {
    #[inline(always)]
    fn from(val: u8) -> NsrS1 {
        NsrS1::from_bits(val)
    }
}
impl From<NsrS1> for u8 {
    #[inline(always)]
    fn from(val: NsrS1) -> u8 {
        NsrS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsrS2 {
    #[doc = "The non-secure supervisor read access is disabled for the second slave."]
    NSR_S2_0 = 0x0,
    #[doc = "The non-secure supervisor read access is enabled for the second slave."]
    NSR_S2_1 = 0x01,
}
impl NsrS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsrS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsrS2 {
    #[inline(always)]
    fn from(val: u8) -> NsrS2 {
        NsrS2::from_bits(val)
    }
}
impl From<NsrS2> for u8 {
    #[inline(always)]
    fn from(val: NsrS2) -> u8 {
        NsrS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NswS1 {
    #[doc = "The non-secure supervisor write access is disabled for the first slave."]
    NSW_S1_0 = 0x0,
    #[doc = "The non-secure supervisor write access is enabled for the first slave"]
    NSW_S1_1 = 0x01,
}
impl NswS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NswS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NswS1 {
    #[inline(always)]
    fn from(val: u8) -> NswS1 {
        NswS1::from_bits(val)
    }
}
impl From<NswS1> for u8 {
    #[inline(always)]
    fn from(val: NswS1) -> u8 {
        NswS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NswS2 {
    #[doc = "The non-secure supervisor write access is disabled for the second slave."]
    NSW_S2_0 = 0x0,
    #[doc = "The non-secure supervisor write access is enabled for the second slave."]
    NSW_S2_1 = 0x01,
}
impl NswS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NswS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NswS2 {
    #[inline(always)]
    fn from(val: u8) -> NswS2 {
        NswS2::from_bits(val)
    }
}
impl From<NswS2> for u8 {
    #[inline(always)]
    fn from(val: NswS2) -> u8 {
        NswS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NurS1 {
    #[doc = "The non-secure user read access is disabled for the first slave."]
    NUR_S1_0 = 0x0,
    #[doc = "The non-secure user read access is enabled for the first slave."]
    NUR_S1_1 = 0x01,
}
impl NurS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NurS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NurS1 {
    #[inline(always)]
    fn from(val: u8) -> NurS1 {
        NurS1::from_bits(val)
    }
}
impl From<NurS1> for u8 {
    #[inline(always)]
    fn from(val: NurS1) -> u8 {
        NurS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NurS2 {
    #[doc = "The non-secure user read access is disabled for the second slave."]
    NUR_S2_0 = 0x0,
    #[doc = "The non-secure user read access is enabled for the second slave."]
    NUR_S2_1 = 0x01,
}
impl NurS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NurS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NurS2 {
    #[inline(always)]
    fn from(val: u8) -> NurS2 {
        NurS2::from_bits(val)
    }
}
impl From<NurS2> for u8 {
    #[inline(always)]
    fn from(val: NurS2) -> u8 {
        NurS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NuwS1 {
    #[doc = "The non-secure user write access is disabled for the first slave."]
    NUW_S1_0 = 0x0,
    #[doc = "The non-secure user write access is enabled for the first slave."]
    NUW_S1_1 = 0x01,
}
impl NuwS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NuwS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NuwS1 {
    #[inline(always)]
    fn from(val: u8) -> NuwS1 {
        NuwS1::from_bits(val)
    }
}
impl From<NuwS1> for u8 {
    #[inline(always)]
    fn from(val: NuwS1) -> u8 {
        NuwS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NuwS2 {
    #[doc = "The non-secure user write access is disabled for the second slave."]
    NUW_S2_0 = 0x0,
    #[doc = "The non-secure user write access is enabled for the second slave."]
    NUW_S2_1 = 0x01,
}
impl NuwS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NuwS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NuwS2 {
    #[inline(always)]
    fn from(val: u8) -> NuwS2 {
        NuwS2::from_bits(val)
    }
}
impl From<NuwS2> for u8 {
    #[inline(always)]
    fn from(val: NuwS2) -> u8 {
        NuwS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLCsi {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1 = 0x01,
}
impl SaLCsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLCsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLCsi {
    #[inline(always)]
    fn from(val: u8) -> SaLCsi {
        SaLCsi::from_bits(val)
    }
}
impl From<SaLCsi> for u8 {
    #[inline(always)]
    fn from(val: SaLCsi) -> u8 {
        SaLCsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLDcp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DCP_1 = 0x01,
}
impl SaLDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLDcp {
    #[inline(always)]
    fn from(val: u8) -> SaLDcp {
        SaLDcp::from_bits(val)
    }
}
impl From<SaLDcp> for u8 {
    #[inline(always)]
    fn from(val: SaLDcp) -> u8 {
        SaLDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLDma {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1 = 0x01,
}
impl SaLDma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLDma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLDma {
    #[inline(always)]
    fn from(val: u8) -> SaLDma {
        SaLDma::from_bits(val)
    }
}
impl From<SaLDma> for u8 {
    #[inline(always)]
    fn from(val: SaLDma) -> u8 {
        SaLDma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLEnet {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1 = 0x01,
}
impl SaLEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLEnet {
    #[inline(always)]
    fn from(val: u8) -> SaLEnet {
        SaLEnet::from_bits(val)
    }
}
impl From<SaLEnet> for u8 {
    #[inline(always)]
    fn from(val: SaLEnet) -> u8 {
        SaLEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLEnet2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET2_1 = 0x01,
}
impl SaLEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLEnet2 {
    #[inline(always)]
    fn from(val: u8) -> SaLEnet2 {
        SaLEnet2::from_bits(val)
    }
}
impl From<SaLEnet2> for u8 {
    #[inline(always)]
    fn from(val: SaLEnet2) -> u8 {
        SaLEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLLcdif {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1 = 0x01,
}
impl SaLLcdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLLcdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLLcdif {
    #[inline(always)]
    fn from(val: u8) -> SaLLcdif {
        SaLLcdif::from_bits(val)
    }
}
impl From<SaLLcdif> for u8 {
    #[inline(always)]
    fn from(val: SaLLcdif) -> u8 {
        SaLLcdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLPxp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1 = 0x01,
}
impl SaLPxp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLPxp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLPxp {
    #[inline(always)]
    fn from(val: u8) -> SaLPxp {
        SaLPxp::from_bits(val)
    }
}
impl From<SaLPxp> for u8 {
    #[inline(always)]
    fn from(val: SaLPxp) -> u8 {
        SaLPxp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLTpsmp {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1 = 0x01,
}
impl SaLTpsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLTpsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLTpsmp {
    #[inline(always)]
    fn from(val: u8) -> SaLTpsmp {
        SaLTpsmp::from_bits(val)
    }
}
impl From<SaLTpsmp> for u8 {
    #[inline(always)]
    fn from(val: SaLTpsmp) -> u8 {
        SaLTpsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLUsb {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1 = 0x01,
}
impl SaLUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLUsb {
    #[inline(always)]
    fn from(val: u8) -> SaLUsb {
        SaLUsb::from_bits(val)
    }
}
impl From<SaLUsb> for u8 {
    #[inline(always)]
    fn from(val: SaLUsb) -> u8 {
        SaLUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLUsdhc1 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1 = 0x01,
}
impl SaLUsdhc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLUsdhc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLUsdhc1 {
    #[inline(always)]
    fn from(val: u8) -> SaLUsdhc1 {
        SaLUsdhc1::from_bits(val)
    }
}
impl From<SaLUsdhc1> for u8 {
    #[inline(always)]
    fn from(val: SaLUsdhc1) -> u8 {
        SaLUsdhc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLUsdhc2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1 = 0x01,
}
impl SaLUsdhc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLUsdhc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLUsdhc2 {
    #[inline(always)]
    fn from(val: u8) -> SaLUsdhc2 {
        SaLUsdhc2::from_bits(val)
    }
}
impl From<SaLUsdhc2> for u8 {
    #[inline(always)]
    fn from(val: SaLUsdhc2) -> u8 {
        SaLUsdhc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrS1 {
    #[doc = "The secure supervisor read access is disabled for the first slave."]
    SSR_S1_0 = 0x0,
    #[doc = "The secure supervisor read access is enabled for the first slave."]
    SSR_S1_1 = 0x01,
}
impl SsrS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrS1 {
    #[inline(always)]
    fn from(val: u8) -> SsrS1 {
        SsrS1::from_bits(val)
    }
}
impl From<SsrS1> for u8 {
    #[inline(always)]
    fn from(val: SsrS1) -> u8 {
        SsrS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrS2 {
    #[doc = "The secure supervisor read access is disabled for the second slave."]
    SSR_S2_0 = 0x0,
    #[doc = "The secure supervisor read access is enabled for the second slave."]
    SSR_S2_1 = 0x01,
}
impl SsrS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrS2 {
    #[inline(always)]
    fn from(val: u8) -> SsrS2 {
        SsrS2::from_bits(val)
    }
}
impl From<SsrS2> for u8 {
    #[inline(always)]
    fn from(val: SsrS2) -> u8 {
        SsrS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SswS1 {
    #[doc = "The secure supervisor write access is disabled for the first slave."]
    SSW_S1_0 = 0x0,
    #[doc = "The secure supervisor write access is enabled for the first slave."]
    SSW_S1_1 = 0x01,
}
impl SswS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SswS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SswS1 {
    #[inline(always)]
    fn from(val: u8) -> SswS1 {
        SswS1::from_bits(val)
    }
}
impl From<SswS1> for u8 {
    #[inline(always)]
    fn from(val: SswS1) -> u8 {
        SswS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SswS2 {
    #[doc = "The secure supervisor write access is disabled for the second slave."]
    SSW_S2_0 = 0x0,
    #[doc = "The secure supervisor write access is enabled for the second slave."]
    SSW_S2_1 = 0x01,
}
impl SswS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SswS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SswS2 {
    #[inline(always)]
    fn from(val: u8) -> SswS2 {
        SswS2::from_bits(val)
    }
}
impl From<SswS2> for u8 {
    #[inline(always)]
    fn from(val: SswS2) -> u8 {
        SswS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SurS1 {
    #[doc = "The secure user read access is disabled for the first slave."]
    SUR_S1_0 = 0x0,
    #[doc = "The secure user read access is enabled for the first slave."]
    SUR_S1_1 = 0x01,
}
impl SurS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SurS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SurS1 {
    #[inline(always)]
    fn from(val: u8) -> SurS1 {
        SurS1::from_bits(val)
    }
}
impl From<SurS1> for u8 {
    #[inline(always)]
    fn from(val: SurS1) -> u8 {
        SurS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SurS2 {
    #[doc = "The secure user read access is disabled for the second slave."]
    SUR_S2_0 = 0x0,
    #[doc = "The secure user read access is enabled for the second slave."]
    SUR_S2_1 = 0x01,
}
impl SurS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SurS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SurS2 {
    #[inline(always)]
    fn from(val: u8) -> SurS2 {
        SurS2::from_bits(val)
    }
}
impl From<SurS2> for u8 {
    #[inline(always)]
    fn from(val: SurS2) -> u8 {
        SurS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SuwS1 {
    #[doc = "The secure user write access is disabled for the first slave."]
    SUW_S1_0 = 0x0,
    #[doc = "The secure user write access is enabled for the first slave."]
    SUW_S1_1 = 0x01,
}
impl SuwS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SuwS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SuwS1 {
    #[inline(always)]
    fn from(val: u8) -> SuwS1 {
        SuwS1::from_bits(val)
    }
}
impl From<SuwS1> for u8 {
    #[inline(always)]
    fn from(val: SuwS1) -> u8 {
        SuwS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SuwS2 {
    #[doc = "The secure user write access is disabled for the second slave."]
    SUW_S2_0 = 0x0,
    #[doc = "The secure user write access is enabled for the second slave."]
    SUW_S2_1 = 0x01,
}
impl SuwS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SuwS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SuwS2 {
    #[inline(always)]
    fn from(val: u8) -> SuwS2 {
        SuwS2::from_bits(val)
    }
}
impl From<SuwS2> for u8 {
    #[inline(always)]
    fn from(val: SuwS2) -> u8 {
        SuwS2::to_bits(val)
    }
}
