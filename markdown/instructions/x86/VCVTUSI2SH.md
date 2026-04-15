> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUSI2SH

Converts an unsigned packed 32-bit integer value to a signed packed 16-bit integer value. The instruction converts the u32 source to an i16 destination; if the value is outside the representable range of a signed 16-bit integer, the result is saturated to the maximum or minimum representable value of i16.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX CPUID feature flag to be supported.

When the input u32 value exceeds the maximum representable value of a signed 16-bit integer (32767), the result is saturated to the maximum signed 16-bit integer. If the input is larger than the capacity of the target register width, the operation is performed element-wise. Failure to ensure the source register contains valid u32 elements may result in unintended saturation or conversion of adjacent data.