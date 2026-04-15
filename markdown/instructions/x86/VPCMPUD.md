> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPUD

Compares unsigned doubleword integers in the source operands according to a specified condition and writes the results to the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm reg | zmm/ymm/xmm reg |
| zmm/ymm/xmm reg | m16/m32/m64 |
| m16/m32/m64 | zmm/ymm/xmm reg |
| imm | zmm/ymm/xmm reg |
| imm | m16/m32/m64 |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode. It is not supported in compatibility mode. It requires a processor supporting the AVX-512 extension set (specifically AVX-512F).

When using a mask register (k-register), elements not selected by the mask are either zeroed or left unchanged based on the masking bit (z or merge). Failure to specify the correct masking behavior may lead to unintentional data corruption in the destination register. Memory operands MUST be aligned to the vector size to avoid performance penalties or general protection faults depending on the alignment check settings.