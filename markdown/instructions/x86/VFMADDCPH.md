> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDCPH

Computes the product of two half-precision floating-point values and adds the result to a third half-precision floating-point value. This instruction operates on packed f16 values within YMM or ZMM registers.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m16/m32/m64 |
| m16/m32/m64 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 FP16 extension to be enabled; execution on a processor that does not support this extension SHALL result in an `#I` (Invalid Operation) exception.

The instruction utilizes the rounding control field in the `imm` operand to override the default rounding mode defined in the MXCSR register. Failure to specify the correct rounding mode for the intended precision may lead to #P. When operating on memory, the memory operand MUST be aligned to the natural boundary of the vector size to avoid potential performance degradation or faults depending on the alignment check settings.