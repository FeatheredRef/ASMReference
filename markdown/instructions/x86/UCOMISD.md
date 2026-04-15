> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UCOMISD

Compares two double-precision floating-point values. The instruction sets the ZF, PF, and CF flags in the EFLAGS register based on the comparison of the destination and source operands. Unlike `COMISD`, `UCOMISD` does not trigger any floating-point exceptions.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m8 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 extension set to be enabled.

To avoid incorrect conditional branching, the programmer MUST check the CF flag first. If the operands are Unordered (e.g., one operand is a NaN), CF and ZF are both set to 1. Failure to check CF before ZF will lead to incorrect logic when handling NaNs, as a NaN comparison results in both flags being set, which would otherwise be interpreted as "Equal" if only ZF were checked.