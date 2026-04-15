> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LES

Loads a 4-byte far pointer from a memory location into a segment register and a 32-bit general-purpose register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | r32, reg |
| imm | #I |
| reg | #I |

DO NOT support LOCK

This instruction is only available in 32-bit mode or in compatibility mode when executing a 32-bit operand-size override. It is NOT supported in 64-bit mode.

The source operand MUST be a memory location; using a register or immediate value is an invalid operation. When executing in compatibility mode, the instruction loads a 32-bit offset into r32 and a 16-bit selector into the specified segment register. Failure to ensure the memory operand is aligned to the architectural requirements of the target platform MAY result in performance penalties or alignment checks.