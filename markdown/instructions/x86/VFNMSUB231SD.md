> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB231SD

Subtracts the product of two double-precision floating-point values from a third value and negates the result. The operation is defined as: destination = -(source1 - (source2 * source3)).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| reg, reg, reg | reg |
| m8, reg, reg | reg |

DO NOT support LOCK

This instruction requires the AVX support feature. It is only available in 64-bit mode or compatibility mode.

The instruction uses the VEX encoding scheme; therefore, it does not support the LOCK prefix. Using a memory operand for the first source operand is permitted, but the destination must be a register (XMM).

When executing this instruction, the precision control in the MXCSR register SHALL determine the rounding direction. If the result of the operation exceeds the maximum representable value for a double-precision float, a #O exception SHALL be triggered. If the result is inexact, a #P exception SHALL be triggered. Ensure that the XMM registers are properly aligned to 16 bytes when using memory operands to avoid general protection faults or performance degradation.