> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRANGESS

This instruction compares a set of floating-point values against a specified range defined by a lower and upper bound. It sets the corresponding bits in the destination mask register if the values fall within the range $[lower, upper]$, inclusive.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | k reg |
| m128/m256/m512 | k reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 foundation extensions to be supported by the processor.

The range bounds must be provided in registers; using memory operands for bounds may incur significant performance penalties. Ensure that the destination mask register is correctly initialized or cleared if partial updates are not intended. The behavior is subject to the floating-point rounding mode and exception masks defined in the MXCSR register.