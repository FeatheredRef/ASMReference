> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRNDSCALESH

Rounds a signed scaled floating-point value to the nearest signed integer according to the specified rounding mode and scales the result by a power of 2.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 foundation extensions.

The scale factor is determined by an immediate byte embedded in the instruction. If the immediate value is not within the valid range defined by the ISA, the instruction is invalid. The operation follows the rounding mode specified in the rounding control field of the instruction's encoding or the MXCSR register.

To avoid precision loss or unexpected results, the programmer MUST ensure that the scaling factor does not result in a value that exceeds the maximum representable range of the destination integer type, as this MAY trigger a #O exception depending on the rounding mode.