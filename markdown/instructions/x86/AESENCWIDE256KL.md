> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESENCWIDE256KL

Performs one round of AES encryption on 256-bit wide data using a 256-bit round key. It executes the ShiftRows, SubBytes, and AddRoundKey steps on the state.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m256 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 and AES-NI instruction set extensions to be enabled.

To avoid undefined behavior or illegal instruction exceptions, the processor MUST support the AVX-512 extension. Since this instruction operates on ZMM registers, the software SHOULD ensure that the state is transitioned to AVX-512 mode to prevent performance penalties associated with transitions between SSE and AVX states.