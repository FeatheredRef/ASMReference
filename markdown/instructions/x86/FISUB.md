> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FISUB

Subtracts an internal floating-point value from another and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(0) | ST(i) |
| mN | ST(i) |

DO NOT support LOCK

FISUB is available in 64-bit mode, but it is primarily used for compatibility with x87 floating-point code. It operates on the x87 floating-point stack and does not affect the general-purpose registers or EFLAGS.

The destination operand ST(i) MUST NOT be ST(0) when using the memory-source form, as the subtraction is performed between ST(0) and the source, and the result is stored in ST(i). If ST(0) is the destination, the operation is performed in-place.

Precision and rounding control are determined by the x87 control word. Operations may trigger #P, #U, or #O depending on the result. If the operands are not valid floating-point numbers, the instruction SHALL signal #I.