#[doc = "Register `CHALTS` writer"]
pub type W = crate::W<CHALTS_SPEC>;
#[doc = "Field `CH0ALTS` writer - Channel 0 Alternate Structure Set"]
pub type CH0ALTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1ALTS` writer - Channel 1 Alternate Structure Set"]
pub type CH1ALTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2ALTS` writer - Channel 2 Alternate Structure Set"]
pub type CH2ALTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3ALTS` writer - Channel 3 Alternate Structure Set"]
pub type CH3ALTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4ALTS` writer - Channel 4 Alternate Structure Set"]
pub type CH4ALTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5ALTS` writer - Channel 5 Alternate Structure Set"]
pub type CH5ALTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0alts(&mut self) -> CH0ALTS_W<CHALTS_SPEC, 0> {
        CH0ALTS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1alts(&mut self) -> CH1ALTS_W<CHALTS_SPEC, 1> {
        CH1ALTS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2alts(&mut self) -> CH2ALTS_W<CHALTS_SPEC, 2> {
        CH2ALTS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3alts(&mut self) -> CH3ALTS_W<CHALTS_SPEC, 3> {
        CH3ALTS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4alts(&mut self) -> CH4ALTS_W<CHALTS_SPEC, 4> {
        CH4ALTS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5alts(&mut self) -> CH5ALTS_W<CHALTS_SPEC, 5> {
        CH5ALTS_W::new(self)
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
#[doc = "Channel Alternate Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chalts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHALTS_SPEC;
impl crate::RegisterSpec for CHALTS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chalts::W`](W) writer structure"]
impl crate::Writable for CHALTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHALTS to value 0"]
impl crate::Resettable for CHALTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
