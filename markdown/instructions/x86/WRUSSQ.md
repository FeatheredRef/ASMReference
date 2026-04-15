> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRUSSQ

Writes the current value of the supervisor-mode execution protection (SMEP) and supervisor-mode access prevention (SMAP) bits into the specified destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal State | reg |
| Internal State | #I |
| Internal State | mN |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be a 64-bit general-purpose register to correctly capture the state of the control register bits. Using a smaller register size may result in the truncation of the returned value.