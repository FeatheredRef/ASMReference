> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DIV

Unsigned divide. The dividend is the value in the RAX, EAX, or AX register (depending on the operand size), combined with a register or memory location. The quotient is stored in the accumulator register, and the remainder is stored in the RDX, EDX, or AX register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | implicit |
| mN | implicit |
| imm | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and compatibility mode. In 64-bit mode, if a 32-bit operand is used, the upper 32 bits of RAX are not affected, and the operation utilizes EAX and EDX.

If the quotient cannot fit in the destination register, or if the divisor is zero, the processor generates a #Z exception. This occurs when the result of the division exceeds the capacity of the accumulator register (e.g., RAX for qword operands).

Users MUST ensure the dividend is correctly placed in the high-order register (RDX:RAX, EDX:EAX, or DX:AX) before executing DIV. Failure to clear or set the RDX/EDX/DX register will result in an incorrect dividend, as the instruction treats the combination as a single large unsigned integer.