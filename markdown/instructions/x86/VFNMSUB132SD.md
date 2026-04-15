> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB132SD

Subtracts the product of two floating-point values from a third floating-point value and stores the result in a destination. Specifically, it computes $dest = -(a \times b) + c$, where $a$ and $b$ are scalar double-precision floating-point values and $c$ is a scalar double-precision floating-point value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| m8, f64, f64 | f64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX instruction set to be supported and enabled by the processor.

The instruction utilizes the YMM registers but operates only on the lower 64 bits (the scalar double-precision portion). The upper bits of the YMM destination register are preserved. Ensure that the processor state allows AVX execution to avoid #UD (Undefined Instruction) exceptions. Floating-point exceptions #D, #Z, #O, #U, and #P may be raised depending on the result and the MXCSR register settings.