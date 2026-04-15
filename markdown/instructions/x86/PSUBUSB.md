> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBUSB

Subtracts two unsigned saturated packed integers. The operation subtracts the source operand from the destination operand, and if the result is less than 0, the result is saturated to 0.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates on packed signed integers; however, the "USB" suffix indicates the subtraction is performed with unsigned saturation.

To avoid unexpected behavior, ensure that the input operands are treated as unsigned integers. If the result of the subtraction is negative, the instruction will clamp the value to 0 rather than allowing a wrap-around, which is critical for maintaining data integrity in signal processing or image manipulation tasks.