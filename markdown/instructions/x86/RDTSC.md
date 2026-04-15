> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDTSC

Reads the time-stamp counter (TSC), which is a 64-bit register that increments every clock cycle. The instruction loads the current value of the TSC into the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | edx: eax |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and compatibility mode. Execution is controlled by the `TSD` (Time Stamp Disable) flag in the `CR4` register; if `CR4.TSD` is set, executing `RDTSC` in CPL 3 SHALL trigger a General Protection exception (#GP).

To avoid issues with instruction reordering, `RDTSC` SHOULD be used in conjunction with `LFENCE` or `CPUID` to ensure that all previous instructions have completed before the timestamp is read. In multi-processor environments, the TSC may not be synchronized across different cores unless the `Invariant TSC` feature is supported and enabled by the BIOS/OS. When using `RDTSC` for high-precision timing, the user MUST be aware that the value returned is the number of cycles since reset, not necessarily wall-clock time.