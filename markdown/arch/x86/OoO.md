# Out of Order

Focusing on Intel for this text, OoO started as a way of bypassing a "brick-wall" of optimization in CPU performance. Before this, CPU was fully synchronous, the N+1 instruction had to wait the N instruction to finish before running, and branches were catastrophic as it used to flush away the pipeline.

For the increasing their performance, Intel created the P6 Architecture. This one implemented a larger [L-Cache]. L2 providing 256KB, 512KB, or 1-MB internal RAM that runs at full clock within the CPU.

With the upgrade, they made a Super-bus, allowing techniques that OoO uses. Which are deep branch prediction, dynamic data flow analysis, and speculative execution.

> Observation: I mentioned specific techniques, the ones I found on the Intel's reference.

## Deep branch prediction

This technique analyse the execution, determining which will be the after-branch instructions. Which allow the CPU to flush away the pipeline less, keeping it full, thus spending more time doing relevant work, rather than flushing and pulling the instructions.

You can optimize your code in this regard by using less branches, which can be done by merging branches, using [cmov], bitwise techniques, or trying to make the flow statistically easy to predict.

## Dynamic data flow analysis

As the code runs, for the CPU not have to wait the N instruction so N+1 can run, it uses dynamic data flow analysis to make N and N+1 running asynchronously possible.

The specific details of how it is done vary by micro-architecture, but in general, CPUs verify if there is an inter dependence in the registers being used. If not, the CPU runs both in parallel.

```asm
; Intel syntax
mov rax, 0
mov rdi, 1
mov rcx, 2
mov rsi, 3

add rax,rsi ; OoO
add rdi,rcx ; OoO

add rax, rsi ; 
add rsi, rax ; This N+1 depend on the N instruction result, no OoO
```

## Speculative execution

This is paired with the aforementioned features, it analyses the flow, depending on CPU-specific details, it will load the pipeline upfront with the after-branch instructions and run.

The feature is akin to deep-branch prediction, as it does the same behavior described in deep branch prediction to unresolved conditional branches. But the difference is that the deep branch prediction covers switch statements, "goto", conditional statements. This is conditional-specific.
