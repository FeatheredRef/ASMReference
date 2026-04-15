> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESENC128KL

Performs one round of AES encryption on a 128-bit state using a 128-bit round key. The operation consists of ShiftRows, SubBytes, MixColumns, and AddRoundKey.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only when the processor supports the AES-NI and AVX extensions. It requires the processor to be in 64-bit mode or compatibility mode.

To avoid undefined behavior or crashes, ensure that the destination register is not used as a source for the round key in the same instruction unless the architectural implementation supports such dependency. The instruction operates exclusively on 128-bit XMM registers; attempting to use larger YMM or ZMM registers in a context where only 128-bit operations are supported may lead to performance degradation or unexpected state transitions.