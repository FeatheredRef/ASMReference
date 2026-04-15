> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FADD

Adds a floating-point value to the contents of a floating-point register (ST(0)) or two floating-point values. The result is stored in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | #I |

DO NOT support LOCK

FADD is available in 64-bit mode, but it operates on the x87 FPU stack. This instruction requires the use of the x87 floating-point unit and is not available for use with XMM/YMM/ZMM registers.

The operation SHALL result in the following floating-point exceptions:
- #O if the result is too large to be represented.
- #U if the result is too small to be represented.
- #P if the result is rounded.
- #Z if an operand is a zero and the result is zero.
- #D if an operand is a denormalized value.

The user MUST ensure the FPU stack is not empty or overflowed, as this instruction pops one operand from the stack unless the form `FADD ST(0), ST(i)` is used. If the memory operand is not naturally aligned, it MAY cause a performance penalty.