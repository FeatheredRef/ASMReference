> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDRND231PD

Computes the fused multiply-add of two double-precision floating-point values and adds a third double-precision floating-point value, with a specified rounding mode. The operation is performed as $dest = (src1 \times src2) + src3$ with a single rounding step at the end.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f64 (reg/m64), f64 (reg/m64), f64 (reg/m64) | f64 (reg/m64) |

DO NOT support LOCK

This instruction REQUIRES AVX-512 support. It is only available in 64-bit mode or compatibility mode.

The rounding mode is specified by the `rn-ctrl` immediate operand, which overrides the rounding control in the MXCSR register. If the `rn-ctrl` field is not specified, the rounding mode from MXCSR is used. The operation produces an intermediate result with infinite precision before the final rounding occurs, preventing intermediate rounding errors.

Users SHOULD ensure that the destination register is not used as one of the source operands if they intend to preserve the original value, as the destination is overwritten. Improper use of the rounding override may lead to inconsistent precision results across different hardware implementations if not aligned with the application's mathematical requirements. Possible exceptions include #D, #O, #U, and #P.