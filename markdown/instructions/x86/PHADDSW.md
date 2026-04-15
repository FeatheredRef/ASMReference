> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHADDSW

Adds paired signed 16-bit integers from two XMM registers and stores the results in a destination XMM register. For each pair of signed word elements, the operation performs a saturated addition: if the result exceeds the maximum signed 16-bit integer value, it is clamped to the maximum value; if it is less than the minimum signed 16-bit integer value, it is clamped to the minimum value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE2 instruction set to be supported by the hardware.

To avoid unexpected behavior, ensure that the input operands are valid signed 16-bit integers. Since the instruction uses saturated arithmetic, overflow will not wrap around but will instead result in the maximum or minimum representable value of a signed word. This prevents typical integer overflow issues but may lead to precision loss if the application requires the exact mathematical sum.