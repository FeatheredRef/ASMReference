> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADCX

Adds the operand and the carry flag (CF) to the destination register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m64 | r64 |
| imm | #I |

DO NOT support LOCK

ADCX is supported only in 64-bit mode. It is NOT supported in compatibility mode.

The instruction does NOT affect the Overflow Flag (OF), SF, ZF, AF, or PF. It only updates the Carry Flag (CF).

To avoid performance degradation or incorrect results in multi-precision arithmetic, the user SHOULD pair ADCX with ADCOX. Because ADCX does not modify the Overflow Flag, it allows the processor to execute multiple additions in parallel without creating a dependency chain on the OF flag.