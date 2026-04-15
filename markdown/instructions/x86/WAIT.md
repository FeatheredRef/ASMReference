> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WAIT

The instruction causes the processor to wait until the FPU `WAIT` state is cleared. The `WAIT` state is set by the `FSAVE` and `FINIT` instructions and cleared by the `FWAIT` instruction or when the FPU has completed a previous operation.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The `WAIT` instruction is primarily used in legacy x87 floating-point operations. In x86-64 mode, this instruction is supported for compatibility with 32-bit applications and legacy x87 code.

To avoid synchronization issues or unnecessary CPU stalls, the programmer SHOULD ensure that `WAIT` is only used when an explicit `FWAIT` or the completion of an asynchronous FPU operation is required. Failure to properly manage the FPU state can lead to performance degradation or incorrect timing in legacy floating-point calculations.