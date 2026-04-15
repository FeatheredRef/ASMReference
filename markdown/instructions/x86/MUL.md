> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MUL

Multiplies the source operand by the accumulator (AL, AX, EAX, or RAX) and stores the result in the accumulator and/or an adjacent register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | #I |

DO NOT support LOCK

The instruction is only available for unsigned multiplication. The destination registers are implicit based on the operand size:
- For u8: Source $\times$ AL $\rightarrow$ AX.
- For u16: Source $\times$ AX $\rightarrow$ DX:AX.
- For u32: Source $\times$ EAX $\rightarrow$ EDX:EAX.
- For u64: Source $\times$ RAX $\rightarrow$ RDX:RAX.

The Carry Flag (CF) and Overflow Flag (OF) are set if the upper half of the result is non-zero; otherwise, they are cleared. This indicates if the result exceeds the capacity of the lower destination register.

When operating in 64-bit mode, the source MUST be a register or memory location. Immediate values are NOT supported. Failure to use the implicit accumulator as the implicit destination will result in an invalid operation.