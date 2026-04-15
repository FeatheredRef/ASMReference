> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESDECWIDE256KL

Performs the AES decrypt operation on a 256-bit state using a 128-bit round key. It processes two 128-bit blocks of data simultaneously (wide) and updates the destination register with the resulting ciphertext.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm register | xmm/ymm register |
| m16 | xmm/ymm register |

DO NOT support LOCK

This instruction SHALL only be executed on processors that support the AVX-512 AES extensions. It REQUIRES the processor to be in 64-bit mode or compatibility mode; execution in 32-bit mode MAY result in an invalid opcode exception unless the specific CPUID feature flag is supported and enabled.

To avoid undefined behavior or application crashes, ensure that the destination and source registers are properly aligned to 16-byte or 32-byte boundaries when using memory operands. Failure to align memory access MAY result in general protection faults or significant performance degradation. Ensure that the YMM registers are used when targeting the 256-bit wide operation to prevent truncation of data.