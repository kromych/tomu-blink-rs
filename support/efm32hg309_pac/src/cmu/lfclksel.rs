#[doc = "Register `LFCLKSEL` reader"]
pub type R = crate::R<LFCLKSEL_SPEC>;
#[doc = "Register `LFCLKSEL` writer"]
pub type W = crate::W<LFCLKSEL_SPEC>;
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LFA_R = crate::FieldReader<LFA_A>;
#[doc = "Clock Select for LFA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFA_A {
    #[doc = "0: LFACLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    LFXO = 2,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2 = 3,
}
impl From<LFA_A> for u8 {
    #[inline(always)]
    fn from(variant: LFA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFA_A {
    type Ux = u8;
}
impl LFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFA_A {
        match self.bits {
            0 => LFA_A::DISABLED,
            1 => LFA_A::LFRCO,
            2 => LFA_A::LFXO,
            3 => LFA_A::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA_A::DISABLED
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA_A::LFRCO
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA_A::LFXO
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFA_A::HFCORECLKLEDIV2
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LFA_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LFA_A>;
impl<'a, REG, const O: u8> LFA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::HFCORECLKLEDIV2)
    }
}
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LFB_R = crate::FieldReader<LFB_A>;
#[doc = "Clock Select for LFB\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFB_A {
    #[doc = "0: LFBCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    LFXO = 2,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2 = 3,
}
impl From<LFB_A> for u8 {
    #[inline(always)]
    fn from(variant: LFB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFB_A {
    type Ux = u8;
}
impl LFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFB_A {
        match self.bits {
            0 => LFB_A::DISABLED,
            1 => LFB_A::LFRCO,
            2 => LFB_A::LFXO,
            3 => LFB_A::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB_A::DISABLED
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB_A::LFRCO
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB_A::LFXO
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFB_A::HFCORECLKLEDIV2
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LFB_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LFB_A>;
impl<'a, REG, const O: u8> LFB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::HFCORECLKLEDIV2)
    }
}
#[doc = "Field `LFC` reader - Clock Select for LFC"]
pub type LFC_R = crate::FieldReader<LFC_A>;
#[doc = "Clock Select for LFC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFC_A {
    #[doc = "0: LFCCLK clock disabled."]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFCCLK clock"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFCCLK clock"]
    LFXO = 2,
}
impl From<LFC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFC_A {
    type Ux = u8;
}
impl LFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFC_A> {
        match self.bits {
            0 => Some(LFC_A::DISABLED),
            1 => Some(LFC_A::LFRCO),
            2 => Some(LFC_A::LFXO),
            _ => None,
        }
    }
    #[doc = "LFCCLK clock disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFC_A::DISABLED
    }
    #[doc = "LFRCO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFC_A::LFRCO
    }
    #[doc = "LFXO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFC_A::LFXO
    }
}
#[doc = "Field `LFC` writer - Clock Select for LFC"]
pub type LFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, LFC_A>;
impl<'a, REG, const O: u8> LFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFCCLK clock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::LFXO)
    }
}
#[doc = "Field `LFAE` reader - Clock Select for LFA Extended"]
pub type LFAE_R = crate::BitReader;
#[doc = "Field `LFAE` writer - Clock Select for LFA Extended"]
pub type LFAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFBE` reader - Clock Select for LFB Extended"]
pub type LFBE_R = crate::BitReader;
#[doc = "Field `LFBE` writer - Clock Select for LFB Extended"]
pub type LFBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LFA_R {
        LFA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LFB_R {
        LFB_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LFC_R {
        LFC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    pub fn lfae(&self) -> LFAE_R {
        LFAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    pub fn lfbe(&self) -> LFBE_R {
        LFBE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    #[must_use]
    pub fn lfa(&mut self) -> LFA_W<LFCLKSEL_SPEC, 0> {
        LFA_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    #[must_use]
    pub fn lfb(&mut self) -> LFB_W<LFCLKSEL_SPEC, 2> {
        LFB_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline(always)]
    #[must_use]
    pub fn lfc(&mut self) -> LFC_W<LFCLKSEL_SPEC, 4> {
        LFC_W::new(self)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lfae(&mut self) -> LFAE_W<LFCLKSEL_SPEC, 16> {
        LFAE_W::new(self)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lfbe(&mut self) -> LFBE_W<LFCLKSEL_SPEC, 20> {
        LFBE_W::new(self)
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
#[doc = "Low Frequency Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFCLKSEL_SPEC;
impl crate::RegisterSpec for LFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksel::R`](R) reader structure"]
impl crate::Readable for LFCLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfclksel::W`](W) writer structure"]
impl crate::Writable for LFCLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFCLKSEL to value 0x15"]
impl crate::Resettable for LFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x15;
}
