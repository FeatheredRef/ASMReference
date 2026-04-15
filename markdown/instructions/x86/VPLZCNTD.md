> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPLZCNTD

Zeroes the most significant bits of the destination operand that are leading zeros in the source operand, then sets the most significant bit of the resulting dword to 1. If the source operand is 0, the destination operand is set to 0.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only when operating in 64-bit mode. It requires AVX support.

The instruction operates on dword elements within XMM registers. It SHALL NOT be used if the processor does not support the AVX instruction set, as it will trigger an invalid opcode exception. Ensure that the destination register is not used as a source for other concurrent operations to avoid data hazards.