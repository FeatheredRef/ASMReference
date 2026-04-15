> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TZCNT

Counts the number of trailing zeros in a source operand. If the source operand is zero, the result is the operand size (e.g., 32 for dword, 64 for qword).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode and 32-bit mode. It REQUIRES the BMI1 feature flag to be supported by the processor; on processors where BMI1 is not supported, this instruction is decoded as `rep bsf`.

When executed on hardware that does not support BMI1, the instruction behaves as `BSF`. In that scenario, if the source operand is zero, the destination register remains unmodified and the ZF flag is set. In contrast, when BMI1 is supported, `TZCNT` will return the operand size (32 or 64) and the ZF flag is NOT set. Programs MUST check for BMI1 support or handle the difference in behavior for zero inputs to avoid undefined state in the destination register.