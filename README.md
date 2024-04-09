# Driver for CS47L63 Cirrus Codec Digital Signal Processor

This driver provides a set of high level abstractions that allow you to setup the DSP for use. This typically involves sending data over the SPI bus to set or get the value of various registers.

The way to use this is to create an instance of a struct of interest in the `registers` module and call `serialize()` on it to turn it into a register-value pair to be sent to the DSP typically over the SPI bus. This driver is meant to be used in conjunction with the CS47L63 Product Specification document as some registers need to be set in a specific order and have meaning explained in the spec. This is therefore an unopinionated low level driver upon which opinionated higher level drivers or helper utilities can be built.