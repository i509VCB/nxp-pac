#[doc = "Debug and Key"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgKey(pub u32);
impl DbgKey {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbgKey {
    #[inline(always)]
    fn default() -> DbgKey {
        DbgKey(0)
    }
}
impl core::fmt::Debug for DbgKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgKey").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgKey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DbgKey {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Flexible Config 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexCfg0(pub u32);
impl FlexCfg0 {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlexCfg0 {
    #[inline(always)]
    fn default() -> FlexCfg0 {
        FlexCfg0(0)
    }
}
impl core::fmt::Debug for FlexCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexCfg0")
            .field("dat", &self.dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexCfg0 {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Flexible Config 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexCfg1(pub u32);
impl FlexCfg1 {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlexCfg1 {
    #[inline(always)]
    fn default() -> FlexCfg1 {
        FlexCfg1(0)
    }
}
impl core::fmt::Debug for FlexCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexCfg1")
            .field("dat", &self.dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexCfg1 {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "NXP Part Config Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn nxp_part_cfg_lock(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "NXP Part Config Lock"]
    #[inline(always)]
    pub const fn set_nxp_part_cfg_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "NXP EXT Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn nxp_ext_lock(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "NXP EXT Lock"]
    #[inline(always)]
    pub const fn set_nxp_ext_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Boot config Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg_lock(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Boot config Lock"]
    #[inline(always)]
    pub const fn set_boot_cfg_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Prince Config Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn prince_cfg_lock(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Prince Config Lock"]
    #[inline(always)]
    pub const fn set_prince_cfg_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "OSCAA Key Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn oscaa_key_lock(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "OSCAA Key Lock"]
    #[inline(always)]
    pub const fn set_oscaa_key_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "CUST Lock 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cust_lock0(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "CUST Lock 0"]
    #[inline(always)]
    pub const fn set_cust_lock0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "CUST Lock 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cust_lock1(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "CUST Lock 1"]
    #[inline(always)]
    pub const fn set_cust_lock1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "CUST Lock 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cust_lock2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "CUST Lock 2"]
    #[inline(always)]
    pub const fn set_cust_lock2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "CUST Lock 3"]
    #[must_use]
    #[inline(always)]
    pub const fn cust_lock3(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x07;
        val as u8
    }
    #[doc = "CUST Lock 3"]
    #[inline(always)]
    pub const fn set_cust_lock3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("nxp_part_cfg_lock", &self.nxp_part_cfg_lock())
            .field("nxp_ext_lock", &self.nxp_ext_lock())
            .field("boot_cfg_lock", &self.boot_cfg_lock())
            .field("prince_cfg_lock", &self.prince_cfg_lock())
            .field("oscaa_key_lock", &self.oscaa_key_lock())
            .field("cust_lock0", &self.cust_lock0())
            .field("cust_lock1", &self.cust_lock1())
            .field("cust_lock2", &self.cust_lock2())
            .field("cust_lock3", &self.cust_lock3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ nxp_part_cfg_lock: {=u8:?}, nxp_ext_lock: {=u8:?}, boot_cfg_lock: {=u8:?}, prince_cfg_lock: {=u8:?}, oscaa_key_lock: {=u8:?}, cust_lock0: {=u8:?}, cust_lock1: {=u8:?}, cust_lock2: {=u8:?}, cust_lock3: {=u8:?} }}",
            self.nxp_part_cfg_lock(),
            self.nxp_ext_lock(),
            self.boot_cfg_lock(),
            self.prince_cfg_lock(),
            self.oscaa_key_lock(),
            self.cust_lock0(),
            self.cust_lock1(),
            self.cust_lock2(),
            self.cust_lock3()
        )
    }
}
#[doc = "MISC Config"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCfg(pub u32);
impl MiscCfg {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MiscCfg {
    #[inline(always)]
    fn default() -> MiscCfg {
        MiscCfg(0)
    }
}
impl core::fmt::Debug for MiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCfg").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MiscCfg {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of fuse bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_fuse(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of fuse bytes"]
    #[inline(always)]
    pub const fn set_num_fuse(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("num_fuse", &self.num_fuse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ num_fuse: {=u16:?} }}", self.num_fuse())
    }
}
#[doc = "Power Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Strong switch request"]
    #[must_use]
    #[inline(always)]
    pub const fn hvreq(&self) -> super::vals::Hvreq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hvreq::from_bits(val as u8)
    }
    #[doc = "Strong switch request"]
    #[inline(always)]
    pub const fn set_hvreq(&mut self, val: super::vals::Hvreq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Weak switch request"]
    #[must_use]
    #[inline(always)]
    pub const fn lvreq(&self) -> super::vals::Lvreq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lvreq::from_bits(val as u8)
    }
    #[doc = "Weak switch request"]
    #[inline(always)]
    pub const fn set_lvreq(&mut self, val: super::vals::Lvreq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Power down request"]
    #[must_use]
    #[inline(always)]
    pub const fn pdreq(&self) -> super::vals::Pdreq {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdreq::from_bits(val as u8)
    }
    #[doc = "Power down request"]
    #[inline(always)]
    pub const fn set_pdreq(&mut self, val: super::vals::Pdreq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        Pcr(0)
    }
}
impl core::fmt::Debug for Pcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr")
            .field("hvreq", &self.hvreq())
            .field("lvreq", &self.lvreq())
            .field("pdreq", &self.pdreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr {{ hvreq: {:?}, lvreq: {:?}, pdreq: {:?} }}",
            self.hvreq(),
            self.lvreq(),
            self.pdreq()
        )
    }
}
#[doc = "PHANTOM Config"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhantomCfg(pub u32);
impl PhantomCfg {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PhantomCfg {
    #[inline(always)]
    fn default() -> PhantomCfg {
        PhantomCfg(0)
    }
}
impl core::fmt::Debug for PhantomCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhantomCfg")
            .field("dat", &self.dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhantomCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PhantomCfg {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Read Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdata(pub u32);
impl Rdata {
    #[doc = "Read data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdata {
    #[inline(always)]
    fn default() -> Rdata {
        Rdata(0)
    }
}
impl core::fmt::Debug for Rdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdata").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdata {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Reload Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlc(pub u32);
impl Rlc {
    #[doc = "Reload shadow registers"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> super::vals::ReloadShadows {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ReloadShadows::from_bits(val as u8)
    }
    #[doc = "Reload shadow registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: super::vals::ReloadShadows) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Rlc {
    #[inline(always)]
    fn default() -> Rlc {
        Rlc(0)
    }
}
impl core::fmt::Debug for Rlc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rlc")
            .field("reload_shadows", &self.reload_shadows())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rlc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rlc {{ reload_shadows: {:?} }}", self.reload_shadows())
    }
}
#[doc = "Read and Write Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwc(pub u32);
impl Rwc {
    #[doc = "EFUSE address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "EFUSE address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Write all 1s"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_all1s(&self) -> super::vals::WrAll1s {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::WrAll1s::from_bits(val as u8)
    }
    #[doc = "Write all 1s"]
    #[inline(always)]
    pub const fn set_wr_all1s(&mut self, val: super::vals::WrAll1s) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Read EFUSE"]
    #[must_use]
    #[inline(always)]
    pub const fn read_efuse(&self) -> super::vals::ReadEfuse {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ReadEfuse::from_bits(val as u8)
    }
    #[doc = "Read EFUSE"]
    #[inline(always)]
    pub const fn set_read_efuse(&mut self, val: super::vals::ReadEfuse) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Read update"]
    #[must_use]
    #[inline(always)]
    pub const fn read_update(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Read update"]
    #[inline(always)]
    pub const fn set_read_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Rwc {
    #[inline(always)]
    fn default() -> Rwc {
        Rwc(0)
    }
}
impl core::fmt::Debug for Rwc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rwc")
            .field("addr", &self.addr())
            .field("wr_all1s", &self.wr_all1s())
            .field("read_efuse", &self.read_efuse())
            .field("read_update", &self.read_update())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rwc {{ addr: {=u8:?}, wr_all1s: {:?}, read_efuse: {:?}, read_update: {=bool:?}, wr_unlock: {=u16:?} }}",
            self.addr(),
            self.wr_all1s(),
            self.read_efuse(),
            self.read_update(),
            self.wr_unlock()
        )
    }
}
#[doc = "Secure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Secure(pub u32);
impl Secure {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Secure {
    #[inline(always)]
    fn default() -> Secure {
        Secure(0)
    }
}
impl core::fmt::Debug for Secure {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Secure").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Secure {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Secure {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Inverted Secure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecureInv(pub u32);
impl SecureInv {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecureInv {
    #[inline(always)]
    fn default() -> SecureInv {
        SecureInv(0)
    }
}
impl core::fmt::Debug for SecureInv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecureInv")
            .field("dat", &self.dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecureInv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SecureInv {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Busy status"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy status"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error flag"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ECC single fault"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_sf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ECC single fault"]
    #[inline(always)]
    pub const fn set_ecc_sf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ECC double fault"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_df(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ECC double fault"]
    #[inline(always)]
    pub const fn set_ecc_df(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Triple voting fault"]
    #[must_use]
    #[inline(always)]
    pub const fn tri_f(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Triple voting fault"]
    #[inline(always)]
    pub const fn set_tri_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read fuse lock error"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_fuse_lock(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Read fuse lock error"]
    #[inline(always)]
    pub const fn set_rd_fuse_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write fuse lock error"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_fuse_lock(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write fuse lock error"]
    #[inline(always)]
    pub const fn set_wr_fuse_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Read register lock error"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_reg_lock(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Read register lock error"]
    #[inline(always)]
    pub const fn set_rd_reg_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write register lock error"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_reg_lock(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write register lock error"]
    #[inline(always)]
    pub const fn set_wr_reg_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write register when busy error"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_reg_busy(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write register when busy error"]
    #[inline(always)]
    pub const fn set_wr_reg_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write when power off error"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_power_off(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write when power off error"]
    #[inline(always)]
    pub const fn set_wr_power_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Finite-state machine error"]
    #[must_use]
    #[inline(always)]
    pub const fn fsm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Finite-state machine error"]
    #[inline(always)]
    pub const fn set_fsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Fuse load counter error"]
    #[must_use]
    #[inline(always)]
    pub const fn flc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Fuse load counter error"]
    #[inline(always)]
    pub const fn set_flc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Address and data compare error"]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Address and data compare error"]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Inverted register compare error"]
    #[must_use]
    #[inline(always)]
    pub const fn irc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Inverted register compare error"]
    #[inline(always)]
    pub const fn set_irc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Fuse and shadow register compare error"]
    #[must_use]
    #[inline(always)]
    pub const fn fsc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Fuse and shadow register compare error"]
    #[inline(always)]
    pub const fn set_fsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("ecc_sf", &self.ecc_sf())
            .field("ecc_df", &self.ecc_df())
            .field("tri_f", &self.tri_f())
            .field("rd_fuse_lock", &self.rd_fuse_lock())
            .field("wr_fuse_lock", &self.wr_fuse_lock())
            .field("rd_reg_lock", &self.rd_reg_lock())
            .field("wr_reg_lock", &self.wr_reg_lock())
            .field("wr_reg_busy", &self.wr_reg_busy())
            .field("wr_power_off", &self.wr_power_off())
            .field("fsm", &self.fsm())
            .field("flc", &self.flc())
            .field("adc", &self.adc())
            .field("irc", &self.irc())
            .field("fsc", &self.fsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ busy: {=bool:?}, error: {=bool:?}, ecc_sf: {=bool:?}, ecc_df: {=bool:?}, tri_f: {=bool:?}, rd_fuse_lock: {=bool:?}, wr_fuse_lock: {=bool:?}, rd_reg_lock: {=bool:?}, wr_reg_lock: {=bool:?}, wr_reg_busy: {=bool:?}, wr_power_off: {=bool:?}, fsm: {=bool:?}, flc: {=bool:?}, adc: {=bool:?}, irc: {=bool:?}, fsc: {=bool:?} }}",
            self.busy(),
            self.error(),
            self.ecc_sf(),
            self.ecc_df(),
            self.tri_f(),
            self.rd_fuse_lock(),
            self.wr_fuse_lock(),
            self.rd_reg_lock(),
            self.wr_reg_lock(),
            self.wr_reg_busy(),
            self.wr_power_off(),
            self.fsm(),
            self.flc(),
            self.adc(),
            self.irc(),
            self.fsc()
        )
    }
}
#[doc = "Timing1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing1(pub u32);
impl Timing1 {
    #[doc = "Address to STROBE setup and hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn taddr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Address to STROBE setup and hold time"]
    #[inline(always)]
    pub const fn set_taddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CSB, PGENB and LOAD to STROBE setup and hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn trelax(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CSB, PGENB and LOAD to STROBE setup and hold time"]
    #[inline(always)]
    pub const fn set_trelax(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Read strobe pulse width time"]
    #[must_use]
    #[inline(always)]
    pub const fn trd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Read strobe pulse width time"]
    #[inline(always)]
    pub const fn set_trd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "PS to CSB setup and hold time between power switch and chip select assertion"]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "PS to CSB setup and hold time between power switch and chip select assertion"]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "PD to CSB setup time between power down signal deassertion and chip select signal assertion"]
    #[must_use]
    #[inline(always)]
    pub const fn tpd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PD to CSB setup time between power down signal deassertion and chip select signal assertion"]
    #[inline(always)]
    pub const fn set_tpd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Timing1 {
    #[inline(always)]
    fn default() -> Timing1 {
        Timing1(0)
    }
}
impl core::fmt::Debug for Timing1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing1")
            .field("taddr", &self.taddr())
            .field("trelax", &self.trelax())
            .field("trd", &self.trd())
            .field("tps", &self.tps())
            .field("tpd", &self.tpd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timing1 {{ taddr: {=u8:?}, trelax: {=u8:?}, trd: {=u8:?}, tps: {=u8:?}, tpd: {=u8:?} }}",
            self.taddr(),
            self.trelax(),
            self.trd(),
            self.tps(),
            self.tpd()
        )
    }
}
#[doc = "Timing2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing2(pub u32);
impl Timing2 {
    #[doc = "Typical program strobe pulse width time"]
    #[must_use]
    #[inline(always)]
    pub const fn tpgm(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Typical program strobe pulse width time"]
    #[inline(always)]
    pub const fn set_tpgm(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Timing2 {
    #[inline(always)]
    fn default() -> Timing2 {
        Timing2(0)
    }
}
impl core::fmt::Debug for Timing2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing2")
            .field("tpgm", &self.tpgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timing2 {{ tpgm: {=u16:?} }}", self.tpgm())
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[doc = "Write Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdata(pub u32);
impl Wdata {
    #[doc = "Write data"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write data"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wdata {
    #[inline(always)]
    fn default() -> Wdata {
        Wdata(0)
    }
}
impl core::fmt::Debug for Wdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdata").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdata {{ dat: {=u32:?} }}", self.dat())
    }
}
