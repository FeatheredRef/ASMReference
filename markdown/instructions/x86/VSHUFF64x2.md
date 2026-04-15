> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSHUFF64x2

This instruction shuffles two 64-bit doublewords from the source operand according to the immediate value. The immediate specifies which 64-bit element from the source is placed into the destination's 64-bit lanes.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires support for the AVX extension.

The immediate value MUST be a valid shuffle control byte; otherwise, the behavior is undefined. Failure to use an AVX-compatible register may trigger a General Protection fault (#GP) if the transition between SSE and AVX state is not managed correctly via VEX-encoded instructions.