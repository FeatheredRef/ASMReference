> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTSD2SI

Converts the scalar double-precision floating-point value to a signed integer. The conversion uses the rounding control specified in the MXCSR register. If the result is too large to fit in the destination, a numeric overflow exception is signaled.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64 | r32 |
| f64 | r64 |
| m8 | r32 |
| m8 | r64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. In 32-bit mode, the destination must be a 32-bit register.

The destination register size must be specified via a prefix or the instruction encoding to distinguish between a dword or qword result. If the converted value exceeds the range of the destination signed integer, #O is signaled and the destination is set to the maximum or minimum representable value for that signed integer size (saturation). If the source value is a NaN, #O is signaled and the destination is set to the integer minimum value.