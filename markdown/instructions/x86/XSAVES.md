> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XSAVES

Saves the current state of extended processor states, including those enabled by the XCR0 register, to a save area. It uses a compacted format to optimize memory usage by omitting components that are in their initial state.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal State | m64 |

DO NOT support LOCK

The instruction SHALL only be executed in CPL 0. It is NOT supported in compatibility mode. The operation requires the `XSAVE` feature to be enabled in the processor and the corresponding state components to be enabled in XCR0.

The save area MUST be aligned to a 64-byte boundary; otherwise, a general-protection exception (#GP) SHALL be generated. The memory region m64 refers to the base address of the save area, which is typically pointed to by a register. Failure to ensure proper alignment or providing an invalid memory address SHALL result in a #GP.