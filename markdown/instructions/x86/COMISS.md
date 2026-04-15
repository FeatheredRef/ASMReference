> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# COMISS

Compares two scalar single-precision floating-point values. The instruction sets the EFLAGS.CF, EFLAGS.PF, and EFLAGS.ZF flags based on the result of the comparison.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

The instruction SHALL be used with scalar single-precision floating-point values. It operates on the lowest 32 bits of the XMM registers. In x86-64 mode, this instruction is available in both 64-bit mode and compatibility mode.

The EFLAGS.CF flag is cleared if the comparison is unordered (i.e., one or both operands are NaN). If the comparison is ordered, EFLAGS.CF is set based on the comparison result. EFLAGS.PF is cleared unless the comparison is unordered. Users SHOULD ensure that the destination register is an XMM register, as memory-to-memory comparisons are NOT supported.