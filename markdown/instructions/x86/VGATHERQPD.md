> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERQPD

Gathers 64-bit floating-point values from memory into a destination register using a base address, a set of indices from a register, and a scale factor. The instruction loads values from addresses calculated as `base + (index * scale)` for each active element in the mask register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, m64 | reg |

DO NOT support LOCK

The instruction SHALL be executed in 64-bit mode. It REQUIRES support for the AVX2 instruction set. The mask register (k-register or zmm register depending on the specific encoding/extension) MUST be initialized; elements with a mask bit set to 0 are skipped, and their corresponding destination elements remain unchanged.

To avoid performance degradation or faults, the user SHALL ensure that all calculated effective addresses are properly aligned to the architectural requirements of the system. If a page fault occurs during the gathering process, the instruction SHALL be restartable, and the mask register SHALL be updated to reflect which elements have already been successfully loaded to prevent redundant memory accesses.