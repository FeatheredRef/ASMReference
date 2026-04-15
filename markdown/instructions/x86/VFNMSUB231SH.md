> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB231SH

Subtracts the product of two floating-point values from a third value, with the result being negated. Specifically, it computes `dest = -(src1 - (src2 * src3))` for the specified precision.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32/f64 (reg), f32/f64 (reg), f32/f64 (reg) | f32/f64 (reg) |
| f32/f64 (mem), f32/f64 (reg), f32/f64 (reg) | f32/f64 (reg) |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 Fused Multiply-Add (FMA) feature set. It requires the processor to be in 64-bit mode or a compatibility mode that supports the AVX-512 instruction set.

The operation is performed as a single single-precision or double-precision fused multiply-subtract. Precision is maintained throughout the internal calculation; rounding only occurs once at the final step. This prevents intermediate rounding errors that would occur if the multiply and subtract were performed as separate instructions.

The instruction may trigger the following exceptions based on the floating-point environment: #P, #O, #U, #D, or #Z. If the result is a NaN, it follows the standard IEEE 754 rules for NaN propagation. The behavior regarding sign is determined by the MXCSR register settings.