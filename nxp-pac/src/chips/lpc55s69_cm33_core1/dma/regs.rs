#[doc = "Channel Abort control for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abort0(pub u32);
impl Abort0 {
    #[doc = "Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn abortctrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub const fn set_abortctrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Abort0 {
    #[inline(always)]
    fn default() -> Abort0 {
        Abort0(0)
    }
}
impl core::fmt::Debug for Abort0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Abort0")
            .field("abortctrl", &self.abortctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abort0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Abort0 {{ abortctrl: {=u32:?} }}", self.abortctrl())
    }
}
#[doc = "Channel Active status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Active0(pub u32);
impl Active0 {
    #[doc = "Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[must_use]
    #[inline(always)]
    pub const fn act(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[inline(always)]
    pub const fn set_act(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Active0 {
    #[inline(always)]
    fn default() -> Active0 {
        Active0(0)
    }
}
impl core::fmt::Debug for Active0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Active0").field("act", &self.act()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Active0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Active0 {{ act: {=u32:?} }}", self.act())
    }
}
#[doc = "Channel Busy status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busy0(pub u32);
impl Busy0 {
    #[doc = "Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[must_use]
    #[inline(always)]
    pub const fn bsy(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[inline(always)]
    pub const fn set_bsy(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Busy0 {
    #[inline(always)]
    fn default() -> Busy0 {
        Busy0(0)
    }
}
impl core::fmt::Debug for Busy0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busy0").field("bsy", &self.bsy()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busy0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busy0 {{ bsy: {=u32:?} }}", self.bsy())
    }
}
#[doc = "Configuration register for DMA channel ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[must_use]
    #[inline(always)]
    pub const fn periphreqen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub const fn set_periphreqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware Triggering Enable for this channel."]
    #[must_use]
    #[inline(always)]
    pub const fn hwtrigen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub const fn set_hwtrigen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> super::vals::Trigpol {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trigpol::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: super::vals::Trigpol) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn trigtype(&self) -> super::vals::Trigtype {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Trigtype::from_bits(val as u8)
    }
    #[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub const fn set_trigtype(&mut self, val: super::vals::Trigtype) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn trigburst(&self) -> super::vals::Trigburst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Trigburst::from_bits(val as u8)
    }
    #[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub const fn set_trigburst(&mut self, val: super::vals::Trigburst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[must_use]
    #[inline(always)]
    pub const fn burstpower(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub const fn set_burstpower(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[must_use]
    #[inline(always)]
    pub const fn srcburstwrap(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub const fn set_srcburstwrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[must_use]
    #[inline(always)]
    pub const fn dstburstwrap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub const fn set_dstburstwrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpriority(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub const fn set_chpriority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("periphreqen", &self.periphreqen())
            .field("hwtrigen", &self.hwtrigen())
            .field("trigpol", &self.trigpol())
            .field("trigtype", &self.trigtype())
            .field("trigburst", &self.trigburst())
            .field("burstpower", &self.burstpower())
            .field("srcburstwrap", &self.srcburstwrap())
            .field("dstburstwrap", &self.dstburstwrap())
            .field("chpriority", &self.chpriority())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ periphreqen: {=bool:?}, hwtrigen: {=bool:?}, trigpol: {:?}, trigtype: {:?}, trigburst: {:?}, burstpower: {=u8:?}, srcburstwrap: {=bool:?}, dstburstwrap: {=bool:?}, chpriority: {=u8:?} }}",
            self.periphreqen(),
            self.hwtrigen(),
            self.trigpol(),
            self.trigtype(),
            self.trigburst(),
            self.burstpower(),
            self.srcburstwrap(),
            self.dstburstwrap(),
            self.chpriority()
        )
    }
}
#[doc = "Control and status register for DMA channel ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctlstat(pub u32);
impl Ctlstat {
    #[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[must_use]
    #[inline(always)]
    pub const fn validpending(&self) -> super::vals::Validpending {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Validpending::from_bits(val as u8)
    }
    #[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub const fn set_validpending(&mut self, val: super::vals::Validpending) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub const fn set_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctlstat {
    #[inline(always)]
    fn default() -> Ctlstat {
        Ctlstat(0)
    }
}
impl core::fmt::Debug for Ctlstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctlstat")
            .field("validpending", &self.validpending())
            .field("trig", &self.trig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctlstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctlstat {{ validpending: {:?}, trig: {=bool:?} }}",
            self.validpending(),
            self.trig()
        )
    }
}
#[doc = "DMA control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "DMA controller master enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA controller master enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrl {{ enable: {=bool:?} }}", self.enable())
    }
}
#[doc = "Channel Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enableclr0(pub u32);
impl Enableclr0 {
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Enableclr0 {
    #[inline(always)]
    fn default() -> Enableclr0 {
        Enableclr0(0)
    }
}
impl core::fmt::Debug for Enableclr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enableclr0")
            .field("clr", &self.clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enableclr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enableclr0 {{ clr: {=u32:?} }}", self.clr())
    }
}
#[doc = "Channel Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enableset0(pub u32);
impl Enableset0 {
    #[doc = "Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ena(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_ena(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Enableset0 {
    #[inline(always)]
    fn default() -> Enableset0 {
        Enableset0(0)
    }
}
impl core::fmt::Debug for Enableset0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enableset0")
            .field("ena", &self.ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enableset0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enableset0 {{ ena: {=u32:?} }}", self.ena())
    }
}
#[doc = "Error Interrupt status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errint0(pub u32);
impl Errint0 {
    #[doc = "Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Errint0 {
    #[inline(always)]
    fn default() -> Errint0 {
        Errint0(0)
    }
}
impl core::fmt::Debug for Errint0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Errint0").field("err", &self.err()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Errint0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Errint0 {{ err: {=u32:?} }}", self.err())
    }
}
#[doc = "Interrupt A status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inta0(pub u32);
impl Inta0 {
    #[doc = "Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[must_use]
    #[inline(always)]
    pub const fn ia(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub const fn set_ia(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Inta0 {
    #[inline(always)]
    fn default() -> Inta0 {
        Inta0(0)
    }
}
impl core::fmt::Debug for Inta0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inta0").field("ia", &self.ia()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inta0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Inta0 {{ ia: {=u32:?} }}", self.ia())
    }
}
#[doc = "Interrupt B status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intb0(pub u32);
impl Intb0 {
    #[doc = "Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[must_use]
    #[inline(always)]
    pub const fn ib(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[inline(always)]
    pub const fn set_ib(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intb0 {
    #[inline(always)]
    fn default() -> Intb0 {
        Intb0(0)
    }
}
impl core::fmt::Debug for Intb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intb0").field("ib", &self.ib()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intb0 {{ ib: {=u32:?} }}", self.ib())
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr0(pub u32);
impl Intenclr0 {
    #[doc = "Writing ones to this register clears corresponding bits in the INTENSET0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears corresponding bits in the INTENSET0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intenclr0 {
    #[inline(always)]
    fn default() -> Intenclr0 {
        Intenclr0(0)
    }
}
impl core::fmt::Debug for Intenclr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr0")
            .field("clr", &self.clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intenclr0 {{ clr: {=u32:?} }}", self.clr())
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset0(pub u32);
impl Intenset0 {
    #[doc = "Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn inten(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub const fn set_inten(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intenset0 {
    #[inline(always)]
    fn default() -> Intenset0 {
        Intenset0(0)
    }
}
impl core::fmt::Debug for Intenset0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset0")
            .field("inten", &self.inten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intenset0 {{ inten: {=u32:?} }}", self.inten())
    }
}
#[doc = "Interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[must_use]
    #[inline(always)]
    pub const fn activeint(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub const fn set_activeint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Summarizes whether any error interrupts are pending."]
    #[must_use]
    #[inline(always)]
    pub const fn activeerrint(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub const fn set_activeerrint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("activeint", &self.activeint())
            .field("activeerrint", &self.activeerrint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ activeint: {=bool:?}, activeerrint: {=bool:?} }}",
            self.activeint(),
            self.activeerrint()
        )
    }
}
#[doc = "Set Trigger control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Settrig0(pub u32);
impl Settrig0 {
    #[doc = "Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn trig(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[inline(always)]
    pub const fn set_trig(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Settrig0 {
    #[inline(always)]
    fn default() -> Settrig0 {
        Settrig0(0)
    }
}
impl core::fmt::Debug for Settrig0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Settrig0")
            .field("trig", &self.trig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Settrig0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Settrig0 {{ trig: {=u32:?} }}", self.trig())
    }
}
#[doc = "Set ValidPending control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setvalid0(pub u32);
impl Setvalid0 {
    #[doc = "SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
    #[must_use]
    #[inline(always)]
    pub const fn sv(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
    #[inline(always)]
    pub const fn set_sv(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Setvalid0 {
    #[inline(always)]
    fn default() -> Setvalid0 {
        Setvalid0(0)
    }
}
impl core::fmt::Debug for Setvalid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setvalid0").field("sv", &self.sv()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setvalid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setvalid0 {{ sv: {=u32:?} }}", self.sv())
    }
}
#[doc = "SRAM address of the channel configuration table."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srambase(pub u32);
impl Srambase {
    #[doc = "Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for Srambase {
    #[inline(always)]
    fn default() -> Srambase {
        Srambase(0)
    }
}
impl core::fmt::Debug for Srambase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srambase")
            .field("offset", &self.offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srambase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srambase {{ offset: {=u32:?} }}", self.offset())
    }
}
#[doc = "Transfer configuration register for DMA channel ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xfercfg(pub u32);
impl Xfercfg {
    #[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgvalid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub const fn set_cfgvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[must_use]
    #[inline(always)]
    pub const fn reload(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub const fn set_reload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Software Trigger."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn clrtrig(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Trigger."]
    #[inline(always)]
    pub const fn set_clrtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[must_use]
    #[inline(always)]
    pub const fn setinta(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub const fn set_setinta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[must_use]
    #[inline(always)]
    pub const fn setintb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub const fn set_setintb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer width used for this DMA channel."]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> super::vals::Width {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Width::from_bits(val as u8)
    }
    #[doc = "Transfer width used for this DMA channel."]
    #[inline(always)]
    pub const fn set_width(&mut self, val: super::vals::Width) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Determines whether the source address is incremented for each DMA transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Srcinc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Srcinc::from_bits(val as u8)
    }
    #[doc = "Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub const fn set_srcinc(&mut self, val: super::vals::Srcinc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Determines whether the destination address is incremented for each DMA transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Dstinc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Dstinc::from_bits(val as u8)
    }
    #[doc = "Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub const fn set_dstinc(&mut self, val: super::vals::Dstinc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[must_use]
    #[inline(always)]
    pub const fn xfercount(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub const fn set_xfercount(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Xfercfg {
    #[inline(always)]
    fn default() -> Xfercfg {
        Xfercfg(0)
    }
}
impl core::fmt::Debug for Xfercfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xfercfg")
            .field("cfgvalid", &self.cfgvalid())
            .field("reload", &self.reload())
            .field("swtrig", &self.swtrig())
            .field("clrtrig", &self.clrtrig())
            .field("setinta", &self.setinta())
            .field("setintb", &self.setintb())
            .field("width", &self.width())
            .field("srcinc", &self.srcinc())
            .field("dstinc", &self.dstinc())
            .field("xfercount", &self.xfercount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xfercfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xfercfg {{ cfgvalid: {=bool:?}, reload: {=bool:?}, swtrig: {=bool:?}, clrtrig: {=bool:?}, setinta: {=bool:?}, setintb: {=bool:?}, width: {:?}, srcinc: {:?}, dstinc: {:?}, xfercount: {=u16:?} }}",
            self.cfgvalid(),
            self.reload(),
            self.swtrig(),
            self.clrtrig(),
            self.setinta(),
            self.setintb(),
            self.width(),
            self.srcinc(),
            self.dstinc(),
            self.xfercount()
        )
    }
}
