> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMULPH

Multiplies two half-precision (16-bit) floating-point values and stores the result in a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 FP16 support.

The instruction operates on half-precision floating-point numbers (f16). Failure to ensure the processor supports the FP16 extension will result in an invalid opcode exception.

If the result cannot be represented in f16 precision, the floating-point control word settings shall determine if the result is rounded or if #P is signaled. Specific exception flags may be raised: #O for overflow, #U for underflow, or #I for invalid operations (e.g., multiplying infinity by zero).