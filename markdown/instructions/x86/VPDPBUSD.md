> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPDPBUSD

Computes the dot product of unsigned byte integers from the first source operand and signed byte integers from the second source operand, adding the result to signed doubleword integers in the destination operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m128/m256/m512 | reg |

DO NOT support LOCK

This instruction requires the AVX-512 BW and AVX-512 VL extensions. It is NOT supported in compatibility mode.

The operation is performed on the lowest 8 bits of each lane. When using masking, masked-off elements in the destination operand retain their original value, while masked-off elements in the source operands are ignored. To avoid precision loss or overflow, ensure the accumulated sum in the destination register does not exceed the range of a signed 32-bit integer (i32), as the instruction does not trigger #O.