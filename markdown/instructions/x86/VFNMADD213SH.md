> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD213SH

Computes the fused multiply-add operation for the specified number of floating-point elements: destination = -(source1 * source2) + source3.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires AVX-512 foundation support.

The instruction operates on ZMM registers; using it on hardware without AVX-512 support will result in an invalid opcode exception. To avoid precision loss and unexpected behavior, ensure the MXCSR register is correctly configured for rounding modes, as this instruction is subject to floating-point exceptions including #D, #O, #U, and #P.