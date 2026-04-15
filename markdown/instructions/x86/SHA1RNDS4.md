> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA1RNDS4

Performs the SHA-1 round operations on the provided operands. It calculates the SHA-1 message schedule and state update for four rounds, updating the state registers based on the SHA-1 algorithm specifications.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode and compatibility mode. It requires the processor to support the SHA extensions.

The instruction operates specifically on XMM registers. Using this instruction on a processor that does not support the SHA extension will result in an invalid opcode exception. Ensure the CPUID leaf `07h.1:ED` bit is set before execution.