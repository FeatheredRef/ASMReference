> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUSH

Decrements the stack pointer (`rsp`) and stores the specified operand on the stack.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | m64 |
| imm | m64 |
| m64 | m64 |
| #I | #I |

DO NOT support LOCK

In x86-64 mode, `PUSH` only supports operands of size qword (64 bits). If a smaller register is specified, it is zero-extended to 64 bits before being pushed. This instruction is NOT available in compatibility mode if the operand size is not consistent with the current operating mode's stack pointer size.

When using `PUSH` with an immediate value, the value is sign-extended to 64 bits before being pushed. To avoid stack misalignment, ensure that the stack pointer `rsp` remains 16-byte aligned before calling functions, as `PUSH` modifies the alignment by 8 bytes. Overflowing the stack memory region may lead to a General Protection exception (#GP) or a Page Fault (#PF).