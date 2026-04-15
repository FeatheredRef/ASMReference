> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTF32x4

Extracts four single-precision floating-point values from a 128-bit lane of a YMM register and stores them in a XMM register.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg (ymm) | reg (xmm) |
| imm (u8) | #I |
| mN | #I |

DO NOT support LOCK

The instruction requires AVX support. It is available in both 64-bit and compatibility mode. The immediate operand MUST be either 0 or 1, specifying which 128-bit half of the ymm register to extract; any other value is an invalid operation.

The destination XMM register is completely overwritten by the extracted 128-bit lane. Users MUST ensure the source YMM register is properly aligned if it was loaded from memory prior to this operation to avoid performance penalties, although the instruction itself operates only on registers.