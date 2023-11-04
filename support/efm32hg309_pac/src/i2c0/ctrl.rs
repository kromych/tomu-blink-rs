#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - I2C Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - I2C Enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAVE` reader - Addressable as Slave"]
pub type SLAVE_R = crate::BitReader;
#[doc = "Field `SLAVE` writer - Addressable as Slave"]
pub type SLAVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOACK` reader - Automatic Acknowledge"]
pub type AUTOACK_R = crate::BitReader;
#[doc = "Field `AUTOACK` writer - Automatic Acknowledge"]
pub type AUTOACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOSE` reader - Automatic STOP when Empty"]
pub type AUTOSE_R = crate::BitReader;
#[doc = "Field `AUTOSE` writer - Automatic STOP when Empty"]
pub type AUTOSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOSN` reader - Automatic STOP on NACK"]
pub type AUTOSN_R = crate::BitReader;
#[doc = "Field `AUTOSN` writer - Automatic STOP on NACK"]
pub type AUTOSN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARBDIS` reader - Arbitration Disable"]
pub type ARBDIS_R = crate::BitReader;
#[doc = "Field `ARBDIS` writer - Arbitration Disable"]
pub type ARBDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCAMEN` reader - General Call Address Match Enable"]
pub type GCAMEN_R = crate::BitReader;
#[doc = "Field `GCAMEN` writer - General Call Address Match Enable"]
pub type GCAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLHR` reader - Clock Low High Ratio"]
pub type CLHR_R = crate::FieldReader<CLHR_A>;
#[doc = "Clock Low High Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLHR_A {
    #[doc = "0: The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    STANDARD = 0,
    #[doc = "1: The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    ASYMMETRIC = 1,
    #[doc = "2: The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    FAST = 2,
}
impl From<CLHR_A> for u8 {
    #[inline(always)]
    fn from(variant: CLHR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLHR_A {
    type Ux = u8;
}
impl CLHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLHR_A> {
        match self.bits {
            0 => Some(CLHR_A::STANDARD),
            1 => Some(CLHR_A::ASYMMETRIC),
            2 => Some(CLHR_A::FAST),
            _ => None,
        }
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == CLHR_A::STANDARD
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    #[inline(always)]
    pub fn is_asymmetric(&self) -> bool {
        *self == CLHR_A::ASYMMETRIC
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == CLHR_A::FAST
    }
}
#[doc = "Field `CLHR` writer - Clock Low High Ratio"]
pub type CLHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CLHR_A>;
impl<'a, REG, const O: u8> CLHR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(CLHR_A::STANDARD)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    #[inline(always)]
    pub fn asymmetric(self) -> &'a mut crate::W<REG> {
        self.variant(CLHR_A::ASYMMETRIC)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(CLHR_A::FAST)
    }
}
#[doc = "Field `BITO` reader - Bus Idle Timeout"]
pub type BITO_R = crate::FieldReader<BITO_A>;
#[doc = "Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITO_A {
    #[doc = "0: Timeout disabled"]
    OFF = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC = 3,
}
impl From<BITO_A> for u8 {
    #[inline(always)]
    fn from(variant: BITO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BITO_A {
    type Ux = u8;
}
impl BITO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BITO_A {
        match self.bits {
            0 => BITO_A::OFF,
            1 => BITO_A::_40PCC,
            2 => BITO_A::_80PCC,
            3 => BITO_A::_160PCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BITO_A::OFF
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == BITO_A::_40PCC
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == BITO_A::_80PCC
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == BITO_A::_160PCC
    }
}
#[doc = "Field `BITO` writer - Bus Idle Timeout"]
pub type BITO_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BITO_A>;
impl<'a, REG, const O: u8> BITO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BITO_A::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(BITO_A::_40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(BITO_A::_80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(BITO_A::_160PCC)
    }
}
#[doc = "Field `GIBITO` reader - Go Idle on Bus Idle Timeout"]
pub type GIBITO_R = crate::BitReader;
#[doc = "Field `GIBITO` writer - Go Idle on Bus Idle Timeout"]
pub type GIBITO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLTO` reader - Clock Low Timeout"]
pub type CLTO_R = crate::FieldReader<CLTO_A>;
#[doc = "Clock Low Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLTO_A {
    #[doc = "0: Timeout disabled"]
    OFF = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC = 3,
    #[doc = "4: Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    _320PPC = 4,
    #[doc = "5: Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    _1024PPC = 5,
}
impl From<CLTO_A> for u8 {
    #[inline(always)]
    fn from(variant: CLTO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLTO_A {
    type Ux = u8;
}
impl CLTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLTO_A> {
        match self.bits {
            0 => Some(CLTO_A::OFF),
            1 => Some(CLTO_A::_40PCC),
            2 => Some(CLTO_A::_80PCC),
            3 => Some(CLTO_A::_160PCC),
            4 => Some(CLTO_A::_320PPC),
            5 => Some(CLTO_A::_1024PPC),
            _ => None,
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLTO_A::OFF
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == CLTO_A::_40PCC
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == CLTO_A::_80PCC
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == CLTO_A::_160PCC
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn is_320ppc(&self) -> bool {
        *self == CLTO_A::_320PPC
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn is_1024ppc(&self) -> bool {
        *self == CLTO_A::_1024PPC
    }
}
#[doc = "Field `CLTO` writer - Clock Low Timeout"]
pub type CLTO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CLTO_A>;
impl<'a, REG, const O: u8> CLTO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO_A::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO_A::_40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO_A::_80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO_A::_160PCC)
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn _320ppc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO_A::_320PPC)
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn _1024ppc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO_A::_1024PPC)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    pub fn autose(&self) -> AUTOSE_R {
        AUTOSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&self) -> AUTOSN_R {
        AUTOSN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&self) -> ARBDIS_R {
        ARBDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&self) -> GCAMEN_R {
        GCAMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&self) -> CLHR_R {
        CLHR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&self) -> GIBITO_R {
        GIBITO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SLAVE_W<CTRL_SPEC, 1> {
        SLAVE_W::new(self)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn autoack(&mut self) -> AUTOACK_W<CTRL_SPEC, 2> {
        AUTOACK_W::new(self)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    #[must_use]
    pub fn autose(&mut self) -> AUTOSE_W<CTRL_SPEC, 3> {
        AUTOSE_W::new(self)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    #[must_use]
    pub fn autosn(&mut self) -> AUTOSN_W<CTRL_SPEC, 4> {
        AUTOSN_W::new(self)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    #[must_use]
    pub fn arbdis(&mut self) -> ARBDIS_W<CTRL_SPEC, 5> {
        ARBDIS_W::new(self)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcamen(&mut self) -> GCAMEN_W<CTRL_SPEC, 6> {
        GCAMEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn clhr(&mut self) -> CLHR_W<CTRL_SPEC, 8> {
        CLHR_W::new(self)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BITO_W<CTRL_SPEC, 12> {
        BITO_W::new(self)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn gibito(&mut self) -> GIBITO_W<CTRL_SPEC, 15> {
        GIBITO_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CLTO_W<CTRL_SPEC, 16> {
        CLTO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
