# Hyper threading

[OoO] is a technique used to run multiple instructions in parallel, even in a synchronous routine. Most of the workloads cannot use the execution ports that it would exploit, due to data-dependency for example. 

With the headroom, running N threads in a single core became possible, doing so is called hyper threading. This ensures that execution ports are being used the majority of time. And even while it allows practical parallelism, since a thread may have to wait for execution ports, the correct terminology is concurrency.

In conclusion, as [OoO] mostly don't use all of a core execution ports, hyper threading is a feature that allows the usage of existing resources. It yielding parallelism, most of the time.
