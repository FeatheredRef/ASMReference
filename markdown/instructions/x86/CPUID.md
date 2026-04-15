> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CPUID

CPUID queries processor identification and feature information. The instruction returns a series of leaf-specific values into the EAX, EBX, ECX, and EDX registers based on the value provided in EAX.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm (via EAX) | r32 (EAX, EBX, ECX, EDX) |
| #I | mN |

DO NOT support LOCK

The instruction is available in both 32-bit and 64-bit modes. In 64-bit mode, the instruction returns 32-bit values into the lower half of the destination registers; the upper 32 bits of the 64-bit registers are not modified.

The behavior of CPUID is dependent on the value loaded into EAX. If the requested leaf is unsupported by the processor, the processor SHALL return the highest supported leaf index or a value of 0, depending on the specific leaf range. Users MUST check the maximum input value returned by leaf 0 to avoid undefined behavior or incorrect feature detection. In virtualized environments, the hypervisor MAY intercept this instruction and synthesize the returned values to hide or emulate specific processor features.