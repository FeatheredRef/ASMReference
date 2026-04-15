> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# IRET

Returns control from an interrupt handler to the previously interrupted program. It pops the return address, code segment selector, and flags from the stack and loads them into the corresponding registers.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16/m32/m64 | r64, r16, r32 |
| #I | #I |

DO NOT support LOCK

In 64-bit mode, IRET is primarily used to return from an interrupt or exception. If the processor is in compatibility mode, the instruction behaves as it does in 32-bit mode. If the return is from a 64-bit mode handler to a 64-bit mode program, the instruction pops the return RIP, CS, and RFLAGS.

The stack frame MUST be correctly aligned and contain the expected values (RIP, CS, and RFLAGS) before execution; otherwise, the processor SHALL trigger a General Protection exception (#GP) or a Stack Fault (#SS). When returning to a different privilege level, the instruction also pops the stack pointer (RSP) and the stack segment (SS) from the stack. Failure to provide a valid stack frame when changing privilege levels will result in an immediate #GP.