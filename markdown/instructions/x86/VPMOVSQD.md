> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVSQD

Moves a quadword integer value from the source operand to the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m8 | xmm |

DO NOT support LOCK

This instruction requires the processor to support the AVX extension. It operates in 64-bit mode or compatibility mode.

To avoid undefined behavior or faults, ensure that memory accesses are aligned to the appropriate boundary if using aligned move variants, although VPMOVSQD typically performs unaligned accesses. The instruction only affects the lower 64 bits of the destination xmm register; the upper 64 bits of the destination xmm register are zeroed.