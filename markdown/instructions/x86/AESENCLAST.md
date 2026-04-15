> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESENCLAST

Performs the final round of an AES encryption. It executes the ShiftRows, SubBytes, and AddRoundKey operations on the data in the destination operand using the round key provided in the source operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the processor to support the AES-NI instruction set extension.

The destination register MUST NOT be the same as the source memory operand to avoid undefined behavior in certain microarchitectures, although the architecture supports register-to-register dependency. Since this is the final round, it SHALL NOT perform the MixColumns operation; using `AESENCLAST` in a round that requires MixColumns will result in incorrect ciphertext.