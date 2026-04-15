> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD213PD

Performs a fused multiply-subtract and add operation on packed double-precision floating-point values. The instruction calculates the result using the formula: $dest = (a \times b) - c + d$, specifically mapped as $dest = (r2 \times r1) - r3 + r0$ (or similar operand mappings depending on the specific VEX/EVEX encoding) for each corresponding element in the vectors.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX and FMA3 instruction set extensions to be enabled in the processor.

The operation is performed with a single rounding step at the end, which prevents the accumulation of rounding errors that would occur if multiply, subtract, and add operations were executed separately. If the instruction is used with a memory operand, the memory region SHALL be aligned to the vector size to avoid performance penalties or faults depending on the alignment check settings. All floating-point exceptions such as #P, #O, #U, #D, and #Z SHALL be handled according to the MXCSR register settings.