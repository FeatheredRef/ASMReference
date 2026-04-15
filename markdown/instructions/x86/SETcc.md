> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SETcc

Sets the byte at the destination to 1 if the condition specified by the suffix (cc) is true, and 0 if the condition is false.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| EFLAGS | m8 |
| EFLAGS | r8 |
| EFLAGS | #I |

DO NOT support LOCK

The instruction is available in both 64-bit mode and compatibility mode. It operates exclusively on 8-bit destinations. If a 32-bit or 64-bit register is specified as the destination, it is an invalid operation.

The destination register or memory location is modified without affecting the upper bits of the register if a larger register (e.g., r64) is used to host the r8 destination; only the lowest 8 bits are updated. To avoid unintended side effects in subsequent calculations, the programmer SHOULD ensure the upper bits of the destination register are cleared if the register is later used as a larger integer type.