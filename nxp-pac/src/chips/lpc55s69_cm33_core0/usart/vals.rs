#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER = 0x0,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK = 0x01,
}
impl Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc {
    #[inline(always)]
    fn from(val: u8) -> Cc {
        Cc::from_bits(val)
    }
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(val: Cc) -> u8 {
        Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkpol {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE = 0x0,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE = 0x01,
}
impl Clkpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkpol {
    #[inline(always)]
    fn from(val: u8) -> Clkpol {
        Clkpol::from_bits(val)
    }
}
impl From<Clkpol> for u8 {
    #[inline(always)]
    fn from(val: Clkpol) -> u8 {
        Clkpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrcconrx {
    #[doc = "No effect. No effect on the CC bit."]
    NO_EFFECT = 0x0,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR = 0x01,
}
impl Clrcconrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrcconrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrcconrx {
    #[inline(always)]
    fn from(val: u8) -> Clrcconrx {
        Clrcconrx::from_bits(val)
    }
}
impl From<Clrcconrx> for u8 {
    #[inline(always)]
    fn from(val: Clrcconrx) -> u8 {
        Clrcconrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datalen {
    #[doc = "7 bit Data length."]
    BIT_7 = 0x0,
    #[doc = "8 bit Data length."]
    BIT_8 = 0x01,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    BIT_9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Datalen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datalen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datalen {
    #[inline(always)]
    fn from(val: u8) -> Datalen {
        Datalen::from_bits(val)
    }
}
impl From<Datalen> for u8 {
    #[inline(always)]
    fn from(val: Datalen) -> u8 {
        Datalen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loop {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK = 0x01,
}
impl Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loop {
    #[inline(always)]
    fn from(val: u8) -> Loop {
        Loop::from_bits(val)
    }
}
impl From<Loop> for u8 {
    #[inline(always)]
    fn from(val: Loop) -> u8 {
        Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oepol {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    LOW = 0x0,
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    HIGH = 0x01,
}
impl Oepol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oepol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oepol {
    #[inline(always)]
    fn from(val: u8) -> Oepol {
        Oepol::from_bits(val)
    }
}
impl From<Oepol> for u8 {
    #[inline(always)]
    fn from(val: Oepol) -> u8 {
        Oepol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oesel {
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    STANDARD = 0x0,
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    RS_485 = 0x01,
}
impl Oesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oesel {
    #[inline(always)]
    fn from(val: u8) -> Oesel {
        Oesel::from_bits(val)
    }
}
impl From<Oesel> for u8 {
    #[inline(always)]
    fn from(val: Oesel) -> u8 {
        Oesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paritysel {
    #[doc = "No parity."]
    NO_PARITY = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY = 0x02,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY = 0x03,
}
impl Paritysel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paritysel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paritysel {
    #[inline(always)]
    fn from(val: u8) -> Paritysel {
        Paritysel::from_bits(val)
    }
}
impl From<Paritysel> for u8 {
    #[inline(always)]
    fn from(val: Paritysel) -> u8 {
        Paritysel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpol {
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0x0,
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 0x01,
}
impl Rxpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpol {
    #[inline(always)]
    fn from(val: u8) -> Rxpol {
        Rxpol::from_bits(val)
    }
}
impl From<Rxpol> for u8 {
    #[inline(always)]
    fn from(val: Rxpol) -> u8 {
        Rxpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stoplen {
    #[doc = "1 stop bit."]
    BIT_1 = 0x0,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    BITS_2 = 0x01,
}
impl Stoplen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stoplen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stoplen {
    #[inline(always)]
    fn from(val: u8) -> Stoplen {
        Stoplen::from_bits(val)
    }
}
impl From<Stoplen> for u8 {
    #[inline(always)]
    fn from(val: Stoplen) -> u8 {
        Stoplen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syncen {
    #[doc = "Asynchronous mode."]
    ASYNCHRONOUS_MODE = 0x0,
    #[doc = "Synchronous mode."]
    SYNCHRONOUS_MODE = 0x01,
}
impl Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syncen {
    #[inline(always)]
    fn from(val: u8) -> Syncen {
        Syncen::from_bits(val)
    }
}
impl From<Syncen> for u8 {
    #[inline(always)]
    fn from(val: Syncen) -> u8 {
        Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syncmst {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE = 0x0,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    MASTER = 0x01,
}
impl Syncmst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syncmst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syncmst {
    #[inline(always)]
    fn from(val: u8) -> Syncmst {
        Syncmst::from_bits(val)
    }
}
impl From<Syncmst> for u8 {
    #[inline(always)]
    fn from(val: Syncmst) -> u8 {
        Syncmst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txbrken {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS = 0x01,
}
impl Txbrken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txbrken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txbrken {
    #[inline(always)]
    fn from(val: u8) -> Txbrken {
        Txbrken::from_bits(val)
    }
}
impl From<Txbrken> for u8 {
    #[inline(always)]
    fn from(val: Txbrken) -> u8 {
        Txbrken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdis {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    ENABLED = 0x0,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED = 0x01,
}
impl Txdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdis {
    #[inline(always)]
    fn from(val: u8) -> Txdis {
        Txdis::from_bits(val)
    }
}
impl From<Txdis> for u8 {
    #[inline(always)]
    fn from(val: Txdis) -> u8 {
        Txdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpol {
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0x0,
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 0x01,
}
impl Txpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpol {
    #[inline(always)]
    fn from(val: u8) -> Txpol {
        Txpol::from_bits(val)
    }
}
impl From<Txpol> for u8 {
    #[inline(always)]
    fn from(val: Txpol) -> u8 {
        Txpol::to_bits(val)
    }
}
