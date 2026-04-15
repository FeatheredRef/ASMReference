> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SWAPGS

Exchanges the current value of the `GS` base register with the value stored in the `KernelGSBase` MSR.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| Internal MSR | GS Base |

DO NOT support LOCK

The `SWAPGS` instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The instruction is intended for use by the operating system to switch between user-mode and kernel-mode `GS` segments. If `SWAPGS` is executed while the processor is already using the kernel `GS` base, it will swap it back to the user `GS` base. Failure to properly track the state of the `GS` base across privilege level transitions SHALL result in the processor accessing incorrect memory regions, potentially leading to a General Protection exception (#GP) or memory corruption.