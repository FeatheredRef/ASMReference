> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LDS

Loads a segment selector from a memory location into the DS segment register and the specified general-purpose register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16 | DS, r16 |
| m32 | DS, r32 |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode; it is only available in compatibility mode. In 64-bit mode, attempting to execute this instruction will result in an invalid opcode exception.

When loading the DS register, the processor SHALL perform segment limit and attribute checks. If the selector is invalid, a General Protection fault (#GP) SHALL occur. Since the instruction modifies a segment register, it MAY trigger a transition in the processor's internal state regarding memory segmentation.