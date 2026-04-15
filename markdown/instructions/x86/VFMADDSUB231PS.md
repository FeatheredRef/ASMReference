> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB231PS

Computes the sum of a product and a third operand for each single-precision floating-point element. The operation is defined as: $dest = (src1 \times src2) + src3$ or $dest = (src1 \times src2) - src3$, depending on the opcode encoding.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg, reg, reg | reg |
| reg, reg, m32 | reg |

DO NOT support LOCK

This instruction requires the FMA3 feature set to be supported by the processor. It is available in 64-bit mode and 32-bit mode. It SHALL NOT be used in compatibility mode if the processor does not support the FMA3 extension.

To avoid precision loss or incorrect results, the user MUST ensure that the MXCSR register is configured for the desired rounding mode and denormal handling. The instruction produces floating-point exceptions such as #O, #U, and #P according to the IEEE 754 standard. If an operand is a signaling NaN, it SHALL trigger an #I exception.