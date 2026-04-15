> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBLENDMQ

Selects elements from two qword sources based on a mask provided by an immediate value and blends them into the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm, m64 | zmm |
| zmm, zmm | zmm |
| imm | #I |

DO NOT support LOCK

This instruction requires AVX-512 support. It is only available in 64-bit mode.

The destination register must not be the same as the memory source if the operation is intended to be atomic, although this instruction is not atomic. Using the same register for both a source and destination (destructive operand) is permitted. Memory operands MUST be aligned to the requirements of the specific AVX-512 subset being used to avoid alignment check exceptions.