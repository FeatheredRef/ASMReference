> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERDPS (1)

Gathers 32-bit floating-point values from memory using a base address and a set of indices. For each element, the instruction calculates the effective address by adding the base address to the index scaled by 4, then loads the 32-bit value into the destination register.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction SHALL only be executed on processors that support the AVX2 instruction set. It requires the use of YMM registers.

To avoid undefined behavior and potential faults, the programmer MUST ensure that all indices provided in the index register are within the valid bounds of the allocated memory region. Because the instruction is masked, elements with a mask bit set to 0 are skipped; however, if a fault occurs on an element with a mask bit set to 1, the instruction SHALL trigger a general-protection exception or a page fault. To prevent performance degradation and ensure correct operation, the programmer SHOULD verify that the memory addresses do not cross page boundaries unexpectedly. Elements that have been successfully gathered are marked by clearing the corresponding bit in the mask register.