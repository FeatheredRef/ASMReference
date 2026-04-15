> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRNDSCALESS

Rounds a floating-point value to the nearest integer using a specified rounding mode and scales the result by a power of 2. The instruction converts the input floating-point value to a signed integer and then multiplies that integer by $2^{imm}$.

The following table covers the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512 Foundation).

The scale factor provided by the immediate operand must be within the range of -63 to 63. If the rounding result exceeds the range of the destination integer type (e.g., 32-bit or 64-bit signed integer), the instruction SHALL trigger #O. Users MUST ensure the input value and the scale factor do not cause an overflow to avoid exception handling. Proper selection of the rounding mode (via the immediate) is required to avoid unexpected #P flags.