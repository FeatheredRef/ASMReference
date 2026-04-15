> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESENCWIDE128KL

Performs one round of AES encryption on a 128-bit block of data using a 128-bit round key. It performs the ShiftRows, SubBytes, MixColumns, and AddRoundKey operations.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m128 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the support of the AVX-512 and AES-NI instruction sets.

The destination register MUST NOT be the same as the source memory operand to avoid memory aliasing issues. Failure to ensure distinct operands may result in undefined behavior during the read-modify-write cycle.