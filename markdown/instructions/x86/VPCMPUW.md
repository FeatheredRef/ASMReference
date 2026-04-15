> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPUW

Compares two unsigned word-size integer vectors according to a specified condition and sets the corresponding bits in the destination vector to all-ones (if the condition is true) or all-zeros (if the condition is false).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm/ymm reg | zmm/ymm reg |
| m16/m32/m64 | zmm/ymm reg |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It REQUIRES AVX-512 support (specifically AVX-512F).

The destination register MUST NOT be the same as the second source register if masking is not used, to avoid potential data hazards in specific microarchitectures, although the ISA generally allows it. When using opmask registers (k-registers), elements that are masked out will either be zeroed or left unchanged based on the masking write-masking (z or m) setting. Failure to specify the correct masking behavior MAY result in unexpected values in the destination vector.