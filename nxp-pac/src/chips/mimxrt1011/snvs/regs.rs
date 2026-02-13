#[doc = "SNVS_HP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpcomr(pub u32);
impl Hpcomr {
    #[doc = "SSM State Transition Transition state of the system security monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn ssm_st(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSM State Transition Transition state of the system security monitor"]
    #[inline(always)]
    pub const fn set_ssm_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[must_use]
    #[inline(always)]
    pub const fn ssm_st_dis(&self) -> super::vals::SsmStDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SsmStDis::from_bits(val as u8)
    }
    #[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    pub const fn set_ssm_st_dis(&mut self, val: super::vals::SsmStDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[must_use]
    #[inline(always)]
    pub const fn ssm_sfns_dis(&self) -> super::vals::SsmSfnsDis {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SsmSfnsDis::from_bits(val as u8)
    }
    #[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    pub const fn set_ssm_sfns_dis(&mut self, val: super::vals::SsmSfnsDis) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_swr(&self) -> super::vals::LpSwr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LpSwr::from_bits(val as u8)
    }
    #[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    #[inline(always)]
    pub const fn set_lp_swr(&mut self, val: super::vals::LpSwr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LP Software Reset Disable When set, disables the LP software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_swr_dis(&self) -> super::vals::LpSwrDis {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LpSwrDis::from_bits(val as u8)
    }
    #[doc = "LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    pub const fn set_lp_swr_dis(&mut self, val: super::vals::LpSwrDis) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_sv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    pub const fn set_sw_sv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_fsv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    pub const fn set_sw_fsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_lpsv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    pub const fn set_sw_lpsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn prog_zmk(&self) -> super::vals::ProgZmk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ProgZmk::from_bits(val as u8)
    }
    #[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[inline(always)]
    pub const fn set_prog_zmk(&mut self, val: super::vals::ProgZmk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[must_use]
    #[inline(always)]
    pub const fn mks_en(&self) -> super::vals::MksEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MksEn::from_bits(val as u8)
    }
    #[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    pub const fn set_mks_en(&mut self, val: super::vals::MksEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    pub const fn set_hac_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_load(&self) -> super::vals::HacLoad {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::HacLoad::from_bits(val as u8)
    }
    #[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[inline(always)]
    pub const fn set_hac_load(&mut self, val: super::vals::HacLoad) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_clear(&self) -> super::vals::HacClear {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HacClear::from_bits(val as u8)
    }
    #[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[inline(always)]
    pub const fn set_hac_clear(&mut self, val: super::vals::HacClear) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_stop(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    pub const fn set_hac_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[must_use]
    #[inline(always)]
    pub const fn npswa_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    pub const fn set_npswa_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpcomr {
    #[inline(always)]
    fn default() -> Hpcomr {
        Hpcomr(0)
    }
}
impl core::fmt::Debug for Hpcomr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpcomr")
            .field("ssm_st", &self.ssm_st())
            .field("ssm_st_dis", &self.ssm_st_dis())
            .field("ssm_sfns_dis", &self.ssm_sfns_dis())
            .field("lp_swr", &self.lp_swr())
            .field("lp_swr_dis", &self.lp_swr_dis())
            .field("sw_sv", &self.sw_sv())
            .field("sw_fsv", &self.sw_fsv())
            .field("sw_lpsv", &self.sw_lpsv())
            .field("prog_zmk", &self.prog_zmk())
            .field("mks_en", &self.mks_en())
            .field("hac_en", &self.hac_en())
            .field("hac_load", &self.hac_load())
            .field("hac_clear", &self.hac_clear())
            .field("hac_stop", &self.hac_stop())
            .field("npswa_en", &self.npswa_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpcomr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpcomr {{ ssm_st: {=bool:?}, ssm_st_dis: {:?}, ssm_sfns_dis: {:?}, lp_swr: {:?}, lp_swr_dis: {:?}, sw_sv: {=bool:?}, sw_fsv: {=bool:?}, sw_lpsv: {=bool:?}, prog_zmk: {:?}, mks_en: {:?}, hac_en: {=bool:?}, hac_load: {:?}, hac_clear: {:?}, hac_stop: {=bool:?}, npswa_en: {=bool:?} }}",
            self.ssm_st(),
            self.ssm_st_dis(),
            self.ssm_sfns_dis(),
            self.lp_swr(),
            self.lp_swr_dis(),
            self.sw_sv(),
            self.sw_fsv(),
            self.sw_lpsv(),
            self.prog_zmk(),
            self.mks_en(),
            self.hac_en(),
            self.hac_load(),
            self.hac_clear(),
            self.hac_stop(),
            self.npswa_en()
        )
    }
}
#[doc = "SNVS_HP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpcr(pub u32);
impl Hpcr {
    #[doc = "HP Real Time Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HP Real Time Counter Enable"]
    #[inline(always)]
    pub const fn set_rtc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn hpta_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    pub const fn set_hpta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Disable periodic interrupt in the functional interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_pi(&self) -> super::vals::DisPi {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DisPi::from_bits(val as u8)
    }
    #[doc = "Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    pub const fn set_dis_pi(&mut self, val: super::vals::DisPi) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn pi_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    pub const fn set_pi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pi_freq(&self) -> super::vals::PiFreq {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::PiFreq::from_bits(val as u8)
    }
    #[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    pub const fn set_pi_freq(&mut self, val: super::vals::PiFreq) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn hpcalb_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    pub const fn set_hpcalb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn hpcalb_val(&self) -> super::vals::HpcalbVal {
        let val = (self.0 >> 10usize) & 0x1f;
        super::vals::HpcalbVal::from_bits(val as u8)
    }
    #[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    pub const fn set_hpcalb_val(&mut self, val: super::vals::HpcalbVal) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "HP Time Synchronize"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_ts(&self) -> super::vals::HpTs {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::HpTs::from_bits(val as u8)
    }
    #[doc = "HP Time Synchronize"]
    #[inline(always)]
    pub const fn set_hp_ts(&mut self, val: super::vals::HpTs) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Button Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn btn_config(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Button Configuration"]
    #[inline(always)]
    pub const fn set_btn_config(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Button interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn btn_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Button interrupt mask"]
    #[inline(always)]
    pub const fn set_btn_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Hpcr {
    #[inline(always)]
    fn default() -> Hpcr {
        Hpcr(0)
    }
}
impl core::fmt::Debug for Hpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpcr")
            .field("rtc_en", &self.rtc_en())
            .field("hpta_en", &self.hpta_en())
            .field("dis_pi", &self.dis_pi())
            .field("pi_en", &self.pi_en())
            .field("pi_freq", &self.pi_freq())
            .field("hpcalb_en", &self.hpcalb_en())
            .field("hpcalb_val", &self.hpcalb_val())
            .field("hp_ts", &self.hp_ts())
            .field("btn_config", &self.btn_config())
            .field("btn_mask", &self.btn_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpcr {{ rtc_en: {=bool:?}, hpta_en: {=bool:?}, dis_pi: {:?}, pi_en: {=bool:?}, pi_freq: {:?}, hpcalb_en: {=bool:?}, hpcalb_val: {:?}, hp_ts: {:?}, btn_config: {=u8:?}, btn_mask: {=bool:?} }}",
            self.rtc_en(),
            self.hpta_en(),
            self.dis_pi(),
            self.pi_en(),
            self.pi_freq(),
            self.hpcalb_en(),
            self.hpcalb_val(),
            self.hp_ts(),
            self.btn_config(),
            self.btn_mask()
        )
    }
}
#[doc = "SNVS_HP High Assurance Counter IV Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hphacivr(pub u32);
impl Hphacivr {
    #[doc = "High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_counter_iv(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    #[inline(always)]
    pub const fn set_hac_counter_iv(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hphacivr {
    #[inline(always)]
    fn default() -> Hphacivr {
        Hphacivr(0)
    }
}
impl core::fmt::Debug for Hphacivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hphacivr")
            .field("hac_counter_iv", &self.hac_counter_iv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hphacivr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hphacivr {{ hac_counter_iv: {=u32:?} }}",
            self.hac_counter_iv()
        )
    }
}
#[doc = "SNVS_HP High Assurance Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hphacr(pub u32);
impl Hphacr {
    #[doc = "High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock"]
    #[inline(always)]
    pub const fn set_hac_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hphacr {
    #[inline(always)]
    fn default() -> Hphacr {
        Hphacr(0)
    }
}
impl core::fmt::Debug for Hphacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hphacr")
            .field("hac_counter", &self.hac_counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hphacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hphacr {{ hac_counter: {=u32:?} }}", self.hac_counter())
    }
}
#[doc = "SNVS_HP Lock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hplr(pub u32);
impl Hplr {
    #[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_wsl(&self) -> super::vals::ZmkWsl {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ZmkWsl::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_wsl(&mut self, val: super::vals::ZmkWsl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_rsl(&self) -> super::vals::ZmkRsl {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ZmkRsl::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_rsl(&mut self, val: super::vals::ZmkRsl) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[must_use]
    #[inline(always)]
    pub const fn srtc_sl(&self) -> super::vals::SrtcSl {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SrtcSl::from_bits(val as u8)
    }
    #[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub const fn set_srtc_sl(&mut self, val: super::vals::SrtcSl) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcalb_sl(&self) -> super::vals::LpcalbSl {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LpcalbSl::from_bits(val as u8)
    }
    #[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub const fn set_lpcalb_sl(&mut self, val: super::vals::LpcalbSl) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[must_use]
    #[inline(always)]
    pub const fn mc_sl(&self) -> super::vals::McSl {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::McSl::from_bits(val as u8)
    }
    #[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub const fn set_mc_sl(&mut self, val: super::vals::McSl) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[must_use]
    #[inline(always)]
    pub const fn gpr_sl(&self) -> super::vals::GprSl {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::GprSl::from_bits(val as u8)
    }
    #[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub const fn set_gpr_sl(&mut self, val: super::vals::GprSl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[must_use]
    #[inline(always)]
    pub const fn lpsvcr_sl(&self) -> super::vals::LpsvcrSl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::LpsvcrSl::from_bits(val as u8)
    }
    #[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub const fn set_lpsvcr_sl(&mut self, val: super::vals::LpsvcrSl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Soft Lock When set, prevents any writes to the LPTGFCR"]
    #[must_use]
    #[inline(always)]
    pub const fn lptgfcr_sl(&self) -> super::vals::LptgfcrSl {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::LptgfcrSl::from_bits(val as u8)
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Soft Lock When set, prevents any writes to the LPTGFCR"]
    #[inline(always)]
    pub const fn set_lptgfcr_sl(&mut self, val: super::vals::LptgfcrSl) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    #[must_use]
    #[inline(always)]
    pub const fn lpsecr_sl(&self) -> super::vals::LpsecrSl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::LpsecrSl::from_bits(val as u8)
    }
    #[doc = "LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub const fn set_lpsecr_sl(&mut self, val: super::vals::LpsecrSl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[must_use]
    #[inline(always)]
    pub const fn mks_sl(&self) -> super::vals::MksSl {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::MksSl::from_bits(val as u8)
    }
    #[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    pub const fn set_mks_sl(&mut self, val: super::vals::MksSl) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[must_use]
    #[inline(always)]
    pub const fn hpsvcr_l(&self) -> super::vals::HpsvcrL {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::HpsvcrL::from_bits(val as u8)
    }
    #[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    pub const fn set_hpsvcr_l(&mut self, val: super::vals::HpsvcrL) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[must_use]
    #[inline(always)]
    pub const fn hpsicr_l(&self) -> super::vals::HpsicrL {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::HpsicrL::from_bits(val as u8)
    }
    #[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    pub const fn set_hpsicr_l(&mut self, val: super::vals::HpsicrL) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[must_use]
    #[inline(always)]
    pub const fn hac_l(&self) -> super::vals::HacL {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HacL::from_bits(val as u8)
    }
    #[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    pub const fn set_hac_l(&mut self, val: super::vals::HacL) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
}
impl Default for Hplr {
    #[inline(always)]
    fn default() -> Hplr {
        Hplr(0)
    }
}
impl core::fmt::Debug for Hplr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hplr")
            .field("zmk_wsl", &self.zmk_wsl())
            .field("zmk_rsl", &self.zmk_rsl())
            .field("srtc_sl", &self.srtc_sl())
            .field("lpcalb_sl", &self.lpcalb_sl())
            .field("mc_sl", &self.mc_sl())
            .field("gpr_sl", &self.gpr_sl())
            .field("lpsvcr_sl", &self.lpsvcr_sl())
            .field("lptgfcr_sl", &self.lptgfcr_sl())
            .field("lpsecr_sl", &self.lpsecr_sl())
            .field("mks_sl", &self.mks_sl())
            .field("hpsvcr_l", &self.hpsvcr_l())
            .field("hpsicr_l", &self.hpsicr_l())
            .field("hac_l", &self.hac_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hplr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hplr {{ zmk_wsl: {:?}, zmk_rsl: {:?}, srtc_sl: {:?}, lpcalb_sl: {:?}, mc_sl: {:?}, gpr_sl: {:?}, lpsvcr_sl: {:?}, lptgfcr_sl: {:?}, lpsecr_sl: {:?}, mks_sl: {:?}, hpsvcr_l: {:?}, hpsicr_l: {:?}, hac_l: {:?} }}",
            self.zmk_wsl(),
            self.zmk_rsl(),
            self.srtc_sl(),
            self.lpcalb_sl(),
            self.mc_sl(),
            self.gpr_sl(),
            self.lpsvcr_sl(),
            self.lptgfcr_sl(),
            self.lpsecr_sl(),
            self.mks_sl(),
            self.hpsvcr_l(),
            self.hpsicr_l(),
            self.hac_l()
        )
    }
}
#[doc = "SNVS_HP Real Time Counter LSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hprtclr(pub u32);
impl Hprtclr {
    #[doc = "HP Real Time Counter least-significant 32 bits"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HP Real Time Counter least-significant 32 bits"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hprtclr {
    #[inline(always)]
    fn default() -> Hprtclr {
        Hprtclr(0)
    }
}
impl core::fmt::Debug for Hprtclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hprtclr").field("rtc", &self.rtc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hprtclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hprtclr {{ rtc: {=u32:?} }}", self.rtc())
    }
}
#[doc = "SNVS_HP Real Time Counter MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hprtcmr(pub u32);
impl Hprtcmr {
    #[doc = "HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Hprtcmr {
    #[inline(always)]
    fn default() -> Hprtcmr {
        Hprtcmr(0)
    }
}
impl core::fmt::Debug for Hprtcmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hprtcmr").field("rtc", &self.rtc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hprtcmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hprtcmr {{ rtc: {=u16:?} }}", self.rtc())
    }
}
#[doc = "SNVS_HP Security Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpsicr(pub u32);
impl Hpsicr {
    #[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sv0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    pub const fn set_sv0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sv1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    pub const fn set_sv1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sv2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    pub const fn set_sv2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sv3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    pub const fn set_sv3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sv4_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    pub const fn set_sv4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn sv5_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    pub const fn set_sv5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[must_use]
    #[inline(always)]
    pub const fn lpsvi_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    pub const fn set_lpsvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpsicr {
    #[inline(always)]
    fn default() -> Hpsicr {
        Hpsicr(0)
    }
}
impl core::fmt::Debug for Hpsicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpsicr")
            .field("sv0_en", &self.sv0_en())
            .field("sv1_en", &self.sv1_en())
            .field("sv2_en", &self.sv2_en())
            .field("sv3_en", &self.sv3_en())
            .field("sv4_en", &self.sv4_en())
            .field("sv5_en", &self.sv5_en())
            .field("lpsvi_en", &self.lpsvi_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpsicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpsicr {{ sv0_en: {=bool:?}, sv1_en: {=bool:?}, sv2_en: {=bool:?}, sv3_en: {=bool:?}, sv4_en: {=bool:?}, sv5_en: {=bool:?}, lpsvi_en: {=bool:?} }}",
            self.sv0_en(),
            self.sv1_en(),
            self.sv2_en(),
            self.sv3_en(),
            self.sv4_en(),
            self.sv5_en(),
            self.lpsvi_en()
        )
    }
}
#[doc = "SNVS_HP Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpsr(pub u32);
impl Hpsr {
    #[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn hpta(&self) -> super::vals::Hpta {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hpta::from_bits(val as u8)
    }
    #[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    pub const fn set_hpta(&mut self, val: super::vals::Hpta) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn pi(&self) -> super::vals::Pi {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pi::from_bits(val as u8)
    }
    #[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    pub const fn set_pi(&mut self, val: super::vals::Pi) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[must_use]
    #[inline(always)]
    pub const fn lpdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[inline(always)]
    pub const fn set_lpdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Button Value of the BTN input"]
    #[must_use]
    #[inline(always)]
    pub const fn btn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Button Value of the BTN input"]
    #[inline(always)]
    pub const fn set_btn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn bi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    pub const fn set_bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[must_use]
    #[inline(always)]
    pub const fn ssm_state(&self) -> super::vals::SsmState {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::SsmState::from_bits(val as u8)
    }
    #[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[inline(always)]
    pub const fn set_ssm_state(&mut self, val: super::vals::SsmState) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "System Security Configuration This field reflects the three security configuration inputs to SNVS"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_security_cfg(&self) -> super::vals::SysSecurityCfg {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::SysSecurityCfg::from_bits(val as u8)
    }
    #[doc = "System Security Configuration This field reflects the three security configuration inputs to SNVS"]
    #[inline(always)]
    pub const fn set_sys_security_cfg(&mut self, val: super::vals::SysSecurityCfg) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_secure_boot(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
    #[inline(always)]
    pub const fn set_sys_secure_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[must_use]
    #[inline(always)]
    pub const fn otpmk_syndrome(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[inline(always)]
    pub const fn set_otpmk_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "One Time Programmable Master Key is Equal to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn otpmk_zero(&self) -> super::vals::OtpmkZero {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OtpmkZero::from_bits(val as u8)
    }
    #[doc = "One Time Programmable Master Key is Equal to Zero"]
    #[inline(always)]
    pub const fn set_otpmk_zero(&mut self, val: super::vals::OtpmkZero) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Zeroizable Master Key is Equal to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_zero(&self) -> super::vals::ZmkZero {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ZmkZero::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key is Equal to Zero"]
    #[inline(always)]
    pub const fn set_zmk_zero(&mut self, val: super::vals::ZmkZero) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpsr {
    #[inline(always)]
    fn default() -> Hpsr {
        Hpsr(0)
    }
}
impl core::fmt::Debug for Hpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpsr")
            .field("hpta", &self.hpta())
            .field("pi", &self.pi())
            .field("lpdis", &self.lpdis())
            .field("btn", &self.btn())
            .field("bi", &self.bi())
            .field("ssm_state", &self.ssm_state())
            .field("sys_security_cfg", &self.sys_security_cfg())
            .field("sys_secure_boot", &self.sys_secure_boot())
            .field("otpmk_syndrome", &self.otpmk_syndrome())
            .field("otpmk_zero", &self.otpmk_zero())
            .field("zmk_zero", &self.zmk_zero())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpsr {{ hpta: {:?}, pi: {:?}, lpdis: {=bool:?}, btn: {=bool:?}, bi: {=bool:?}, ssm_state: {:?}, sys_security_cfg: {:?}, sys_secure_boot: {=bool:?}, otpmk_syndrome: {=u16:?}, otpmk_zero: {:?}, zmk_zero: {:?} }}",
            self.hpta(),
            self.pi(),
            self.lpdis(),
            self.btn(),
            self.bi(),
            self.ssm_state(),
            self.sys_security_cfg(),
            self.sys_secure_boot(),
            self.otpmk_syndrome(),
            self.otpmk_zero(),
            self.zmk_zero()
        )
    }
}
#[doc = "SNVS_HP Security Violation Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpsvcr(pub u32);
impl Hpsvcr {
    #[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv0_cfg(&self) -> super::vals::Sv0Cfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sv0Cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv0_cfg(&mut self, val: super::vals::Sv0Cfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv1_cfg(&self) -> super::vals::Sv1Cfg {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sv1Cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv1_cfg(&mut self, val: super::vals::Sv1Cfg) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv2_cfg(&self) -> super::vals::Sv2Cfg {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sv2Cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv2_cfg(&mut self, val: super::vals::Sv2Cfg) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv3_cfg(&self) -> super::vals::Sv3Cfg {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sv3Cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv3_cfg(&mut self, val: super::vals::Sv3Cfg) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv4_cfg(&self) -> super::vals::Sv4Cfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sv4Cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv4_cfg(&mut self, val: super::vals::Sv4Cfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv5_cfg(&self) -> super::vals::Sv5Cfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Sv5Cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv5_cfg(&mut self, val: super::vals::Sv5Cfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "LP Security Violation Configuration This field configures the LP security violation source."]
    #[must_use]
    #[inline(always)]
    pub const fn lpsv_cfg(&self) -> super::vals::LpsvCfg {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::LpsvCfg::from_bits(val as u8)
    }
    #[doc = "LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    pub const fn set_lpsv_cfg(&mut self, val: super::vals::LpsvCfg) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Hpsvcr {
    #[inline(always)]
    fn default() -> Hpsvcr {
        Hpsvcr(0)
    }
}
impl core::fmt::Debug for Hpsvcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpsvcr")
            .field("sv0_cfg", &self.sv0_cfg())
            .field("sv1_cfg", &self.sv1_cfg())
            .field("sv2_cfg", &self.sv2_cfg())
            .field("sv3_cfg", &self.sv3_cfg())
            .field("sv4_cfg", &self.sv4_cfg())
            .field("sv5_cfg", &self.sv5_cfg())
            .field("lpsv_cfg", &self.lpsv_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpsvcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpsvcr {{ sv0_cfg: {:?}, sv1_cfg: {:?}, sv2_cfg: {:?}, sv3_cfg: {:?}, sv4_cfg: {:?}, sv5_cfg: {:?}, lpsv_cfg: {:?} }}",
            self.sv0_cfg(),
            self.sv1_cfg(),
            self.sv2_cfg(),
            self.sv3_cfg(),
            self.sv4_cfg(),
            self.sv5_cfg(),
            self.lpsv_cfg()
        )
    }
}
#[doc = "SNVS_HP Security Violation Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpsvsr(pub u32);
impl Hpsvsr {
    #[doc = "Security Violation 0 security violation was detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sv0(&self) -> super::vals::Sv0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sv0::from_bits(val as u8)
    }
    #[doc = "Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv0(&mut self, val: super::vals::Sv0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sv1(&self) -> super::vals::Sv1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sv1::from_bits(val as u8)
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv1(&mut self, val: super::vals::Sv1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sv2(&self) -> super::vals::Sv2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sv2::from_bits(val as u8)
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv2(&mut self, val: super::vals::Sv2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sv3(&self) -> super::vals::Sv3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sv3::from_bits(val as u8)
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv3(&mut self, val: super::vals::Sv3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sv4(&self) -> super::vals::Sv4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sv4::from_bits(val as u8)
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv4(&mut self, val: super::vals::Sv4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[must_use]
    #[inline(always)]
    pub const fn sv5(&self) -> super::vals::Sv5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sv5::from_bits(val as u8)
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv5(&mut self, val: super::vals::Sv5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_sv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn set_sw_sv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_fsv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn set_sw_fsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_lpsv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn set_sw_lpsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_syndrome(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[inline(always)]
    pub const fn set_zmk_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_ecc_fail(&self) -> super::vals::ZmkEccFail {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ZmkEccFail::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    pub const fn set_zmk_ecc_fail(&mut self, val: super::vals::ZmkEccFail) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "LP Security Violation A security volation was detected in the SNVS low power section"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_sec_vio(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Violation A security volation was detected in the SNVS low power section"]
    #[inline(always)]
    pub const fn set_lp_sec_vio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpsvsr {
    #[inline(always)]
    fn default() -> Hpsvsr {
        Hpsvsr(0)
    }
}
impl core::fmt::Debug for Hpsvsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpsvsr")
            .field("sv0", &self.sv0())
            .field("sv1", &self.sv1())
            .field("sv2", &self.sv2())
            .field("sv3", &self.sv3())
            .field("sv4", &self.sv4())
            .field("sv5", &self.sv5())
            .field("sw_sv", &self.sw_sv())
            .field("sw_fsv", &self.sw_fsv())
            .field("sw_lpsv", &self.sw_lpsv())
            .field("zmk_syndrome", &self.zmk_syndrome())
            .field("zmk_ecc_fail", &self.zmk_ecc_fail())
            .field("lp_sec_vio", &self.lp_sec_vio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpsvsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpsvsr {{ sv0: {:?}, sv1: {:?}, sv2: {:?}, sv3: {:?}, sv4: {:?}, sv5: {:?}, sw_sv: {=bool:?}, sw_fsv: {=bool:?}, sw_lpsv: {=bool:?}, zmk_syndrome: {=u16:?}, zmk_ecc_fail: {:?}, lp_sec_vio: {=bool:?} }}",
            self.sv0(),
            self.sv1(),
            self.sv2(),
            self.sv3(),
            self.sv4(),
            self.sv5(),
            self.sw_sv(),
            self.sw_fsv(),
            self.sw_lpsv(),
            self.zmk_syndrome(),
            self.zmk_ecc_fail(),
            self.lp_sec_vio()
        )
    }
}
#[doc = "SNVS_HP Time Alarm LSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptalr(pub u32);
impl Hptalr {
    #[doc = "HP Time Alarm, 32 least-significant bits"]
    #[must_use]
    #[inline(always)]
    pub const fn hpta_ls(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HP Time Alarm, 32 least-significant bits"]
    #[inline(always)]
    pub const fn set_hpta_ls(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hptalr {
    #[inline(always)]
    fn default() -> Hptalr {
        Hptalr(0)
    }
}
impl core::fmt::Debug for Hptalr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hptalr")
            .field("hpta_ls", &self.hpta_ls())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hptalr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hptalr {{ hpta_ls: {=u32:?} }}", self.hpta_ls())
    }
}
#[doc = "SNVS_HP Time Alarm MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptamr(pub u32);
impl Hptamr {
    #[doc = "HP Time Alarm, most-significant 15 bits"]
    #[must_use]
    #[inline(always)]
    pub const fn hpta_ms(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    pub const fn set_hpta_ms(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Hptamr {
    #[inline(always)]
    fn default() -> Hptamr {
        Hptamr(0)
    }
}
impl core::fmt::Debug for Hptamr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hptamr")
            .field("hpta_ms", &self.hpta_ms())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hptamr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hptamr {{ hpta_ms: {=u16:?} }}", self.hpta_ms())
    }
}
#[doc = "SNVS_HP Version ID Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpvidr1(pub u32);
impl Hpvidr1 {
    #[doc = "SNVS block minor version number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS block minor version number"]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SNVS block major version number"]
    #[must_use]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS block major version number"]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "SNVS block ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "SNVS block ID"]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hpvidr1 {
    #[inline(always)]
    fn default() -> Hpvidr1 {
        Hpvidr1(0)
    }
}
impl core::fmt::Debug for Hpvidr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpvidr1")
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpvidr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpvidr1 {{ minor_rev: {=u8:?}, major_rev: {=u8:?}, ip_id: {=u16:?} }}",
            self.minor_rev(),
            self.major_rev(),
            self.ip_id()
        )
    }
}
#[doc = "SNVS_HP Version ID Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpvidr2(pub u32);
impl Hpvidr2 {
    #[doc = "SNVS Configuration Options"]
    #[must_use]
    #[inline(always)]
    pub const fn config_opt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS Configuration Options"]
    #[inline(always)]
    pub const fn set_config_opt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SNVS ECO Revision"]
    #[must_use]
    #[inline(always)]
    pub const fn eco_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS ECO Revision"]
    #[inline(always)]
    pub const fn set_eco_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "SNVS Integration Options"]
    #[must_use]
    #[inline(always)]
    pub const fn intg_opt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS Integration Options"]
    #[inline(always)]
    pub const fn set_intg_opt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_era(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
    #[inline(always)]
    pub const fn set_ip_era(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hpvidr2 {
    #[inline(always)]
    fn default() -> Hpvidr2 {
        Hpvidr2(0)
    }
}
impl core::fmt::Debug for Hpvidr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpvidr2")
            .field("config_opt", &self.config_opt())
            .field("eco_rev", &self.eco_rev())
            .field("intg_opt", &self.intg_opt())
            .field("ip_era", &self.ip_era())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpvidr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpvidr2 {{ config_opt: {=u8:?}, eco_rev: {=u8:?}, intg_opt: {=u8:?}, ip_era: {=u8:?} }}",
            self.config_opt(),
            self.eco_rev(),
            self.intg_opt(),
            self.ip_era()
        )
    }
}
#[doc = "SNVS_LP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpcr(pub u32);
impl Lpcr {
    #[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[must_use]
    #[inline(always)]
    pub const fn srtc_env(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    pub const fn set_srtc_env(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[must_use]
    #[inline(always)]
    pub const fn lpta_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    pub const fn set_lpta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[must_use]
    #[inline(always)]
    pub const fn mc_env(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    pub const fn set_mc_env(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[must_use]
    #[inline(always)]
    pub const fn lpwui_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    pub const fn set_lpwui_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[must_use]
    #[inline(always)]
    pub const fn srtc_inv_en(&self) -> super::vals::SrtcInvEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SrtcInvEn::from_bits(val as u8)
    }
    #[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    pub const fn set_srtc_inv_en(&mut self, val: super::vals::SrtcInvEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Dumb PMIC Enabled When set, software can control the system power"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_en(&self) -> super::vals::DpEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DpEn::from_bits(val as u8)
    }
    #[doc = "Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    pub const fn set_dp_en(&mut self, val: super::vals::DpEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[must_use]
    #[inline(always)]
    pub const fn top(&self) -> super::vals::Top {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Top::from_bits(val as u8)
    }
    #[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    pub const fn set_top(&mut self, val: super::vals::Top) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    pub const fn set_lvd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcalb_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    pub const fn set_lpcalb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LP Calibration Value Defines signed calibration value for SRTC"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcalb_val(&self) -> super::vals::LpcalbVal {
        let val = (self.0 >> 10usize) & 0x1f;
        super::vals::LpcalbVal::from_bits(val as u8)
    }
    #[doc = "LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    pub const fn set_lpcalb_val(&mut self, val: super::vals::LpcalbVal) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "This field configures the button press time out values for the PMIC Logic"]
    #[must_use]
    #[inline(always)]
    pub const fn btn_press_time(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    pub const fn set_btn_press_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "This field configures the amount of debounce time for the BTN input signal"]
    #[must_use]
    #[inline(always)]
    pub const fn debounce(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    pub const fn set_debounce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[must_use]
    #[inline(always)]
    pub const fn on_time(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    pub const fn set_on_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[must_use]
    #[inline(always)]
    pub const fn pk_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    pub const fn set_pk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[must_use]
    #[inline(always)]
    pub const fn pk_override(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    pub const fn set_pk_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "General Purpose Registers Zeroization Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpr_z_dis(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    pub const fn set_gpr_z_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Lpcr {
    #[inline(always)]
    fn default() -> Lpcr {
        Lpcr(0)
    }
}
impl core::fmt::Debug for Lpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpcr")
            .field("srtc_env", &self.srtc_env())
            .field("lpta_en", &self.lpta_en())
            .field("mc_env", &self.mc_env())
            .field("lpwui_en", &self.lpwui_en())
            .field("srtc_inv_en", &self.srtc_inv_en())
            .field("dp_en", &self.dp_en())
            .field("top", &self.top())
            .field("lvd_en", &self.lvd_en())
            .field("lpcalb_en", &self.lpcalb_en())
            .field("lpcalb_val", &self.lpcalb_val())
            .field("btn_press_time", &self.btn_press_time())
            .field("debounce", &self.debounce())
            .field("on_time", &self.on_time())
            .field("pk_en", &self.pk_en())
            .field("pk_override", &self.pk_override())
            .field("gpr_z_dis", &self.gpr_z_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpcr {{ srtc_env: {=bool:?}, lpta_en: {=bool:?}, mc_env: {=bool:?}, lpwui_en: {=bool:?}, srtc_inv_en: {:?}, dp_en: {:?}, top: {:?}, lvd_en: {=bool:?}, lpcalb_en: {=bool:?}, lpcalb_val: {:?}, btn_press_time: {=u8:?}, debounce: {=u8:?}, on_time: {=u8:?}, pk_en: {=bool:?}, pk_override: {=bool:?}, gpr_z_dis: {=bool:?} }}",
            self.srtc_env(),
            self.lpta_en(),
            self.mc_env(),
            self.lpwui_en(),
            self.srtc_inv_en(),
            self.dp_en(),
            self.top(),
            self.lvd_en(),
            self.lpcalb_en(),
            self.lpcalb_val(),
            self.btn_press_time(),
            self.debounce(),
            self.on_time(),
            self.pk_en(),
            self.pk_override(),
            self.gpr_z_dis()
        )
    }
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpgpr(pub u32);
impl Lpgpr {
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn gpr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    pub const fn set_gpr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lpgpr {
    #[inline(always)]
    fn default() -> Lpgpr {
        Lpgpr(0)
    }
}
impl core::fmt::Debug for Lpgpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpgpr").field("gpr", &self.gpr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpgpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpgpr {{ gpr: {=u32:?} }}", self.gpr())
    }
}
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpgpr0LegacyAlias(pub u32);
impl Lpgpr0LegacyAlias {
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn gpr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    pub const fn set_gpr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lpgpr0LegacyAlias {
    #[inline(always)]
    fn default() -> Lpgpr0LegacyAlias {
        Lpgpr0LegacyAlias(0)
    }
}
impl core::fmt::Debug for Lpgpr0LegacyAlias {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpgpr0LegacyAlias")
            .field("gpr", &self.gpr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpgpr0LegacyAlias {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpgpr0LegacyAlias {{ gpr: {=u32:?} }}", self.gpr())
    }
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpgprAlias(pub u32);
impl LpgprAlias {
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn gpr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    pub const fn set_gpr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LpgprAlias {
    #[inline(always)]
    fn default() -> LpgprAlias {
        LpgprAlias(0)
    }
}
impl core::fmt::Debug for LpgprAlias {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpgprAlias")
            .field("gpr", &self.gpr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpgprAlias {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpgprAlias {{ gpr: {=u32:?} }}", self.gpr())
    }
}
#[doc = "SNVS_LP Lock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lplr(pub u32);
impl Lplr {
    #[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_whl(&self) -> super::vals::ZmkWhl {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ZmkWhl::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_whl(&mut self, val: super::vals::ZmkWhl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_rhl(&self) -> super::vals::ZmkRhl {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ZmkRhl::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_rhl(&mut self, val: super::vals::ZmkRhl) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[must_use]
    #[inline(always)]
    pub const fn srtc_hl(&self) -> super::vals::SrtcHl {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SrtcHl::from_bits(val as u8)
    }
    #[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub const fn set_srtc_hl(&mut self, val: super::vals::SrtcHl) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcalb_hl(&self) -> super::vals::LpcalbHl {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LpcalbHl::from_bits(val as u8)
    }
    #[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub const fn set_lpcalb_hl(&mut self, val: super::vals::LpcalbHl) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[must_use]
    #[inline(always)]
    pub const fn mc_hl(&self) -> super::vals::McHl {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::McHl::from_bits(val as u8)
    }
    #[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub const fn set_mc_hl(&mut self, val: super::vals::McHl) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[must_use]
    #[inline(always)]
    pub const fn gpr_hl(&self) -> super::vals::GprHl {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::GprHl::from_bits(val as u8)
    }
    #[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub const fn set_gpr_hl(&mut self, val: super::vals::GprHl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[must_use]
    #[inline(always)]
    pub const fn lpsvcr_hl(&self) -> super::vals::LpsvcrHl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::LpsvcrHl::from_bits(val as u8)
    }
    #[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub const fn set_lpsvcr_hl(&mut self, val: super::vals::LpsvcrHl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Hard Lock When set, prevents any writes to the LPTGFCR"]
    #[must_use]
    #[inline(always)]
    pub const fn lptgfcr_hl(&self) -> super::vals::LptgfcrHl {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::LptgfcrHl::from_bits(val as u8)
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Hard Lock When set, prevents any writes to the LPTGFCR"]
    #[inline(always)]
    pub const fn set_lptgfcr_hl(&mut self, val: super::vals::LptgfcrHl) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    #[must_use]
    #[inline(always)]
    pub const fn lpsecr_hl(&self) -> super::vals::LpsecrHl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::LpsecrHl::from_bits(val as u8)
    }
    #[doc = "LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub const fn set_lpsecr_hl(&mut self, val: super::vals::LpsecrHl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[must_use]
    #[inline(always)]
    pub const fn mks_hl(&self) -> super::vals::MksHl {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::MksHl::from_bits(val as u8)
    }
    #[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    pub const fn set_mks_hl(&mut self, val: super::vals::MksHl) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Lplr {
    #[inline(always)]
    fn default() -> Lplr {
        Lplr(0)
    }
}
impl core::fmt::Debug for Lplr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lplr")
            .field("zmk_whl", &self.zmk_whl())
            .field("zmk_rhl", &self.zmk_rhl())
            .field("srtc_hl", &self.srtc_hl())
            .field("lpcalb_hl", &self.lpcalb_hl())
            .field("mc_hl", &self.mc_hl())
            .field("gpr_hl", &self.gpr_hl())
            .field("lpsvcr_hl", &self.lpsvcr_hl())
            .field("lptgfcr_hl", &self.lptgfcr_hl())
            .field("lpsecr_hl", &self.lpsecr_hl())
            .field("mks_hl", &self.mks_hl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lplr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lplr {{ zmk_whl: {:?}, zmk_rhl: {:?}, srtc_hl: {:?}, lpcalb_hl: {:?}, mc_hl: {:?}, gpr_hl: {:?}, lpsvcr_hl: {:?}, lptgfcr_hl: {:?}, lpsecr_hl: {:?}, mks_hl: {:?} }}",
            self.zmk_whl(),
            self.zmk_rhl(),
            self.srtc_hl(),
            self.lpcalb_hl(),
            self.mc_hl(),
            self.gpr_hl(),
            self.lpsvcr_hl(),
            self.lptgfcr_hl(),
            self.lpsecr_hl(),
            self.mks_hl()
        )
    }
}
#[doc = "SNVS_LP Digital Low-Voltage Detector Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lplvdr(pub u32);
impl Lplvdr {
    #[doc = "Low-Voltage Detector Value"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Low-Voltage Detector Value"]
    #[inline(always)]
    pub const fn set_lvd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lplvdr {
    #[inline(always)]
    fn default() -> Lplvdr {
        Lplvdr(0)
    }
}
impl core::fmt::Debug for Lplvdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lplvdr").field("lvd", &self.lvd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lplvdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lplvdr {{ lvd: {=u32:?} }}", self.lvd())
    }
}
#[doc = "SNVS_LP Master Key Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpmkcr(pub u32);
impl Lpmkcr {
    #[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[must_use]
    #[inline(always)]
    pub const fn master_key_sel(&self) -> super::vals::MasterKeySel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MasterKeySel::from_bits(val as u8)
    }
    #[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    pub const fn set_master_key_sel(&mut self, val: super::vals::MasterKeySel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_hwp(&self) -> super::vals::ZmkHwp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ZmkHwp::from_bits(val as u8)
    }
    #[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    pub const fn set_zmk_hwp(&mut self, val: super::vals::ZmkHwp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    pub const fn set_zmk_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_ecc_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    pub const fn set_zmk_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk_ecc_value(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[inline(always)]
    pub const fn set_zmk_ecc_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
}
impl Default for Lpmkcr {
    #[inline(always)]
    fn default() -> Lpmkcr {
        Lpmkcr(0)
    }
}
impl core::fmt::Debug for Lpmkcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpmkcr")
            .field("master_key_sel", &self.master_key_sel())
            .field("zmk_hwp", &self.zmk_hwp())
            .field("zmk_val", &self.zmk_val())
            .field("zmk_ecc_en", &self.zmk_ecc_en())
            .field("zmk_ecc_value", &self.zmk_ecc_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpmkcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpmkcr {{ master_key_sel: {:?}, zmk_hwp: {:?}, zmk_val: {=bool:?}, zmk_ecc_en: {=bool:?}, zmk_ecc_value: {=u16:?} }}",
            self.master_key_sel(),
            self.zmk_hwp(),
            self.zmk_val(),
            self.zmk_ecc_en(),
            self.zmk_ecc_value()
        )
    }
}
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsmclr(pub u32);
impl Lpsmclr {
    #[doc = "Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    pub const fn set_mon_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lpsmclr {
    #[inline(always)]
    fn default() -> Lpsmclr {
        Lpsmclr(0)
    }
}
impl core::fmt::Debug for Lpsmclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpsmclr")
            .field("mon_counter", &self.mon_counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpsmclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpsmclr {{ mon_counter: {=u32:?} }}", self.mon_counter())
    }
}
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsmcmr(pub u32);
impl Lpsmcmr {
    #[doc = "Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    pub const fn set_mon_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    #[must_use]
    #[inline(always)]
    pub const fn mc_era_bits(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    #[inline(always)]
    pub const fn set_mc_era_bits(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Lpsmcmr {
    #[inline(always)]
    fn default() -> Lpsmcmr {
        Lpsmcmr(0)
    }
}
impl core::fmt::Debug for Lpsmcmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpsmcmr")
            .field("mon_counter", &self.mon_counter())
            .field("mc_era_bits", &self.mc_era_bits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpsmcmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpsmcmr {{ mon_counter: {=u16:?}, mc_era_bits: {=u16:?} }}",
            self.mon_counter(),
            self.mc_era_bits()
        )
    }
}
#[doc = "SNVS_LP Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsr(pub u32);
impl Lpsr {
    #[doc = "LP Time Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn lpta(&self) -> super::vals::Lpta {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpta::from_bits(val as u8)
    }
    #[doc = "LP Time Alarm"]
    #[inline(always)]
    pub const fn set_lpta(&mut self, val: super::vals::Lpta) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure Real Time Counter Rollover"]
    #[must_use]
    #[inline(always)]
    pub const fn srtcr(&self) -> super::vals::Srtcr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Srtcr::from_bits(val as u8)
    }
    #[doc = "Secure Real Time Counter Rollover"]
    #[inline(always)]
    pub const fn set_srtcr(&mut self, val: super::vals::Srtcr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Monotonic Counter Rollover"]
    #[must_use]
    #[inline(always)]
    pub const fn mcr(&self) -> super::vals::Mcr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mcr::from_bits(val as u8)
    }
    #[doc = "Monotonic Counter Rollover"]
    #[inline(always)]
    pub const fn set_mcr(&mut self, val: super::vals::Mcr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Digital Low Voltage Event Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd(&self) -> super::vals::Lvd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lvd::from_bits(val as u8)
    }
    #[doc = "Digital Low Voltage Event Detected"]
    #[inline(always)]
    pub const fn set_lvd(&mut self, val: super::vals::Lvd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Tampering Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn ctd(&self) -> super::vals::Ctd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ctd::from_bits(val as u8)
    }
    #[doc = "Clock Tampering Detected"]
    #[inline(always)]
    pub const fn set_ctd(&mut self, val: super::vals::Ctd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Temperature Tamper Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn ttd(&self) -> super::vals::Ttd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ttd::from_bits(val as u8)
    }
    #[doc = "Temperature Tamper Detected"]
    #[inline(always)]
    pub const fn set_ttd(&mut self, val: super::vals::Ttd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Voltage Tampering Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn vtd(&self) -> super::vals::Vtd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Vtd::from_bits(val as u8)
    }
    #[doc = "Voltage Tampering Detected"]
    #[inline(always)]
    pub const fn set_vtd(&mut self, val: super::vals::Vtd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Wire-Mesh Tampering 1 Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn wmt1d(&self) -> super::vals::Wmt1d {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wmt1d::from_bits(val as u8)
    }
    #[doc = "Wire-Mesh Tampering 1 Detected"]
    #[inline(always)]
    pub const fn set_wmt1d(&mut self, val: super::vals::Wmt1d) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Wire-Mesh Tampering 2 Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn wmt2d(&self) -> super::vals::Wmt2d {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wmt2d::from_bits(val as u8)
    }
    #[doc = "Wire-Mesh Tampering 2 Detected"]
    #[inline(always)]
    pub const fn set_wmt2d(&mut self, val: super::vals::Wmt2d) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "External Tampering 1 Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn et1d(&self) -> super::vals::Et1d {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Et1d::from_bits(val as u8)
    }
    #[doc = "External Tampering 1 Detected"]
    #[inline(always)]
    pub const fn set_et1d(&mut self, val: super::vals::Et1d) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "External Tampering 2 Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn et2d(&self) -> super::vals::Et2d {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Et2d::from_bits(val as u8)
    }
    #[doc = "External Tampering 2 Detected"]
    #[inline(always)]
    pub const fn set_et2d(&mut self, val: super::vals::Et2d) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[must_use]
    #[inline(always)]
    pub const fn esvd(&self) -> super::vals::Esvd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Esvd::from_bits(val as u8)
    }
    #[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    pub const fn set_esvd(&mut self, val: super::vals::Esvd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Emergency Off This bit is set when a power off is requested."]
    #[must_use]
    #[inline(always)]
    pub const fn eo(&self) -> super::vals::Eo {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Eo::from_bits(val as u8)
    }
    #[doc = "Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    pub const fn set_eo(&mut self, val: super::vals::Eo) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[must_use]
    #[inline(always)]
    pub const fn spof(&self) -> super::vals::Spof {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Spof::from_bits(val as u8)
    }
    #[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub const fn set_spof(&mut self, val: super::vals::Spof) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
    #[must_use]
    #[inline(always)]
    pub const fn spon(&self) -> super::vals::Spon {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Spon::from_bits(val as u8)
    }
    #[doc = "Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub const fn set_spon(&mut self, val: super::vals::Spon) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[must_use]
    #[inline(always)]
    pub const fn lpns(&self) -> super::vals::Lpns {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Lpns::from_bits(val as u8)
    }
    #[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[inline(always)]
    pub const fn set_lpns(&mut self, val: super::vals::Lpns) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[must_use]
    #[inline(always)]
    pub const fn lps(&self) -> super::vals::Lps {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lps::from_bits(val as u8)
    }
    #[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[inline(always)]
    pub const fn set_lps(&mut self, val: super::vals::Lps) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Lpsr {
    #[inline(always)]
    fn default() -> Lpsr {
        Lpsr(0)
    }
}
impl core::fmt::Debug for Lpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpsr")
            .field("lpta", &self.lpta())
            .field("srtcr", &self.srtcr())
            .field("mcr", &self.mcr())
            .field("lvd", &self.lvd())
            .field("ctd", &self.ctd())
            .field("ttd", &self.ttd())
            .field("vtd", &self.vtd())
            .field("wmt1d", &self.wmt1d())
            .field("wmt2d", &self.wmt2d())
            .field("et1d", &self.et1d())
            .field("et2d", &self.et2d())
            .field("esvd", &self.esvd())
            .field("eo", &self.eo())
            .field("spof", &self.spof())
            .field("spon", &self.spon())
            .field("lpns", &self.lpns())
            .field("lps", &self.lps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpsr {{ lpta: {:?}, srtcr: {:?}, mcr: {:?}, lvd: {:?}, ctd: {:?}, ttd: {:?}, vtd: {:?}, wmt1d: {:?}, wmt2d: {:?}, et1d: {:?}, et2d: {:?}, esvd: {:?}, eo: {:?}, spof: {:?}, spon: {:?}, lpns: {:?}, lps: {:?} }}",
            self.lpta(),
            self.srtcr(),
            self.mcr(),
            self.lvd(),
            self.ctd(),
            self.ttd(),
            self.vtd(),
            self.wmt1d(),
            self.wmt2d(),
            self.et1d(),
            self.et2d(),
            self.esvd(),
            self.eo(),
            self.spof(),
            self.spon(),
            self.lpns(),
            self.lps()
        )
    }
}
#[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsrtclr(pub u32);
impl Lpsrtclr {
    #[doc = "LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
    #[must_use]
    #[inline(always)]
    pub const fn srtc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
    #[inline(always)]
    pub const fn set_srtc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lpsrtclr {
    #[inline(always)]
    fn default() -> Lpsrtclr {
        Lpsrtclr(0)
    }
}
impl core::fmt::Debug for Lpsrtclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpsrtclr")
            .field("srtc", &self.srtc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpsrtclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpsrtclr {{ srtc: {=u32:?} }}", self.srtc())
    }
}
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsrtcmr(pub u32);
impl Lpsrtcmr {
    #[doc = "LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[must_use]
    #[inline(always)]
    pub const fn srtc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    pub const fn set_srtc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Lpsrtcmr {
    #[inline(always)]
    fn default() -> Lpsrtcmr {
        Lpsrtcmr(0)
    }
}
impl core::fmt::Debug for Lpsrtcmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpsrtcmr")
            .field("srtc", &self.srtc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpsrtcmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpsrtcmr {{ srtc: {=u16:?} }}", self.srtc())
    }
}
#[doc = "SNVS_LP Security Violation Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsvcr(pub u32);
impl Lpsvcr {
    #[doc = "Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline(always)]
    pub const fn set_sv0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline(always)]
    pub const fn set_sv1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline(always)]
    pub const fn set_sv2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline(always)]
    pub const fn set_sv3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv4_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline(always)]
    pub const fn set_sv4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[must_use]
    #[inline(always)]
    pub const fn sv5_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline(always)]
    pub const fn set_sv5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Lpsvcr {
    #[inline(always)]
    fn default() -> Lpsvcr {
        Lpsvcr(0)
    }
}
impl core::fmt::Debug for Lpsvcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpsvcr")
            .field("sv0_en", &self.sv0_en())
            .field("sv1_en", &self.sv1_en())
            .field("sv2_en", &self.sv2_en())
            .field("sv3_en", &self.sv3_en())
            .field("sv4_en", &self.sv4_en())
            .field("sv5_en", &self.sv5_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpsvcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpsvcr {{ sv0_en: {=bool:?}, sv1_en: {=bool:?}, sv2_en: {=bool:?}, sv3_en: {=bool:?}, sv4_en: {=bool:?}, sv5_en: {=bool:?} }}",
            self.sv0_en(),
            self.sv1_en(),
            self.sv2_en(),
            self.sv3_en(),
            self.sv4_en(),
            self.sv5_en()
        )
    }
}
#[doc = "SNVS_LP Time Alarm Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptar(pub u32);
impl Lptar {
    #[doc = "LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
    #[must_use]
    #[inline(always)]
    pub const fn lpta(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
    #[inline(always)]
    pub const fn set_lpta(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lptar {
    #[inline(always)]
    fn default() -> Lptar {
        Lptar(0)
    }
}
impl core::fmt::Debug for Lptar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptar").field("lpta", &self.lpta()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lptar {{ lpta: {=u32:?} }}", self.lpta())
    }
}
#[doc = "SNVS_LP Tamper Detect Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptdcr(pub u32);
impl Lptdcr {
    #[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[must_use]
    #[inline(always)]
    pub const fn srtcr_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    pub const fn set_srtcr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[must_use]
    #[inline(always)]
    pub const fn mcr_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    pub const fn set_mcr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Tamper Enable When set, a clock monitor tamper generates an LP security violation."]
    #[must_use]
    #[inline(always)]
    pub const fn ct_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Tamper Enable When set, a clock monitor tamper generates an LP security violation."]
    #[inline(always)]
    pub const fn set_ct_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Temperature Tamper Enable When set, a temperature monitor tamper generates an LP security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn tt_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Tamper Enable When set, a temperature monitor tamper generates an LP security violation"]
    #[inline(always)]
    pub const fn set_tt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Voltage Tamper Enable Voltage Tamper Enable should be enabled 500 us after setting SCSC_SOSC_CTR \\[VOLT_TEMP_TAMPER_EN\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn vt_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Tamper Enable Voltage Tamper Enable should be enabled 500 us after setting SCSC_SOSC_CTR \\[VOLT_TEMP_TAMPER_EN\\]"]
    #[inline(always)]
    pub const fn set_vt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Wire-Mesh Tampering 1 Enable When set, wire-mesh tampering 1 detection generates an LP security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn wmt1_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tampering 1 Enable When set, wire-mesh tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_wmt1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wire-Mesh Tampering 2 Enable When set, wire-mesh tampering 2 detection generates an LP security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn wmt2_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tampering 2 Enable When set, wire-mesh tampering 2 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_wmt2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn et1_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_et1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "External Tampering 2 Enable When set, external tampering 2 detection generates an LP security violation"]
    #[must_use]
    #[inline(always)]
    pub const fn et2_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 2 Enable When set, external tampering 2 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_et2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[must_use]
    #[inline(always)]
    pub const fn et1p(&self) -> super::vals::Et1p {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Et1p::from_bits(val as u8)
    }
    #[doc = "External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline(always)]
    pub const fn set_et1p(&mut self, val: super::vals::Et1p) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "External Tampering 2 Polarity This bit is used to determine the polarity of external tamper 2."]
    #[must_use]
    #[inline(always)]
    pub const fn et2p(&self) -> super::vals::Et2p {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Et2p::from_bits(val as u8)
    }
    #[doc = "External Tampering 2 Polarity This bit is used to determine the polarity of external tamper 2."]
    #[inline(always)]
    pub const fn set_et2p(&mut self, val: super::vals::Et2p) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_observ(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    pub const fn set_pfd_observ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[must_use]
    #[inline(always)]
    pub const fn por_observ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    pub const fn set_por_observ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    #[must_use]
    #[inline(always)]
    pub const fn ltdc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub const fn set_ltdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    #[must_use]
    #[inline(always)]
    pub const fn htdc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    #[inline(always)]
    pub const fn set_htdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    #[must_use]
    #[inline(always)]
    pub const fn vrc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub const fn set_vrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[must_use]
    #[inline(always)]
    pub const fn oscb(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    pub const fn set_oscb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Lptdcr {
    #[inline(always)]
    fn default() -> Lptdcr {
        Lptdcr(0)
    }
}
impl core::fmt::Debug for Lptdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptdcr")
            .field("srtcr_en", &self.srtcr_en())
            .field("mcr_en", &self.mcr_en())
            .field("ct_en", &self.ct_en())
            .field("tt_en", &self.tt_en())
            .field("vt_en", &self.vt_en())
            .field("wmt1_en", &self.wmt1_en())
            .field("wmt2_en", &self.wmt2_en())
            .field("et1_en", &self.et1_en())
            .field("et2_en", &self.et2_en())
            .field("et1p", &self.et1p())
            .field("et2p", &self.et2p())
            .field("pfd_observ", &self.pfd_observ())
            .field("por_observ", &self.por_observ())
            .field("ltdc", &self.ltdc())
            .field("htdc", &self.htdc())
            .field("vrc", &self.vrc())
            .field("oscb", &self.oscb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lptdcr {{ srtcr_en: {=bool:?}, mcr_en: {=bool:?}, ct_en: {=bool:?}, tt_en: {=bool:?}, vt_en: {=bool:?}, wmt1_en: {=bool:?}, wmt2_en: {=bool:?}, et1_en: {=bool:?}, et2_en: {=bool:?}, et1p: {:?}, et2p: {:?}, pfd_observ: {=bool:?}, por_observ: {=bool:?}, ltdc: {=u8:?}, htdc: {=u8:?}, vrc: {=u8:?}, oscb: {=bool:?} }}",
            self.srtcr_en(),
            self.mcr_en(),
            self.ct_en(),
            self.tt_en(),
            self.vt_en(),
            self.wmt1_en(),
            self.wmt2_en(),
            self.et1_en(),
            self.et2_en(),
            self.et1p(),
            self.et2p(),
            self.pfd_observ(),
            self.por_observ(),
            self.ltdc(),
            self.htdc(),
            self.vrc(),
            self.oscb()
        )
    }
}
#[doc = "SNVS_LP Tamper Glitch Filters Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptgfcr(pub u32);
impl Lptgfcr {
    #[doc = "Wire-Mesh Tamper Glitch Filter Configures the length of the digital glitch filter for the wire-mesh tamper 1 and 2 pins between 1 and 63 SRTC clock cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn wmtgf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Configures the length of the digital glitch filter for the wire-mesh tamper 1 and 2 pins between 1 and 63 SRTC clock cycles"]
    #[inline(always)]
    pub const fn set_wmtgf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Enable When set, enables the wire-mesh tamper glitch filter"]
    #[must_use]
    #[inline(always)]
    pub const fn wmtgf_en(&self) -> super::vals::WmtgfEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::WmtgfEn::from_bits(val as u8)
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Enable When set, enables the wire-mesh tamper glitch filter"]
    #[inline(always)]
    pub const fn set_wmtgf_en(&mut self, val: super::vals::WmtgfEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "External Tamper Glitch Filter 1 Configures the length of the digital glitch filter for the external tamper 1 pin between 128 and 32640 SRTC clock cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn etgf1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "External Tamper Glitch Filter 1 Configures the length of the digital glitch filter for the external tamper 1 pin between 128 and 32640 SRTC clock cycles"]
    #[inline(always)]
    pub const fn set_etgf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "External Tamper Glitch Filter 1 Enable When set, enables the external tamper glitch filter 1."]
    #[must_use]
    #[inline(always)]
    pub const fn etgf1_en(&self) -> super::vals::Etgf1En {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Etgf1En::from_bits(val as u8)
    }
    #[doc = "External Tamper Glitch Filter 1 Enable When set, enables the external tamper glitch filter 1."]
    #[inline(always)]
    pub const fn set_etgf1_en(&mut self, val: super::vals::Etgf1En) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "External Tamper Glitch Filter 2 Configures the length of the digital glitch filter for the external tamper 2 pin between 128 and 32640 SRTC clock cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn etgf2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "External Tamper Glitch Filter 2 Configures the length of the digital glitch filter for the external tamper 2 pin between 128 and 32640 SRTC clock cycles"]
    #[inline(always)]
    pub const fn set_etgf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "External Tamper Glitch Filter 2 Enable When set, enables the external tamper glitch filter 2."]
    #[must_use]
    #[inline(always)]
    pub const fn etgf2_en(&self) -> super::vals::Etgf2En {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Etgf2En::from_bits(val as u8)
    }
    #[doc = "External Tamper Glitch Filter 2 Enable When set, enables the external tamper glitch filter 2."]
    #[inline(always)]
    pub const fn set_etgf2_en(&mut self, val: super::vals::Etgf2En) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Lptgfcr {
    #[inline(always)]
    fn default() -> Lptgfcr {
        Lptgfcr(0)
    }
}
impl core::fmt::Debug for Lptgfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptgfcr")
            .field("wmtgf", &self.wmtgf())
            .field("wmtgf_en", &self.wmtgf_en())
            .field("etgf1", &self.etgf1())
            .field("etgf1_en", &self.etgf1_en())
            .field("etgf2", &self.etgf2())
            .field("etgf2_en", &self.etgf2_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptgfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lptgfcr {{ wmtgf: {=u8:?}, wmtgf_en: {:?}, etgf1: {=u8:?}, etgf1_en: {:?}, etgf2: {=u8:?}, etgf2_en: {:?} }}",
            self.wmtgf(),
            self.wmtgf_en(),
            self.etgf1(),
            self.etgf1_en(),
            self.etgf2(),
            self.etgf2_en()
        )
    }
}
#[doc = "SNVS_LP Zeroizable Master Key Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpzmkr(pub u32);
impl Lpzmkr {
    #[doc = "Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value"]
    #[must_use]
    #[inline(always)]
    pub const fn zmk(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value"]
    #[inline(always)]
    pub const fn set_zmk(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lpzmkr {
    #[inline(always)]
    fn default() -> Lpzmkr {
        Lpzmkr(0)
    }
}
impl core::fmt::Debug for Lpzmkr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpzmkr").field("zmk", &self.zmk()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpzmkr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpzmkr {{ zmk: {=u32:?} }}", self.zmk())
    }
}
