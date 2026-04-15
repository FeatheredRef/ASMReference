> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RET

Pops the top qword from the stack into the instruction pointer (RIP), effectively returning from a procedure.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | RIP |

DO NOT support LOCK

In 64-bit mode, the instruction pops a qword from the stack. In compatibility mode, the behavior depends on the operand size override; if the operand size is 32-bit, it pops a dword and zero-extends the upper 32 bits of RIP.

The stack pointer (RSP) is incremented by 8 bytes (or 4 bytes in compatibility mode with a 32-bit operand size) after the return address is popped. If the stack pointer is not properly aligned or if the value popped from the stack is non-canonical, a general-protection exception (#GP) SHALL occur.

When using `RET n`, the instruction pops the return address and then adds `n` bytes to the stack pointer (RSP). This is used to clean up arguments passed on the stack (callee cleanup). The immediate `n` MUST be a byte.