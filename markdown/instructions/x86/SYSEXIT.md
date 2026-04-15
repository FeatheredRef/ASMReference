> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SYSEXIT

Exits the system call state by loading the `RIP` from `RCX` and the `RFLAGS` from `RDX`, then returns execution to the CPL specified in the `STAR` MSR.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | #I |

DO NOT support LOCK

SYSEXIT is ONLY available in 64-bit mode. It SHALL NOT be used in compatibility mode. The instruction requires that the processor is currently executing at CPL 0.

To avoid an invalid state or general protection fault, the `STAR` MSR MUST be correctly initialized. If the processor is in 64-bit mode, SYSEXIT will transition the processor to the target CPL (typically CPL 3) and load the segment selectors for `CS` and `SS` from the `STAR` MSR. If `RCX` or `RDX` contain invalid addresses or flag combinations, the behavior may result in immediate exceptions upon returning to the target privilege level.