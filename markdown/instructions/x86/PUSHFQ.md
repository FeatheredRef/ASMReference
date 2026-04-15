> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUSHFQ

Pushes the RFLAGS register onto the stack. This involves decrementing the RSP register by 8 and storing the 64-bit contents of RFLAGS at the memory location pointed to by the updated RSP.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| RFLAGS | m8 (via RSP) |
| #I | imm |
| #I | reg |

DO NOT support LOCK

This instruction SHALL only be used in 64-bit mode. It is NOT supported in compatibility mode; attempting to execute it in compatibility mode will result in an invalid opcode exception.

The instruction implicitly modifies the RSP register. If the stack pointer is not properly aligned or points to a non-writable memory region, a general-protection fault or page fault MAY occur. Ensure that the stack is correctly initialized to avoid memory access violations.