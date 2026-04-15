> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDTSCP

Reads the time-stamp counter (TSC) and writes the 64-bit value into destination registers. It also writes the processor ID (IA32_TSC_AUX) into a destination register. This instruction acts as a fence to ensure all previous instructions in the pipeline have executed before the TSC is read.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | r64, r64 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It is NOT available in 32-bit mode (where `RDTSC` is used). It SHALL be used to obtain the TSC value while ensuring that no previous instructions have been reordered across the instruction.

To avoid timing inaccuracies, the programmer MUST ensure that the `IA32_TSC_AUX` register is correctly initialized, as the value returned in the second destination register depends on the contents of this MSR. Because `RDTSCP` is a serializing instruction for all previous instructions, it may incur a performance penalty compared to `RDTSC`.