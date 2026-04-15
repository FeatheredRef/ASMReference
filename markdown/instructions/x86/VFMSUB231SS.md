> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB231SS

Computes the fused multiply-subtract operation on scalar single-precision floating-point values. The instruction calculates $dest = (a \times b) - c$, where $a$, $b$, and $c$ are the source operands.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m4 | reg |
| reg, m4, reg | reg |
| m4, reg, reg | reg |

DO NOT support LOCK

This instruction is only available when the processor supports the FMA3 instruction set. It requires a processor that supports the AVX extensions.

The instruction operates on the lowest 32 bits of the XMM registers. The upper bits of the destination register are not modified.

The operation is performed with a single rounding step at the end, which prevents intermediate rounding errors. If the result cannot be represented as a scalar single-precision floating-point number, the following exceptions MAY be raised: #O, #U, #P, or #D. Invalid operations involving NaNs SHALL result in #I.