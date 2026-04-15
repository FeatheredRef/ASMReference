> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PAUSE

Provides a hint to the processor that the code is currently executing in a spin-wait loop. This allows the processor to reduce power consumption and improve performance by delaying the execution of the next instruction.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The PAUSE instruction does not have any architectural constraints regarding operating modes; it is supported in 64-bit mode and compatibility mode. It does not affect any flags or registers.

To avoid performance degradation in spin-locks, the instruction SHOULD be placed within the loop that monitors a memory location. Failure to use PAUSE in a tight spin-loop MAY cause the processor to speculate a large number of iterations, leading to a pipeline flush and increased latency when the lock is finally released.