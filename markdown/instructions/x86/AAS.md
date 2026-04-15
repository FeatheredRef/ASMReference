> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AAS

ASCII Adjust AL after Subtraction subtracts 6 from AL if the Auxiliary Carry Flag (AF) is set, and then subtracts 6 from AL if the Carry Flag (CF) is set. If the result in AL is less than 30h, the Carry Flag (CF) is set to 1 and AL is set to 30h through 39h (representing ASCII digits); otherwise, CF is cleared.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| implicit | r8 (AL) |

DO NOT support LOCK

This instruction is only available in 32-bit operand size mode. In 64-bit mode, it is only supported when the operand size is overridden to 32-bit or when executing in compatibility mode.

The instruction operates exclusively on the AL register. Users MUST ensure that the result of the subtraction preceding AAS is stored in AL to achieve the intended ASCII adjustment. Failure to use the AL register will result in an incorrect adjustment as no other registers are targeted.