#[doc = "Register `OSCENCMD` writer"]
pub type W = crate::W<OSCENCMD_SPEC>;
#[doc = "Field `HFRCOEN` writer - HFRCO Enable"]
pub type HFRCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub type HFRCODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub type HFXOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub type HFXODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub type AUXHFRCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub type AUXHFRCODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub type LFRCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub type LFRCODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub type LFXOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub type LFXODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USHFRCOEN` writer - USHFRCO Enable"]
pub type USHFRCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USHFRCODIS` writer - USHFRCO Disable"]
pub type USHFRCODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcoen(&mut self) -> HFRCOEN_W<OSCENCMD_SPEC, 0> {
        HFRCOEN_W::new(self)
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<OSCENCMD_SPEC, 1> {
        HFRCODIS_W::new(self)
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoen(&mut self) -> HFXOEN_W<OSCENCMD_SPEC, 2> {
        HFXOEN_W::new(self)
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodis(&mut self) -> HFXODIS_W<OSCENCMD_SPEC, 3> {
        HFXODIS_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcoen(&mut self) -> AUXHFRCOEN_W<OSCENCMD_SPEC, 4> {
        AUXHFRCOEN_W::new(self)
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcodis(&mut self) -> AUXHFRCODIS_W<OSCENCMD_SPEC, 5> {
        AUXHFRCODIS_W::new(self)
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoen(&mut self) -> LFRCOEN_W<OSCENCMD_SPEC, 6> {
        LFRCOEN_W::new(self)
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcodis(&mut self) -> LFRCODIS_W<OSCENCMD_SPEC, 7> {
        LFRCODIS_W::new(self)
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoen(&mut self) -> LFXOEN_W<OSCENCMD_SPEC, 8> {
        LFXOEN_W::new(self)
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxodis(&mut self) -> LFXODIS_W<OSCENCMD_SPEC, 9> {
        LFXODIS_W::new(self)
    }
    #[doc = "Bit 10 - USHFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcoen(&mut self) -> USHFRCOEN_W<OSCENCMD_SPEC, 10> {
        USHFRCOEN_W::new(self)
    }
    #[doc = "Bit 11 - USHFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcodis(&mut self) -> USHFRCODIS_W<OSCENCMD_SPEC, 11> {
        USHFRCODIS_W::new(self)
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
#[doc = "Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscencmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCENCMD_SPEC;
impl crate::RegisterSpec for OSCENCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oscencmd::W`](W) writer structure"]
impl crate::Writable for OSCENCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OSCENCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
