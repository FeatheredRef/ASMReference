> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFCMADDCPH

Performs a fused multiply-add operation on packed half-precision floating-point values. The instruction multiplies the source operands and adds the result to the destination operand using the formula: $dest = dest + (src1 \times src2)$.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 extension to be supported by the processor.

The operation uses the precision control and rounding mode specified in the MXCSR register. If the input operands are not in the supported half-precision format, the instruction SHALL trigger #I. Precision loss during the intermediate multiplication or the final addition MAY result in #P. Results exceeding the maximum representable value for f16 SHALL trigger #O.