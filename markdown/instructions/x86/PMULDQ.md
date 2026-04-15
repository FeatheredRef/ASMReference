> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULDQ

Multiplies two signed 64-bit integers packed into 128-bit XMM registers. The instruction multiplies the lower 64 bits of the first operand by the lower 64 bits of the second operand, and the upper 64 bits of the first operand by the upper 64 bits of the second operand. The lower 64 bits of each result are stored in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It is NOT supported in 32-bit mode.

The instruction performs a signed multiplication; therefore, operands SHALL be treated as i64. Since only the lower 64 bits of the 128-bit product are retained for each lane, the operation is mathematically equivalent to a modulo $2^{64}$ operation, avoiding numeric overflow exceptions. Failure to ensure the operands are properly signed may result in incorrect numerical output.