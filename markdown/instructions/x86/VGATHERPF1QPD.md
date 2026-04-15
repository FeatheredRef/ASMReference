> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF1QPD

Gathers 64-bit floating-point values from memory using a base address, a vector of indices, and a scale factor. The instruction reads a mask register to determine which elements are to be gathered; elements with a mask bit set to 0 are skipped, and their corresponding destination elements remain unchanged. Once a value is successfully gathered, the corresponding mask bit is cleared to 0.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m64 | zmm / ymm |
| zmm / ymm | zmm / ymm |
| imm | zmm / ymm |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT supported in compatibility mode.

Faults occurring during the gather operation are handled such that the instruction is restartable. If a page fault occurs, the mask register reflects which elements have already been successfully loaded, allowing the instruction to resume from the point of failure without duplicating loads.

The index vector must be properly aligned to the scale factor provided. Accessing memory that crosses a page boundary may result in performance degradation or specific fault behavior depending on the microarchitecture. The base address must be a 64-bit register. All indices must be within the range of the addressable memory space to avoid general protection faults.