> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INT3

Generates a breakpoint exception. It is a single-byte instruction that triggers a software interrupt, causing the processor to call the breakpoint exception handler.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

INT3 is supported in 64-bit mode, compatibility mode, and 32-bit mode. Unlike the general INT i instruction, INT3 is encoded as a single byte (0xCC) to ensure that it can replace a single byte of an existing instruction without affecting the alignment or execution of subsequent instructions.

When using INT3 for debugging, the instruction pointer (RIP) pushed onto the stack points to the INT3 instruction itself. If the debugger intends to resume execution of the program, it MUST increment the saved RIP by 1 byte before returning from the exception handler to avoid an infinite loop.