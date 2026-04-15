> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINUW

Computes the minimum of two unsigned word operands and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r16 | r16 |
| m2 | r16 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand MUST be a register. The instruction does NOT affect any EFLAGS register. When the source is a memory operand (m2), the operation is performed on a word-sized value; accessing memory across a page boundary MAY trigger a general protection fault or page fault.