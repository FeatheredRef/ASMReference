> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MULX

Multiplies two unsigned operands and stores the result in two registers. The instruction performs an unsigned multiplication of two 64-bit operands, producing a 128-bit product.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m8 | reg, reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It SHALL NOT be used in compatibility mode.

Unlike the `MUL` instruction, `MULX` does NOT affect any flags in the RFLAGS register. Users MUST NOT rely on the Carry Flag (CF) or Overflow Flag (OF) to detect overflow of the 64-bit result, as these flags remain unchanged. Additionally, `MULX` does NOT use the implicit `RAX` register, allowing for non-destructive multiplication of operands.