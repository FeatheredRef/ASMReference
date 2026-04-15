> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULUDQ

Multiplies a u64 value by a u64 value and stores the 128-bit result across two 64-bit registers. The lower 64 bits of the product are stored in the destination register, and the upper 64 bits of the product are stored in RDX.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | reg |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The RDX register is implicitly used to store the upper 64 bits of the result; therefore, any value previously stored in RDX SHALL be overwritten regardless of the specified destination register. If the destination register is RDX, the operation remains valid, but the upper 64 bits of the product will effectively overwrite the lower 64 bits of the result if not handled correctly in the instruction sequence.