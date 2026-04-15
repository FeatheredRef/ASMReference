> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDZ

Loads the scalar floating-point value 0.0 into the specified floating-point register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| memory | #I |
| implicit | st(n) |

DO NOT support LOCK.

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the x87 floating-point stack.

The destination register must be specified as `st(n)`, where `n` is the index of the register in the floating-point stack. The instruction does not affect the floating-point status word or control word, and no floating-point exceptions are generated. Using an invalid register index for `st(n)` SHALL result in an invalid operand exception at assembly time.