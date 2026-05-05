#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Base {
    #[doc = "Is offset from 0 in host memory or IO space."]
    Offset = 0x0,
    #[doc = "Uses Base1 offset in host memory (see MAPBASE reg)."]
    Base1 = 0x01,
    #[doc = "Uses Base2 offset in host memory (see MAPBASE reg)."]
    Base2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Base {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Base {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Base {
    #[inline(always)]
    fn from(val: u8) -> Base {
        Base::from_bits(val)
    }
}
impl From<Base> for u8 {
    #[inline(always)]
    fn from(val: Base) -> u8 {
        Base::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0en {
    #[doc = "Disabled. The DMA channel is not used."]
    Disabled = 0x0,
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory."]
    TriggerdOnHostRead = 0x01,
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    TriggersOnHostWrite = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dma0en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0en {
    #[inline(always)]
    fn from(val: u8) -> Dma0en {
        Dma0en::from_bits(val)
    }
}
impl From<Dma0en> for u8 {
    #[inline(always)]
    fn from(val: Dma0en) -> u8 {
        Dma0en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1en {
    #[doc = "Disabled. The DMA channel is not used."]
    Disabled = 0x0,
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory."]
    TriggerdOnHostRead = 0x01,
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    TriggersOnHostWrite = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dma1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1en {
    #[inline(always)]
    fn from(val: u8) -> Dma1en {
        Dma1en::from_bits(val)
    }
}
impl From<Dma1en> for u8 {
    #[inline(always)]
    fn from(val: Dma1en) -> u8 {
        Dma1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "Disabled. Block is not operational."]
    Disabled = 0x0,
    #[doc = "eSPI."]
    Espi = 0x01,
    #[doc = "LPC."]
    Lpc = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashmx {
    #[doc = "64 bytes."]
    Byte64 = 0x0,
    #[doc = "128 bytes."]
    Byte128 = 0x01,
    #[doc = "256 bytes."]
    Byte256 = 0x02,
    #[doc = "512 bytes."]
    Byte512 = 0x03,
}
impl Flashmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashmx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashmx {
    #[inline(always)]
    fn from(val: u8) -> Flashmx {
        Flashmx::from_bits(val)
    }
}
impl From<Flashmx> for u8 {
    #[inline(always)]
    fn from(val: Flashmx) -> u8 {
        Flashmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flshera {
    #[doc = "Flash is not enabled."]
    Disabled = 0x0,
    #[doc = "Erase is 4K."]
    Erase4k = 0x01,
    #[doc = "Erase is 64K."]
    Erase64k = 0x02,
    #[doc = "Erase allows 4K and 64K."]
    Erase4k64k = 0x03,
    #[doc = "Erase is 128K."]
    Erase128k = 0x04,
    #[doc = "Erase is 256K."]
    Erase256k = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flshera {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flshera {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flshera {
    #[inline(always)]
    fn from(val: u8) -> Flshera {
        Flshera::from_bits(val)
    }
}
impl From<Flshera> for u8 {
    #[inline(always)]
    fn from(val: Flshera) -> u8 {
        Flshera::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Maxspd {
    #[doc = "<=20MHz."]
    SmallThan20m = 0x0,
    #[doc = "<=25MHz (may be 24Mhz)."]
    SmallThan25m = 0x01,
    #[doc = "<=33MHz (may be 30MHz)."]
    SmallThan33m = 0x02,
    #[doc = "<=50MHz (may be 48MHz)."]
    SmallThan50m = 0x03,
    #[doc = "<= 66MHz (may be 60MHz)."]
    SmallThan66m = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Maxspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maxspd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maxspd {
    #[inline(always)]
    fn from(val: u8) -> Maxspd {
        Maxspd::from_bits(val)
    }
}
impl From<Maxspd> for u8 {
    #[inline(always)]
    fn from(val: Maxspd) -> u8 {
        Maxspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0addrBaseOrAsz {
    #[doc = "Is offset from 0 in host memory or IO space."]
    OffsetFrom0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory."]
    UseBase0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory."]
    UseBase1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P0addrBaseOrAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0addrBaseOrAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0addrBaseOrAsz {
    #[inline(always)]
    fn from(val: u8) -> P0addrBaseOrAsz {
        P0addrBaseOrAsz::from_bits(val)
    }
}
impl From<P0addrBaseOrAsz> for u8 {
    #[inline(always)]
    fn from(val: P0addrBaseOrAsz) -> u8 {
        P0addrBaseOrAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0cfgType {
    #[doc = "Unconfigured (reset condition)."]
    Unconfigured = 0x0,
    #[doc = "ACPI style Endpoint."]
    AcpiEnd = 0x01,
    #[doc = "ACPI style Index/Data."]
    AcpiIndex = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Mem Single."]
    BusMMemS = 0x04,
    #[doc = "Bus Master Flash Single."]
    BusMFlashS = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared."]
    MailboxShared = 0x08,
    #[doc = "Mailbox Single."]
    MailboxSingle = 0x09,
    #[doc = "Mailbox Split."]
    MailboxSplit = 0x0a,
    #[doc = "Mailbox OOB Split."]
    MailboxOobSplit = 0x0b,
    #[doc = "Mailbox OEM."]
    MailboxOem = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P0cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0cfgType {
    #[inline(always)]
    fn from(val: u8) -> P0cfgType {
        P0cfgType::from_bits(val)
    }
}
impl From<P0cfgType> for u8 {
    #[inline(always)]
    fn from(val: P0cfgType) -> u8 {
        P0cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1addrBaseOrAsz {
    #[doc = "Is offset from 0 in host memory or IO space."]
    OffsetFrom0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory."]
    UseBase0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory."]
    UseBase1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P1addrBaseOrAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1addrBaseOrAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1addrBaseOrAsz {
    #[inline(always)]
    fn from(val: u8) -> P1addrBaseOrAsz {
        P1addrBaseOrAsz::from_bits(val)
    }
}
impl From<P1addrBaseOrAsz> for u8 {
    #[inline(always)]
    fn from(val: P1addrBaseOrAsz) -> u8 {
        P1addrBaseOrAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1cfgType {
    #[doc = "Unconfigured (reset condition)."]
    Unconfigured = 0x0,
    #[doc = "ACPI style Endpoint."]
    AcpiEnd = 0x01,
    #[doc = "ACPI style Index/Data."]
    AcpiIndex = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Mem Single."]
    BusMMemS = 0x04,
    #[doc = "Bus Master Flash Single."]
    BusMFlashS = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared."]
    MailboxShared = 0x08,
    #[doc = "Mailbox Single."]
    MailboxSingle = 0x09,
    #[doc = "Mailbox Split."]
    MailboxSplit = 0x0a,
    #[doc = "Mailbox OOB Split."]
    MailboxOobSplit = 0x0b,
    #[doc = "Mailbox OEM."]
    MailboxOem = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P1cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1cfgType {
    #[inline(always)]
    fn from(val: u8) -> P1cfgType {
        P1cfgType::from_bits(val)
    }
}
impl From<P1cfgType> for u8 {
    #[inline(always)]
    fn from(val: P1cfgType) -> u8 {
        P1cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2addrBaseOrAsz {
    #[doc = "Is offset from 0 in host memory or IO space."]
    OffsetFrom0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory."]
    UseBase0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory."]
    UseBase1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P2addrBaseOrAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2addrBaseOrAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2addrBaseOrAsz {
    #[inline(always)]
    fn from(val: u8) -> P2addrBaseOrAsz {
        P2addrBaseOrAsz::from_bits(val)
    }
}
impl From<P2addrBaseOrAsz> for u8 {
    #[inline(always)]
    fn from(val: P2addrBaseOrAsz) -> u8 {
        P2addrBaseOrAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2cfgType {
    #[doc = "Unconfigured (reset condition)."]
    Unconfigured = 0x0,
    #[doc = "ACPI style Endpoint."]
    AcpiEnd = 0x01,
    #[doc = "ACPI style Index/Data."]
    AcpiIndex = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Mem Single."]
    BusMMemS = 0x04,
    #[doc = "Bus Master Flash Single."]
    BusMFlashS = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared."]
    MailboxShared = 0x08,
    #[doc = "Mailbox Single."]
    MailboxSingle = 0x09,
    #[doc = "Mailbox Split."]
    MailboxSplit = 0x0a,
    #[doc = "Mailbox OOB Split."]
    MailboxOobSplit = 0x0b,
    #[doc = "Mailbox OEM."]
    MailboxOem = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P2cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2cfgType {
    #[inline(always)]
    fn from(val: u8) -> P2cfgType {
        P2cfgType::from_bits(val)
    }
}
impl From<P2cfgType> for u8 {
    #[inline(always)]
    fn from(val: P2cfgType) -> u8 {
        P2cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3addrBaseOrAsz {
    #[doc = "Is offset from 0 in host memory or IO space."]
    OffsetFrom0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory."]
    UseBase0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory."]
    UseBase1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P3addrBaseOrAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3addrBaseOrAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3addrBaseOrAsz {
    #[inline(always)]
    fn from(val: u8) -> P3addrBaseOrAsz {
        P3addrBaseOrAsz::from_bits(val)
    }
}
impl From<P3addrBaseOrAsz> for u8 {
    #[inline(always)]
    fn from(val: P3addrBaseOrAsz) -> u8 {
        P3addrBaseOrAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3cfgType {
    #[doc = "Unconfigured (reset condition)."]
    Unconfigured = 0x0,
    #[doc = "ACPI style Endpoint."]
    AcpiEnd = 0x01,
    #[doc = "ACPI style Index/Data."]
    AcpiIndex = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Mem Single."]
    BusMMemS = 0x04,
    #[doc = "Bus Master Flash Single."]
    BusMFlashS = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared."]
    MailboxShared = 0x08,
    #[doc = "Mailbox Single."]
    MailboxSingle = 0x09,
    #[doc = "Mailbox Split."]
    MailboxSplit = 0x0a,
    #[doc = "Mailbox OOB Split."]
    MailboxOobSplit = 0x0b,
    #[doc = "Mailbox OEM."]
    MailboxOem = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P3cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3cfgType {
    #[inline(always)]
    fn from(val: u8) -> P3cfgType {
        P3cfgType::from_bits(val)
    }
}
impl From<P3cfgType> for u8 {
    #[inline(always)]
    fn from(val: P3cfgType) -> u8 {
        P3cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4addrBaseOrAsz {
    #[doc = "Is offset from 0 in host memory or IO space."]
    OffsetFrom0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory."]
    UseBase0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory."]
    UseBase1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P4addrBaseOrAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4addrBaseOrAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4addrBaseOrAsz {
    #[inline(always)]
    fn from(val: u8) -> P4addrBaseOrAsz {
        P4addrBaseOrAsz::from_bits(val)
    }
}
impl From<P4addrBaseOrAsz> for u8 {
    #[inline(always)]
    fn from(val: P4addrBaseOrAsz) -> u8 {
        P4addrBaseOrAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4cfgType {
    #[doc = "Unconfigured (reset condition)."]
    Unconfigured = 0x0,
    #[doc = "ACPI style Endpoint."]
    AcpiEnd = 0x01,
    #[doc = "ACPI style Index/Data."]
    AcpiIndex = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Mem Single."]
    BusMMemS = 0x04,
    #[doc = "Bus Master Flash Single."]
    BusMFlashS = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared."]
    MailboxShared = 0x08,
    #[doc = "Mailbox Single."]
    MailboxSingle = 0x09,
    #[doc = "Mailbox Split."]
    MailboxSplit = 0x0a,
    #[doc = "Mailbox OOB Split."]
    MailboxOobSplit = 0x0b,
    #[doc = "Mailbox OEM."]
    MailboxOem = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P4cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4cfgType {
    #[inline(always)]
    fn from(val: u8) -> P4cfgType {
        P4cfgType::from_bits(val)
    }
}
impl From<P4cfgType> for u8 {
    #[inline(always)]
    fn from(val: P4cfgType) -> u8 {
        P4cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Safera {
    #[doc = "Minimally 2KB."]
    Min2kb = 0x0,
    #[doc = "Minimally 4KB."]
    Min4kb = 0x01,
    #[doc = "Minimally 8KB."]
    Min8kb = 0x02,
    #[doc = "Minimally 16KB."]
    Min16kb = 0x03,
}
impl Safera {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Safera {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Safera {
    #[inline(always)]
    fn from(val: u8) -> Safera {
        Safera::from_bits(val)
    }
}
impl From<Safera> for u8 {
    #[inline(always)]
    fn from(val: Safera) -> u8 {
        Safera::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spicap {
    #[doc = "SPI only."]
    Spi = 0x0,
    #[doc = "BiSPI and SPI."]
    BspiSpi = 0x01,
    #[doc = "FLEXSPI and SPI."]
    FlexspiSpi = 0x02,
    #[doc = "any."]
    Any = 0x03,
}
impl Spicap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spicap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spicap {
    #[inline(always)]
    fn from(val: u8) -> Spicap {
        Spicap::from_bits(val)
    }
}
impl From<Spicap> for u8 {
    #[inline(always)]
    fn from(val: Spicap) -> u8 {
        Spicap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spimod {
    #[doc = "SPI."]
    Spi = 0x0,
    #[doc = "BiSPI."]
    Bspi = 0x01,
    #[doc = "FLEXSPI."]
    Flexspi = 0x02,
    _RESERVED_3 = 0x03,
}
impl Spimod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spimod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spimod {
    #[inline(always)]
    fn from(val: u8) -> Spimod {
        Spimod::from_bits(val)
    }
}
impl From<Spimod> for u8 {
    #[inline(always)]
    fn from(val: Spimod) -> u8 {
        Spimod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spispd {
    #[doc = "20MHz or less."]
    LessAnd20m = 0x0,
    #[doc = "25MHz or 24MHz."]
    Freq25m24m = 0x01,
    #[doc = "33MHz or 30MHz."]
    Freq33m30m = 0x02,
    #[doc = "50MHz or 48MHz."]
    Freq50m48m = 0x03,
    #[doc = "66MHz or 60MHz."]
    Freq66m60m = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Spispd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spispd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spispd {
    #[inline(always)]
    fn from(val: u8) -> Spispd {
        Spispd::from_bits(val)
    }
}
impl From<Spispd> for u8 {
    #[inline(always)]
    fn from(val: Spispd) -> u8 {
        Spispd::to_bits(val)
    }
}
