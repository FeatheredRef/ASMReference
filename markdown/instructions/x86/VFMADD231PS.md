> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD231PS

Computes the product of two single-precision floating-point values and adds the result to a third single-precision floating-point value. The operation is performed as: $dest = (src1 \times src2) + src3$, where the operand order is defined by the "231" sequence (source 2, source 3, then source 1).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m32 | reg |

DO NOT support LOCK

This instruction SHALL be used in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode. It REQUIRES the FMA3 feature flag to be supported by the processor.

The instruction uses the VEX encoding scheme; therefore, it SHALL NOT be used in code targeting processors that do not support VEX. Failure to ensure the appropriate AVX/FMA state can result in performance penalties due to transitions between AVX and SSE states.

The precision of the result is governed by the MXCSR register. The operation performs a single rounding step at the end of the fused multiply-add, which avoids intermediate rounding errors. The following exceptions may be raised based on the result: #D, #O, #U, #P.