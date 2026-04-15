> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VREDUCESD

Reduces a double-precision floating-point value to the nearest double-precision floating-point value in the range [-1, 1]. If the input value is outside this range, the instruction reduces it by subtracting or adding an integer multiple of 2.0.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64 | f64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in AVX-supported modes. It requires the VEX prefix; the non-VEX version is not supported in 64-bit mode.

The result is subject to the current floating-point rounding mode. If the input value is NaN or infinity, the instruction SHALL return the input value unchanged. Precision (#P) and Underflow (#U) exceptions may be raised depending on the result of the reduction.