> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPSS

Compares two scalar single-precision floating-point values according to the specified predicate and sets the condition flags (CF, ZF, PF) in the EFLAGS register based on the result.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction requires the SSE extension. It is available in both 32-bit and 64-bit mode.

The comparison result depends on the predicate specified in the immediate operand. If either operand is a Signaling NaN, a #I exception is generated. If the predicate is "unordered" and either operand is a NaN, the ZF flag is cleared and CF is set. Users MUST ensure the destination register is an xmm register to avoid invalid operand exceptions.