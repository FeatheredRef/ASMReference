> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VZEROALL

Sets the destination operands to zero. This instruction zeroes all YMM and ZMM registers.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | YMM/ZMM registers |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It is not supported in 32-bit mode.

VZEROALL is used to eliminate false dependencies on the upper bits of YMM and ZMM registers. Failure to use VZEROALL or VZEROUPPER when transitioning between AVX and non-AVX code may result in significant performance penalties due to state transitions.