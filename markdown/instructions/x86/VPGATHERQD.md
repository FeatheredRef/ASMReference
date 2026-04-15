> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERQD

Gathers 64-bit signed integer values from memory using a base address and a set of 64-bit signed indices. For each index, the instruction calculates the effective address by adding the index (scaled by the size of the element) to the base address and loads the resulting qword into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT supported in compatibility mode.

The mask register is updated during execution; bits are cleared as elements are successfully loaded. If a mask bit is 0, the corresponding element is not loaded. If a page fault or memory violation occurs, the instruction SHALL save the state of the mask and the current index, allowing the instruction to be resumed after the exception is handled.

To avoid performance degradation or incorrect behavior, ensure that the mask register is properly initialized. If the mask register is not managed, the instruction may attempt to load from invalid memory addresses. Use of this instruction requires AVX2 or AVX-512 support depending on the specific operand size and register set used.