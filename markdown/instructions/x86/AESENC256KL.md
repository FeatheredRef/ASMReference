> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESENC256KL

Performs one round of AES encryption using the 256-bit key variant. The instruction operates on the destination operand by performing a ShiftRows operation, a SubBytes operation, and an AddRoundKey operation using the source operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

The instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX and AES-NI instruction set extensions to be enabled in the CPUID.

To avoid undefined behavior or General Protection faults, the destination and source registers MUST be of the same vector length. Using mismatched register sizes (e.g., mixing XMM and YMM) is NOT supported.