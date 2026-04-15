> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RCPPS

Rounds the shortest single-precision floating-point value in each packed element of the source operands to the precision specified by the rounding control field in the immediate operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m32 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE support.

The rounding mode is controlled by a 3-bit immediate value; if the immediate is not within the range of 0-7, it SHALL be treated as an invalid encoding. The instruction does not modify the MXCSR rounding control field, as the rounding mode is specified explicitly per-instruction.

To avoid unexpected results, ensure that the destination register is not used as a source if the original values must be preserved, as the destination is overwritten. Precision (#P) may be flagged if the result of the rounding operation differs from the original value.