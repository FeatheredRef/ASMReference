> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESDEC

Performs one round of the AES decryption last-round sequence on the data in the first operand using the round key in the second operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AES-NI feature flag to be enabled.

The instruction operates on 128-bit blocks; if the input data is not properly aligned to 16 bytes when accessing memory (via associated instructions), a general-protection exception may occur depending on the alignment check settings. This instruction does not modify any flags.