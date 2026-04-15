> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINUD

Computes the minimum of two unsigned integers. The instruction compares two unsigned operands and stores the smaller value in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| imm | #I |

DO NOT support LOCK

This instruction is only available when the processor is in 64-bit mode or compatibility mode. It operates exclusively on the XMM registers (ymm/zmm for AVX/AVX-512 variants).

The instruction treats the input values as unsigned integers. If the operands are provided as floating-point types, the bit patterns are interpreted as unsigned integers; performing this operation on signed integers will yield incorrect results. Ensure that the destination register is of the same size as the source to avoid truncation or undefined behavior.