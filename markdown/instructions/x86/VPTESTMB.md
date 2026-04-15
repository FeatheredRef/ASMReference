> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTMB

Performs a bitwise logical AND operation between two byte masks and sets the ZF flag in the EFLAGS register based on the result. If the result of the logical AND is non-zero, ZF is cleared; otherwise, ZF is set.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation and the AVX-512BW instruction set extensions to be enabled.

The destination register is overwritten by the result of the logical AND operation. Users MUST ensure that the mask registers used are correctly initialized, as the instruction operates on the byte-level granularity of the mask. Failure to account for the mask length (k-register width) may lead to unintended results in the ZF flag calculation.