> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SYSENTER

SYSENTER transfers execution to the system entry point specified by the MSR_SYSENTER_CS, MSR_SYSENTER_EIP, and MSR_SYSENTER_ESP Model Specific Registers. It switches the processor to the privilege level specified in MSR_SYSENTER_CS and sets the instruction pointer to the value in MSR_SYSENTER_EIP.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

In x86-64 architecture, SYSENTER is ONLY available in compatibility mode. It SHALL NOT be used in 64-bit mode; the SYSCALL instruction MUST be used instead for fast system calls in 64-bit mode.

The instruction DOES NOT save the return address (RIP) or the current stack pointer (RSP) to the stack or a register. The software MUST implement a mechanism to save the return address if it is required. Because it relies on Model Specific Registers, the OS MUST properly initialize MSR_SYSENTER_CS, MSR_SYSENTER_EIP, and MSR_SYSENTER_ESP before the instruction is executed to avoid unpredictable behavior or processor faults.