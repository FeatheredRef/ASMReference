> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPLZCNTQ

Counts the number of leading zeros in a quadword element of a SIMD register. If the input is zero, the result is the operand size (64).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It requires the AVX-512 Foundation (AVX512F) instruction set extensions to be enabled.

The instruction's behavior is dependent on the LZCNT feature flag. If the processor does not support LZCNT, this instruction MAY result in an `#I` (Invalid Operation) or exhibit undefined behavior. When executing in a context where the LZCNT bit is not set in the CPUID, the instruction MUST NOT be used. Ensure that the target hardware supports AVX-512F to avoid illegal instruction exceptions.