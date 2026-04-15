> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFCMULCSH

Multiplies two floating-point values, then shifts the result to the right by the specified immediate value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm (f32/f64) | zmm/ymm/xmm (f32/f64) |
| imm | N/A |

DO NOT support LOCK

This instruction is only available when the processor supports AVX-512 and the corresponding feature bit is enabled. It requires the processor to be in 64-bit mode or compatibility mode.

The shift amount is encoded as an immediate value in the instruction; it SHALL NOT be provided via a register. If the shift amount is not valid for the specified precision, the operation may result in #I. The precision of the operation (single or double) MUST match the operand size. The result is subject to rounding as defined by the current MXCSR rounding mode.

Incorrectly specifying the shift immediate for the operand size (f32 vs f64) will lead to an invalid operation exception (#I). Ensure the mask register is correctly configured if using the masked version of the instruction to avoid undesired memory faults or incorrect element updates.