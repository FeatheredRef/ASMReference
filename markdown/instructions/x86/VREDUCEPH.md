> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VREDUCEPH

Performs a floating-point reduction operation on a set of half-precision floating-point numbers. It sums the elements of a source vector into a single scalar result, rounding the final sum to the specified precision.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 FP16 extension. It requires the CPU to be in 64-bit mode or compatibility mode.

The operation uses the rounding mode specified in the `rm` field of the instruction encoding. If the `rm` field is set to `00` (static rounding), the rounding mode is defined by the immediate byte; otherwise, it uses the current rounding mode in the MXCSR register. Failure to specify the correct rounding mode can lead to precision loss or incorrect results in sensitive numerical computations.

Incorrect alignment of memory operands (if used in related vector loads) may result in performance penalties or general protection faults depending on the alignment check settings. Ensure that the destination register is cleared if used as an accumulator in a loop to avoid adding garbage values from previous operations.