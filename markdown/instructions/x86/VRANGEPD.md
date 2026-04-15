> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRANGEPD

Checks whether each double-precision floating-point element in a source operand is within a specified range. For each element, the instruction compares the value against the lower and upper bounds. If the value is greater than or equal to the lower bound and less than the upper bound, the corresponding mask bit is set to 1; otherwise, it is set to 0.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm reg | k reg |
| m64/m32/m16 | k reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 foundation extensions.

The instruction uses the destination mask register (`k reg`) to store the result of the range check. If the destination mask register is not specified or is not available in the current execution environment, the operation SHALL fail. Ensure that the selected `k` register is of sufficient width to accommodate the number of elements processed in the `zmm`, `ymm`, or `xmm` registers to avoid unexpected masking behavior.

The instruction is subject to floating-point exceptions based on the current MXCSR register settings. Specifically, it may trigger #D, #O, #U, or #P if the comparison involves denormalized numbers or results in an inexact representation.