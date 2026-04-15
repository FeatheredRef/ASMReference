> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDD

Adds two packed signed 32-bit integers from the source and destination operands, storing the result in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE3 extension set to be enabled.

The operation performs a signed addition of packed `i32` values. If an overflow occurs for any of the individual 32-bit elements, the result wraps around according to two's complement arithmetic. This instruction does not set any flags in the EFLAGS register.