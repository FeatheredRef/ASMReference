> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SIDT

Stores the contents of the Interrupt Descriptor Table Register (IDTR) into the specified memory destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal IDTR | m10 |

DO NOT support LOCK

The instruction is available in both 64-bit mode and compatibility mode. In 64-bit mode, the instruction stores a 10-byte value consisting of a 16-bit IDT limit and a 64-bit base address. In compatibility mode, it stores a 6-byte value consisting of a 16-bit limit and a 32-bit base address.

The destination operand MUST be a memory location. Use of a register as a destination will result in an invalid operation. When executing in 64-bit mode, the memory destination MUST be aligned to avoid potential performance penalties or exceptions depending on the alignment check (AC) flag in CR0.