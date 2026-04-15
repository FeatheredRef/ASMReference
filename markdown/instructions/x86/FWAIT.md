> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FWAIT

Waits for the floating-point entity to be idle. If the operand is a memory location, it waits until the floating-point entity is idle and then reads the word at that location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| None | None |
| m2 | None |

DO NOT support LOCK

FWAIT is supported in 64-bit mode, though it is primarily used to synchronize the x87 FPU with the processor. In modern x86-64 implementations, the FPU is typically integrated and synchronized, making the instruction's effect negligible unless dealing with specific legacy hardware or external FPU coprocessors.

If the operand is m2, the processor must perform a memory read. This may trigger a page fault if the memory region is not present or accessible. Failure to synchronize the FPU before executing floating-point instructions on certain legacy platforms may lead to the execution of subsequent instructions before the FPU has completed previous operations.