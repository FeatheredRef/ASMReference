> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESDEC256KL

Performs one round of the AES-256 decryption last round on the 128-bit data in the first operand using the 128-bit round key in the second operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available if the processor supports the AES-NI instruction set. It operates on XMM registers and is supported in 64-bit mode and compatibility mode.

The instruction requires the destination register to be the same as the source register for in-place operations. Using an incorrect round key for the final round will result in incorrect decryption, as this instruction omits the MixColumns transformation required in intermediate rounds.