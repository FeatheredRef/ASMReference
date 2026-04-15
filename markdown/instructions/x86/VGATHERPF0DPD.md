> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF0DPD

Gathers 64-bit floating-point values from non-contiguous memory locations into a destination vector register. It uses a base address, a vector of indices, and a scale factor to calculate the effective addresses. A mask register is used to track which elements have been successfully loaded; elements with a mask bit set to 0 are skipped, and elements successfully loaded are cleared in the mask.

The following table describes the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It REQUIRES support for the AVX-512 foundation instructions.

The instruction is fault-tolerant; if a memory access triggers a page fault or general protection fault, the processor SHALL save the state in the mask register and allow the exception handler to resolve the issue. Upon returning from the exception, the instruction resumes from the first element that failed to load. To avoid infinite loops or repeated faults, the programmer MUST ensure the mask register is correctly updated or checked. Alignment of the base address is NOT required, but unaligned accesses may incur performance penalties.