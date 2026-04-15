> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSHUFD

Shuffles 32-bit doublewords within a 128-bit XMM register based on an immediate byte. For each 32-bit lane in the destination, the corresponding 2 bits of the immediate specify which 32-bit doubleword from the source register is selected.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| imm | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit mode. It requires the SSE2 instruction set extension to be supported by the processor.

The immediate operand MUST be a byte; if the instruction is encoded with a different immediate size, it is an invalid encoding. Since this is a SIMD instruction, it does not affect the EFLAGS register. Use of a memory operand as a destination is NOT supported; the destination MUST be an xmm register.