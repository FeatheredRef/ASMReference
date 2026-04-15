> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LODSB

Loads a byte from the memory location pointed to by the address in the RSI register into the AL register. The value of RSI is then incremented or decremented by 1, depending on the Direction Flag (DF).

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| m1 | r8 |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. It is restricted to using the RSI (or ESI/SI) register as the source pointer and the AL register as the destination.

The behavior of the pointer increment/decrement is determined by the DF flag: if DF=0, RSI is incremented; if DF=1, RSI is decremented. Failure to correctly set the DF flag prior to execution SHALL result in the pointer moving in the unintended direction, leading to incorrect memory access.