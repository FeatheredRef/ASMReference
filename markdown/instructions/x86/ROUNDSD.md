> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ROUNDSD

Rounds a scalar double-precision floating-point value according to the rounding control passed in the second operand. The rounding is performed based on the specified rounding mode (e.g., round to nearest, round up, round down, or truncate) and the result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m64 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The rounding mode is specified by a 3-bit immediate value (bits 0-2) within the second operand's `imm` field. If the immediate value exceeds the valid range for rounding modes, the behavior is undefined. The instruction affects the floating-point status register, specifically triggering #P if the result is inexact. Use of this instruction avoids the need to modify the MXCSR rounding control bits, preventing the overhead of flushing the pipeline associated with updating control registers.