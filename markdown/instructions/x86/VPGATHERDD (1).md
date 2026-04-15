> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERDD (1)

VPGATHERDD retrieves dword values from memory using a base address and a vector of 32-bit signed indices. For each element in the index vector, the instruction calculates an effective address by adding the scaled index to the base address, then loads the dword from that address into the destination vector. A mask register tracks which elements are processed; only elements with a corresponding mask bit set to 1 are gathered, and the mask bit is cleared upon a successful load.

The following table specifies the supported source and destination operands.

| Source | Destination(s) |
| :--- | :--- |
| reg, m8 | reg |
| reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES AVX2 and AVX512 support depending on the specific vector register width used.

The instruction DOES NOT guarantee atomicity for the individual memory accesses. If a fault occurs during the gathering of one element, the mask register is updated to reflect which elements were successfully loaded before the fault occurred, allowing the operating system to resume execution after the exception handler returns. Users SHOULD be aware that memory accesses are NOT guaranteed to occur in any specific order.