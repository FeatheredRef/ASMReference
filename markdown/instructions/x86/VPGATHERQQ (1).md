> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERQQ (1)

Gathers 64-bit signed integers from memory using 64-bit signed indices. For each element in the index register, the instruction calculates a memory address by adding the scaled index to a base address, fetches the 64-bit value, and stores it in the destination register. A mask register controls which elements are gathered; if a mask bit is cleared, the corresponding element in the destination register remains unchanged.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, m64 | reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode. It requires support for the AVX-512 foundation and the AVX-512BW extension.

To avoid unexpected behavior, the user MUST ensure that the mask register is properly initialized, as the instruction only updates elements where the mask bit is set to 1. If a memory access faults for a masked-out element, the fault SHALL NOT be reported. Memory accesses MUST be aligned to the requirements of the data type to avoid performance degradation, although the instruction supports unaligned accesses. All indices MUST be within the valid addressable range to prevent general protection faults.