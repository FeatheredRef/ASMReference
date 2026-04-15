> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PEXTRQ

PEXTRQ extracts bits from the source operand according to the mask specified by the second source operand. For each bit set to 1 in the mask, the corresponding bit from the source is extracted and packed into the destination register, maintaining the original relative order of the extracted bits. The remaining bits of the destination register are set to 0.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg / m64 | reg |
| reg / m64 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the BMI2 (Bit Manipulation Instruction Set 2) extension.

The destination register must be different from the mask source operand if the source operand is a memory region, to avoid potential read-after-write hazards in specific microarchitectural implementations. Ensure the target register is 64-bit (r64) to avoid truncation of the extracted bit sequence.