> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMD

Permutes 32-bit doublewords within a 256-bit YMM register based on indices provided in a second YMM register. For each 32-bit element in the destination, the instruction selects a 32-bit element from the source register based on the corresponding index in the index register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ymm reg | ymm reg |
| ymm m32 | ymm reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX2 instruction set.

The indices in the index register are treated as u8; only the lower 8 bits of each 32-bit lane are used. If an index is greater than or equal to 8, the corresponding destination element is set to zero. Because VPERMD is a non-destructive instruction in some encodings but destructive in others, the user SHALL ensure the correct VEX encoding is used to avoid overwriting the source operand if it is required for subsequent operations.