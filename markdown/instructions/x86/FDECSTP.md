> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FDECSTP

Converts a 64-bit extended precision floating-point value to a signed decimal integer and stores the result in memory. The result is rounded according to the current rounding control word. If the result is too large to fit in the destination, a numeric overflow exception is generated.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m32 |
| reg | m64 |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode. It is only available in compatibility mode.

The destination memory operand SHALL be aligned to the size of the destination (4 bytes for dword, 8 bytes for qword). If the source value is NaN, an invalid operation exception (#I) SHALL be generated. If the source value is infinity or the result exceeds the range of the destination signed integer, a numeric overflow exception (#O) SHALL be generated.

The instruction uses the ST(0) register as the implicit source. Ensure that the value in ST(0) is a valid extended precision float to avoid #I or #O exceptions during the conversion process.