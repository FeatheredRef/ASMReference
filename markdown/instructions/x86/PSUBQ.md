> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBQ

Subtracts two 64-bit packed integers.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE2 instruction set.

The operation performs a saturated subtraction. If the result of the subtraction is smaller than the minimum possible signed 64-bit integer, the result is saturated to the minimum signed 64-bit integer value. If the result is larger than the maximum possible signed 64-bit integer, it is saturated to the maximum signed 64-bit integer value.