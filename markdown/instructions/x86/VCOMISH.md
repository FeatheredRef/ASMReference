> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCOMISH

Compares two packed scalar single-precision floating-point values. The instruction compares the most significant dword of the first operand with the most significant dword of the second operand and updates the EFLAGS register based on the result.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The VCOMISH instruction only compares the lowest 32 bits (the most significant dword) of the XMM registers. The remaining elements in the XMM registers are unaffected. Because this is a scalar instruction, it does not perform packed operations across the entire vector. Failure to account for the scalar nature of the operation may lead to incorrect logic when processing arrays.