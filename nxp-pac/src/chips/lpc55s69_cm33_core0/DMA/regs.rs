#[doc = "Channel Abort control for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ABORT0(pub u32);
impl ABORT0 {
    #[doc = "Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn ABORTCTRL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub const fn set_ABORTCTRL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ABORT0 {
    #[inline(always)]
    fn default() -> ABORT0 {
        ABORT0(0)
    }
}
impl core::fmt::Debug for ABORT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ABORT0")
            .field("ABORTCTRL", &self.ABORTCTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ABORT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ABORT0 {{ ABORTCTRL: {=u32:?} }}", self.ABORTCTRL())
    }
}
#[doc = "Channel Active status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ACTIVE0(pub u32);
impl ACTIVE0 {
    #[doc = "Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[must_use]
    #[inline(always)]
    pub const fn ACT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[inline(always)]
    pub const fn set_ACT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ACTIVE0 {
    #[inline(always)]
    fn default() -> ACTIVE0 {
        ACTIVE0(0)
    }
}
impl core::fmt::Debug for ACTIVE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTIVE0").field("ACT", &self.ACT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ACTIVE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ACTIVE0 {{ ACT: {=u32:?} }}", self.ACT())
    }
}
#[doc = "Channel Busy status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BUSY0(pub u32);
impl BUSY0 {
    #[doc = "Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[must_use]
    #[inline(always)]
    pub const fn BSY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[inline(always)]
    pub const fn set_BSY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BUSY0 {
    #[inline(always)]
    fn default() -> BUSY0 {
        BUSY0(0)
    }
}
impl core::fmt::Debug for BUSY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSY0").field("BSY", &self.BSY()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BUSY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BUSY0 {{ BSY: {=u32:?} }}", self.BSY())
    }
}
#[doc = "Configuration register for DMA channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[must_use]
    #[inline(always)]
    pub const fn PERIPHREQEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub const fn set_PERIPHREQEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware Triggering Enable for this channel."]
    #[must_use]
    #[inline(always)]
    pub const fn HWTRIGEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub const fn set_HWTRIGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGPOL(&self) -> super::vals::TRIGPOL {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TRIGPOL::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub const fn set_TRIGPOL(&mut self, val: super::vals::TRIGPOL) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGTYPE(&self) -> super::vals::TRIGTYPE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TRIGTYPE::from_bits(val as u8)
    }
    #[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub const fn set_TRIGTYPE(&mut self, val: super::vals::TRIGTYPE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGBURST(&self) -> super::vals::TRIGBURST {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TRIGBURST::from_bits(val as u8)
    }
    #[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub const fn set_TRIGBURST(&mut self, val: super::vals::TRIGBURST) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[must_use]
    #[inline(always)]
    pub const fn BURSTPOWER(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub const fn set_BURSTPOWER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[must_use]
    #[inline(always)]
    pub const fn SRCBURSTWRAP(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub const fn set_SRCBURSTWRAP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[must_use]
    #[inline(always)]
    pub const fn DSTBURSTWRAP(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub const fn set_DSTBURSTWRAP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[must_use]
    #[inline(always)]
    pub const fn CHPRIORITY(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub const fn set_CHPRIORITY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("PERIPHREQEN", &self.PERIPHREQEN())
            .field("HWTRIGEN", &self.HWTRIGEN())
            .field("TRIGPOL", &self.TRIGPOL())
            .field("TRIGTYPE", &self.TRIGTYPE())
            .field("TRIGBURST", &self.TRIGBURST())
            .field("BURSTPOWER", &self.BURSTPOWER())
            .field("SRCBURSTWRAP", &self.SRCBURSTWRAP())
            .field("DSTBURSTWRAP", &self.DSTBURSTWRAP())
            .field("CHPRIORITY", &self.CHPRIORITY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ PERIPHREQEN: {=bool:?}, HWTRIGEN: {=bool:?}, TRIGPOL: {:?}, TRIGTYPE: {:?}, TRIGBURST: {:?}, BURSTPOWER: {=u8:?}, SRCBURSTWRAP: {=bool:?}, DSTBURSTWRAP: {=bool:?}, CHPRIORITY: {=u8:?} }}",
            self.PERIPHREQEN(),
            self.HWTRIGEN(),
            self.TRIGPOL(),
            self.TRIGTYPE(),
            self.TRIGBURST(),
            self.BURSTPOWER(),
            self.SRCBURSTWRAP(),
            self.DSTBURSTWRAP(),
            self.CHPRIORITY()
        )
    }
}
#[doc = "Control and status register for DMA channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTLSTAT(pub u32);
impl CTLSTAT {
    #[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[must_use]
    #[inline(always)]
    pub const fn VALIDPENDING(&self) -> super::vals::VALIDPENDING {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VALIDPENDING::from_bits(val as u8)
    }
    #[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub const fn set_VALIDPENDING(&mut self, val: super::vals::VALIDPENDING) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIG(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub const fn set_TRIG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for CTLSTAT {
    #[inline(always)]
    fn default() -> CTLSTAT {
        CTLSTAT(0)
    }
}
impl core::fmt::Debug for CTLSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTLSTAT")
            .field("VALIDPENDING", &self.VALIDPENDING())
            .field("TRIG", &self.TRIG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTLSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTLSTAT {{ VALIDPENDING: {:?}, TRIG: {=bool:?} }}",
            self.VALIDPENDING(),
            self.TRIG()
        )
    }
}
#[doc = "DMA control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "DMA controller master enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA controller master enable."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ENABLE", &self.ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTRL {{ ENABLE: {=bool:?} }}", self.ENABLE())
    }
}
#[doc = "Channel Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENABLECLR0(pub u32);
impl ENABLECLR0 {
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ENABLECLR0 {
    #[inline(always)]
    fn default() -> ENABLECLR0 {
        ENABLECLR0(0)
    }
}
impl core::fmt::Debug for ENABLECLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLECLR0")
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENABLECLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ENABLECLR0 {{ CLR: {=u32:?} }}", self.CLR())
    }
}
#[doc = "Channel Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENABLESET0(pub u32);
impl ENABLESET0 {
    #[doc = "Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_ENA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ENABLESET0 {
    #[inline(always)]
    fn default() -> ENABLESET0 {
        ENABLESET0(0)
    }
}
impl core::fmt::Debug for ENABLESET0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLESET0")
            .field("ENA", &self.ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENABLESET0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ENABLESET0 {{ ENA: {=u32:?} }}", self.ENA())
    }
}
#[doc = "Error Interrupt status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERRINT0(pub u32);
impl ERRINT0 {
    #[doc = "Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ERRINT0 {
    #[inline(always)]
    fn default() -> ERRINT0 {
        ERRINT0(0)
    }
}
impl core::fmt::Debug for ERRINT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERRINT0").field("ERR", &self.ERR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERRINT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ERRINT0 {{ ERR: {=u32:?} }}", self.ERR())
    }
}
#[doc = "Interrupt A status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTA0(pub u32);
impl INTA0 {
    #[doc = "Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[must_use]
    #[inline(always)]
    pub const fn IA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub const fn set_IA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INTA0 {
    #[inline(always)]
    fn default() -> INTA0 {
        INTA0(0)
    }
}
impl core::fmt::Debug for INTA0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTA0").field("IA", &self.IA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTA0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTA0 {{ IA: {=u32:?} }}", self.IA())
    }
}
#[doc = "Interrupt B status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTB0(pub u32);
impl INTB0 {
    #[doc = "Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[must_use]
    #[inline(always)]
    pub const fn IB(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[inline(always)]
    pub const fn set_IB(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INTB0 {
    #[inline(always)]
    fn default() -> INTB0 {
        INTB0(0)
    }
}
impl core::fmt::Debug for INTB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTB0").field("IB", &self.IB()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTB0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTB0 {{ IB: {=u32:?} }}", self.IB())
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR0(pub u32);
impl INTENCLR0 {
    #[doc = "Writing ones to this register clears corresponding bits in the INTENSET0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears corresponding bits in the INTENSET0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INTENCLR0 {
    #[inline(always)]
    fn default() -> INTENCLR0 {
        INTENCLR0(0)
    }
}
impl core::fmt::Debug for INTENCLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR0")
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTENCLR0 {{ CLR: {=u32:?} }}", self.CLR())
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET0(pub u32);
impl INTENSET0 {
    #[doc = "Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn INTEN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub const fn set_INTEN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INTENSET0 {
    #[inline(always)]
    fn default() -> INTENSET0 {
        INTENSET0(0)
    }
}
impl core::fmt::Debug for INTENSET0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET0")
            .field("INTEN", &self.INTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTENSET0 {{ INTEN: {=u32:?} }}", self.INTEN())
    }
}
#[doc = "Interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVEINT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub const fn set_ACTIVEINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Summarizes whether any error interrupts are pending."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVEERRINT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub const fn set_ACTIVEERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("ACTIVEINT", &self.ACTIVEINT())
            .field("ACTIVEERRINT", &self.ACTIVEERRINT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ ACTIVEINT: {=bool:?}, ACTIVEERRINT: {=bool:?} }}",
            self.ACTIVEINT(),
            self.ACTIVEERRINT()
        )
    }
}
#[doc = "Set Trigger control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETTRIG0(pub u32);
impl SETTRIG0 {
    #[doc = "Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[inline(always)]
    pub const fn set_TRIG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETTRIG0 {
    #[inline(always)]
    fn default() -> SETTRIG0 {
        SETTRIG0(0)
    }
}
impl core::fmt::Debug for SETTRIG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETTRIG0")
            .field("TRIG", &self.TRIG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETTRIG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETTRIG0 {{ TRIG: {=u32:?} }}", self.TRIG())
    }
}
#[doc = "Set ValidPending control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETVALID0(pub u32);
impl SETVALID0 {
    #[doc = "SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn SV(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n."]
    #[inline(always)]
    pub const fn set_SV(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETVALID0 {
    #[inline(always)]
    fn default() -> SETVALID0 {
        SETVALID0(0)
    }
}
impl core::fmt::Debug for SETVALID0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETVALID0").field("SV", &self.SV()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETVALID0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETVALID0 {{ SV: {=u32:?} }}", self.SV())
    }
}
#[doc = "SRAM address of the channel configuration table."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SRAMBASE(pub u32);
impl SRAMBASE {
    #[doc = "Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub const fn set_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for SRAMBASE {
    #[inline(always)]
    fn default() -> SRAMBASE {
        SRAMBASE(0)
    }
}
impl core::fmt::Debug for SRAMBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMBASE")
            .field("OFFSET", &self.OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SRAMBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SRAMBASE {{ OFFSET: {=u32:?} }}", self.OFFSET())
    }
}
#[doc = "Transfer configuration register for DMA channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XFERCFG(pub u32);
impl XFERCFG {
    #[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[must_use]
    #[inline(always)]
    pub const fn CFGVALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub const fn set_CFGVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOAD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub const fn set_RELOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn SWTRIG(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Software Trigger."]
    #[inline(always)]
    pub const fn set_SWTRIG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRTRIG(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Trigger."]
    #[inline(always)]
    pub const fn set_CLRTRIG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[must_use]
    #[inline(always)]
    pub const fn SETINTA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub const fn set_SETINTA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[must_use]
    #[inline(always)]
    pub const fn SETINTB(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub const fn set_SETINTB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer width used for this DMA channel."]
    #[must_use]
    #[inline(always)]
    pub const fn WIDTH(&self) -> super::vals::WIDTH {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::WIDTH::from_bits(val as u8)
    }
    #[doc = "Transfer width used for this DMA channel."]
    #[inline(always)]
    pub const fn set_WIDTH(&mut self, val: super::vals::WIDTH) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Determines whether the source address is incremented for each DMA transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn SRCINC(&self) -> super::vals::SRCINC {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SRCINC::from_bits(val as u8)
    }
    #[doc = "Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub const fn set_SRCINC(&mut self, val: super::vals::SRCINC) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Determines whether the destination address is incremented for each DMA transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn DSTINC(&self) -> super::vals::DSTINC {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::DSTINC::from_bits(val as u8)
    }
    #[doc = "Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub const fn set_DSTINC(&mut self, val: super::vals::DSTINC) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[must_use]
    #[inline(always)]
    pub const fn XFERCOUNT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub const fn set_XFERCOUNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for XFERCFG {
    #[inline(always)]
    fn default() -> XFERCFG {
        XFERCFG(0)
    }
}
impl core::fmt::Debug for XFERCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XFERCFG")
            .field("CFGVALID", &self.CFGVALID())
            .field("RELOAD", &self.RELOAD())
            .field("SWTRIG", &self.SWTRIG())
            .field("CLRTRIG", &self.CLRTRIG())
            .field("SETINTA", &self.SETINTA())
            .field("SETINTB", &self.SETINTB())
            .field("WIDTH", &self.WIDTH())
            .field("SRCINC", &self.SRCINC())
            .field("DSTINC", &self.DSTINC())
            .field("XFERCOUNT", &self.XFERCOUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XFERCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XFERCFG {{ CFGVALID: {=bool:?}, RELOAD: {=bool:?}, SWTRIG: {=bool:?}, CLRTRIG: {=bool:?}, SETINTA: {=bool:?}, SETINTB: {=bool:?}, WIDTH: {:?}, SRCINC: {:?}, DSTINC: {:?}, XFERCOUNT: {=u16:?} }}",
            self.CFGVALID(),
            self.RELOAD(),
            self.SWTRIG(),
            self.CLRTRIG(),
            self.SETINTA(),
            self.SETINTB(),
            self.WIDTH(),
            self.SRCINC(),
            self.DSTINC(),
            self.XFERCOUNT()
        )
    }
}
