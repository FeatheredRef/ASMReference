> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD132PS

Computes the fused multiply-add operation without sign for packed single-precision floating-point values. The operation is defined as $dest = \text{abs}(a \times b) + c$, where the order of operands is determined by the 132 sequence (the first operand is multiplied by the second, and the result is added to the third).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m32 | reg |
| reg, m32, reg | reg |
| m32, reg, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode. It requires AVX support. In 32-bit mode, the processor MUST be operating in a state where AVX is enabled.

The result is computed with a single rounding step at the end, which prevents intermediate precision loss. If the MXCSR.RM field is set to rounding-toward-zero, the absolute value is calculated before the multiply-add. Users MUST ensure that the target register is an XMM or YMM register; attempting to use this instruction on general-purpose registers will result in an #I. Failure to align memory operands to their respective boundaries MAY result in performance degradation or general protection faults depending on the alignment check settings.