> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FFREE

Frees the floating-point register specified by the operand. This instruction marks the register as empty, which causes any subsequent attempt to use the register before it is reloaded to trigger a stack underflow exception.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is only available in compatibility mode when executing in 64-bit mode. It is not supported in native 64-bit mode.

The operand MUST be an immediate value representing the register index (ST(0) through ST(7)). Using any other operand type will result in an invalid operation. Failure to properly manage the floating-point stack via FFREE and FLDZ can lead to stack overflow or underflow conditions if the stack pointer is not correctly tracked.