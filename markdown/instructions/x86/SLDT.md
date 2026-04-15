> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SLDT

Stores the contents of the Local Descriptor Table register (LDTR) into the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal LDTR | reg |
| Internal LDTR | m16 |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. In 64-bit mode, the destination is limited to a 16-bit register or memory location, as the LDTR holds a 16-bit selector.

The destination operand MUST be a 16-bit register or memory location; using a 32-bit or 64-bit register will result in incorrect data placement or potential faults depending on the addressing mode. If the destination is a memory operand, the operation is subject to alignment checks if enabled.