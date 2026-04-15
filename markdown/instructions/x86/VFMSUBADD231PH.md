> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD231PH

Performs a fused multiply-subtract and add operation on packed half-precision floating-point values. For each corresponding element of the input vectors, the instruction computes $(a \times b) - c + d$ and stores the result in the destination.

The following table specifies the supported source and destination types.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | #I |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode and requires AVX-512 support.

The operation is performed using the precision and rounding mode specified in the MXCSR register. The result is subject to standard IEEE 754 floating-point exceptions, including #D, #O, #U, and #P. If the operation results in an overflow, #O is signaled.