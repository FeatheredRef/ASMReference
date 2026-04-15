> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POPF

Pops the top word from the stack and loads it into the RFLAGS register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m2 | RFLAGS |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates on a word (16-bit) value from the stack. Only the low 16 bits of the popped value are used to update the RFLAGS register.

The instruction only modifies the lower 16 bits of RFLAGS; the upper 48 bits remain unchanged. In 64-bit mode, the stack pointer (RSP) is incremented by 2 bytes after the operation. Programmers MUST ensure the stack is properly aligned to avoid potential faults when accessing memory.