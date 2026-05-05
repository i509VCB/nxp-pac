#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "AHBSC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbsc {
    ptr: *mut u8,
}
unsafe impl Send for Ahbsc {}
unsafe impl Sync for Ahbsc {}
impl Ahbsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash00_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash01_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash IFR0 Rule register."]
    #[inline(always)]
    pub const fn flash02_mem_rule(self) -> crate::pac::common::Reg<Rule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash03_mem_rule(self) -> crate::pac::common::Reg<Rule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ROM Memory Rule."]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMX Memory Rule."]
    #[inline(always)]
    pub const fn ramx_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMA Memory Rule 0."]
    #[inline(always)]
    pub const fn rama_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMB Memory Rule 0."]
    #[inline(always)]
    pub const fn ramb_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Slave Port 5 Rule Register."]
    #[inline(always)]
    pub const fn ahb_slave_port_p5_slave_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbSlavePortP5slaveRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule Register 0."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0memRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule Register 1."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0memRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0memRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0memRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup1memRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup1memRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "AHB Secure Control Peripheral Rule."]
    #[inline(always)]
    pub const fn ahb_secure_ctrl_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<Rule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0memRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 2."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0memRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 3."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0memRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 4."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule4(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0memRule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 5."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule5(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0memRule5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 4."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule4(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 5."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule5(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 6."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule6(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 7."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule7(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 8."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule8(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 9."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule9(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule9, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 10."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule10(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule10, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 11."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule11(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule11, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 12."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule12(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule12, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 13."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule13(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2memRule13, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 0 Memory Rule."]
    #[inline(always)]
    pub const fn flexspi0_region0_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Rule8, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE."]
    #[inline(always)]
    pub const fn flexspi0_region1_6_mem_rule(self, n: usize) -> Flexspi0region16memRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0region16memRule::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 16usize) as _)
        }
    }
    #[doc = "Security Violation Address."]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecVioAddr, crate::pac::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Miscellaneous Information at Address."]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecVioMiscInfo, crate::pac::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Info Validity for Address."]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::pac::common::Reg<SecVioInfoValid, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecGpReg, crate::pac::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize + n * 4usize) as _)
        }
    }
    #[doc = "Master Secure Level."]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::pac::common::Reg<MasterSecLevel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "Master Secure Level."]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(
        self,
    ) -> crate::pac::common::Reg<MasterSecAntiPolReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscellaneous CPU0 Control Signals."]
    #[inline(always)]
    pub const fn cpu0_lock_reg(
        self,
    ) -> crate::pac::common::Reg<Cpu0lockReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Secure Control Duplicate."]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::pac::common::Reg<MiscCtrlDpReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Secure Control."]
    #[inline(always)]
    pub const fn misc_ctrl_reg(
        self,
    ) -> crate::pac::common::Reg<MiscCtrlReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0region16memRule {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi0region16memRule {}
unsafe impl Sync for Flexspi0region16memRule {}
impl Flexspi0region16memRule {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLEXSPI0 Region index Memory Rule."]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule(
        self,
    ) -> crate::pac::common::Reg<Rule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0memRule1(pub u32);
impl AhbPeripheral0memRule1 {
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO0 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO0 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio0_alias(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0memRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral0memRule1 {
        AhbPeripheral0memRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0memRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0memRule1")
            .field("gpio0", &self.gpio0())
            .field("gpio0_alias", &self.gpio0_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0memRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0memRule1 {{ gpio0: {:?}, gpio0_alias: {:?} }}",
            self.gpio0(),
            self.gpio0_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0memRule2(pub u32);
impl AhbPeripheral0memRule2 {
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO1 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO1 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio1_alias(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0memRule2 {
    #[inline(always)]
    fn default() -> AhbPeripheral0memRule2 {
        AhbPeripheral0memRule2(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0memRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0memRule2")
            .field("gpio1", &self.gpio1())
            .field("gpio1_alias", &self.gpio1_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0memRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0memRule2 {{ gpio1: {:?}, gpio1_alias: {:?} }}",
            self.gpio1(),
            self.gpio1_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0memRule3(pub u32);
impl AhbPeripheral0memRule3 {
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO2 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO2 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio2_alias(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0memRule3 {
    #[inline(always)]
    fn default() -> AhbPeripheral0memRule3 {
        AhbPeripheral0memRule3(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0memRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0memRule3")
            .field("gpio2", &self.gpio2())
            .field("gpio2_alias", &self.gpio2_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0memRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0memRule3 {{ gpio2: {:?}, gpio2_alias: {:?} }}",
            self.gpio2(),
            self.gpio2_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0memRule4(pub u32);
impl AhbPeripheral0memRule4 {
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO3 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO3 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio3_alias(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0memRule4 {
    #[inline(always)]
    fn default() -> AhbPeripheral0memRule4 {
        AhbPeripheral0memRule4(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0memRule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0memRule4")
            .field("gpio3", &self.gpio3())
            .field("gpio3_alias", &self.gpio3_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0memRule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0memRule4 {{ gpio3: {:?}, gpio3_alias: {:?} }}",
            self.gpio3(),
            self.gpio3_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0memRule5(pub u32);
impl AhbPeripheral0memRule5 {
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO4 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO4 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio4_alias(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0memRule5 {
    #[inline(always)]
    fn default() -> AhbPeripheral0memRule5 {
        AhbPeripheral0memRule5(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0memRule5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0memRule5")
            .field("gpio4", &self.gpio4())
            .field("gpio4_alias", &self.gpio4_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0memRule5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0memRule5 {{ gpio4: {:?}, gpio4_alias: {:?} }}",
            self.gpio4(),
            self.gpio4_alias()
        )
    }
}
#[doc = "AHB Slave Port 5 Rule Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP5slaveRule0(pub u32);
impl AhbSlavePortP5slaveRule0 {
    #[doc = "CDOG0."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CDOG0."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CDOG1."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CDOG1."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DEBUG_MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DEBUG_MAILBOX."]
    #[inline(always)]
    pub const fn set_debug_mailbox(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AhbSlavePortP5slaveRule0 {
    #[inline(always)]
    fn default() -> AhbSlavePortP5slaveRule0 {
        AhbSlavePortP5slaveRule0(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP5slaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP5slaveRule0")
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("debug_mailbox", &self.debug_mailbox())
            .field("mau0", &self.mau0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP5slaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP5slaveRule0 {{ cdog0: {:?}, cdog1: {:?}, debug_mailbox: {:?}, mau0: {:?} }}",
            self.cdog0(),
            self.cdog1(),
            self.debug_mailbox(),
            self.mau0()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0memRule0(pub u32);
impl AipsBridgeGroup0memRule0 {
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ROMCP."]
    #[must_use]
    #[inline(always)]
    pub const fn romcp(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ROMCP."]
    #[inline(always)]
    pub const fn set_romcp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_1_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_mp(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_1_MP."]
    #[inline(always)]
    pub const fn set_dma_1_mp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_1_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH0."]
    #[inline(always)]
    pub const fn set_dma_1_ch0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA_1_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH1."]
    #[inline(always)]
    pub const fn set_dma_1_ch1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA_1_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH2."]
    #[inline(always)]
    pub const fn set_dma_1_ch2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA_1_CH3."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch3(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH3."]
    #[inline(always)]
    pub const fn set_dma_1_ch3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0memRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0memRule0 {
        AipsBridgeGroup0memRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0memRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0memRule0")
            .field("ewm0", &self.ewm0())
            .field("romcp", &self.romcp())
            .field("pkc0", &self.pkc0())
            .field("dma_1_mp", &self.dma_1_mp())
            .field("dma_1_ch0", &self.dma_1_ch0())
            .field("dma_1_ch1", &self.dma_1_ch1())
            .field("dma_1_ch2", &self.dma_1_ch2())
            .field("dma_1_ch3", &self.dma_1_ch3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0memRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0memRule0 {{ ewm0: {:?}, romcp: {:?}, pkc0: {:?}, dma_1_mp: {:?}, dma_1_ch0: {:?}, dma_1_ch1: {:?}, dma_1_ch2: {:?}, dma_1_ch3: {:?} }}",
            self.ewm0(),
            self.romcp(),
            self.pkc0(),
            self.dma_1_mp(),
            self.dma_1_ch0(),
            self.dma_1_ch1(),
            self.dma_1_ch2(),
            self.dma_1_ch3()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0memRule1(pub u32);
impl AipsBridgeGroup0memRule1 {
    #[doc = "ENET0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0_0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ENET0_0."]
    #[inline(always)]
    pub const fn set_enet0_0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ENET0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0_1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ENET0_1."]
    #[inline(always)]
    pub const fn set_enet0_1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn e_spi(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eSPI."]
    #[inline(always)]
    pub const fn set_e_spi(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0memRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0memRule1 {
        AipsBridgeGroup0memRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0memRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0memRule1")
            .field("enet0_0", &self.enet0_0())
            .field("enet0_1", &self.enet0_1())
            .field("e_spi", &self.e_spi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0memRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0memRule1 {{ enet0_0: {:?}, enet0_1: {:?}, e_spi: {:?} }}",
            self.enet0_0(),
            self.enet0_1(),
            self.e_spi()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1memRule0(pub u32);
impl AipsBridgeGroup1memRule0 {
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPSPI2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI2."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPSPI3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI3."]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPSPI4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI4."]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI5."]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AipsBridgeGroup1memRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1memRule0 {
        AipsBridgeGroup1memRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1memRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1memRule0")
            .field("flexspi0", &self.flexspi0())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1memRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1memRule0 {{ flexspi0: {:?}, lpspi2: {:?}, lpspi3: {:?}, lpspi4: {:?}, lpspi5: {:?} }}",
            self.flexspi0(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1memRule1(pub u32);
impl AipsBridgeGroup1memRule1 {
    #[doc = "SPI_FILETER0."]
    #[must_use]
    #[inline(always)]
    pub const fn spi_fileter0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SPI_FILETER0."]
    #[inline(always)]
    pub const fn set_spi_fileter0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "10BASE_T1S0."]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "10BASE_T1S0."]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1_PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "USB1_PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1memRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1memRule1 {
        AipsBridgeGroup1memRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1memRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1memRule1")
            .field("spi_fileter0", &self.spi_fileter0())
            .field("t1s0", &self.t1s0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1memRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1memRule1 {{ spi_fileter0: {:?}, t1s0: {:?}, usb1: {:?}, usb1_phy: {:?} }}",
            self.spi_fileter0(),
            self.t1s0(),
            self.usb1(),
            self.usb1_phy()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule0(pub u32);
impl AipsBridgeGroup2memRule0 {
    #[doc = "DMA_0_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_mp(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_MP."]
    #[inline(always)]
    pub const fn set_dma_0_mp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA_0_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH0."]
    #[inline(always)]
    pub const fn set_dma_0_ch0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA_0_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch1(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH1."]
    #[inline(always)]
    pub const fn set_dma_0_ch1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_0_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch2(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH2."]
    #[inline(always)]
    pub const fn set_dma_0_ch2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_0_CH3."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch3(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH3."]
    #[inline(always)]
    pub const fn set_dma_0_ch3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA_0_CH4."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch4(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH4."]
    #[inline(always)]
    pub const fn set_dma_0_ch4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA_0_CH5."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch5(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH5."]
    #[inline(always)]
    pub const fn set_dma_0_ch5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA_0_CH6."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch6(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH6."]
    #[inline(always)]
    pub const fn set_dma_0_ch6(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule0 {
        AipsBridgeGroup2memRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule0")
            .field("dma_0_mp", &self.dma_0_mp())
            .field("dma_0_ch0", &self.dma_0_ch0())
            .field("dma_0_ch1", &self.dma_0_ch1())
            .field("dma_0_ch2", &self.dma_0_ch2())
            .field("dma_0_ch3", &self.dma_0_ch3())
            .field("dma_0_ch4", &self.dma_0_ch4())
            .field("dma_0_ch5", &self.dma_0_ch5())
            .field("dma_0_ch6", &self.dma_0_ch6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule0 {{ dma_0_mp: {:?}, dma_0_ch0: {:?}, dma_0_ch1: {:?}, dma_0_ch2: {:?}, dma_0_ch3: {:?}, dma_0_ch4: {:?}, dma_0_ch5: {:?}, dma_0_ch6: {:?} }}",
            self.dma_0_mp(),
            self.dma_0_ch0(),
            self.dma_0_ch1(),
            self.dma_0_ch2(),
            self.dma_0_ch3(),
            self.dma_0_ch4(),
            self.dma_0_ch5(),
            self.dma_0_ch6()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule1(pub u32);
impl AipsBridgeGroup2memRule1 {
    #[doc = "DMA_0_CH7."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch7(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH7."]
    #[inline(always)]
    pub const fn set_dma_0_ch7(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA_0_CH8."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch8(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH8."]
    #[inline(always)]
    pub const fn set_dma_0_ch8(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA_0_CH9."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch9(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH9."]
    #[inline(always)]
    pub const fn set_dma_0_ch9(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_0_CH10."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch10(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH10."]
    #[inline(always)]
    pub const fn set_dma_0_ch10(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_0_CH11."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch11(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH11."]
    #[inline(always)]
    pub const fn set_dma_0_ch11(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AipsBridgeGroup2memRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule1 {
        AipsBridgeGroup2memRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule1")
            .field("dma_0_ch7", &self.dma_0_ch7())
            .field("dma_0_ch8", &self.dma_0_ch8())
            .field("dma_0_ch9", &self.dma_0_ch9())
            .field("dma_0_ch10", &self.dma_0_ch10())
            .field("dma_0_ch11", &self.dma_0_ch11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule1 {{ dma_0_ch7: {:?}, dma_0_ch8: {:?}, dma_0_ch9: {:?}, dma_0_ch10: {:?}, dma_0_ch11: {:?} }}",
            self.dma_0_ch7(),
            self.dma_0_ch8(),
            self.dma_0_ch9(),
            self.dma_0_ch10(),
            self.dma_0_ch11()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule10(pub u32);
impl AipsBridgeGroup2memRule10 {
    #[doc = "CAN1 Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 0."]
    #[inline(always)]
    pub const fn set_can1_region0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAN1 Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region1(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 1."]
    #[inline(always)]
    pub const fn set_can1_region1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CAN1 Region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region2(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 2."]
    #[inline(always)]
    pub const fn set_can1_region2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN1 Region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region3(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 3."]
    #[inline(always)]
    pub const fn set_can1_region3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LPI2C4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C4."]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup2memRule10 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule10 {
        AipsBridgeGroup2memRule10(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule10")
            .field("can1_region0", &self.can1_region0())
            .field("can1_region1", &self.can1_region1())
            .field("can1_region2", &self.can1_region2())
            .field("can1_region3", &self.can1_region3())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule10 {{ can1_region0: {:?}, can1_region1: {:?}, can1_region2: {:?}, can1_region3: {:?}, lpi2c2: {:?}, lpi2c3: {:?}, lpi2c4: {:?} }}",
            self.can1_region0(),
            self.can1_region1(),
            self.can1_region2(),
            self.can1_region3(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule11(pub u32);
impl AipsBridgeGroup2memRule11 {
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "I3C3."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "I3C3."]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO5."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO5."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule11 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule11 {
        AipsBridgeGroup2memRule11(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule11")
            .field("lpuart5", &self.lpuart5())
            .field("i3c3", &self.i3c3())
            .field("gpio5", &self.gpio5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule11 {{ lpuart5: {:?}, i3c3: {:?}, gpio5: {:?} }}",
            self.lpuart5(),
            self.i3c3(),
            self.gpio5()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule12(pub u32);
impl AipsBridgeGroup2memRule12 {
    #[doc = "GPIO5_ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS."]
    #[inline(always)]
    pub const fn set_gpio5_alias(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PORT5."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT5."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DGDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DGDET0."]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ITRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ITRC0."]
    #[inline(always)]
    pub const fn set_itrc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule12 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule12 {
        AipsBridgeGroup2memRule12(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule12")
            .field("gpio5_alias", &self.gpio5_alias())
            .field("port5", &self.port5())
            .field("dgdet0", &self.dgdet0())
            .field("itrc0", &self.itrc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule12 {{ gpio5_alias: {:?}, port5: {:?}, dgdet0: {:?}, itrc0: {:?} }}",
            self.gpio5_alias(),
            self.port5(),
            self.dgdet0(),
            self.itrc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule13(pub u32);
impl AipsBridgeGroup2memRule13 {
    #[doc = "GLIKEY0."]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GLIKEY0."]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "TDET0."]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SECCON."]
    #[must_use]
    #[inline(always)]
    pub const fn seccon(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SECCON."]
    #[inline(always)]
    pub const fn set_seccon(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "RTC0."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc0(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "RTC0."]
    #[inline(always)]
    pub const fn set_rtc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup2memRule13 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule13 {
        AipsBridgeGroup2memRule13(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule13")
            .field("glikey0", &self.glikey0())
            .field("tdet0", &self.tdet0())
            .field("seccon", &self.seccon())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("rtc0", &self.rtc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule13 {{ glikey0: {:?}, tdet0: {:?}, seccon: {:?}, sgi0: {:?}, trng0: {:?}, udf0: {:?}, rtc0: {:?} }}",
            self.glikey0(),
            self.tdet0(),
            self.seccon(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.rtc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule2(pub u32);
impl AipsBridgeGroup2memRule2 {
    #[doc = "SYSCON."]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SYSCON."]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "WUU."]
    #[must_use]
    #[inline(always)]
    pub const fn wuu(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WUU."]
    #[inline(always)]
    pub const fn set_wuu(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "VBAT."]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "VBAT."]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FMU."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FMU."]
    #[inline(always)]
    pub const fn set_fmu(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup2memRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule2 {
        AipsBridgeGroup2memRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule2")
            .field("syscon", &self.syscon())
            .field("wuu", &self.wuu())
            .field("vbat", &self.vbat())
            .field("fmc", &self.fmc())
            .field("fmu", &self.fmu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule2 {{ syscon: {:?}, wuu: {:?}, vbat: {:?}, fmc: {:?}, fmu: {:?} }}",
            self.syscon(),
            self.wuu(),
            self.vbat(),
            self.fmc(),
            self.fmu()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule3(pub u32);
impl AipsBridgeGroup2memRule3 {
    #[doc = "FLEXIO."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FLEXIO."]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "I3C2."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "I3C2."]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule3 {
        AipsBridgeGroup2memRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule3")
            .field("flexio", &self.flexio())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("i3c2", &self.i3c2())
            .field("lpuart0", &self.lpuart0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule3 {{ flexio: {:?}, lpi2c0: {:?}, lpi2c1: {:?}, lpspi0: {:?}, lpspi1: {:?}, i3c2: {:?}, lpuart0: {:?} }}",
            self.flexio(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.i3c2(),
            self.lpuart0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule4(pub u32);
impl AipsBridgeGroup2memRule4 {
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AipsBridgeGroup2memRule4 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule4 {
        AipsBridgeGroup2memRule4(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule4")
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule4 {{ lpuart1: {:?}, lpuart2: {:?}, lpuart3: {:?}, lpuart4: {:?} }}",
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule5(pub u32);
impl AipsBridgeGroup2memRule5 {
    #[doc = "LPTMR."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPTMR."]
    #[inline(always)]
    pub const fn set_lptmr(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "OSTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "OSTIMER."]
    #[inline(always)]
    pub const fn set_ostimer(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "WAKE_TIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_timer(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WAKE_TIMER."]
    #[inline(always)]
    pub const fn set_wake_timer(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule5 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule5 {
        AipsBridgeGroup2memRule5(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule5")
            .field("lptmr", &self.lptmr())
            .field("ostimer", &self.ostimer())
            .field("wake_timer", &self.wake_timer())
            .field("adc0", &self.adc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule5 {{ lptmr: {:?}, ostimer: {:?}, wake_timer: {:?}, adc0: {:?} }}",
            self.lptmr(),
            self.ostimer(),
            self.wake_timer(),
            self.adc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule6(pub u32);
impl AipsBridgeGroup2memRule6 {
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup2memRule6 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule6 {
        AipsBridgeGroup2memRule6(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule6")
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule6 {{ adc1: {:?}, cmp0: {:?}, dac0: {:?}, dac1: {:?} }}",
            self.adc1(),
            self.cmp0(),
            self.dac0(),
            self.dac1()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule7(pub u32);
impl AipsBridgeGroup2memRule7 {
    #[doc = "VREF0."]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "VREF0."]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule7 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule7 {
        AipsBridgeGroup2memRule7(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule7")
            .field("vref0", &self.vref0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule7 {{ vref0: {:?}, port0: {:?}, port1: {:?}, port2: {:?}, port3: {:?} }}",
            self.vref0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule8(pub u32);
impl AipsBridgeGroup2memRule8 {
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TSI0."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "TSI0."]
    #[inline(always)]
    pub const fn set_tsi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CMC."]
    #[must_use]
    #[inline(always)]
    pub const fn cmc(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CMC."]
    #[inline(always)]
    pub const fn set_cmc(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "EIM."]
    #[must_use]
    #[inline(always)]
    pub const fn eim(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "EIM."]
    #[inline(always)]
    pub const fn set_eim(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule8 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule8 {
        AipsBridgeGroup2memRule8(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule8")
            .field("port4", &self.port4())
            .field("tsi0", &self.tsi0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("cmc", &self.cmc())
            .field("eim", &self.eim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule8 {{ port4: {:?}, tsi0: {:?}, aoi0: {:?}, crc0: {:?}, cmc: {:?}, eim: {:?} }}",
            self.port4(),
            self.tsi0(),
            self.aoi0(),
            self.crc0(),
            self.cmc(),
            self.eim()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2memRule9(pub u32);
impl AipsBridgeGroup2memRule9 {
    #[doc = "ERM."]
    #[must_use]
    #[inline(always)]
    pub const fn erm(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ERM."]
    #[inline(always)]
    pub const fn set_erm(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "MBC."]
    #[must_use]
    #[inline(always)]
    pub const fn mbc(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "MBC."]
    #[inline(always)]
    pub const fn set_mbc(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SCG."]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SCG."]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SPC."]
    #[must_use]
    #[inline(always)]
    pub const fn spc(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SPC."]
    #[inline(always)]
    pub const fn set_spc(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CAN0 Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 0."]
    #[inline(always)]
    pub const fn set_can0_region0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CAN0 Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 1."]
    #[inline(always)]
    pub const fn set_can0_region1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CAN0 Region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 2."]
    #[inline(always)]
    pub const fn set_can0_region2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CAN0 Region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region3(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 3."]
    #[inline(always)]
    pub const fn set_can0_region3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2memRule9 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2memRule9 {
        AipsBridgeGroup2memRule9(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2memRule9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2memRule9")
            .field("erm", &self.erm())
            .field("mbc", &self.mbc())
            .field("scg", &self.scg())
            .field("spc", &self.spc())
            .field("can0_region0", &self.can0_region0())
            .field("can0_region1", &self.can0_region1())
            .field("can0_region2", &self.can0_region2())
            .field("can0_region3", &self.can0_region3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2memRule9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2memRule9 {{ erm: {:?}, mbc: {:?}, scg: {:?}, spc: {:?}, can0_region0: {:?}, can0_region1: {:?}, can0_region2: {:?}, can0_region3: {:?} }}",
            self.erm(),
            self.mbc(),
            self.scg(),
            self.spc(),
            self.can0_region0(),
            self.can0_region1(),
            self.can0_region2(),
            self.can0_region3()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0memRule0(pub u32);
impl ApbPeripheralGroup0memRule0 {
    #[doc = "INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "INPUTMUX."]
    #[inline(always)]
    pub const fn set_inputmux(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbPeripheralGroup0memRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0memRule0 {
        ApbPeripheralGroup0memRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0memRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0memRule0")
            .field("inputmux", &self.inputmux())
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0memRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0memRule0 {{ inputmux: {:?}, i3c0: {:?}, i3c1: {:?}, ctimer0: {:?}, ctimer1: {:?}, ctimer2: {:?}, ctimer3: {:?} }}",
            self.inputmux(),
            self.i3c0(),
            self.i3c1(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0memRule1(pub u32);
impl ApbPeripheralGroup0memRule1 {
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UTCIK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "UTCIK0."]
    #[inline(always)]
    pub const fn set_utick(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SmartDMA."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup0memRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0memRule1 {
        ApbPeripheralGroup0memRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0memRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0memRule1")
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick", &self.utick())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("smartdma", &self.smartdma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0memRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0memRule1 {{ ctimer4: {:?}, freqme: {:?}, utick: {:?}, wwdt0: {:?}, wwdt1: {:?}, smartdma: {:?} }}",
            self.ctimer4(),
            self.freqme(),
            self.utick(),
            self.wwdt0(),
            self.wwdt1(),
            self.smartdma()
        )
    }
}
#[doc = "Miscellaneous CPU0 Control Signals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0lockReg(pub u32);
impl Cpu0lockReg {
    #[doc = "LOCK_NS_VTOR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> LockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        LockNsVtor::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_VTOR."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: LockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LOCK_NS_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> LockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        LockNsMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_MPU."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: LockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtaircr(&self) -> LockSVtaircr {
        let val = (self.0 >> 4usize) & 0x03;
        LockSVtaircr::from_bits(val as u8)
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[inline(always)]
    pub const fn set_lock_s_vtaircr(&mut self, val: LockSVtaircr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LOCK_S_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        LockSMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_S_MPU."]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "LOCK_SAU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        LockSau::from_bits(val as u8)
    }
    #[doc = "LOCK_SAU."]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Cpu0lockReg {
    #[inline(always)]
    fn default() -> Cpu0lockReg {
        Cpu0lockReg(0)
    }
}
impl core::fmt::Debug for Cpu0lockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0lockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtaircr", &self.lock_s_vtaircr())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0lockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0lockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtaircr: {:?}, lock_s_mpu: {:?}, lock_sau: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtaircr(),
            self.lock_s_mpu(),
            self.lock_sau()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> MasterSec {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> MasterSec {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> MasterSec {
        let val = (self.0 >> 8usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSec {
        let val = (self.0 >> 10usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> MasterSec {
        let val = (self.0 >> 24usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> MasterSec {
        let val = (self.0 >> 26usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for MasterSecAntiPolReg {
    #[inline(always)]
    fn default() -> MasterSecAntiPolReg {
        MasterSecAntiPolReg(0)
    }
}
impl core::fmt::Debug for MasterSecAntiPolReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecAntiPolReg")
            .field("smartdma", &self.smartdma())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc", &self.pkc())
            .field("enet0", &self.enet0())
            .field("usb1", &self.usb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ smartdma: {:?}, dma0: {:?}, dma1: {:?}, pkc: {:?}, enet0: {:?}, usb1: {:?} }}",
            self.smartdma(),
            self.dma0(),
            self.dma1(),
            self.pkc(),
            self.enet0(),
            self.usb1()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> MasterSec {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> MasterSec {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> MasterSec {
        let val = (self.0 >> 8usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSec {
        let val = (self.0 >> 10usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> MasterSec {
        let val = (self.0 >> 24usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> MasterSec {
        let val = (self.0 >> 26usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for MasterSecLevel {
    #[inline(always)]
    fn default() -> MasterSecLevel {
        MasterSecLevel(0)
    }
}
impl core::fmt::Debug for MasterSecLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevel")
            .field("smartdma", &self.smartdma())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc", &self.pkc())
            .field("enet0", &self.enet0())
            .field("usb1", &self.usb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ smartdma: {:?}, dma0: {:?}, dma1: {:?}, pkc: {:?}, enet0: {:?}, usb1: {:?} }}",
            self.smartdma(),
            self.dma0(),
            self.dma1(),
            self.pkc(),
            self.enet0(),
            self.usb1()
        )
    }
}
#[doc = "Secure Control Duplicate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlDpReg(pub u32);
impl MiscCtrlDpReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlDpReg {
    #[inline(always)]
    fn default() -> MiscCtrlDpReg {
        MiscCtrlDpReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlDpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlDpReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlDpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlDpReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "Secure Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "AHB Secure Control Peripheral Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rule4(pub u32);
impl Rule4 {
    #[doc = "AHBSC."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "AHBSC."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Rule4 {
    #[inline(always)]
    fn default() -> Rule4 {
        Rule4(0)
    }
}
impl core::fmt::Debug for Rule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rule4")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rule4 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize)
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rule8(pub u32);
impl Rule8 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Rule8 {
    #[inline(always)]
    fn default() -> Rule8 {
        Rule8(0)
    }
}
impl core::fmt::Debug for Rule8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rule8")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rule8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rule8 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg(pub u32);
impl SecGpReg {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_ipd_req(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma_ipd_req(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for SecGpReg {
    #[inline(always)]
    fn default() -> SecGpReg {
        SecGpReg(0)
    }
}
impl core::fmt::Debug for SecGpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg")
            .field("dma_ipd_req[0]", &self.dma_ipd_req(0usize))
            .field("dma_ipd_req[1]", &self.dma_ipd_req(1usize))
            .field("dma_ipd_req[2]", &self.dma_ipd_req(2usize))
            .field("dma_ipd_req[3]", &self.dma_ipd_req(3usize))
            .field("dma_ipd_req[4]", &self.dma_ipd_req(4usize))
            .field("dma_ipd_req[5]", &self.dma_ipd_req(5usize))
            .field("dma_ipd_req[6]", &self.dma_ipd_req(6usize))
            .field("dma_ipd_req[7]", &self.dma_ipd_req(7usize))
            .field("dma_ipd_req[8]", &self.dma_ipd_req(8usize))
            .field("dma_ipd_req[9]", &self.dma_ipd_req(9usize))
            .field("dma_ipd_req[10]", &self.dma_ipd_req(10usize))
            .field("dma_ipd_req[11]", &self.dma_ipd_req(11usize))
            .field("dma_ipd_req[12]", &self.dma_ipd_req(12usize))
            .field("dma_ipd_req[13]", &self.dma_ipd_req(13usize))
            .field("dma_ipd_req[14]", &self.dma_ipd_req(14usize))
            .field("dma_ipd_req[15]", &self.dma_ipd_req(15usize))
            .field("dma_ipd_req[16]", &self.dma_ipd_req(16usize))
            .field("dma_ipd_req[17]", &self.dma_ipd_req(17usize))
            .field("dma_ipd_req[18]", &self.dma_ipd_req(18usize))
            .field("dma_ipd_req[19]", &self.dma_ipd_req(19usize))
            .field("dma_ipd_req[20]", &self.dma_ipd_req(20usize))
            .field("dma_ipd_req[21]", &self.dma_ipd_req(21usize))
            .field("dma_ipd_req[22]", &self.dma_ipd_req(22usize))
            .field("dma_ipd_req[23]", &self.dma_ipd_req(23usize))
            .field("dma_ipd_req[24]", &self.dma_ipd_req(24usize))
            .field("dma_ipd_req[25]", &self.dma_ipd_req(25usize))
            .field("dma_ipd_req[26]", &self.dma_ipd_req(26usize))
            .field("dma_ipd_req[27]", &self.dma_ipd_req(27usize))
            .field("dma_ipd_req[28]", &self.dma_ipd_req(28usize))
            .field("dma_ipd_req[29]", &self.dma_ipd_req(29usize))
            .field("dma_ipd_req[30]", &self.dma_ipd_req(30usize))
            .field("dma_ipd_req[31]", &self.dma_ipd_req(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg {{ dma_ipd_req[0]: {=bool:?}, dma_ipd_req[1]: {=bool:?}, dma_ipd_req[2]: {=bool:?}, dma_ipd_req[3]: {=bool:?}, dma_ipd_req[4]: {=bool:?}, dma_ipd_req[5]: {=bool:?}, dma_ipd_req[6]: {=bool:?}, dma_ipd_req[7]: {=bool:?}, dma_ipd_req[8]: {=bool:?}, dma_ipd_req[9]: {=bool:?}, dma_ipd_req[10]: {=bool:?}, dma_ipd_req[11]: {=bool:?}, dma_ipd_req[12]: {=bool:?}, dma_ipd_req[13]: {=bool:?}, dma_ipd_req[14]: {=bool:?}, dma_ipd_req[15]: {=bool:?}, dma_ipd_req[16]: {=bool:?}, dma_ipd_req[17]: {=bool:?}, dma_ipd_req[18]: {=bool:?}, dma_ipd_req[19]: {=bool:?}, dma_ipd_req[20]: {=bool:?}, dma_ipd_req[21]: {=bool:?}, dma_ipd_req[22]: {=bool:?}, dma_ipd_req[23]: {=bool:?}, dma_ipd_req[24]: {=bool:?}, dma_ipd_req[25]: {=bool:?}, dma_ipd_req[26]: {=bool:?}, dma_ipd_req[27]: {=bool:?}, dma_ipd_req[28]: {=bool:?}, dma_ipd_req[29]: {=bool:?}, dma_ipd_req[30]: {=bool:?}, dma_ipd_req[31]: {=bool:?} }}",
            self.dma_ipd_req(0usize),
            self.dma_ipd_req(1usize),
            self.dma_ipd_req(2usize),
            self.dma_ipd_req(3usize),
            self.dma_ipd_req(4usize),
            self.dma_ipd_req(5usize),
            self.dma_ipd_req(6usize),
            self.dma_ipd_req(7usize),
            self.dma_ipd_req(8usize),
            self.dma_ipd_req(9usize),
            self.dma_ipd_req(10usize),
            self.dma_ipd_req(11usize),
            self.dma_ipd_req(12usize),
            self.dma_ipd_req(13usize),
            self.dma_ipd_req(14usize),
            self.dma_ipd_req(15usize),
            self.dma_ipd_req(16usize),
            self.dma_ipd_req(17usize),
            self.dma_ipd_req(18usize),
            self.dma_ipd_req(19usize),
            self.dma_ipd_req(20usize),
            self.dma_ipd_req(21usize),
            self.dma_ipd_req(22usize),
            self.dma_ipd_req(23usize),
            self.dma_ipd_req(24usize),
            self.dma_ipd_req(25usize),
            self.dma_ipd_req(26usize),
            self.dma_ipd_req(27usize),
            self.dma_ipd_req(28usize),
            self.dma_ipd_req(29usize),
            self.dma_ipd_req(30usize),
            self.dma_ipd_req(31usize)
        )
    }
}
#[doc = "Security Violation Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[inline(always)]
    pub const fn set_sec_vio_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecVioAddr {
    #[inline(always)]
    fn default() -> SecVioAddr {
        SecVioAddr(0)
    }
}
impl core::fmt::Debug for SecVioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioAddr")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioAddr {{ sec_vio_addr: {=u32:?} }}",
            self.sec_vio_addr()
        )
    }
}
#[doc = "Security Violation Info Validity for Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioInfoValid(pub u32);
impl SecVioInfoValid {
    #[doc = "Violation information valid flag for AHB port 0."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 0."]
    #[inline(always)]
    pub const fn set_vio_info_valid(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for SecVioInfoValid {
    #[inline(always)]
    fn default() -> SecVioInfoValid {
        SecVioInfoValid(0)
    }
}
impl core::fmt::Debug for SecVioInfoValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioInfoValid")
            .field("vio_info_valid[0]", &self.vio_info_valid(0usize))
            .field("vio_info_valid[1]", &self.vio_info_valid(1usize))
            .field("vio_info_valid[2]", &self.vio_info_valid(2usize))
            .field("vio_info_valid[3]", &self.vio_info_valid(3usize))
            .field("vio_info_valid[4]", &self.vio_info_valid(4usize))
            .field("vio_info_valid[5]", &self.vio_info_valid(5usize))
            .field("vio_info_valid[6]", &self.vio_info_valid(6usize))
            .field("vio_info_valid[7]", &self.vio_info_valid(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid[0]: {=bool:?}, vio_info_valid[1]: {=bool:?}, vio_info_valid[2]: {=bool:?}, vio_info_valid[3]: {=bool:?}, vio_info_valid[4]: {=bool:?}, vio_info_valid[5]: {=bool:?}, vio_info_valid[6]: {=bool:?}, vio_info_valid[7]: {=bool:?} }}",
            self.vio_info_valid(0usize),
            self.vio_info_valid(1usize),
            self.vio_info_valid(2usize),
            self.vio_info_valid(3usize),
            self.vio_info_valid(4usize),
            self.vio_info_valid(5usize),
            self.vio_info_valid(6usize),
            self.vio_info_valid(7usize)
        )
    }
}
#[doc = "Security Violation Miscellaneous Information at Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "Security violation access read/write indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> SecVioInfoWrite {
        let val = (self.0 >> 0usize) & 0x01;
        SecVioInfoWrite::from_bits(val as u8)
    }
    #[doc = "Security violation access read/write indicator."]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: SecVioInfoWrite) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation Info Data Access."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> SecVioInfoDataAccess {
        let val = (self.0 >> 1usize) & 0x01;
        SecVioInfoDataAccess::from_bits(val as u8)
    }
    #[doc = "Security Violation Info Data Access."]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: SecVioInfoDataAccess) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master_sec_level(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master_sec_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Security violation master number."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master(&self) -> SecVioInfoMaster {
        let val = (self.0 >> 8usize) & 0x1f;
        SecVioInfoMaster::from_bits(val as u8)
    }
    #[doc = "Security violation master number."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: SecVioInfoMaster) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
}
impl Default for SecVioMiscInfo {
    #[inline(always)]
    fn default() -> SecVioMiscInfo {
        SecVioMiscInfo(0)
    }
}
impl core::fmt::Debug for SecVioMiscInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioMiscInfo")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioMiscInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioMiscInfo {{ sec_vio_info_write: {:?}, sec_vio_info_data_access: {:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1."]
    LockNsMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0."]
    LockNsMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> LockNsMpu {
        LockNsMpu::from_bits(val)
    }
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: LockNsMpu) -> u8 {
        LockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1."]
    LockNsVtorEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0."]
    LockNsVtorEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> LockNsVtor {
        LockNsVtor::from_bits(val)
    }
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: LockNsVtor) -> u8 {
        LockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 1."]
    LockSMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0."]
    LockSMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSMpu {
    #[inline(always)]
    fn from(val: u8) -> LockSMpu {
        LockSMpu::from_bits(val)
    }
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(val: LockSMpu) -> u8 {
        LockSMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSVtaircr {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 1."]
    LockSVtaircrEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0."]
    LockSVtaircrEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtaircr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtaircr {
    #[inline(always)]
    fn from(val: u8) -> LockSVtaircr {
        LockSVtaircr::from_bits(val)
    }
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(val: LockSVtaircr) -> u8 {
        LockSVtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSau {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_SAU is 1."]
    LockSauEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0."]
    LockSauEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSau {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSau {
    #[inline(always)]
    fn from(val: u8) -> LockSau {
        LockSau::from_bits(val)
    }
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(val: LockSau) -> u8 {
        LockSau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSec {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSec {
    #[inline(always)]
    fn from(val: u8) -> MasterSec {
        MasterSec::from_bits(val)
    }
}
impl From<MasterSec> for u8 {
    #[inline(always)]
    fn from(val: MasterSec) -> u8 {
        MasterSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlEnable {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of non-secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlEnable {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlEnable {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlEnable {
        MiscCtrlEnable::from_bits(val)
    }
}
impl From<MiscCtrlEnable> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlEnable) -> u8 {
        MiscCtrlEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master can access memories and peripherals at the same level or below that level."]
    Ahbtm = 0x01,
    #[doc = "Master can access memories and peripherals at same level only."]
    Ahbsm1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableStrictMode {
        MiscCtrlRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableStrictMode) -> u8 {
        MiscCtrlRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NoAbort = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    Abort = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableViolationAbort {
        MiscCtrlRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableViolationAbort) -> u8 {
        MiscCtrlRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    Disabled = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    Enabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegIdauAllNs {
        MiscCtrlRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegIdauAllNs) -> u8 {
        MiscCtrlRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    Locked = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NotLocked = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegWriteLock {
        MiscCtrlRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegWriteLock) -> u8 {
        MiscCtrlRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rule {
    #[inline(always)]
    fn from(val: u8) -> Rule {
        Rule::from_bits(val)
    }
}
impl From<Rule> for u8 {
    #[inline(always)]
    fn from(val: Rule) -> u8 {
        Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code."]
    Code = 0x0,
    #[doc = "Data."]
    Data = 0x01,
}
impl SecVioInfoDataAccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoDataAccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoDataAccess {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoDataAccess {
        SecVioInfoDataAccess::from_bits(val)
    }
}
impl From<SecVioInfoDataAccess> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoDataAccess) -> u8 {
        SecVioInfoDataAccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoMaster {
    #[doc = "CM33 Code."]
    Cpu0Code = 0x0,
    #[doc = "CM33 System."]
    Cpu0Sys = 0x01,
    #[doc = "SMARTDMA Instruction."]
    SdmaInstr = 0x02,
    #[doc = "SMARTDMA Data."]
    SdmaData = 0x03,
    #[doc = "eDMA1."]
    EDma1 = 0x04,
    #[doc = "eDMA0."]
    EDma0 = 0x05,
    #[doc = "USB HS."]
    UsbHs = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "eSPI."]
    Espi = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PKC."]
    Pkc = 0x0c,
    #[doc = "Ethernet."]
    Enet = 0x0d,
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
}
impl SecVioInfoMaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoMaster {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoMaster {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoMaster {
        SecVioInfoMaster::from_bits(val)
    }
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoMaster) -> u8 {
        SecVioInfoMaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoWrite {
    #[doc = "Read access."]
    Read = 0x0,
    #[doc = "Write access."]
    Write = 0x01,
}
impl SecVioInfoWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoWrite {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoWrite {
        SecVioInfoWrite::from_bits(val)
    }
}
impl From<SecVioInfoWrite> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoWrite) -> u8 {
        SecVioInfoWrite::to_bits(val)
    }
}
