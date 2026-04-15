> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBLENDMW

Selects elements from two source operands based on a mask provided by an immediate value and blends them into a destination operand. For each 16-bit word, if the corresponding bit in the mask is set to 1, the element from the second source operand is selected; otherwise, the element from the first source operand is selected.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m16 |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It requires the YMM register state; therefore, it is only available on processors supporting the AVX instruction set.

To avoid undefined behavior or general protection faults, ensure that the destination memory operand is aligned to the required boundary for the specific vector length. Failure to align memory operands may result in performance degradation or exceptions depending on the processor's alignment check settings.