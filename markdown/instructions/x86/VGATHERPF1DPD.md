> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF1DPD

Gathers 64-bit floating-point values from memory using a base address, an index vector, and a scale factor. The instruction uses a mask register to track which elements have been processed; if a mask bit is set, the corresponding element is fetched from the calculated address and the mask bit is cleared. The process continues until all mask bits are 0 or a maximum number of iterations is reached.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, m64 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX2 support.

To avoid infinite loops or performance degradation, the programmer MUST ensure that the mask register is correctly initialized. If the mask register is not updated due to a fault or an external interrupt, the instruction will resume from the last uncleared mask bit upon return from the exception handler. Faults occurring during the gather operation are reported as standard memory faults, but the state of the mask register allows the processor to recover the exact element that caused the fault.