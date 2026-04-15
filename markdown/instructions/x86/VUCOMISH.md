> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VUCOMISH

Compares a scalar unsigned integer (represented as a floating-point value) with a scalar unsigned integer (represented as a floating-point value) and sets the floating-point condition codes.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm (f64) | EFLAGS/MXCSR |
| m8 (u8) | EFLAGS/MXCSR |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

To avoid undefined behavior, ensure that the memory operand is aligned to the size of the operand to prevent alignment check exceptions. The instruction compares the unsigned integer value in the memory operand against the double-precision floating-point value in the xmm register; if the xmm register contains a value that cannot be represented as an unsigned integer of the specified size, the result is implementation-defined.

The instruction may trigger the following floating-point exceptions depending on the input values:
- #O: If the value exceeds the representable range.
- #P: If the result requires rounding.