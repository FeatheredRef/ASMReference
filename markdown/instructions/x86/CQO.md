> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CQO

Sets the canonical form of the quadword operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | reg |
| #I | m8 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The instruction modifies the destination operand to ensure it is in canonical form. If the destination is a register, the upper 64 bits of the internal register representation are cleared, and the canonical form is applied to the r64 field. If the destination is a memory location, the value is updated in the m8 region.

Failure to maintain canonical form when performing memory accesses or using certain registers may result in a General Protection fault (#GP). Use CQO to normalize the quadword operand before it is used as a linear address.