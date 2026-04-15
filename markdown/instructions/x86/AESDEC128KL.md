> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESDEC128KL

Performs one round of the AES-128 decryption common cipher operation on a 128-bit state. The instruction applies the Inverse ShiftRows, InvMixColumns, and AddRoundKey transformations.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only when the processor supports the AES-NI instruction set. It requires the processor to be operating in 64-bit mode or compatibility mode.

The destination register is overwritten by the result of the AES decryption round; therefore, the original state SHALL be preserved in a separate register if needed for subsequent operations. Use of this instruction without the corresponding AES-NI CPUID flag being set SHALL result in an invalid opcode exception.