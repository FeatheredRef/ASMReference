> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRMSR

Writes to a model-specific register (MSR). The instruction loads a 64-bit value from the ECX and EDX registers into the MSR specified by the value in ECX.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 (EDX:ECX) | MSR (ECX) |
| imm | #I |
| mN | #I |

DO NOT support LOCK

The instruction SHALL be executed only in CPL 0. Execution by a program at a higher privilege level SHALL trigger a general-protection exception (#GP). WRMSR is supported in 64-bit mode and compatibility mode.

The MSR index is specified in ECX. The value to be written is loaded from EDX (upper 32 bits) and ECX (lower 32 bits). Note that ECX is used both as the register specifying the MSR and as the source for the lower 32 bits of the data.

Attempts to write to a reserved or non-existent MSR SHALL trigger a general-protection exception (#GP). Some MSRs are read-only; attempting to write to them MAY result in a #GP or be silently ignored depending on the processor implementation.