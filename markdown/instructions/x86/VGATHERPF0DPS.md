> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF0DPS

Gathers 32-bit single-precision floating-point values from memory using a base address and a set of indices. For each index in the index register, the instruction calculates an effective address by adding the scaled index to the base address. It then loads the 32-bit floating-point value from that address into the destination register. A mask register is used to track which elements have been successfully loaded; bits in the mask are cleared upon a successful load.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | f128/f256/f512 |
| reg | f128/f256/f512 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires a masking register (k-register) to manage the gather process; if the mask bit for a specific element is 0, that element is skipped.

To avoid undefined behavior or segmentation faults, the programmer MUST ensure that the calculated effective addresses are within the allocated memory bounds. If a memory fault occurs during the gather operation, the instruction will trigger a general protection fault or page fault, and the mask register will reflect which elements were processed before the fault. The instruction is not atomic; if an interrupt occurs, the mask register allows the instruction to resume from where it left off.