#[doc = "Register `CHUSEBURSTC` writer"]
pub type W = crate::W<CHUSEBURSTC_SPEC>;
#[doc = "Field `CH0USEBURSTC` writer - Channel 0 Useburst Clear"]
pub type CH0USEBURSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1USEBURSTC` writer - Channel 1 Useburst Clear"]
pub type CH1USEBURSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2USEBURSTC` writer - Channel 2 Useburst Clear"]
pub type CH2USEBURSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3USEBURSTC` writer - Channel 3 Useburst Clear"]
pub type CH3USEBURSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4USEBURSTC` writer - Channel 4 Useburst Clear"]
pub type CH4USEBURSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5USEBURSTC` writer - Channel 5 Useburst Clear"]
pub type CH5USEBURSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0useburstc(&mut self) -> CH0USEBURSTC_W<CHUSEBURSTC_SPEC, 0> {
        CH0USEBURSTC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1useburstc(&mut self) -> CH1USEBURSTC_W<CHUSEBURSTC_SPEC, 1> {
        CH1USEBURSTC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2useburstc(&mut self) -> CH2USEBURSTC_W<CHUSEBURSTC_SPEC, 2> {
        CH2USEBURSTC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3useburstc(&mut self) -> CH3USEBURSTC_W<CHUSEBURSTC_SPEC, 3> {
        CH3USEBURSTC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4useburstc(&mut self) -> CH4USEBURSTC_W<CHUSEBURSTC_SPEC, 4> {
        CH4USEBURSTC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5useburstc(&mut self) -> CH5USEBURSTC_W<CHUSEBURSTC_SPEC, 5> {
        CH5USEBURSTC_W::new(self)
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
#[doc = "Channel Useburst Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chuseburstc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHUSEBURSTC_SPEC;
impl crate::RegisterSpec for CHUSEBURSTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chuseburstc::W`](W) writer structure"]
impl crate::Writable for CHUSEBURSTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHUSEBURSTC to value 0"]
impl crate::Resettable for CHUSEBURSTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
