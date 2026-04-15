> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSUBSH

Subtracts two packed signed 16-bit integers from the source operands and stores the result in the destination register.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| xmm | xmm |
| imm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The instruction operates on the lower 128 bits of the xmm registers; the upper bits of the destination register are not modified if the destination is the same as one of the sources (non-destructive). When using an immediate value, it is sign-extended to 16 bits before the subtraction. No flags are affected by this operation.