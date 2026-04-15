> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBROADCASTQ

Broadcasts a 64-bit quadword from a source operand to all 64-bit elements of the destination vector register.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m8 | xmm / ymm / zmm |
| r64 | xmm / ymm / zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination register is zero-extended if the destination is a zmm register and the instruction is executed with VEX encoding (unless EVEX is used to specify a larger register). When writing to a ymm or zmm register, the upper bits of the register are overwritten; users SHALL ensure the destination register is properly initialized if they intend to preserve higher-order bits. Memory operands MUST be aligned to the operand size to avoid potential performance penalties, although the instruction itself does not require strict alignment for correctness.