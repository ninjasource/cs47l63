# Driver for CS47L63 Cirrus Codec Digital Signal Processor

This driver provides a set of high level abstractions that allow you to setup the DSP for use. This typically involves sending data over the SPI bus to set or get the value of various registers.

The way to use this is to create an instance of a struct of interest in the resisters module and call `serialize` on it to turn it into bytes to be sent to the DSP. 

This driver is meant to be used in conjustion with the CS47L63 Product Specification document as some resgisters need to be set in a specific order and have meaning explained in the spec.
