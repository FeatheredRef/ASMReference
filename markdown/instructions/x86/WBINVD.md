> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WBINVD

Writes back and invalidates all caches in the processor.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

WBINVD is a privileged instruction and SHALL only be executed in CPL0. Execution of this instruction in a lower privilege level SHALL trigger a General Protection exception (#GP). It is supported in 64-bit mode, 32-bit mode, and compatibility mode.

The instruction is not a serializing instruction. In multi-processor environments, WBINVD only affects the caches of the local processor; it DOES NOT broadcast the invalidation to other processors in the system. To ensure global cache coherency across multiple cores, software MUST execute WBINVD on each logical processor, typically via an Inter-Processor Interrupt (IPI). Failure to do so may result in stale data remaining in the caches of other cores.