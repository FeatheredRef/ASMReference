> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXORQ

Performs a bitwise logical exclusive OR operation on the source and destination operands. The result is stored in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It operates on XMM registers (128-bit) using the specified operand size.

When the destination register is the same as the source register, the instruction may be used by the assembler or compiler to zero out the register. However, because this is a SIMD operation on XMM registers, the behavior regarding the upper bits of the register depends on the specific operand size used (e.g., `KXORQ` vs `KXORPS`). Ensure the correct register size is targeted to avoid unintended side effects on the upper bits of the XMM register.