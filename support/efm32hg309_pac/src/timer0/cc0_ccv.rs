#[doc = "Register `CC0_CCV` reader"]
pub type R = crate::R<CC0_CCV_SPEC>;
#[doc = "Register `CC0_CCV` writer"]
pub type W = crate::W<CC0_CCV_SPEC>;
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CCV_R = crate::FieldReader<u16>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CCV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccv(&mut self) -> CCV_W<CC0_CCV_SPEC, 0> {
        CCV_W::new(self)
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
#[doc = "CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ccv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_CCV_SPEC;
impl crate::RegisterSpec for CC0_CCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccv::R`](R) reader structure"]
impl crate::Readable for CC0_CCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc0_ccv::W`](W) writer structure"]
impl crate::Writable for CC0_CCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC0_CCV to value 0"]
impl crate::Resettable for CC0_CCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
