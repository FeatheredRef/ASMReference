> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD231SS

Multiplies a scalar single-precision floating-point value from a source by a scalar single-precision floating-point value from another source, adds the result to a scalar single-precision floating-point value from a third source, and stores the result in the destination. The operation is defined as: $dest = (src1 \times src2) + src3$.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m4 | reg |

DO NOT support LOCK

This instruction SHALL be used in 64-bit mode or 32-bit mode. It REQUIRES the FMA3 feature flag to be supported by the processor. The instruction uses the XMM registers; it is NOT available in compatibility mode for 16-bit segments unless the processor supports the relevant extensions.

The destination register is also used as one of the operands (the addend), meaning the original value of the destination register is overwritten. To avoid data loss, the value in the destination register MUST be backed up if it is needed for subsequent operations.

The operation is subject to floating-point exceptions including #I, #Z, #D, #O, #U, and #P according to the MXCSR register settings. Precision is handled based on the rounding mode specified in the MXCSR register.