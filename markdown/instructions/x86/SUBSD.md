> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SUBSD

Subtracts a scalar-double precision floating-point value from another scalar-double precision floating-point value and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |
| r64 | m8 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It is not supported in 32-bit mode.

The instruction performs the operation according to IEEE 754 standards. It affects the MXCSR register by updating status flags such as #P, #O, #U, and #D based on the result of the subtraction. Ensure that the destination register is an XMM register to avoid invalid opcode exceptions.