> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERQPS

Gathers packed single-precision floating-point values from non-contiguous memory locations into a destination register. It uses a base address and a set of indices provided in a register to calculate the effective addresses of the elements to be loaded.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32/m64 | zmm/ymm/xmm |
| reg | zmm/ymm/xmm |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512F). If the index register contains negative values, the resulting effective addresses are calculated by adding the signed offset to the base address.

Masking is REQUIRED for this instruction; it utilizes an opmask register (kN). Elements where the corresponding mask bit is 0 are not loaded, and their corresponding destination elements remain unchanged. The mask is updated during execution: as each element is successfully loaded, the corresponding mask bit is cleared to 0. If a fault occurs, the mask bits for the successfully loaded elements are updated, while the bit for the element that caused the fault and all subsequent bits remain 1. This prevents the instruction from triggering multiple faults for the same memory access upon retry.

The base address MUST be a valid pointer. To avoid undefined behavior or segmentation faults, the user MUST ensure that `base + (index * scale)` remains within the allocated memory boundaries for all active mask bits. Memory alignment is not strictly enforced for individual elements, but unaligned accesses may incur a performance penalty.