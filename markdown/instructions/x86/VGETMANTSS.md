> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETMANTSS

Extracts the mantissa from a scalar single-precision floating-point value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode and requires the AVX instruction set to be enabled.

The destination register is overwritten with the extracted mantissa; previous values in the register are lost. If the source operand is a signaling NaN, a #I exception SHALL be generated. For quiet NaNs, the mantissa is extracted without triggering an exception. The result is stored in the destination as a single-precision floating-point value where the mantissa represents the fractional part of the original number.