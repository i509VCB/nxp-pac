#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autoack {
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    NORMAL = 0x0,
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    AUTOMATIC_ACK = 0x01,
}
impl Autoack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autoack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autoack {
    #[inline(always)]
    fn from(val: u8) -> Autoack {
        Autoack::from_bits(val)
    }
}
impl From<Autoack> for u8 {
    #[inline(always)]
    fn from(val: Autoack) -> u8 {
        Autoack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Automatchread {
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    I2C_WRITE = 0x0,
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    I2C_READ = 0x01,
}
impl Automatchread {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Automatchread {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Automatchread {
    #[inline(always)]
    fn from(val: u8) -> Automatchread {
        Automatchread::from_bits(val)
    }
}
impl From<Automatchread> for u8 {
    #[inline(always)]
    fn from(val: Automatchread) -> u8 {
        Automatchread::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autonack {
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    NORMAL = 0x0,
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC = 0x01,
}
impl Autonack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autonack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autonack {
    #[inline(always)]
    fn from(val: u8) -> Autonack {
        Autonack::from_bits(val)
    }
}
impl From<Autonack> for u8 {
    #[inline(always)]
    fn from(val: Autonack) -> u8 {
        Autonack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eventtimeout {
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    NO_TIMEOUT = 0x0,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    EVEN_TIMEOUT = 0x01,
}
impl Eventtimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eventtimeout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eventtimeout {
    #[inline(always)]
    fn from(val: u8) -> Eventtimeout {
        Eventtimeout::from_bits(val)
    }
}
impl From<Eventtimeout> for u8 {
    #[inline(always)]
    fn from(val: Eventtimeout) -> u8 {
        Eventtimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hscapable {
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    FAST_MODE_PLUS = 0x0,
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HIGH_SPEED = 0x01,
}
impl Hscapable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hscapable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hscapable {
    #[inline(always)]
    fn from(val: u8) -> Hscapable {
        Hscapable::from_bits(val)
    }
}
impl From<Hscapable> for u8 {
    #[inline(always)]
    fn from(val: Hscapable) -> u8 {
        Hscapable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Monnack {
    #[doc = "Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    ACKNOWLEDGED = 0x0,
    #[doc = "Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    NOT_ACKNOWLEDGED = 0x01,
}
impl Monnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Monnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Monnack {
    #[inline(always)]
    fn from(val: u8) -> Monnack {
        Monnack::from_bits(val)
    }
}
impl From<Monnack> for u8 {
    #[inline(always)]
    fn from(val: Monnack) -> u8 {
        Monnack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Monrdy {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA = 0x0,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING = 0x01,
}
impl Monrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Monrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Monrdy {
    #[inline(always)]
    fn from(val: u8) -> Monrdy {
        Monrdy::from_bits(val)
    }
}
impl From<Monrdy> for u8 {
    #[inline(always)]
    fn from(val: Monrdy) -> u8 {
        Monrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstarbloss {
    #[doc = "No Arbitration Loss has occurred."]
    NO_LOSS = 0x0,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS = 0x01,
}
impl Mstarbloss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstarbloss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstarbloss {
    #[inline(always)]
    fn from(val: u8) -> Mstarbloss {
        Mstarbloss::from_bits(val)
    }
}
impl From<Mstarbloss> for u8 {
    #[inline(always)]
    fn from(val: Mstarbloss) -> u8 {
        Mstarbloss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstcontinue {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE = 0x01,
}
impl Mstcontinue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstcontinue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstcontinue {
    #[inline(always)]
    fn from(val: u8) -> Mstcontinue {
        Mstcontinue::from_bits(val)
    }
}
impl From<Mstcontinue> for u8 {
    #[inline(always)]
    fn from(val: Mstcontinue) -> u8 {
        Mstcontinue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstpending {
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    IN_PROGRESS = 0x0,
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    PENDING = 0x01,
}
impl Mstpending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstpending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstpending {
    #[inline(always)]
    fn from(val: u8) -> Mstpending {
        Mstpending::from_bits(val)
    }
}
impl From<Mstpending> for u8 {
    #[inline(always)]
    fn from(val: Mstpending) -> u8 {
        Mstpending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstsclhigh {
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    CLOCKS_2 = 0x0,
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    CLOCKS_3 = 0x01,
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    CLOCKS_4 = 0x02,
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    CLOCKS_5 = 0x03,
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    CLOCKS_6 = 0x04,
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    CLOCKS_7 = 0x05,
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    CLOCKS_8 = 0x06,
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9 = 0x07,
}
impl Mstsclhigh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstsclhigh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstsclhigh {
    #[inline(always)]
    fn from(val: u8) -> Mstsclhigh {
        Mstsclhigh::from_bits(val)
    }
}
impl From<Mstsclhigh> for u8 {
    #[inline(always)]
    fn from(val: Mstsclhigh) -> u8 {
        Mstsclhigh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstscllow {
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    CLOCKS_2 = 0x0,
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    CLOCKS_3 = 0x01,
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    CLOCKS_4 = 0x02,
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    CLOCKS_5 = 0x03,
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    CLOCKS_6 = 0x04,
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    CLOCKS_7 = 0x05,
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    CLOCKS_8 = 0x06,
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9 = 0x07,
}
impl Mstscllow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstscllow {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstscllow {
    #[inline(always)]
    fn from(val: u8) -> Mstscllow {
        Mstscllow::from_bits(val)
    }
}
impl From<Mstscllow> for u8 {
    #[inline(always)]
    fn from(val: Mstscllow) -> u8 {
        Mstscllow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mststart {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START = 0x01,
}
impl Mststart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mststart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mststart {
    #[inline(always)]
    fn from(val: u8) -> Mststart {
        Mststart::from_bits(val)
    }
}
impl From<Mststart> for u8 {
    #[inline(always)]
    fn from(val: Mststart) -> u8 {
        Mststart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mststate {
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    IDLE = 0x0,
    #[doc = "Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY = 0x01,
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY = 0x02,
    #[doc = "NACK Address. Slave NACKed address."]
    NACK_ADDRESS = 0x03,
    #[doc = "NACK Data. Slave NACKed transmitted data."]
    NACK_DATA = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mststate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mststate {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mststate {
    #[inline(always)]
    fn from(val: u8) -> Mststate {
        Mststate::from_bits(val)
    }
}
impl From<Mststate> for u8 {
    #[inline(always)]
    fn from(val: Mststate) -> u8 {
        Mststate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mststop {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP = 0x01,
}
impl Mststop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mststop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mststop {
    #[inline(always)]
    fn from(val: u8) -> Mststop {
        Mststop::from_bits(val)
    }
}
impl From<Mststop> for u8 {
    #[inline(always)]
    fn from(val: Mststop) -> u8 {
        Mststop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qualmode0 {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK = 0x0,
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND = 0x01,
}
impl Qualmode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qualmode0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qualmode0 {
    #[inline(always)]
    fn from(val: u8) -> Qualmode0 {
        Qualmode0::from_bits(val)
    }
}
impl From<Qualmode0> for u8 {
    #[inline(always)]
    fn from(val: Qualmode0) -> u8 {
        Qualmode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvadr0Sadisable {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl Slvadr0Sadisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvadr0Sadisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvadr0Sadisable {
    #[inline(always)]
    fn from(val: u8) -> Slvadr0Sadisable {
        Slvadr0Sadisable::from_bits(val)
    }
}
impl From<Slvadr0Sadisable> for u8 {
    #[inline(always)]
    fn from(val: Slvadr0Sadisable) -> u8 {
        Slvadr0Sadisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvadr1Sadisable {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl Slvadr1Sadisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvadr1Sadisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvadr1Sadisable {
    #[inline(always)]
    fn from(val: u8) -> Slvadr1Sadisable {
        Slvadr1Sadisable::from_bits(val)
    }
}
impl From<Slvadr1Sadisable> for u8 {
    #[inline(always)]
    fn from(val: Slvadr1Sadisable) -> u8 {
        Slvadr1Sadisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvadr2Sadisable {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl Slvadr2Sadisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvadr2Sadisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvadr2Sadisable {
    #[inline(always)]
    fn from(val: u8) -> Slvadr2Sadisable {
        Slvadr2Sadisable::from_bits(val)
    }
}
impl From<Slvadr2Sadisable> for u8 {
    #[inline(always)]
    fn from(val: Slvadr2Sadisable) -> u8 {
        Slvadr2Sadisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvadr3Sadisable {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl Slvadr3Sadisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvadr3Sadisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvadr3Sadisable {
    #[inline(always)]
    fn from(val: u8) -> Slvadr3Sadisable {
        Slvadr3Sadisable::from_bits(val)
    }
}
impl From<Slvadr3Sadisable> for u8 {
    #[inline(always)]
    fn from(val: Slvadr3Sadisable) -> u8 {
        Slvadr3Sadisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvcontinue {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    CONTINUE = 0x01,
}
impl Slvcontinue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvcontinue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvcontinue {
    #[inline(always)]
    fn from(val: u8) -> Slvcontinue {
        Slvcontinue::from_bits(val)
    }
}
impl From<Slvcontinue> for u8 {
    #[inline(always)]
    fn from(val: Slvcontinue) -> u8 {
        Slvcontinue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvidx {
    #[doc = "Address 0. Slave address 0 was matched."]
    ADDRESS0 = 0x0,
    #[doc = "Address 1. Slave address 1 was matched."]
    ADDRESS1 = 0x01,
    #[doc = "Address 2. Slave address 2 was matched."]
    ADDRESS2 = 0x02,
    #[doc = "Address 3. Slave address 3 was matched."]
    ADDRESS3 = 0x03,
}
impl Slvidx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvidx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvidx {
    #[inline(always)]
    fn from(val: u8) -> Slvidx {
        Slvidx::from_bits(val)
    }
}
impl From<Slvidx> for u8 {
    #[inline(always)]
    fn from(val: Slvidx) -> u8 {
        Slvidx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvnack {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK = 0x01,
}
impl Slvnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvnack {
    #[inline(always)]
    fn from(val: u8) -> Slvnack {
        Slvnack::from_bits(val)
    }
}
impl From<Slvnack> for u8 {
    #[inline(always)]
    fn from(val: Slvnack) -> u8 {
        Slvnack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvnotstr {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING = 0x0,
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING = 0x01,
}
impl Slvnotstr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvnotstr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvnotstr {
    #[inline(always)]
    fn from(val: u8) -> Slvnotstr {
        Slvnotstr::from_bits(val)
    }
}
impl From<Slvnotstr> for u8 {
    #[inline(always)]
    fn from(val: Slvnotstr) -> u8 {
        Slvnotstr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvpending {
    #[doc = "In progress. The Slave function does not currently need service."]
    IN_PROGRESS = 0x0,
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    PENDING = 0x01,
}
impl Slvpending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvpending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvpending {
    #[inline(always)]
    fn from(val: u8) -> Slvpending {
        Slvpending::from_bits(val)
    }
}
impl From<Slvpending> for u8 {
    #[inline(always)]
    fn from(val: Slvpending) -> u8 {
        Slvpending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvstate {
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SLAVE_ADDRESS = 0x0,
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    SLAVE_RECEIVE = 0x01,
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SLAVE_TRANSMIT = 0x02,
    _RESERVED_3 = 0x03,
}
impl Slvstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvstate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvstate {
    #[inline(always)]
    fn from(val: u8) -> Slvstate {
        Slvstate::from_bits(val)
    }
}
impl From<Slvstate> for u8 {
    #[inline(always)]
    fn from(val: Slvstate) -> u8 {
        Slvstate::to_bits(val)
    }
}
