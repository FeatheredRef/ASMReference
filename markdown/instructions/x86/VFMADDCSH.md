> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDCSH

Performs a fused multiply-add operation on half-precision floating-point values. The instruction computes the product of the first and second source operands, then adds the result to the third source operand. The "CS" suffix indicates that the multiply-add is performed as (a * b) + c, where the result is stored in the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m16 |
| m16 | reg |

DO NOT support LOCK

The instruction REQUIRES the AVX-512 FP16 extension. It SHALL only operate on ZMM registers or memory locations when the corresponding CPUID feature flag is supported. Execution in compatibility mode is SUBJECT to the availability of the AVX-512 feature set.

The operation SHALL be performed with a single rounding step at the end of the calculation to avoid intermediate precision loss. If the destination register is also used as a source, the original value SHALL be preserved until the final result is computed. The instruction MAY trigger the following floating-point exceptions: #I, #D, #O, #U, and #P.