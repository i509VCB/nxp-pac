#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderrcode {
    #[doc = "No error."]
    AHBCMDERRCODE_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence."]
    AHBCMDERRCODE_2 = 0x02,
    #[doc = "There is unknown instruction opcode in the sequence."]
    AHBCMDERRCODE_3 = 0x03,
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    AHBCMDERRCODE_4 = 0x04,
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    AHBCMDERRCODE_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout."]
    AHBCMDERRCODE_14 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ahbcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderrcode {
        Ahbcmderrcode::from_bits(val)
    }
}
impl From<Ahbcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderrcode) -> u8 {
        Ahbcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aparen {
    #[doc = "Flash will be accessed in Individual mode."]
    APAREN_0 = 0x0,
    #[doc = "Flash will be accessed in Parallel mode."]
    APAREN_1 = 0x01,
}
impl Aparen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aparen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aparen {
    #[inline(always)]
    fn from(val: u8) -> Aparen {
        Aparen::from_bits(val)
    }
}
impl From<Aparen> for u8 {
    #[inline(always)]
    fn from(val: Aparen) -> u8 {
        Aparen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbcmdsrc {
    #[doc = "Triggered by AHB read command (triggered by AHB read)."]
    ARBCMDSRC_0 = 0x0,
    #[doc = "Triggered by AHB write command (triggered by AHB Write)."]
    ARBCMDSRC_1 = 0x01,
    #[doc = "Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
    ARBCMDSRC_2 = 0x02,
    #[doc = "Triggered by suspended command (resumed)."]
    ARBCMDSRC_3 = 0x03,
}
impl Arbcmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbcmdsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbcmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Arbcmdsrc {
        Arbcmdsrc::from_bits(val)
    }
}
impl From<Arbcmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Arbcmdsrc) -> u8 {
        Arbcmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Awrwaitunit {
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    AWRWAITUNIT_0 = 0x0,
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    AWRWAITUNIT_1 = 0x01,
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    AWRWAITUNIT_2 = 0x02,
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    AWRWAITUNIT_3 = 0x03,
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    AWRWAITUNIT_4 = 0x04,
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    AWRWAITUNIT_5 = 0x05,
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    AWRWAITUNIT_6 = 0x06,
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    AWRWAITUNIT_7 = 0x07,
}
impl Awrwaitunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Awrwaitunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Awrwaitunit {
    #[inline(always)]
    fn from(val: u8) -> Awrwaitunit {
        Awrwaitunit::from_bits(val)
    }
}
impl From<Awrwaitunit> for u8 {
    #[inline(always)]
    fn from(val: Awrwaitunit) -> u8 {
        Awrwaitunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bufferableen {
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    BUFFERABLEEN_0 = 0x0,
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    BUFFERABLEEN_1 = 0x01,
}
impl Bufferableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bufferableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bufferableen {
    #[inline(always)]
    fn from(val: u8) -> Bufferableen {
        Bufferableen::from_bits(val)
    }
}
impl From<Bufferableen> for u8 {
    #[inline(always)]
    fn from(val: Bufferableen) -> u8 {
        Bufferableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cachableen {
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    CACHABLEEN_0 = 0x0,
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    CACHABLEEN_1 = 0x01,
}
impl Cachableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cachableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cachableen {
    #[inline(always)]
    fn from(val: u8) -> Cachableen {
        Cachableen::from_bits(val)
    }
}
impl From<Cachableen> for u8 {
    #[inline(always)]
    fn from(val: Cachableen) -> u8 {
        Cachableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbbufopt {
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_0 = 0x0,
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_1 = 0x01,
}
impl Clrahbbufopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbbufopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbbufopt {
    #[inline(always)]
    fn from(val: u8) -> Clrahbbufopt {
        Clrahbbufopt::from_bits(val)
    }
}
impl From<Clrahbbufopt> for u8 {
    #[inline(always)]
    fn from(val: Clrahbbufopt) -> u8 {
        Clrahbbufopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csintervalunit {
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    CSINTERVALUNIT_0 = 0x0,
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    CSINTERVALUNIT_1 = 0x01,
}
impl Csintervalunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csintervalunit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csintervalunit {
    #[inline(always)]
    fn from(val: u8) -> Csintervalunit {
        Csintervalunit::from_bits(val)
    }
}
impl From<Csintervalunit> for u8 {
    #[inline(always)]
    fn from(val: Csintervalunit) -> u8 {
        Csintervalunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "Doze mode support disabled. AHB clock and serial clock will not be gated off when there is doze mode request from system."]
    DOZEEN_0 = 0x0,
    #[doc = "Doze mode support enabled. AHB clock and serial clock will be gated off when there is doze mode request from system."]
    DOZEEN_1 = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsen {
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands."]
    HSEN_0 = 0x0,
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands."]
    HSEN_1 = 0x01,
}
impl Hsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsen {
    #[inline(always)]
    fn from(val: u8) -> Hsen {
        Hsen::from_bits(val)
    }
}
impl From<Hsen> for u8 {
    #[inline(always)]
    fn from(val: Hsen) -> u8 {
        Hsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iparen {
    #[doc = "Flash will be accessed in Individual mode."]
    IPAREN_0 = 0x0,
    #[doc = "Flash will be accessed in Parallel mode."]
    IPAREN_1 = 0x01,
}
impl Iparen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iparen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iparen {
    #[inline(always)]
    fn from(val: u8) -> Iparen {
        Iparen::from_bits(val)
    }
}
impl From<Iparen> for u8 {
    #[inline(always)]
    fn from(val: Iparen) -> u8 {
        Iparen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderrcode {
    #[doc = "No error."]
    IPCMDERRCODE_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence."]
    IPCMDERRCODE_2 = 0x02,
    #[doc = "There is unknown instruction opcode in the sequence."]
    IPCMDERRCODE_3 = 0x03,
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    IPCMDERRCODE_4 = 0x04,
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    IPCMDERRCODE_5 = 0x05,
    #[doc = "Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    IPCMDERRCODE_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout."]
    IPCMDERRCODE_14 = 0x0e,
    #[doc = "Flash boundary crossed."]
    IPCMDERRCODE_15 = 0x0f,
}
impl Ipcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderrcode {
        Ipcmderrcode::from_bits(val)
    }
}
impl From<Ipcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderrcode) -> u8 {
        Ipcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Learnen {
    #[doc = "Disable."]
    LEARNEN_0 = 0x0,
    #[doc = "Enable."]
    LEARNEN_1 = 0x01,
}
impl Learnen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Learnen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Learnen {
    #[inline(always)]
    fn from(val: u8) -> Learnen {
        Learnen::from_bits(val)
    }
}
impl From<Learnen> for u8 {
    #[inline(always)]
    fn from(val: Learnen) -> u8 {
        Learnen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readaddropt {
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    READADDROPT_0 = 0x0,
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
    READADDROPT_1 = 0x01,
}
impl Readaddropt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readaddropt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readaddropt {
    #[inline(always)]
    fn from(val: u8) -> Readaddropt {
        Readaddropt::from_bits(val)
    }
}
impl From<Readaddropt> for u8 {
    #[inline(always)]
    fn from(val: Readaddropt) -> u8 {
        Readaddropt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remapen {
    #[doc = "HADDR REMAP Disabled"]
    REMAPEN_0 = 0x0,
    #[doc = "HADDR REMAP Enabled"]
    REMAPEN_1 = 0x01,
}
impl Remapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remapen {
    #[inline(always)]
    fn from(val: u8) -> Remapen {
        Remapen::from_bits(val)
    }
}
impl From<Remapen> for u8 {
    #[inline(always)]
    fn from(val: Remapen) -> u8 {
        Remapen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxclksrc {
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
    RXCLKSRC_0 = 0x0,
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
    RXCLKSRC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Flash provided Read strobe and input from DQS pad"]
    RXCLKSRC_3 = 0x03,
}
impl Rxclksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxclksrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxclksrc {
    #[inline(always)]
    fn from(val: u8) -> Rxclksrc {
        Rxclksrc::from_bits(val)
    }
}
impl From<Rxclksrc> for u8 {
    #[inline(always)]
    fn from(val: Rxclksrc) -> u8 {
        Rxclksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdmaen {
    #[doc = "IP RX FIFO would be read by processor."]
    RXDMAEN_0 = 0x0,
    #[doc = "IP RX FIFO would be read by DMA."]
    RXDMAEN_1 = 0x01,
}
impl Rxdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdmaen {
    #[inline(always)]
    fn from(val: u8) -> Rxdmaen {
        Rxdmaen::from_bits(val)
    }
}
impl From<Rxdmaen> for u8 {
    #[inline(always)]
    fn from(val: Rxdmaen) -> u8 {
        Rxdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Samedeviceen {
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    SAMEDEVICEEN_0 = 0x0,
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    SAMEDEVICEEN_1 = 0x01,
}
impl Samedeviceen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Samedeviceen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Samedeviceen {
    #[inline(always)]
    fn from(val: u8) -> Samedeviceen {
        Samedeviceen::from_bits(val)
    }
}
impl From<Samedeviceen> for u8 {
    #[inline(always)]
    fn from(val: Samedeviceen) -> u8 {
        Samedeviceen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckbdiffopt {
    #[doc = "B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
    SCKBDIFFOPT_0 = 0x0,
    #[doc = "B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
    SCKBDIFFOPT_1 = 0x01,
}
impl Sckbdiffopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckbdiffopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckbdiffopt {
    #[inline(always)]
    fn from(val: u8) -> Sckbdiffopt {
        Sckbdiffopt::from_bits(val)
    }
}
impl From<Sckbdiffopt> for u8 {
    #[inline(always)]
    fn from(val: Sckbdiffopt) -> u8 {
        Sckbdiffopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckfreerunen {
    #[doc = "Disable."]
    SCKFREERUNEN_0 = 0x0,
    #[doc = "Enable."]
    SCKFREERUNEN_1 = 0x01,
}
impl Sckfreerunen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckfreerunen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckfreerunen {
    #[inline(always)]
    fn from(val: u8) -> Sckfreerunen {
        Sckfreerunen::from_bits(val)
    }
}
impl From<Sckfreerunen> for u8 {
    #[inline(always)]
    fn from(val: Sckfreerunen) -> u8 {
        Sckfreerunen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Serclkdiv {
    #[doc = "Divided by 1"]
    SERCLKDIV_0 = 0x0,
    #[doc = "Divided by 2"]
    SERCLKDIV_1 = 0x01,
    #[doc = "Divided by 3"]
    SERCLKDIV_2 = 0x02,
    #[doc = "Divided by 4"]
    SERCLKDIV_3 = 0x03,
    #[doc = "Divided by 5"]
    SERCLKDIV_4 = 0x04,
    #[doc = "Divided by 6"]
    SERCLKDIV_5 = 0x05,
    #[doc = "Divided by 7"]
    SERCLKDIV_6 = 0x06,
    #[doc = "Divided by 8"]
    SERCLKDIV_7 = 0x07,
}
impl Serclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Serclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Serclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Serclkdiv {
        Serclkdiv::from_bits(val)
    }
}
impl From<Serclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Serclkdiv) -> u8 {
        Serclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdmaen {
    #[doc = "IP TX FIFO would be filled by processor."]
    TXDMAEN_0 = 0x0,
    #[doc = "IP TX FIFO would be filled by DMA."]
    TXDMAEN_1 = 0x01,
}
impl Txdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdmaen {
    #[inline(always)]
    fn from(val: u8) -> Txdmaen {
        Txdmaen::from_bits(val)
    }
}
impl From<Txdmaen> for u8 {
    #[inline(always)]
    fn from(val: Txdmaen) -> u8 {
        Txdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmena {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENA_0 = 0x0,
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENA_1 = 0x01,
}
impl Wmena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmena {
    #[inline(always)]
    fn from(val: u8) -> Wmena {
        Wmena::from_bits(val)
    }
}
impl From<Wmena> for u8 {
    #[inline(always)]
    fn from(val: Wmena) -> u8 {
        Wmena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmenb {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENB_0 = 0x0,
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENB_1 = 0x01,
}
impl Wmenb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmenb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmenb {
    #[inline(always)]
    fn from(val: u8) -> Wmenb {
        Wmenb::from_bits(val)
    }
}
impl From<Wmenb> for u8 {
    #[inline(always)]
    fn from(val: Wmenb) -> u8 {
        Wmenb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmopt1 {
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_0 = 0x0,
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_1 = 0x01,
}
impl Wmopt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmopt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmopt1 {
    #[inline(always)]
    fn from(val: u8) -> Wmopt1 {
        Wmopt1::from_bits(val)
    }
}
impl From<Wmopt1> for u8 {
    #[inline(always)]
    fn from(val: Wmopt1) -> u8 {
        Wmopt1::to_bits(val)
    }
}
