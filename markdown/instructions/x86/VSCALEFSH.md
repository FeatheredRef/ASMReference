> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCALEFSH

This instruction scales floating-point values in a destination vector by a scalar value specified by an immediate. The scaling is performed by multiplying the destination elements by $2^{imm}$, where the immediate is a signed integer.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | fN (reg) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires support for the AVX-512 foundation instructions.

The immediate value is treated as a signed integer. If the result of the scaling operation exceeds the maximum representable value for the destination floating-point format, the result is set to $\pm\infty$ and #O is signaled. If the result is smaller than the minimum representable value, it may result in #U or #D. Precision loss during the operation will signal #P.