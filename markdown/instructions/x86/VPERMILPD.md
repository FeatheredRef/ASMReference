> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMILPD

Permutes two double-precision floating-point values from two sources according to the immediate operand. The instruction selects one f64 value from each of the two source operands and places them into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg / m128 | xmm reg |

DO NOT support LOCK

This instruction SHALL be used in 64-bit mode or 32-bit mode. It is NOT available in compatibility mode if the CPU does not support the AVX instruction set.

The immediate operand MUST be a valid 2-bit index to specify the permutation pattern. If the immediate is not 0, 1, 2, or 3, the behavior is undefined. To avoid unexpected results, ensure the immediate value corresponds to the desired source selection:
- 00b: [f64_0 from src1, f64_0 from src2]
- 01b: [f64_0 from src1, f64_1 from src2]
- 10b: [f64_1 from src1, f64_0 from src2]
- 11b: [f64_1 from src1, f64_1 from src2]