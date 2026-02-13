#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodf {
    #[doc = "divide by 1"]
    AHB_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    AHB_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    AHB_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    AHB_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    AHB_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    AHB_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    AHB_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    AHB_PODF_7 = 0x07,
}
impl AhbPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodf {
    #[inline(always)]
    fn from(val: u8) -> AhbPodf {
        AhbPodf::from_bits(val)
    }
}
impl From<AhbPodf> for u8 {
    #[inline(always)]
    fn from(val: AhbPodf) -> u8 {
        AhbPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodfBusy {
    #[doc = "divider is not busy and its value represents the actual division."]
    AHB_PODF_BUSY_0 = 0x0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied."]
    AHB_PODF_BUSY_1 = 0x01,
}
impl AhbPodfBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodfBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodfBusy {
    #[inline(always)]
    fn from(val: u8) -> AhbPodfBusy {
        AhbPodfBusy::from_bits(val)
    }
}
impl From<AhbPodfBusy> for u8 {
    #[inline(always)]
    fn from(val: AhbPodfBusy) -> u8 {
        AhbPodfBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodfLoaded {
    #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_0 = 0x0,
    #[doc = "interrupt generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_1 = 0x01,
}
impl AhbPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> AhbPodfLoaded {
        AhbPodfLoaded::from_bits(val)
    }
}
impl From<AhbPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: AhbPodfLoaded) -> u8 {
        AhbPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmClkDisOnLpm {
    #[doc = "Arm clock enabled on wait mode."]
    ARM_CLK_DIS_ON_LPM_0 = 0x0,
    #[doc = "Arm clock disabled on wait mode. ."]
    ARM_CLK_DIS_ON_LPM_1 = 0x01,
}
impl ArmClkDisOnLpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmClkDisOnLpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmClkDisOnLpm {
    #[inline(always)]
    fn from(val: u8) -> ArmClkDisOnLpm {
        ArmClkDisOnLpm::from_bits(val)
    }
}
impl From<ArmClkDisOnLpm> for u8 {
    #[inline(always)]
    fn from(val: ArmClkDisOnLpm) -> u8 {
        ArmClkDisOnLpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPodf {
    #[doc = "divide by 1"]
    ARM_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    ARM_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    ARM_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    ARM_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    ARM_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    ARM_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    ARM_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    ARM_PODF_7 = 0x07,
}
impl ArmPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPodf {
    #[inline(always)]
    fn from(val: u8) -> ArmPodf {
        ArmPodf::from_bits(val)
    }
}
impl From<ArmPodf> for u8 {
    #[inline(always)]
    fn from(val: ArmPodf) -> u8 {
        ArmPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPodfBusy {
    #[doc = "divider is not busy and its value represents the actual division."]
    ARM_PODF_BUSY_0 = 0x0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the arm_podf will be applied."]
    ARM_PODF_BUSY_1 = 0x01,
}
impl ArmPodfBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPodfBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPodfBusy {
    #[inline(always)]
    fn from(val: u8) -> ArmPodfBusy {
        ArmPodfBusy::from_bits(val)
    }
}
impl From<ArmPodfBusy> for u8 {
    #[inline(always)]
    fn from(val: ArmPodfBusy) -> u8 {
        ArmPodfBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Camp2Ready {
    #[doc = "CAMP2 is not ready."]
    CAMP2_READY_0 = 0x0,
    #[doc = "CAMP2 is ready."]
    CAMP2_READY_1 = 0x01,
}
impl Camp2Ready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Camp2Ready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Camp2Ready {
    #[inline(always)]
    fn from(val: u8) -> Camp2Ready {
        Camp2Ready::from_bits(val)
    }
}
impl From<Camp2Ready> for u8 {
    #[inline(always)]
    fn from(val: Camp2Ready) -> u8 {
        Camp2Ready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl CanClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanClkPodf {
    #[inline(always)]
    fn from(val: u8) -> CanClkPodf {
        CanClkPodf::from_bits(val)
    }
}
impl From<CanClkPodf> for u8 {
    #[inline(always)]
    fn from(val: CanClkPodf) -> u8 {
        CanClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanClkSel {
    #[doc = "derive clock from pll3_sw_clk divided clock (60M)"]
    CAN_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk (24M)"]
    CAN_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from pll3_sw_clk divided clock (80M)"]
    CAN_CLK_SEL_2 = 0x02,
    #[doc = "Disable FlexCAN clock"]
    CAN_CLK_SEL_3 = 0x03,
}
impl CanClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanClkSel {
    #[inline(always)]
    fn from(val: u8) -> CanClkSel {
        CanClkSel::from_bits(val)
    }
}
impl From<CanClkSel> for u8 {
    #[inline(always)]
    fn from(val: CanClkSel) -> u8 {
        CanClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CimrArmPodfLoaded {
    #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    ARM_PODF_LOADED_0 = 0x0,
    #[doc = "mask interrupt due to frequency change of arm_podf"]
    ARM_PODF_LOADED_1 = 0x01,
}
impl CimrArmPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CimrArmPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CimrArmPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> CimrArmPodfLoaded {
        CimrArmPodfLoaded::from_bits(val)
    }
}
impl From<CimrArmPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: CimrArmPodfLoaded) -> u8 {
        CimrArmPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CisrArmPodfLoaded {
    #[doc = "interrupt is not generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_0 = 0x0,
    #[doc = "interrupt generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_1 = 0x01,
}
impl CisrArmPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CisrArmPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CisrArmPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> CisrArmPodfLoaded {
        CisrArmPodfLoaded::from_bits(val)
    }
}
impl From<CisrArmPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: CisrArmPodfLoaded) -> u8 {
        CisrArmPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CisrCoscReady {
    #[doc = "interrupt is not generated due to on board oscillator ready"]
    COSC_READY_0 = 0x0,
    #[doc = "interrupt generated due to on board oscillator ready"]
    COSC_READY_1 = 0x01,
}
impl CisrCoscReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CisrCoscReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CisrCoscReady {
    #[inline(always)]
    fn from(val: u8) -> CisrCoscReady {
        CisrCoscReady::from_bits(val)
    }
}
impl From<CisrCoscReady> for u8 {
    #[inline(always)]
    fn from(val: CisrCoscReady) -> u8 {
        CisrCoscReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkOutSel {
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
    CLK_OUT_SEL_0 = 0x0,
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
    CLK_OUT_SEL_1 = 0x01,
}
impl ClkOutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkOutSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkOutSel {
    #[inline(always)]
    fn from(val: u8) -> ClkOutSel {
        ClkOutSel::from_bits(val)
    }
}
impl From<ClkOutSel> for u8 {
    #[inline(always)]
    fn from(val: ClkOutSel) -> u8 {
        ClkOutSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1Div {
    #[doc = "divide by 1"]
    CLKO1_DIV_0 = 0x0,
    #[doc = "divide by 2"]
    CLKO1_DIV_1 = 0x01,
    #[doc = "divide by 3"]
    CLKO1_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    CLKO1_DIV_3 = 0x03,
    #[doc = "divide by 5"]
    CLKO1_DIV_4 = 0x04,
    #[doc = "divide by 6"]
    CLKO1_DIV_5 = 0x05,
    #[doc = "divide by 7"]
    CLKO1_DIV_6 = 0x06,
    #[doc = "divide by 8"]
    CLKO1_DIV_7 = 0x07,
}
impl Clko1Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1Div {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1Div {
    #[inline(always)]
    fn from(val: u8) -> Clko1Div {
        Clko1Div::from_bits(val)
    }
}
impl From<Clko1Div> for u8 {
    #[inline(always)]
    fn from(val: Clko1Div) -> u8 {
        Clko1Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1En {
    #[doc = "CCM_CLKO1 disabled."]
    CLKO1_EN_0 = 0x0,
    #[doc = "CCM_CLKO1 enabled."]
    CLKO1_EN_1 = 0x01,
}
impl Clko1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1En {
    #[inline(always)]
    fn from(val: u8) -> Clko1En {
        Clko1En::from_bits(val)
    }
}
impl From<Clko1En> for u8 {
    #[inline(always)]
    fn from(val: Clko1En) -> u8 {
        Clko1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1Sel {
    #[doc = "USB1 PLL clock (divided by 2)"]
    CLKO1_SEL_0 = 0x0,
    #[doc = "SYS PLL clock (divided by 2)"]
    CLKO1_SEL_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "VIDEO PLL clock (divided by 2)"]
    CLKO1_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "semc_clk_root"]
    CLKO1_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "lcdif_pix_clk_root"]
    CLKO1_SEL_10 = 0x0a,
    #[doc = "ahb_clk_root"]
    CLKO1_SEL_11 = 0x0b,
    #[doc = "ipg_clk_root"]
    CLKO1_SEL_12 = 0x0c,
    #[doc = "perclk_root"]
    CLKO1_SEL_13 = 0x0d,
    #[doc = "ckil_sync_clk_root"]
    CLKO1_SEL_14 = 0x0e,
    #[doc = "pll4_main_clk"]
    CLKO1_SEL_15 = 0x0f,
}
impl Clko1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1Sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1Sel {
    #[inline(always)]
    fn from(val: u8) -> Clko1Sel {
        Clko1Sel::from_bits(val)
    }
}
impl From<Clko1Sel> for u8 {
    #[inline(always)]
    fn from(val: Clko1Sel) -> u8 {
        Clko1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2Div {
    #[doc = "divide by 1"]
    CLKO2_DIV_0 = 0x0,
    #[doc = "divide by 2"]
    CLKO2_DIV_1 = 0x01,
    #[doc = "divide by 3"]
    CLKO2_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    CLKO2_DIV_3 = 0x03,
    #[doc = "divide by 5"]
    CLKO2_DIV_4 = 0x04,
    #[doc = "divide by 6"]
    CLKO2_DIV_5 = 0x05,
    #[doc = "divide by 7"]
    CLKO2_DIV_6 = 0x06,
    #[doc = "divide by 8"]
    CLKO2_DIV_7 = 0x07,
}
impl Clko2Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2Div {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2Div {
    #[inline(always)]
    fn from(val: u8) -> Clko2Div {
        Clko2Div::from_bits(val)
    }
}
impl From<Clko2Div> for u8 {
    #[inline(always)]
    fn from(val: Clko2Div) -> u8 {
        Clko2Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2En {
    #[doc = "CCM_CLKO2 disabled."]
    CLKO2_EN_0 = 0x0,
    #[doc = "CCM_CLKO2 enabled."]
    CLKO2_EN_1 = 0x01,
}
impl Clko2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2En {
    #[inline(always)]
    fn from(val: u8) -> Clko2En {
        Clko2En::from_bits(val)
    }
}
impl From<Clko2En> for u8 {
    #[inline(always)]
    fn from(val: Clko2En) -> u8 {
        Clko2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2Sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "usdhc1_clk_root"]
    CLKO2_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "lpi2c_clk_root"]
    CLKO2_SEL_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "csi_clk_root"]
    CLKO2_SEL_11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "osc_clk"]
    CLKO2_SEL_14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "usdhc2_clk_root"]
    CLKO2_SEL_17 = 0x11,
    #[doc = "sai1_clk_root"]
    CLKO2_SEL_18 = 0x12,
    #[doc = "sai2_clk_root"]
    CLKO2_SEL_19 = 0x13,
    #[doc = "sai3_clk_root (shared with ADC1 and ADC2 alt_clk root)"]
    CLKO2_SEL_20 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "can_clk_root (FlexCAN, shared with CANFD)"]
    CLKO2_SEL_23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "flexspi_clk_root"]
    CLKO2_SEL_27 = 0x1b,
    #[doc = "uart_clk_root"]
    CLKO2_SEL_28 = 0x1c,
    #[doc = "spdif0_clk_root"]
    CLKO2_SEL_29 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Clko2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2Sel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2Sel {
    #[inline(always)]
    fn from(val: u8) -> Clko2Sel {
        Clko2Sel::from_bits(val)
    }
}
impl From<Clko2Sel> for u8 {
    #[inline(always)]
    fn from(val: Clko2Sel) -> u8 {
        Clko2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoscEn {
    #[doc = "disable on chip oscillator"]
    COSC_EN_0 = 0x0,
    #[doc = "enable on chip oscillator"]
    COSC_EN_1 = 0x01,
}
impl CoscEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoscEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoscEn {
    #[inline(always)]
    fn from(val: u8) -> CoscEn {
        CoscEn::from_bits(val)
    }
}
impl From<CoscEn> for u8 {
    #[inline(always)]
    fn from(val: CoscEn) -> u8 {
        CoscEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoscPwrdown {
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    COSC_PWRDOWN_0 = 0x0,
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    COSC_PWRDOWN_1 = 0x01,
}
impl CoscPwrdown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoscPwrdown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoscPwrdown {
    #[inline(always)]
    fn from(val: u8) -> CoscPwrdown {
        CoscPwrdown::from_bits(val)
    }
}
impl From<CoscPwrdown> for u8 {
    #[inline(always)]
    fn from(val: CoscPwrdown) -> u8 {
        CoscPwrdown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiClkSel {
    #[doc = "derive clock from osc_clk (24M)"]
    CSI_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD2"]
    CSI_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from pll3_120M"]
    CSI_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL3 PFD1"]
    CSI_CLK_SEL_3 = 0x03,
}
impl CsiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiClkSel {
    #[inline(always)]
    fn from(val: u8) -> CsiClkSel {
        CsiClkSel::from_bits(val)
    }
}
impl From<CsiClkSel> for u8 {
    #[inline(always)]
    fn from(val: CsiClkSel) -> u8 {
        CsiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiPodf {
    #[doc = "divide by 1"]
    CSI_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    CSI_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    CSI_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    CSI_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    CSI_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    CSI_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    CSI_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    CSI_PODF_7 = 0x07,
}
impl CsiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiPodf {
    #[inline(always)]
    fn from(val: u8) -> CsiPodf {
        CsiPodf::from_bits(val)
    }
}
impl From<CsiPodf> for u8 {
    #[inline(always)]
    fn from(val: CsiPodf) -> u8 {
        CsiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsrCoscReady {
    #[doc = "on board oscillator is not ready."]
    COSC_READY_0 = 0x0,
    #[doc = "on board oscillator is ready."]
    COSC_READY_1 = 0x01,
}
impl CsrCoscReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsrCoscReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsrCoscReady {
    #[inline(always)]
    fn from(val: u8) -> CsrCoscReady {
        CsrCoscReady::from_bits(val)
    }
}
impl From<CsrCoscReady> for u8 {
    #[inline(always)]
    fn from(val: CsrCoscReady) -> u8 {
        CsrCoscReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisRefOsc {
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    DIS_REF_OSC_0 = 0x0,
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    DIS_REF_OSC_1 = 0x01,
}
impl DisRefOsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisRefOsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisRefOsc {
    #[inline(always)]
    fn from(val: u8) -> DisRefOsc {
        DisRefOsc::from_bits(val)
    }
}
impl From<DisRefOsc> for u8 {
    #[inline(always)]
    fn from(val: DisRefOsc) -> u8 {
        DisRefOsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EfuseProgSupplyGate {
    #[doc = "fuse programing supply voltage is gated off to the efuse module"]
    EFUSE_PROG_SUPPLY_GATE_0 = 0x0,
    #[doc = "allow fuse programing."]
    EFUSE_PROG_SUPPLY_GATE_1 = 0x01,
}
impl EfuseProgSupplyGate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EfuseProgSupplyGate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EfuseProgSupplyGate {
    #[inline(always)]
    fn from(val: u8) -> EfuseProgSupplyGate {
        EfuseProgSupplyGate::from_bits(val)
    }
}
impl From<EfuseProgSupplyGate> for u8 {
    #[inline(always)]
    fn from(val: EfuseProgSupplyGate) -> u8 {
        EfuseProgSupplyGate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1ClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Flexio1ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1ClkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Flexio1ClkPodf {
        Flexio1ClkPodf::from_bits(val)
    }
}
impl From<Flexio1ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Flexio1ClkPodf) -> u8 {
        Flexio1ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1ClkPred {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Flexio1ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Flexio1ClkPred {
        Flexio1ClkPred::from_bits(val)
    }
}
impl From<Flexio1ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Flexio1ClkPred) -> u8 {
        Flexio1ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1ClkSel {
    #[doc = "derive clock from PLL4"]
    FLEXIO1_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2"]
    FLEXIO1_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5"]
    FLEXIO1_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO1_CLK_SEL_3 = 0x03,
}
impl Flexio1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexio1ClkSel {
        Flexio1ClkSel::from_bits(val)
    }
}
impl From<Flexio1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexio1ClkSel) -> u8 {
        Flexio1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2ClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Flexio2ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2ClkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Flexio2ClkPodf {
        Flexio2ClkPodf::from_bits(val)
    }
}
impl From<Flexio2ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Flexio2ClkPodf) -> u8 {
        Flexio2ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2ClkPred {
    #[doc = "divide by 1"]
    FLEXIO2_CLK_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    FLEXIO2_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXIO2_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXIO2_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXIO2_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXIO2_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXIO2_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXIO2_CLK_PRED_7 = 0x07,
}
impl Flexio2ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Flexio2ClkPred {
        Flexio2ClkPred::from_bits(val)
    }
}
impl From<Flexio2ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Flexio2ClkPred) -> u8 {
        Flexio2ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2ClkSel {
    #[doc = "derive clock from PLL4 divided clock"]
    FLEXIO2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2 clock"]
    FLEXIO2_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5 clock"]
    FLEXIO2_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO2_CLK_SEL_3 = 0x03,
}
impl Flexio2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexio2ClkSel {
        Flexio2ClkSel::from_bits(val)
    }
}
impl From<Flexio2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexio2ClkSel) -> u8 {
        Flexio2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ClkSel {
    #[doc = "derive clock from PLL2 PFD2"]
    FLEXSPI2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD0"]
    FLEXSPI2_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL3 PFD1"]
    FLEXSPI2_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 (pll2_main_clk)"]
    FLEXSPI2_CLK_SEL_3 = 0x03,
}
impl Flexspi2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ClkSel {
        Flexspi2ClkSel::from_bits(val)
    }
}
impl From<Flexspi2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ClkSel) -> u8 {
        Flexspi2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Podf {
    #[doc = "divide by 1"]
    FLEXSPI2_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    FLEXSPI2_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXSPI2_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXSPI2_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXSPI2_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXSPI2_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXSPI2_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXSPI2_PODF_7 = 0x07,
}
impl Flexspi2Podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Podf {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Podf {
        Flexspi2Podf::from_bits(val)
    }
}
impl From<Flexspi2Podf> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Podf) -> u8 {
        Flexspi2Podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiClkSel {
    #[doc = "derive clock from semc_clk_root_pre"]
    FLEXSPI_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXSPI_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD2"]
    FLEXSPI_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL3 PFD0"]
    FLEXSPI_CLK_SEL_3 = 0x03,
}
impl FlexspiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiClkSel {
    #[inline(always)]
    fn from(val: u8) -> FlexspiClkSel {
        FlexspiClkSel::from_bits(val)
    }
}
impl From<FlexspiClkSel> for u8 {
    #[inline(always)]
    fn from(val: FlexspiClkSel) -> u8 {
        FlexspiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiPodf {
    #[doc = "divide by 1"]
    FLEXSPI_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    FLEXSPI_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXSPI_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXSPI_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXSPI_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXSPI_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXSPI_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXSPI_PODF_7 = 0x07,
}
impl FlexspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiPodf {
    #[inline(always)]
    fn from(val: u8) -> FlexspiPodf {
        FlexspiPodf::from_bits(val)
    }
}
impl From<FlexspiPodf> for u8 {
    #[inline(always)]
    fn from(val: FlexspiPodf) -> u8 {
        FlexspiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fpl {
    #[doc = "Engage PLL enable default way."]
    FPL_0 = 0x0,
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    FPL_1 = 0x01,
}
impl Fpl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fpl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fpl {
    #[inline(always)]
    fn from(val: u8) -> Fpl {
        Fpl::from_bits(val)
    }
}
impl From<Fpl> for u8 {
    #[inline(always)]
    fn from(val: Fpl) -> u8 {
        Fpl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMemClkLpm {
    #[doc = "Disable the clock to the Arm platform memories when entering Low Power Mode"]
    INT_MEM_CLK_LPM_0 = 0x0,
    #[doc = "Keep the clocks to the Arm platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    INT_MEM_CLK_LPM_1 = 0x01,
}
impl IntMemClkLpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMemClkLpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMemClkLpm {
    #[inline(always)]
    fn from(val: u8) -> IntMemClkLpm {
        IntMemClkLpm::from_bits(val)
    }
}
impl From<IntMemClkLpm> for u8 {
    #[inline(always)]
    fn from(val: IntMemClkLpm) -> u8 {
        IntMemClkLpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpgPodf {
    #[doc = "divide by 1"]
    IPG_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    IPG_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    IPG_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    IPG_PODF_3 = 0x03,
}
impl IpgPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpgPodf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpgPodf {
    #[inline(always)]
    fn from(val: u8) -> IpgPodf {
        IpgPodf::from_bits(val)
    }
}
impl From<IpgPodf> for u8 {
    #[inline(always)]
    fn from(val: IpgPodf) -> u8 {
        IpgPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdifPodf {
    #[doc = "divide by 1"]
    LCDIF_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    LCDIF_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    LCDIF_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    LCDIF_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    LCDIF_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    LCDIF_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    LCDIF_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    LCDIF_PODF_7 = 0x07,
}
impl LcdifPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdifPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdifPodf {
    #[inline(always)]
    fn from(val: u8) -> LcdifPodf {
        LcdifPodf::from_bits(val)
    }
}
impl From<LcdifPodf> for u8 {
    #[inline(always)]
    fn from(val: LcdifPodf) -> u8 {
        LcdifPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdifPreClkSel {
    #[doc = "derive clock from PLL2"]
    LCDIF_PRE_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD3"]
    LCDIF_PRE_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5"]
    LCDIF_PRE_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 PFD0"]
    LCDIF_PRE_CLK_SEL_3 = 0x03,
    #[doc = "derive clock from PLL2 PFD1"]
    LCDIF_PRE_CLK_SEL_4 = 0x04,
    #[doc = "derive clock from PLL3 PFD1"]
    LCDIF_PRE_CLK_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LcdifPreClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdifPreClkSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdifPreClkSel {
    #[inline(always)]
    fn from(val: u8) -> LcdifPreClkSel {
        LcdifPreClkSel::from_bits(val)
    }
}
impl From<LcdifPreClkSel> for u8 {
    #[inline(always)]
    fn from(val: LcdifPreClkSel) -> u8 {
        LcdifPreClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdifPred {
    #[doc = "divide by 1"]
    LCDIF_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    LCDIF_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    LCDIF_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    LCDIF_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    LCDIF_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    LCDIF_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    LCDIF_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    LCDIF_PRED_7 = 0x07,
}
impl LcdifPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdifPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdifPred {
    #[inline(always)]
    fn from(val: u8) -> LcdifPred {
        LcdifPred::from_bits(val)
    }
}
impl From<LcdifPred> for u8 {
    #[inline(always)]
    fn from(val: LcdifPred) -> u8 {
        LcdifPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Lpi2cClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkPodf {
        Lpi2cClkPodf::from_bits(val)
    }
}
impl From<Lpi2cClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkPodf) -> u8 {
        Lpi2cClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkSel {
    #[doc = "derive clock from pll3_60m"]
    LPI2C_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk"]
    LPI2C_CLK_SEL_1 = 0x01,
}
impl Lpi2cClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkSel {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkSel {
        Lpi2cClkSel::from_bits(val)
    }
}
impl From<Lpi2cClkSel> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkSel) -> u8 {
        Lpi2cClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpm {
    #[doc = "Remain in run mode"]
    LPM_0 = 0x0,
    #[doc = "Transfer to wait mode"]
    LPM_1 = 0x01,
    #[doc = "Transfer to stop mode"]
    LPM_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpm {
    #[inline(always)]
    fn from(val: u8) -> Lpm {
        Lpm::from_bits(val)
    }
}
impl From<Lpm> for u8 {
    #[inline(always)]
    fn from(val: Lpm) -> u8 {
        Lpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiClkSel {
    #[doc = "derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD0"]
    LPSPI_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2"]
    LPSPI_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 PFD2"]
    LPSPI_CLK_SEL_3 = 0x03,
}
impl LpspiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiClkSel {
    #[inline(always)]
    fn from(val: u8) -> LpspiClkSel {
        LpspiClkSel::from_bits(val)
    }
}
impl From<LpspiClkSel> for u8 {
    #[inline(always)]
    fn from(val: LpspiClkSel) -> u8 {
        LpspiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiPodf {
    #[doc = "divide by 1"]
    LPSPI_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    LPSPI_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    LPSPI_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    LPSPI_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    LPSPI_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    LPSPI_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    LPSPI_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    LPSPI_PODF_7 = 0x07,
}
impl LpspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiPodf {
    #[inline(always)]
    fn from(val: u8) -> LpspiPodf {
        LpspiPodf::from_bits(val)
    }
}
impl From<LpspiPodf> for u8 {
    #[inline(always)]
    fn from(val: LpspiPodf) -> u8 {
        LpspiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LrfPll {
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_0 = 0x0,
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_1 = 0x01,
}
impl LrfPll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LrfPll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LrfPll {
    #[inline(always)]
    fn from(val: u8) -> LrfPll {
        LrfPll::from_bits(val)
    }
}
impl From<LrfPll> for u8 {
    #[inline(always)]
    fn from(val: LrfPll) -> u8 {
        LrfPll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskAhbPodfLoaded {
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    MASK_AHB_PODF_LOADED_0 = 0x0,
    #[doc = "mask interrupt due to frequency change of ahb_podf"]
    MASK_AHB_PODF_LOADED_1 = 0x01,
}
impl MaskAhbPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskAhbPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskAhbPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskAhbPodfLoaded {
        MaskAhbPodfLoaded::from_bits(val)
    }
}
impl From<MaskAhbPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskAhbPodfLoaded) -> u8 {
        MaskAhbPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskCore0Wfi {
    #[doc = "WFI of core0 is not masked"]
    MASK_CORE0_WFI_0 = 0x0,
    #[doc = "WFI of core0 is masked"]
    MASK_CORE0_WFI_1 = 0x01,
}
impl MaskCore0Wfi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskCore0Wfi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskCore0Wfi {
    #[inline(always)]
    fn from(val: u8) -> MaskCore0Wfi {
        MaskCore0Wfi::from_bits(val)
    }
}
impl From<MaskCore0Wfi> for u8 {
    #[inline(always)]
    fn from(val: MaskCore0Wfi) -> u8 {
        MaskCore0Wfi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskCoscReady {
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    MASK_COSC_READY_0 = 0x0,
    #[doc = "mask interrupt due to on board oscillator ready"]
    MASK_COSC_READY_1 = 0x01,
}
impl MaskCoscReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskCoscReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskCoscReady {
    #[inline(always)]
    fn from(val: u8) -> MaskCoscReady {
        MaskCoscReady::from_bits(val)
    }
}
impl From<MaskCoscReady> for u8 {
    #[inline(always)]
    fn from(val: MaskCoscReady) -> u8 {
        MaskCoscReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskL2ccIdle {
    #[doc = "L2CC IDLE is not masked"]
    MASK_L2CC_IDLE_0 = 0x0,
    #[doc = "L2CC IDLE is masked"]
    MASK_L2CC_IDLE_1 = 0x01,
}
impl MaskL2ccIdle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskL2ccIdle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskL2ccIdle {
    #[inline(always)]
    fn from(val: u8) -> MaskL2ccIdle {
        MaskL2ccIdle::from_bits(val)
    }
}
impl From<MaskL2ccIdle> for u8 {
    #[inline(always)]
    fn from(val: MaskL2ccIdle) -> u8 {
        MaskL2ccIdle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskLrfPll {
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    MASK_LRF_PLL_0 = 0x0,
    #[doc = "mask interrupt due to lrf of PLLs"]
    MASK_LRF_PLL_1 = 0x01,
}
impl MaskLrfPll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskLrfPll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskLrfPll {
    #[inline(always)]
    fn from(val: u8) -> MaskLrfPll {
        MaskLrfPll::from_bits(val)
    }
}
impl From<MaskLrfPll> for u8 {
    #[inline(always)]
    fn from(val: MaskLrfPll) -> u8 {
        MaskLrfPll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskPeriph2ClkSelLoaded {
    #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    MASK_PERIPH2_CLK_SEL_LOADED_0 = 0x0,
    #[doc = "mask interrupt due to update of periph2_clk_sel"]
    MASK_PERIPH2_CLK_SEL_LOADED_1 = 0x01,
}
impl MaskPeriph2ClkSelLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskPeriph2ClkSelLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskPeriph2ClkSelLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskPeriph2ClkSelLoaded {
        MaskPeriph2ClkSelLoaded::from_bits(val)
    }
}
impl From<MaskPeriph2ClkSelLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskPeriph2ClkSelLoaded) -> u8 {
        MaskPeriph2ClkSelLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskPeriphClkSelLoaded {
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    MASK_PERIPH_CLK_SEL_LOADED_0 = 0x0,
    #[doc = "mask interrupt due to update of periph_clk_sel"]
    MASK_PERIPH_CLK_SEL_LOADED_1 = 0x01,
}
impl MaskPeriphClkSelLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskPeriphClkSelLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskPeriphClkSelLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskPeriphClkSelLoaded {
        MaskPeriphClkSelLoaded::from_bits(val)
    }
}
impl From<MaskPeriphClkSelLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskPeriphClkSelLoaded) -> u8 {
        MaskPeriphClkSelLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskScuIdle {
    #[doc = "SCU IDLE is not masked"]
    MASK_SCU_IDLE_0 = 0x0,
    #[doc = "SCU IDLE is masked"]
    MASK_SCU_IDLE_1 = 0x01,
}
impl MaskScuIdle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskScuIdle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskScuIdle {
    #[inline(always)]
    fn from(val: u8) -> MaskScuIdle {
        MaskScuIdle::from_bits(val)
    }
}
impl From<MaskScuIdle> for u8 {
    #[inline(always)]
    fn from(val: MaskScuIdle) -> u8 {
        MaskScuIdle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskSemcPodfLoaded {
    #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    MASK_SEMC_PODF_LOADED_0 = 0x0,
    #[doc = "mask interrupt due to frequency change of semc_podf"]
    MASK_SEMC_PODF_LOADED_1 = 0x01,
}
impl MaskSemcPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskSemcPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskSemcPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskSemcPodfLoaded {
        MaskSemcPodfLoaded::from_bits(val)
    }
}
impl From<MaskSemcPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskSemcPodfLoaded) -> u8 {
        MaskSemcPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvCan1Cpi {
    #[doc = "don't overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_0 = 0x0,
    #[doc = "overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_1 = 0x01,
}
impl ModEnOvCan1Cpi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvCan1Cpi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvCan1Cpi {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvCan1Cpi {
        ModEnOvCan1Cpi::from_bits(val)
    }
}
impl From<ModEnOvCan1Cpi> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvCan1Cpi) -> u8 {
        ModEnOvCan1Cpi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvCan2Cpi {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_CAN2_CPI_0 = 0x0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_CAN2_CPI_1 = 0x01,
}
impl ModEnOvCan2Cpi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvCan2Cpi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvCan2Cpi {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvCan2Cpi {
        ModEnOvCan2Cpi::from_bits(val)
    }
}
impl From<ModEnOvCan2Cpi> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvCan2Cpi) -> u8 {
        ModEnOvCan2Cpi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvCanfdCpi {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_CANFD_CPI_0 = 0x0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_CANFD_CPI_1 = 0x01,
}
impl ModEnOvCanfdCpi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvCanfdCpi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvCanfdCpi {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvCanfdCpi {
        ModEnOvCanfdCpi::from_bits(val)
    }
}
impl From<ModEnOvCanfdCpi> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvCanfdCpi) -> u8 {
        ModEnOvCanfdCpi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvGpt {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_GPT_0 = 0x0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_GPT_1 = 0x01,
}
impl ModEnOvGpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvGpt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvGpt {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvGpt {
        ModEnOvGpt::from_bits(val)
    }
}
impl From<ModEnOvGpt> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvGpt) -> u8 {
        ModEnOvGpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvPit {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_PIT_0 = 0x0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_PIT_1 = 0x01,
}
impl ModEnOvPit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvPit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvPit {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvPit {
        ModEnOvPit::from_bits(val)
    }
}
impl From<ModEnOvPit> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvPit) -> u8 {
        ModEnOvPit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvTrng {
    #[doc = "don't override module enable signal"]
    MOD_EN_OV_TRNG_0 = 0x0,
    #[doc = "override module enable signal"]
    MOD_EN_OV_TRNG_1 = 0x01,
}
impl ModEnOvTrng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvTrng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvTrng {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvTrng {
        ModEnOvTrng::from_bits(val)
    }
}
impl From<ModEnOvTrng> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvTrng) -> u8 {
        ModEnOvTrng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnUsdhc {
    #[doc = "don't override module enable signal"]
    MOD_EN_USDHC_0 = 0x0,
    #[doc = "override module enable signal"]
    MOD_EN_USDHC_1 = 0x01,
}
impl ModEnUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnUsdhc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnUsdhc {
    #[inline(always)]
    fn from(val: u8) -> ModEnUsdhc {
        ModEnUsdhc::from_bits(val)
    }
}
impl From<ModEnUsdhc> for u8 {
    #[inline(always)]
    fn from(val: ModEnUsdhc) -> u8 {
        ModEnUsdhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkClkSel {
    #[doc = "derive clock from ipg clk root"]
    PERCLK_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk"]
    PERCLK_CLK_SEL_1 = 0x01,
}
impl PerclkClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkClkSel {
    #[inline(always)]
    fn from(val: u8) -> PerclkClkSel {
        PerclkClkSel::from_bits(val)
    }
}
impl From<PerclkClkSel> for u8 {
    #[inline(always)]
    fn from(val: PerclkClkSel) -> u8 {
        PerclkClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl PerclkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkPodf {
    #[inline(always)]
    fn from(val: u8) -> PerclkPodf {
        PerclkPodf::from_bits(val)
    }
}
impl From<PerclkPodf> for u8 {
    #[inline(always)]
    fn from(val: PerclkPodf) -> u8 {
        PerclkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Periph2ClkSelBusy {
    #[doc = "mux is not busy and its value represents the actual division."]
    PERIPH2_CLK_SEL_BUSY_0 = 0x0,
    #[doc = "mux is busy with handshake process with module. The value read in the periph2_clk_sel represents the previous value of select, and after the handshake periph2_clk_sel value will be applied."]
    PERIPH2_CLK_SEL_BUSY_1 = 0x01,
}
impl Periph2ClkSelBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Periph2ClkSelBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Periph2ClkSelBusy {
    #[inline(always)]
    fn from(val: u8) -> Periph2ClkSelBusy {
        Periph2ClkSelBusy::from_bits(val)
    }
}
impl From<Periph2ClkSelBusy> for u8 {
    #[inline(always)]
    fn from(val: Periph2ClkSelBusy) -> u8 {
        Periph2ClkSelBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Periph2ClkSelLoaded {
    #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_0 = 0x0,
    #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_1 = 0x01,
}
impl Periph2ClkSelLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Periph2ClkSelLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Periph2ClkSelLoaded {
    #[inline(always)]
    fn from(val: u8) -> Periph2ClkSelLoaded {
        Periph2ClkSelLoaded::from_bits(val)
    }
}
impl From<Periph2ClkSelLoaded> for u8 {
    #[inline(always)]
    fn from(val: Periph2ClkSelLoaded) -> u8 {
        Periph2ClkSelLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClk2Podf {
    #[doc = "divide by 1"]
    PERIPH_CLK2_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    PERIPH_CLK2_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    PERIPH_CLK2_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    PERIPH_CLK2_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    PERIPH_CLK2_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    PERIPH_CLK2_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    PERIPH_CLK2_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    PERIPH_CLK2_PODF_7 = 0x07,
}
impl PeriphClk2Podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClk2Podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClk2Podf {
    #[inline(always)]
    fn from(val: u8) -> PeriphClk2Podf {
        PeriphClk2Podf::from_bits(val)
    }
}
impl From<PeriphClk2Podf> for u8 {
    #[inline(always)]
    fn from(val: PeriphClk2Podf) -> u8 {
        PeriphClk2Podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClk2Sel {
    #[doc = "derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk (pll1_ref_clk)"]
    PERIPH_CLK2_SEL_1 = 0x01,
    #[doc = "derive clock from pll2_bypass_clk"]
    PERIPH_CLK2_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PeriphClk2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClk2Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClk2Sel {
    #[inline(always)]
    fn from(val: u8) -> PeriphClk2Sel {
        PeriphClk2Sel::from_bits(val)
    }
}
impl From<PeriphClk2Sel> for u8 {
    #[inline(always)]
    fn from(val: PeriphClk2Sel) -> u8 {
        PeriphClk2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClkSel {
    #[doc = "derive clock from pre_periph_clk_sel"]
    PERIPH_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from periph_clk2_clk_divided"]
    PERIPH_CLK_SEL_1 = 0x01,
}
impl PeriphClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClkSel {
    #[inline(always)]
    fn from(val: u8) -> PeriphClkSel {
        PeriphClkSel::from_bits(val)
    }
}
impl From<PeriphClkSel> for u8 {
    #[inline(always)]
    fn from(val: PeriphClkSel) -> u8 {
        PeriphClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClkSelBusy {
    #[doc = "mux is not busy and its value represents the actual division."]
    PERIPH_CLK_SEL_BUSY_0 = 0x0,
    #[doc = "mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied."]
    PERIPH_CLK_SEL_BUSY_1 = 0x01,
}
impl PeriphClkSelBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClkSelBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClkSelBusy {
    #[inline(always)]
    fn from(val: u8) -> PeriphClkSelBusy {
        PeriphClkSelBusy::from_bits(val)
    }
}
impl From<PeriphClkSelBusy> for u8 {
    #[inline(always)]
    fn from(val: PeriphClkSelBusy) -> u8 {
        PeriphClkSelBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClkSelLoaded {
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_0 = 0x0,
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_1 = 0x01,
}
impl PeriphClkSelLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClkSelLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClkSelLoaded {
    #[inline(always)]
    fn from(val: u8) -> PeriphClkSelLoaded {
        PeriphClkSelLoaded::from_bits(val)
    }
}
impl From<PeriphClkSelLoaded> for u8 {
    #[inline(always)]
    fn from(val: PeriphClkSelLoaded) -> u8 {
        PeriphClkSelLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll3SwClkSel {
    #[doc = "pll3_main_clk"]
    PLL3_SW_CLK_SEL_0 = 0x0,
    #[doc = "pll3 bypass clock"]
    PLL3_SW_CLK_SEL_1 = 0x01,
}
impl Pll3SwClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll3SwClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll3SwClkSel {
    #[inline(always)]
    fn from(val: u8) -> Pll3SwClkSel {
        Pll3SwClkSel::from_bits(val)
    }
}
impl From<Pll3SwClkSel> for u8 {
    #[inline(always)]
    fn from(val: Pll3SwClkSel) -> u8 {
        Pll3SwClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmicDelayScaler {
    #[doc = "clock is not divided"]
    PMIC_DELAY_SCALER_0 = 0x0,
    #[doc = "clock is divided /8"]
    PMIC_DELAY_SCALER_1 = 0x01,
}
impl PmicDelayScaler {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmicDelayScaler {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmicDelayScaler {
    #[inline(always)]
    fn from(val: u8) -> PmicDelayScaler {
        PmicDelayScaler::from_bits(val)
    }
}
impl From<PmicDelayScaler> for u8 {
    #[inline(always)]
    fn from(val: PmicDelayScaler) -> u8 {
        PmicDelayScaler::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrePeriphClkSel {
    #[doc = "derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD2"]
    PRE_PERIPH_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD0"]
    PRE_PERIPH_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from divided PLL1"]
    PRE_PERIPH_CLK_SEL_3 = 0x03,
}
impl PrePeriphClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrePeriphClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrePeriphClkSel {
    #[inline(always)]
    fn from(val: u8) -> PrePeriphClkSel {
        PrePeriphClkSel::from_bits(val)
    }
}
impl From<PrePeriphClkSel> for u8 {
    #[inline(always)]
    fn from(val: PrePeriphClkSel) -> u8 {
        PrePeriphClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbcEn {
    #[doc = "REG_BYPASS_COUNTER disabled"]
    RBC_EN_0 = 0x0,
    #[doc = "REG_BYPASS_COUNTER enabled."]
    RBC_EN_1 = 0x01,
}
impl RbcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbcEn {
    #[inline(always)]
    fn from(val: u8) -> RbcEn {
        RbcEn::from_bits(val)
    }
}
impl From<RbcEn> for u8 {
    #[inline(always)]
    fn from(val: RbcEn) -> u8 {
        RbcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefEnB {
    #[doc = "value of CCM_REF_EN_B is '0'"]
    REF_EN_B_0 = 0x0,
    #[doc = "value of CCM_REF_EN_B is '1'"]
    REF_EN_B_1 = 0x01,
}
impl RefEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefEnB {
    #[inline(always)]
    fn from(val: u8) -> RefEnB {
        RefEnB::from_bits(val)
    }
}
impl From<RefEnB> for u8 {
    #[inline(always)]
    fn from(val: RefEnB) -> u8 {
        RefEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegBypassCount {
    #[doc = "no delay"]
    REG_BYPASS_COUNT_0 = 0x0,
    #[doc = "1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "63 CKIL clock periods delay"]
    REG_BYPASS_COUNT_63 = 0x3f,
}
impl RegBypassCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegBypassCount {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegBypassCount {
    #[inline(always)]
    fn from(val: u8) -> RegBypassCount {
        RegBypassCount::from_bits(val)
    }
}
impl From<RegBypassCount> for u8 {
    #[inline(always)]
    fn from(val: RegBypassCount) -> u8 {
        RegBypassCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1ClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Sai1ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1ClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai1ClkPodf {
        Sai1ClkPodf::from_bits(val)
    }
}
impl From<Sai1ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai1ClkPodf) -> u8 {
        Sai1ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1ClkPred {
    #[doc = "divide by 1"]
    SAI1_CLK_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    SAI1_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    SAI1_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    SAI1_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    SAI1_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    SAI1_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    SAI1_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    SAI1_CLK_PRED_7 = 0x07,
}
impl Sai1ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai1ClkPred {
        Sai1ClkPred::from_bits(val)
    }
}
impl From<Sai1ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai1ClkPred) -> u8 {
        Sai1ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1ClkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI1_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL5"]
    SAI1_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL4"]
    SAI1_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1ClkSel {
        Sai1ClkSel::from_bits(val)
    }
}
impl From<Sai1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1ClkSel) -> u8 {
        Sai1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2ClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Sai2ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2ClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai2ClkPodf {
        Sai2ClkPodf::from_bits(val)
    }
}
impl From<Sai2ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai2ClkPodf) -> u8 {
        Sai2ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2ClkPred {
    #[doc = "divide by 1"]
    SAI2_CLK_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    SAI2_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    SAI2_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    SAI2_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    SAI2_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    SAI2_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    SAI2_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    SAI2_CLK_PRED_7 = 0x07,
}
impl Sai2ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai2ClkPred {
        Sai2ClkPred::from_bits(val)
    }
}
impl From<Sai2ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai2ClkPred) -> u8 {
        Sai2ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2ClkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL5"]
    SAI2_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL4"]
    SAI2_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai2ClkSel {
        Sai2ClkSel::from_bits(val)
    }
}
impl From<Sai2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai2ClkSel) -> u8 {
        Sai2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3ClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Sai3ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3ClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai3ClkPodf {
        Sai3ClkPodf::from_bits(val)
    }
}
impl From<Sai3ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai3ClkPodf) -> u8 {
        Sai3ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3ClkPred {
    #[doc = "divide by 1"]
    SAI3_CLK_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    SAI3_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    SAI3_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    SAI3_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    SAI3_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    SAI3_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    SAI3_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    SAI3_CLK_PRED_7 = 0x07,
}
impl Sai3ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai3ClkPred {
        Sai3ClkPred::from_bits(val)
    }
}
impl From<Sai3ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai3ClkPred) -> u8 {
        Sai3ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3ClkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI3_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL5"]
    SAI3_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL4"]
    SAI3_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai3ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai3ClkSel {
        Sai3ClkSel::from_bits(val)
    }
}
impl From<Sai3ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai3ClkSel) -> u8 {
        Sai3ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbyos {
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    SBYOS_0 = 0x0,
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    SBYOS_1 = 0x01,
}
impl Sbyos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbyos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbyos {
    #[inline(always)]
    fn from(val: u8) -> Sbyos {
        Sbyos::from_bits(val)
    }
}
impl From<Sbyos> for u8 {
    #[inline(always)]
    fn from(val: Sbyos) -> u8 {
        Sbyos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcAltClkSel {
    #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_0 = 0x0,
    #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_1 = 0x01,
}
impl SemcAltClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcAltClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcAltClkSel {
    #[inline(always)]
    fn from(val: u8) -> SemcAltClkSel {
        SemcAltClkSel::from_bits(val)
    }
}
impl From<SemcAltClkSel> for u8 {
    #[inline(always)]
    fn from(val: SemcAltClkSel) -> u8 {
        SemcAltClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcClkSel {
    #[doc = "Periph_clk output will be used as SEMC clock root"]
    SEMC_CLK_SEL_0 = 0x0,
    #[doc = "SEMC alternative clock will be used as SEMC clock root"]
    SEMC_CLK_SEL_1 = 0x01,
}
impl SemcClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcClkSel {
    #[inline(always)]
    fn from(val: u8) -> SemcClkSel {
        SemcClkSel::from_bits(val)
    }
}
impl From<SemcClkSel> for u8 {
    #[inline(always)]
    fn from(val: SemcClkSel) -> u8 {
        SemcClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcPodf {
    #[doc = "divide by 1"]
    SEMC_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    SEMC_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    SEMC_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    SEMC_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    SEMC_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    SEMC_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    SEMC_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    SEMC_PODF_7 = 0x07,
}
impl SemcPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcPodf {
    #[inline(always)]
    fn from(val: u8) -> SemcPodf {
        SemcPodf::from_bits(val)
    }
}
impl From<SemcPodf> for u8 {
    #[inline(always)]
    fn from(val: SemcPodf) -> u8 {
        SemcPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcPodfBusy {
    #[doc = "divider is not busy and its value represents the actual division."]
    SEMC_PODF_BUSY_0 = 0x0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the semc_podf will be applied."]
    SEMC_PODF_BUSY_1 = 0x01,
}
impl SemcPodfBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcPodfBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcPodfBusy {
    #[inline(always)]
    fn from(val: u8) -> SemcPodfBusy {
        SemcPodfBusy::from_bits(val)
    }
}
impl From<SemcPodfBusy> for u8 {
    #[inline(always)]
    fn from(val: SemcPodfBusy) -> u8 {
        SemcPodfBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcPodfLoaded {
    #[doc = "interrupt is not generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_0 = 0x0,
    #[doc = "interrupt generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_1 = 0x01,
}
impl SemcPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> SemcPodfLoaded {
        SemcPodfLoaded::from_bits(val)
    }
}
impl From<SemcPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: SemcPodfLoaded) -> u8 {
        SemcPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0ClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Spdif0ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0ClkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Spdif0ClkPodf {
        Spdif0ClkPodf::from_bits(val)
    }
}
impl From<Spdif0ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Spdif0ClkPodf) -> u8 {
        Spdif0ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0ClkPred {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Spdif0ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Spdif0ClkPred {
        Spdif0ClkPred::from_bits(val)
    }
}
impl From<Spdif0ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Spdif0ClkPred) -> u8 {
        Spdif0ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0ClkSel {
    #[doc = "derive clock from PLL4"]
    SPDIF0_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5"]
    SPDIF0_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk"]
    SPDIF0_CLK_SEL_3 = 0x03,
}
impl Spdif0ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Spdif0ClkSel {
        Spdif0ClkSel::from_bits(val)
    }
}
impl From<Spdif0ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Spdif0ClkSel) -> u8 {
        Spdif0ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StbyCount {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0 = 0x0,
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_1 = 0x01,
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_2 = 0x02,
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_3 = 0x03,
}
impl StbyCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StbyCount {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StbyCount {
    #[inline(always)]
    fn from(val: u8) -> StbyCount {
        StbyCount::from_bits(val)
    }
}
impl From<StbyCount> for u8 {
    #[inline(always)]
    fn from(val: StbyCount) -> u8 {
        StbyCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysMemDsCtrl {
    #[doc = "Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0 = 0x0,
    #[doc = "Enable memory (outside Arm platform) DS mode when system STOP and PLL are disabled"]
    SYS_MEM_DS_CTRL_1 = 0x01,
    #[doc = "enable memory (outside Arm platform) DS mode when system is in STOP mode"]
    SYS_MEM_DS_CTRL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SysMemDsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysMemDsCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysMemDsCtrl {
    #[inline(always)]
    fn from(val: u8) -> SysMemDsCtrl {
        SysMemDsCtrl::from_bits(val)
    }
}
impl From<SysMemDsCtrl> for u8 {
    #[inline(always)]
    fn from(val: SysMemDsCtrl) -> u8 {
        SysMemDsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceClkSel {
    #[doc = "derive clock from PLL2"]
    TRACE_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD2"]
    TRACE_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD0"]
    TRACE_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 PFD1"]
    TRACE_CLK_SEL_3 = 0x03,
}
impl TraceClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceClkSel {
    #[inline(always)]
    fn from(val: u8) -> TraceClkSel {
        TraceClkSel::from_bits(val)
    }
}
impl From<TraceClkSel> for u8 {
    #[inline(always)]
    fn from(val: TraceClkSel) -> u8 {
        TraceClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TracePodf {
    #[doc = "divide by 1"]
    TRACE_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    TRACE_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    TRACE_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    TRACE_PODF_3 = 0x03,
}
impl TracePodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TracePodf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TracePodf {
    #[inline(always)]
    fn from(val: u8) -> TracePodf {
        TracePodf::from_bits(val)
    }
}
impl From<TracePodf> for u8 {
    #[inline(always)]
    fn from(val: TracePodf) -> u8 {
        TracePodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl UartClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartClkPodf {
    #[inline(always)]
    fn from(val: u8) -> UartClkPodf {
        UartClkPodf::from_bits(val)
    }
}
impl From<UartClkPodf> for u8 {
    #[inline(always)]
    fn from(val: UartClkPodf) -> u8 {
        UartClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartClkSel {
    #[doc = "derive clock from pll3_80m"]
    UART_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk"]
    UART_CLK_SEL_1 = 0x01,
}
impl UartClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartClkSel {
    #[inline(always)]
    fn from(val: u8) -> UartClkSel {
        UartClkSel::from_bits(val)
    }
}
impl From<UartClkSel> for u8 {
    #[inline(always)]
    fn from(val: UartClkSel) -> u8 {
        UartClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1ClkSel {
    #[doc = "derive clock from PLL2 PFD2"]
    USDHC1_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD0"]
    USDHC1_CLK_SEL_1 = 0x01,
}
impl Usdhc1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1ClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1ClkSel {
        Usdhc1ClkSel::from_bits(val)
    }
}
impl From<Usdhc1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1ClkSel) -> u8 {
        Usdhc1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1Podf {
    #[doc = "divide by 1"]
    USDHC1_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    USDHC1_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    USDHC1_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    USDHC1_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    USDHC1_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    USDHC1_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    USDHC1_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    USDHC1_PODF_7 = 0x07,
}
impl Usdhc1Podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1Podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1Podf {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1Podf {
        Usdhc1Podf::from_bits(val)
    }
}
impl From<Usdhc1Podf> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1Podf) -> u8 {
        Usdhc1Podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2ClkSel {
    #[doc = "derive clock from PLL2 PFD2"]
    USDHC2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD0"]
    USDHC2_CLK_SEL_1 = 0x01,
}
impl Usdhc2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2ClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2ClkSel {
        Usdhc2ClkSel::from_bits(val)
    }
}
impl From<Usdhc2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2ClkSel) -> u8 {
        Usdhc2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Podf {
    #[doc = "divide by 1"]
    USDHC2_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    USDHC2_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    USDHC2_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    USDHC2_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    USDHC2_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    USDHC2_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    USDHC2_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    USDHC2_PODF_7 = 0x07,
}
impl Usdhc2Podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Podf {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Podf {
        Usdhc2Podf::from_bits(val)
    }
}
impl From<Usdhc2Podf> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Podf) -> u8 {
        Usdhc2Podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vstby {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0 = 0x0,
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    VSTBY_1 = 0x01,
}
impl Vstby {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vstby {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vstby {
    #[inline(always)]
    fn from(val: u8) -> Vstby {
        Vstby::from_bits(val)
    }
}
impl From<Vstby> for u8 {
    #[inline(always)]
    fn from(val: Vstby) -> u8 {
        Vstby::to_bits(val)
    }
}
