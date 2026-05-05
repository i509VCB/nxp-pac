#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUTOACK {
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    NORMAL = 0x0,
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    AUTOMATIC_ACK = 0x01,
}
impl AUTOACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUTOACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUTOACK {
    #[inline(always)]
    fn from(val: u8) -> AUTOACK {
        AUTOACK::from_bits(val)
    }
}
impl From<AUTOACK> for u8 {
    #[inline(always)]
    fn from(val: AUTOACK) -> u8 {
        AUTOACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUTOMATCHREAD {
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    I2C_WRITE = 0x0,
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    I2C_READ = 0x01,
}
impl AUTOMATCHREAD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUTOMATCHREAD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUTOMATCHREAD {
    #[inline(always)]
    fn from(val: u8) -> AUTOMATCHREAD {
        AUTOMATCHREAD::from_bits(val)
    }
}
impl From<AUTOMATCHREAD> for u8 {
    #[inline(always)]
    fn from(val: AUTOMATCHREAD) -> u8 {
        AUTOMATCHREAD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUTONACK {
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    NORMAL = 0x0,
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC = 0x01,
}
impl AUTONACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUTONACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUTONACK {
    #[inline(always)]
    fn from(val: u8) -> AUTONACK {
        AUTONACK::from_bits(val)
    }
}
impl From<AUTONACK> for u8 {
    #[inline(always)]
    fn from(val: AUTONACK) -> u8 {
        AUTONACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVENTTIMEOUT {
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    NO_TIMEOUT = 0x0,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    EVEN_TIMEOUT = 0x01,
}
impl EVENTTIMEOUT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVENTTIMEOUT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVENTTIMEOUT {
    #[inline(always)]
    fn from(val: u8) -> EVENTTIMEOUT {
        EVENTTIMEOUT::from_bits(val)
    }
}
impl From<EVENTTIMEOUT> for u8 {
    #[inline(always)]
    fn from(val: EVENTTIMEOUT) -> u8 {
        EVENTTIMEOUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HSCAPABLE {
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,."]
    FAST_MODE_PLUS = 0x0,
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HIGH_SPEED = 0x01,
}
impl HSCAPABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HSCAPABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HSCAPABLE {
    #[inline(always)]
    fn from(val: u8) -> HSCAPABLE {
        HSCAPABLE::from_bits(val)
    }
}
impl From<HSCAPABLE> for u8 {
    #[inline(always)]
    fn from(val: HSCAPABLE) -> u8 {
        HSCAPABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MONNACK {
    #[doc = "Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    ACKNOWLEDGED = 0x0,
    #[doc = "Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    NOT_ACKNOWLEDGED = 0x01,
}
impl MONNACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MONNACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MONNACK {
    #[inline(always)]
    fn from(val: u8) -> MONNACK {
        MONNACK::from_bits(val)
    }
}
impl From<MONNACK> for u8 {
    #[inline(always)]
    fn from(val: MONNACK) -> u8 {
        MONNACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MONRDY {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA = 0x0,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING = 0x01,
}
impl MONRDY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MONRDY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MONRDY {
    #[inline(always)]
    fn from(val: u8) -> MONRDY {
        MONRDY::from_bits(val)
    }
}
impl From<MONRDY> for u8 {
    #[inline(always)]
    fn from(val: MONRDY) -> u8 {
        MONRDY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTARBLOSS {
    #[doc = "No Arbitration Loss has occurred."]
    NO_LOSS = 0x0,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS = 0x01,
}
impl MSTARBLOSS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTARBLOSS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTARBLOSS {
    #[inline(always)]
    fn from(val: u8) -> MSTARBLOSS {
        MSTARBLOSS::from_bits(val)
    }
}
impl From<MSTARBLOSS> for u8 {
    #[inline(always)]
    fn from(val: MSTARBLOSS) -> u8 {
        MSTARBLOSS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTCONTINUE {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE = 0x01,
}
impl MSTCONTINUE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTCONTINUE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTCONTINUE {
    #[inline(always)]
    fn from(val: u8) -> MSTCONTINUE {
        MSTCONTINUE::from_bits(val)
    }
}
impl From<MSTCONTINUE> for u8 {
    #[inline(always)]
    fn from(val: MSTCONTINUE) -> u8 {
        MSTCONTINUE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTPENDING {
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    IN_PROGRESS = 0x0,
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    PENDING = 0x01,
}
impl MSTPENDING {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTPENDING {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTPENDING {
    #[inline(always)]
    fn from(val: u8) -> MSTPENDING {
        MSTPENDING::from_bits(val)
    }
}
impl From<MSTPENDING> for u8 {
    #[inline(always)]
    fn from(val: MSTPENDING) -> u8 {
        MSTPENDING::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTSCLHIGH {
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    CLOCKS_2 = 0x0,
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider."]
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
impl MSTSCLHIGH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTSCLHIGH {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTSCLHIGH {
    #[inline(always)]
    fn from(val: u8) -> MSTSCLHIGH {
        MSTSCLHIGH::from_bits(val)
    }
}
impl From<MSTSCLHIGH> for u8 {
    #[inline(always)]
    fn from(val: MSTSCLHIGH) -> u8 {
        MSTSCLHIGH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTSCLLOW {
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
impl MSTSCLLOW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTSCLLOW {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTSCLLOW {
    #[inline(always)]
    fn from(val: u8) -> MSTSCLLOW {
        MSTSCLLOW::from_bits(val)
    }
}
impl From<MSTSCLLOW> for u8 {
    #[inline(always)]
    fn from(val: MSTSCLLOW) -> u8 {
        MSTSCLLOW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTSTART {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START = 0x01,
}
impl MSTSTART {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTSTART {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTSTART {
    #[inline(always)]
    fn from(val: u8) -> MSTSTART {
        MSTSTART::from_bits(val)
    }
}
impl From<MSTSTART> for u8 {
    #[inline(always)]
    fn from(val: MSTSTART) -> u8 {
        MSTSTART::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTSTATE {
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
impl MSTSTATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTSTATE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTSTATE {
    #[inline(always)]
    fn from(val: u8) -> MSTSTATE {
        MSTSTATE::from_bits(val)
    }
}
impl From<MSTSTATE> for u8 {
    #[inline(always)]
    fn from(val: MSTSTATE) -> u8 {
        MSTSTATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTSTOP {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP = 0x01,
}
impl MSTSTOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTSTOP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTSTOP {
    #[inline(always)]
    fn from(val: u8) -> MSTSTOP {
        MSTSTOP::from_bits(val)
    }
}
impl From<MSTSTOP> for u8 {
    #[inline(always)]
    fn from(val: MSTSTOP) -> u8 {
        MSTSTOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QUALMODE0 {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK = 0x0,
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND = 0x01,
}
impl QUALMODE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QUALMODE0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QUALMODE0 {
    #[inline(always)]
    fn from(val: u8) -> QUALMODE0 {
        QUALMODE0::from_bits(val)
    }
}
impl From<QUALMODE0> for u8 {
    #[inline(always)]
    fn from(val: QUALMODE0) -> u8 {
        QUALMODE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVADR0_SADISABLE {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl SLVADR0_SADISABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVADR0_SADISABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVADR0_SADISABLE {
    #[inline(always)]
    fn from(val: u8) -> SLVADR0_SADISABLE {
        SLVADR0_SADISABLE::from_bits(val)
    }
}
impl From<SLVADR0_SADISABLE> for u8 {
    #[inline(always)]
    fn from(val: SLVADR0_SADISABLE) -> u8 {
        SLVADR0_SADISABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVADR1_SADISABLE {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl SLVADR1_SADISABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVADR1_SADISABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVADR1_SADISABLE {
    #[inline(always)]
    fn from(val: u8) -> SLVADR1_SADISABLE {
        SLVADR1_SADISABLE::from_bits(val)
    }
}
impl From<SLVADR1_SADISABLE> for u8 {
    #[inline(always)]
    fn from(val: SLVADR1_SADISABLE) -> u8 {
        SLVADR1_SADISABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVADR2_SADISABLE {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl SLVADR2_SADISABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVADR2_SADISABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVADR2_SADISABLE {
    #[inline(always)]
    fn from(val: u8) -> SLVADR2_SADISABLE {
        SLVADR2_SADISABLE::from_bits(val)
    }
}
impl From<SLVADR2_SADISABLE> for u8 {
    #[inline(always)]
    fn from(val: SLVADR2_SADISABLE) -> u8 {
        SLVADR2_SADISABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVADR3_SADISABLE {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED = 0x0,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED = 0x01,
}
impl SLVADR3_SADISABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVADR3_SADISABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVADR3_SADISABLE {
    #[inline(always)]
    fn from(val: u8) -> SLVADR3_SADISABLE {
        SLVADR3_SADISABLE::from_bits(val)
    }
}
impl From<SLVADR3_SADISABLE> for u8 {
    #[inline(always)]
    fn from(val: SLVADR3_SADISABLE) -> u8 {
        SLVADR3_SADISABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVCONTINUE {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    CONTINUE = 0x01,
}
impl SLVCONTINUE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVCONTINUE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVCONTINUE {
    #[inline(always)]
    fn from(val: u8) -> SLVCONTINUE {
        SLVCONTINUE::from_bits(val)
    }
}
impl From<SLVCONTINUE> for u8 {
    #[inline(always)]
    fn from(val: SLVCONTINUE) -> u8 {
        SLVCONTINUE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVIDX {
    #[doc = "Address 0. Slave address 0 was matched."]
    ADDRESS0 = 0x0,
    #[doc = "Address 1. Slave address 1 was matched."]
    ADDRESS1 = 0x01,
    #[doc = "Address 2. Slave address 2 was matched."]
    ADDRESS2 = 0x02,
    #[doc = "Address 3. Slave address 3 was matched."]
    ADDRESS3 = 0x03,
}
impl SLVIDX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVIDX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVIDX {
    #[inline(always)]
    fn from(val: u8) -> SLVIDX {
        SLVIDX::from_bits(val)
    }
}
impl From<SLVIDX> for u8 {
    #[inline(always)]
    fn from(val: SLVIDX) -> u8 {
        SLVIDX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVNACK {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK = 0x01,
}
impl SLVNACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVNACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVNACK {
    #[inline(always)]
    fn from(val: u8) -> SLVNACK {
        SLVNACK::from_bits(val)
    }
}
impl From<SLVNACK> for u8 {
    #[inline(always)]
    fn from(val: SLVNACK) -> u8 {
        SLVNACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVNOTSTR {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING = 0x0,
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING = 0x01,
}
impl SLVNOTSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVNOTSTR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVNOTSTR {
    #[inline(always)]
    fn from(val: u8) -> SLVNOTSTR {
        SLVNOTSTR::from_bits(val)
    }
}
impl From<SLVNOTSTR> for u8 {
    #[inline(always)]
    fn from(val: SLVNOTSTR) -> u8 {
        SLVNOTSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVPENDING {
    #[doc = "In progress. The Slave function does not currently need service."]
    IN_PROGRESS = 0x0,
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    PENDING = 0x01,
}
impl SLVPENDING {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVPENDING {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVPENDING {
    #[inline(always)]
    fn from(val: u8) -> SLVPENDING {
        SLVPENDING::from_bits(val)
    }
}
impl From<SLVPENDING> for u8 {
    #[inline(always)]
    fn from(val: SLVPENDING) -> u8 {
        SLVPENDING::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLVSTATE {
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SLAVE_ADDRESS = 0x0,
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    SLAVE_RECEIVE = 0x01,
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SLAVE_TRANSMIT = 0x02,
    _RESERVED_3 = 0x03,
}
impl SLVSTATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLVSTATE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLVSTATE {
    #[inline(always)]
    fn from(val: u8) -> SLVSTATE {
        SLVSTATE::from_bits(val)
    }
}
impl From<SLVSTATE> for u8 {
    #[inline(always)]
    fn from(val: SLVSTATE) -> u8 {
        SLVSTATE::to_bits(val)
    }
}
