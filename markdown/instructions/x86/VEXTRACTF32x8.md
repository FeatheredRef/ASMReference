> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTF32x8

Extracts eight 32-bit floating-point elements from a single 256-bit YMM register and stores them into a destination register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (YMM) | reg (YMM) |
| imm | #I |
| mN | #I |

DO NOT support LOCK

The instruction is only available in 64-bit mode and 32-bit mode; it is NOT supported in 16-bit compatibility mode. It requires the AVX (Advanced Vector Extensions) CPU feature flag to be enabled.

To avoid performance degradation due to AVX-SSE transitions, ensure that the YMM register is not mixed with legacy XMM instructions without utilizing the VEX-encoded versions of those instructions, as this may trigger state transitions. All destination bits outside the range of the extracted elements SHALL be zeroed.