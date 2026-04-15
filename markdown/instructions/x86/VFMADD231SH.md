> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD231SH

Computes the product of two floating-point operands and adds the result to a third floating-point operand, then stores the result in the destination. Specifically, it performs the operation: $dest = (src1 \times src2) + src3$, where the operands are formatted as $f16$ (half-precision).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 FP16 extension.

The operation is performed using the precision and rounding mode specified in the MXCSR register. If the operation results in a value that cannot be represented as an $f16$, the instruction may trigger #O or #U exceptions. The result is subject to #P if the result is not exact. If any operand is a signaling NaN, it SHALL trigger #I.