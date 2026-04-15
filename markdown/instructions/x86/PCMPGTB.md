> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPGTB

Compares each byte in the first operand to the corresponding byte in the second operand. For each pair of bytes, the result is set to 0xFF if the first byte is greater than the second byte (signed interpretation), and 0x00 otherwise. The results are stored in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| xmm | m16 |
| m16 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension to be supported by the processor.

The destination operand MUST NOT be the same as both source operands if using a memory destination to avoid undefined behavior during write-back. The operands MUST be 128-bit XMM registers or 16-byte aligned memory addresses to avoid alignment check exceptions on certain processor configurations.