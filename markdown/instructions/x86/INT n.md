> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INT n

Triggers a software interrupt by referencing the interrupt vector `n` in the Interrupt Descriptor Table (IDT). The processor pushes the FLAGS register, the current code segment (CS), and the instruction pointer (RIP) onto the stack before transferring control to the interrupt handler defined in the IDT.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | Internal State |

DO NOT support LOCK

In x86-64 mode, the `INT n` instruction is supported; however, its behavior is primarily used for legacy compatibility or specific software-triggered exceptions. While it functions in 64-bit mode, it is often replaced by `SYSCALL` for system calls to reduce overhead.

The user SHALL ensure that the interrupt vector `n` is properly initialized in the IDT before execution; otherwise, a General Protection fault (#GP) will occur. Because `INT n` performs a stack push of the return address and flags, the stack pointer (RSP) MUST be valid and point to a writable memory region to avoid a Stack Fault (#SS).