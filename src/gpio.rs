use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The type to split the GPIO into.
    type Parts;

    /// Splits the GPIO block into independent pins and registers.
    fn split(self) -> Self::Parts;
}

pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Pin assigned to peripheral A (type state)
pub struct PeripheralA;
/// Pin assigned to peripheral B (type state)
pub struct PeripheralB;
/// Pin assigned to peripheral C (type state)
pub struct PeripheralC;
/// Pin assigned to peripheral D (type state)
pub struct PeripheralD;

macro_rules! pio {
    ($PIOX:ident, $piox:ident, $pioy:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// PIO
        pub mod $piox {
            use core::marker::PhantomData;

            use embedded_hal::digital::OutputPin;
            pub use atsam4s16b::{$pioy, $PIOX};

            use super::{
                Input, Floating, PullUp, PullDown, GpioExt,
                Output, PeripheralA, PeripheralB, PeripheralC, PeripheralD
            };

            pub struct Parts {
                /// Opaque ABCDSR1 register
                pub abcdsr1: ABCDSR1,
                /// Opaque ABCDSR2 register
                pub abcdsr2: ABCDSR2,
                /// Opaque PER register
                pub per: PER,
                /// Opaque PDR register
                pub pdr: PDR,
                /// Opaque OER register
                pub oer: OER,
                /// Opaque ODR register
                pub odr: ODR,

                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $PIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        abcdsr1: ABCDSR1 { _0: () },
                        abcdsr2: ABCDSR2 { _0: () },
                        per: PER { _0: () },
                        pdr: PDR { _0: () },
                        oer: OER { _0: () },
                        odr: ODR { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            pub struct ABCDSR1 {
                _0: (),
            }

            impl ABCDSR1 {
                pub(crate) fn abcdsr1(&mut self) -> &$pioy::ABCDSR {
                    unsafe { &(*$PIOX::ptr()).abcdsr[0] }
                }
            }

            pub struct ABCDSR2 {
                _0: (),
            }

            impl ABCDSR2 {
                pub(crate) fn abcdsr2(&mut self) -> &$pioy::ABCDSR {
                    unsafe { &(*$PIOX::ptr()).abcdsr[1] }
                }
            }

            pub struct PER {
                _0: (),
            }

            impl PER {
                pub(crate) fn per(&mut self) -> &$pioy::PER {
                    unsafe { &(*$PIOX::ptr()).per }
                }
            }

            pub struct PDR {
                _0: (),
            }

            impl PDR {
                pub(crate) fn pdr(&mut self) -> &$pioy::PDR {
                    unsafe { &(*$PIOX::ptr()).pdr }
                }
            }

            pub struct OER {
                _0: (),
            }

            impl OER {
                pub(crate) fn oer(&mut self) -> &$pioy::OER {
                    unsafe { &(*$PIOX::ptr()).oer }
                }
            }

            pub struct ODR {
                _0: (),
            }

            impl ODR {
                pub(crate) fn odr(&mut self) -> &$pioy::ODR {
                    unsafe { &(*$PIOX::ptr()).odr }
                }
            }

            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$PIOX::ptr()).sodr.write(|w| w.bits(1 << self.i)) }
                }

                fn set_low(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$PIOX::ptr()).codr.write(|w| w.bits(1 << self.i)) }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_peripheralA(
                        self,
                        pdr: &mut PDR,
                        abcdsr1: &mut ABCDSR1,
                        abcdsr2: &mut ABCDSR2,
                    ) -> $PXi<PeripheralA> {
                        // Disable PIO for this pin (enables peripheral)
                        pdr.pdr().write(|w| unsafe { w.bits(1 << $i) });

                        // Set ABCDSR1 to 0 and ABCDSR2 to 0 for Peripheral A.
                        abcdsr1.abcdsr1().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) & !(1 << $i))
                        });
                        abcdsr2.abcdsr2().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) & !(1 << $i))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_peripheralB(
                        self,
                        pdr: &mut PDR,
                        abcdsr1: &mut ABCDSR1,
                        abcdsr2: &mut ABCDSR2,
                    ) -> $PXi<PeripheralB> {
                        // Disable PIO for this pin (enables peripheral)
                        pdr.pdr().write(|w| unsafe { w.bits(1 << $i) });

                        // Set ABCDSR1 to 0 and ABCDSR2 to 1 for Peripheral B.
                        abcdsr1.abcdsr1().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) & !(1 << $i))
                        });
                        abcdsr2.abcdsr2().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) | (1 << $i))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_peripheralC(
                        self,
                        pdr: &mut PDR,
                        abcdsr1: &mut ABCDSR1,
                        abcdsr2: &mut ABCDSR2,
                    ) -> $PXi<PeripheralC> {
                        // Disable PIO for this pin (enables peripheral)
                        pdr.pdr().write(|w| unsafe { w.bits(1 << $i) });

                        // Set ABCDSR1 to 1 and ABCDSR2 to 0 for Peripheral C.
                        abcdsr1.abcdsr1().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) | (1 << $i))
                        });
                        abcdsr2.abcdsr2().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) & !(1 << $i))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_peripheralD(
                        self,
                        pdr: &mut PDR,
                        abcdsr1: &mut ABCDSR1,
                        abcdsr2: &mut ABCDSR2,
                    ) -> $PXi<PeripheralD> {
                        // Disable PIO for this pin (enables peripheral)
                        pdr.pdr().write(|w| unsafe { w.bits(1 << $i) });

                        // Set ABCDSR1 to 1 and ABCDSR2 to 1 for Peripheral D.
                        abcdsr1.abcdsr1().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) | (1 << $i))
                        });
                        abcdsr2.abcdsr2().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(1 << $i)) | (1 << $i))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_output(
                        self,
                        oer: &mut OER,
                    ) -> $PXi<Output<()>> {
                        // Enable output for this pin
                        oer.oer().write(|w| unsafe { w.bits(1 << $i) });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$PIOX::ptr()).sodr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$PIOX::ptr()).codr.write(|w| w.bits(1 << $i)) }
                    }
                }
            )+
        }
    }
}

pio! { PIOA, pioa, pioa, PAx, [
    PA0: (pa0, 0, Input<Floating>),
    PA1: (pa1, 1, Input<Floating>),
    PA2: (pa2, 2, Input<Floating>),
    PA3: (pa3, 3, Input<Floating>),
    PA4: (pa4, 4, Input<Floating>),
    PA5: (pa5, 5, Input<Floating>),
    PA6: (pa6, 6, Input<Floating>),
    PA7: (pa7, 7, Input<Floating>),
    PA8: (pa8, 8, Input<Floating>),
    PA9: (pa9, 9, Input<Floating>),
    PA10: (pa10, 10, Input<Floating>),
    PA11: (pa11, 11, Input<Floating>),
    PA12: (pa12, 12, Input<Floating>),
    PA13: (pa13, 13, Input<Floating>),
    PA14: (pa14, 14, Input<Floating>),
    PA15: (pa15, 15, Input<Floating>),
    PA16: (pa16, 16, Input<Floating>),
    PA17: (pa17, 17, Input<Floating>),
    PA18: (pa18, 18, Input<Floating>),
    PA19: (pa19, 19, Input<Floating>),
    PA20: (pa20, 20, Input<Floating>),
    PA21: (pa21, 21, Input<Floating>),
    PA22: (pa22, 22, Input<Floating>),
    PA23: (pa23, 23, Input<Floating>),
    PA24: (pa24, 24, Input<Floating>),
    PA25: (pa25, 25, Input<Floating>),
    PA26: (pa26, 26, Input<Floating>),
    PA27: (pa27, 27, Input<Floating>),
    PA28: (pa28, 28, Input<Floating>),
    PA29: (pa29, 29, Input<Floating>),
    PA30: (pa30, 30, Input<Floating>),
    PA31: (pa31, 31, Input<Floating>),
]}

pio! { PIOB, piob, piob, PBx, [
    PB0: (pb0, 0, Input<Floating>),
    PB1: (pb1, 1, Input<Floating>),
    PB2: (pb2, 2, Input<Floating>),
    PB3: (pb3, 3, Input<Floating>),
    PB4: (pb4, 4, Input<Floating>),
    PB5: (pb5, 5, Input<Floating>),
    PB6: (pb6, 6, Input<Floating>),
    PB7: (pb7, 7, Input<Floating>),
    PB8: (pb8, 8, Input<Floating>),
    PB9: (pb9, 9, Input<Floating>),
    PB10: (pb10, 10, Input<Floating>),
    PB11: (pb11, 11, Input<Floating>),
    PB12: (pb12, 12, Input<Floating>),
    PB13: (pb13, 13, Input<Floating>),
    PB14: (pb14, 14, Input<Floating>),
    PB15: (pb15, 15, Input<Floating>),
    PB16: (pb16, 16, Input<Floating>),
    PB17: (pb17, 17, Input<Floating>),
    PB18: (pb18, 18, Input<Floating>),
    PB19: (pb19, 19, Input<Floating>),
    PB20: (pb20, 20, Input<Floating>),
    PB21: (pb21, 21, Input<Floating>),
    PB22: (pb22, 22, Input<Floating>),
    PB23: (pb23, 23, Input<Floating>),
    PB24: (pb24, 24, Input<Floating>),
    PB25: (pb25, 25, Input<Floating>),
    PB26: (pb26, 26, Input<Floating>),
    PB27: (pb27, 27, Input<Floating>),
    PB28: (pb28, 28, Input<Floating>),
    PB29: (pb29, 29, Input<Floating>),
    PB30: (pb30, 30, Input<Floating>),
    PB31: (pb31, 31, Input<Floating>),
]}

pio! { PIOC, pioc, pioc, PCx, [
    PC0: (pc0, 0, Input<Floating>),
    PC1: (pc1, 1, Input<Floating>),
    PC2: (pc2, 2, Input<Floating>),
    PC3: (pc3, 3, Input<Floating>),
    PC4: (pc4, 4, Input<Floating>),
    PC5: (pc5, 5, Input<Floating>),
    PC6: (pc6, 6, Input<Floating>),
    PC7: (pc7, 7, Input<Floating>),
    PC8: (pc8, 8, Input<Floating>),
    PC9: (pc9, 9, Input<Floating>),
    PC10: (pc10, 10, Input<Floating>),
    PC11: (pc11, 11, Input<Floating>),
    PC12: (pc12, 12, Input<Floating>),
    PC13: (pc13, 13, Input<Floating>),
    PC14: (pc14, 14, Input<Floating>),
    PC15: (pc15, 15, Input<Floating>),
    PC16: (pc16, 16, Input<Floating>),
    PC17: (pc17, 17, Input<Floating>),
    PC18: (pc18, 18, Input<Floating>),
    PC19: (pc19, 19, Input<Floating>),
    PC20: (pc20, 20, Input<Floating>),
    PC21: (pc21, 21, Input<Floating>),
    PC22: (pc22, 22, Input<Floating>),
    PC23: (pc23, 23, Input<Floating>),
    PC24: (pc24, 24, Input<Floating>),
    PC25: (pc25, 25, Input<Floating>),
    PC26: (pc26, 26, Input<Floating>),
    PC27: (pc27, 27, Input<Floating>),
    PC28: (pc28, 28, Input<Floating>),
    PC29: (pc29, 29, Input<Floating>),
    PC30: (pc30, 30, Input<Floating>),
    PC31: (pc31, 31, Input<Floating>),
]}
