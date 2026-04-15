> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FUCOMI

Compares a floating-point value in a register with another floating-point value in a register or memory and sets the floating-point condition flags based on the result.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction is only available in x86-64 mode when operating in compatibility mode. It targets the x87 FPU register stack.

The instruction operates on the ST(0) register as the implicit destination and the specified operand as the source. The comparison does not modify the values in the registers. The result is reflected in the FPU Status Word.

Precision control and rounding modes set in the FPU Control Word may affect the comparison result if the operands are not exactly representable. If either operand is a Signaling NaN, a #I exception SHALL be generated. If either operand is a Quiet NaN, the condition flags are set to indicate an unordered comparison.