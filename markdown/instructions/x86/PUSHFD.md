> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUSHFD

Pushes the current value of the EFLAGS register onto the stack and decrements the stack pointer.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| EFLAGS | m32 |
| #I | imm |
| #I | reg |

DO NOT support LOCK

In x86-64 architecture, PUSHFD is only supported in compatibility mode. In 64-bit mode, this instruction is invalid; PUSHFQ SHALL be used instead to push the RFLAGS register.

When executing PUSHFD, the stack pointer (ESP) is decremented by 4 bytes. If the stack is aligned to a specific boundary, this operation will change that alignment. Users MUST ensure that the stack is properly aligned before calling functions after a PUSHFD operation to avoid performance degradation or crashes on certain ABIs.