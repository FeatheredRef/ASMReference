> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VREDUCEPD

Reduces two double-precision floating-point values to a single double-precision floating-point value. The instruction performs a reduction operation on the elements of the source operands, typically used in the context of AVX-512 algorithms to sum or combine vector elements into a scalar result.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64 | f64 |
| m64 | f64 |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES AVX-512 support and the corresponding CPUID feature flags to be enabled.

Ensure that the destination register is not used as a source for other concurrent operations to avoid data hazards. Improper alignment of memory operands m64 MAY result in general-protection exceptions (#GP) if the memory access crosses a page boundary. Resulting values SHALL be subject to floating-point exceptions #D, #O, #U, and #P depending on the precision and range of the operands.