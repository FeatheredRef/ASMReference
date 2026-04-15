> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BEXTR

Extracts a contiguous range of bits from the source operand, starting at the index specified by the control operand, and writes the result to the destination operand. The destination is zero-extended to the size of the operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

BEXTR is available only in 64-bit mode or 32-bit mode (via BMI1). It is NOT supported in 16-bit compatibility mode.

The control operand (reg or mN) MUST provide the start index in bits [7:0] and the length in bits [15:8]. If the start index or the length exceeds the size of the source operand, the instruction SHALL return 0. The result is always zero-extended, meaning bits higher than the extracted length are cleared.