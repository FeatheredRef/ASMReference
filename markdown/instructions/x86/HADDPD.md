> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# HADDPD

Adds adjacent pairs of 64-bit floating-point numbers from two XMM operands and stores the results in the destination XMM register. Specifically, it adds the low 64 bits of the first operand to the low 64 bits of the second operand, and the high 64 bits of the first operand to the high 64 bits of the second operand. Subsequently, it adds the resulting low 64-bit sum to the resulting high 64-bit sum, storing the final scalar result in both the low and high 64 bits of the destination register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, m128 | reg |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It requires the SSE3 instruction set extension.

The instruction performs addition based on the current MXCSR rounding mode and precision control. If an exception occurs during the intermediate additions, the corresponding floating-point exception flags (#P, #U, #Z, #O, #D) SHALL be set in the MXCSR register. Using a memory operand for the source SHALL incur a memory access penalty. Improper alignment of the m128 operand MAY result in performance degradation or alignment checks if enabled.