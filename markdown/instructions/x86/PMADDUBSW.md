> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMADDUBSW

Multiplies unsigned 8-bit integers from the first source operand by signed 8-bit integers from the second source operand, adds the adjacent pairs of products to a signed 16-bit result, and stores the results in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| mN | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode and requires the SSE4.1 extension. It operates exclusively on XMM registers.

The operation performs saturation on the 16-bit signed integer sums; if the result of the addition exceeds the range of a signed 16-bit integer, it is clamped to the maximum or minimum representable value (i32: -32768 to 32767). Failure to account for this saturation behavior may lead to logic errors in signal processing applications expecting wrap-around arithmetic.