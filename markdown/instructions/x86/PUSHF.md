> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUSHF

Pushes the RFLAGS register onto the stack. The RSP register is decremented by 4 or 8 bytes, depending on the operating mode, and the flags are stored at the new stack pointer location.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| RFLAGS | m64/m32 |
| #I | imm |
| #I | reg |

DO NOT support LOCK

In 64-bit mode, PUSHF is not supported; PUSHF is available only in compatibility mode. In 64-bit mode, PUSHFQ MUST be used to push the RFLAGS register.

When operating in compatibility mode, the instruction pushes a dword. If the processor is executing in 32-bit mode, it also pushes a dword. Failure to use the correct stack alignment or instruction variant in 64-bit mode will result in an invalid opcode exception.