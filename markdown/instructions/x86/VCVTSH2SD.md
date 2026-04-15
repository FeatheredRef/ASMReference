> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSH2SD

Converts a signed 16-bit integer (word) from a source operand to a 64-bit floating-point number (double precision) and stores the result in a destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m16 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires CPUID feature flag AVX to be supported.

The source operand, when using a memory reference (m16), must be naturally aligned to a 2-byte boundary to avoid performance penalties or alignment faults depending on the processor configuration. The conversion process may trigger the #P (Inexact result) exception if the result cannot be represented exactly in the destination floating-point format.