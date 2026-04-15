> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PINSRD

Inserts a dword value from a source operand into a destination XMM register at a specified index.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m4 | xmm |
| r32 | xmm |
| imm | #I |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit mode. It requires the SSE4.1 instruction set extension.

The index for the insertion SHALL be specified by an immediate value or a general-purpose register. If the index is greater than 3, the behavior is undefined and may result in a general protection exception (#GP). The destination register MUST be an XMM register; using other register types is prohibited.