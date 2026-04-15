> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTSH2SI

Converts a packed signed 16-bit integer (short) from an XMM register to a signed 32-bit or 64-bit integer, truncating the result. The conversion is performed using the "truncate" rounding mode, which rounds the signed value toward negative infinity.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm (f128) | xmm (f128) |
| m16 | xmm (f128) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination register size must match the intended target integer size (i32 or i64) as defined by the specific opcode variant used. If the source value exceeds the range of the destination signed integer type, the result is the indefinite value (NaN). Failure to ensure the input values are within the representable range of the target integer type MAY result in unexpected saturation or NaN values in the destination register.