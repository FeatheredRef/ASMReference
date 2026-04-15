> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[SENTER]

Initiates a Measured Launch Environment (MLE) by invoking the Authenticated Code Module (ACM). It triggers the hardware to verify the ACM signature and establishes a trusted execution environment by performing a series of security checks and clearing specific system states.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction SHALL only be executed in CPL0. It is only available in 64-bit mode. Execution of `GETSEC[SENTER]` requires the processor to support Intel TXT (Trusted Execution Technology) and requires the `SENTER` bit to be enabled in the corresponding configuration registers.

The `GETSEC[SENTER]` instruction SHALL fail if the processor is in a virtualized state (guest mode); it MUST be executed by the VMM or the host. To avoid a system shutdown or a General Protection fault (#GP), the user SHALL ensure that the SINIT ACM is correctly loaded into memory and that the memory region containing the ACM is properly aligned. Failure to provide a valid ACM will result in the instruction failing to launch the MLE.