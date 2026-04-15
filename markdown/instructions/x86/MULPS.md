> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MULPS

Multiplies four single-precision floating-point numbers from a source operand by four single-precision floating-point numbers from a destination operand and stores the results in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction requires SSE support. It is available in both 64-bit mode and compatibility mode.

The operation is performed according to the IEEE 754 standard. The floating-point unit control word determines the rounding behavior and exception masking.

Incorrect alignment of the m128 operand may result in a general-protection exception (#GP) if alignment checking is enabled in the CR0 register. Ensure the memory operand is aligned on a 16-byte boundary to avoid performance degradation or faults.