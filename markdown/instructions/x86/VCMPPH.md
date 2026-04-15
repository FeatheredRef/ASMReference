> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCMPPH

Compares two packed half-precision floating-point values according to the specified predicate and stores the result in the destination. The operation produces a mask of all ones (1s) for elements that satisfy the predicate and all zeros (0s) for those that do not.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| xmm/ymm/zmm | xmm/ymm/zmm |
| imm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction SHALL ONLY be executed in 64-bit mode or 32-bit mode when the AVX-512 FP16 extension is supported by the processor. It is NOT available in compatibility mode.

The destination register MUST NOT be one of the source registers if the instruction is intended to be non-destructive; however, the VEX/EVEX encoding allows for three-operand forms to avoid this. The precision of the comparison depends on the MXCSR register settings, specifically the rounding mode and flush-to-zero (FTZ) or denormals-are-zero (DAZ) flags.

To avoid incorrect results, ensure that the operand registers are aligned to the required boundary for the vector length used (e.g., 64-byte alignment for zmm). Failure to align memory operands may result in performance degradation or general protection faults depending on the memory type.