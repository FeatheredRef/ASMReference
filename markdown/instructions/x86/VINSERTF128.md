> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTF128

Inserts 128 bits from a source operand into a destination XMM or YMM register. The insertion is controlled by an immediate value that determines whether the low 128 bits or the high 128 bits of the destination register are replaced.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | ymm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the AVX CPUID feature flag. When the destination is an xmm register, the immediate value SHALL be 0; if the immediate is 1, the behavior is undefined or may trigger an exception depending on the processor implementation.

To avoid unintended data corruption, ensure that the immediate value corresponds to the destination register size: use `imm8` 0 for the lower 128 bits and `imm8` 1 for the upper 128 bits (only applicable when the destination is a ymm register). Using an immediate other than 0 or 1 is invalid.