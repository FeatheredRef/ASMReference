> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# IRETD

Pop the return address and the flags register from the stack to return from an interrupt.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16 | r16, r16 |

DO NOT support LOCK

This instruction is only available in 16-bit mode or when the processor is operating in compatibility mode. It is NOT supported in 64-bit mode.

To avoid unexpected behavior or processor exceptions, ensure that the stack pointer is correctly aligned and points to a valid memory region containing the saved flags and instruction pointer. Failure to provide a valid return address on the stack will result in the processor jumping to an arbitrary memory location.