> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERDPD (1)

Gathers 64-bit floating-point values from memory using a base address and a vector of 32-bit signed indices. For each element in the index vector, the instruction calculates the address as the base plus the index scaled by 8, fetches the 64-bit value, and stores it in the destination vector. A mask register tracks which elements are processed; elements with a mask bit set to 0 are skipped and their corresponding destination elements remain unchanged.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It requires the AVX extension.

The mask register is updated during execution; elements that have been successfully gathered have their corresponding mask bit cleared (set to 0). If a memory fault occurs during the gathering process, the instruction SHALL save the state of the mask register, allowing the handler to resume the operation without reprocessing already gathered elements. All index values SHALL be treated as i32 and scaled by 8 bytes. Failure to initialize the mask register prior to execution MAY result in undefined behavior regarding which elements are gathered.