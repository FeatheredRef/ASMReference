> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB132SH

Subtracts the product of a floating-point source (multiplied by a specified coefficient) from another floating-point source with sign-magnitude subtraction, producing a result in a destination register. Specifically, it computes: $dest = - (src1 - (src2 \times \text{coeff}))$.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f32/f64 (reg) | f32/f64 (reg) |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires a processor supporting the AVX-512 foundation and the AVX-512 FP16 extensions for half-precision variants, though the primary operation targets f32 and f64.

The instruction performs an internal conversion of the operands to the widest precision among the inputs before the computation. If the result cannot be represented within the destination precision, it MAY trigger #O or #U. The precision of the intermediate product is handled according to the rounding mode specified in the MXCSR register, which MAY result in #P.