> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XSAVEOPT

XSAVEOPT saves the current state of the processor's extended registers to a memory location. It optimizes the save operation by not writing components of the state if the corresponding register contents have not changed since the last save or if they are in their initial state, based on the internal tracking of the processor.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | mN |
| imm | mN |

DO NOT support LOCK

XSAVEOPT SHALL only be executed in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode. The instruction REQUIRES the `Xsave` feature to be enabled in the processor via `CR4.OSXSAVE`. If `CR4.OSXSAVE` is 0, execution of XSAVEOPT SHALL result in an #UD (Undefined Instruction) exception.

The memory region mN SHALL be aligned on a 64-byte boundary. If the destination address is not 64-byte aligned, a #GP (General Protection) exception SHALL occur. Users SHOULD ensure the memory area is sufficiently large to hold the state specified by the mask to avoid #GP exceptions. XSAVEOPT behaves similarly to XSAVE but may skip writes to memory to improve performance; however, it DOES NOT provide the same deterministic "compressed" format as XSAVEC.