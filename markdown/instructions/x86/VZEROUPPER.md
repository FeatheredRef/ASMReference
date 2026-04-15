> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VZEROUPPER

Sets the upper 256 bits of the YMM registers to zero. This instruction is used to transition from AVX state to SSE state to prevent performance penalties associated with AVX-SSE transitions.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It SHALL NOT be used in 32-bit mode.

Failure to execute VZEROUPPER before executing legacy SSE instructions after using AVX instructions MAY result in a transition penalty. This penalty occurs because the processor MUST save the upper half of the YMM registers before executing SSE instructions to maintain architectural state, leading to significant performance degradation in mixed-code environments.