# Simularyr

Simularyr is a cloud platform simulator aiming to re create interference between task within a cloud node. The main idea of the simualtor was to avoid a line per line static analysis and to focus on blocks of instructions.

# Usage

To launch a simulation, simply run `cargo run [machine_desc_file] [tasks_desc_file]`. The description files are json files, you can find multiple simple examples in the examples directory.

## Machine Description

A machine contains 3 main elements : 

+ One or more core
+ A L3 Cache
+ A RAM

Here is a list of all available parameters for the core :  
(You can omit parameters in _italic_)

| Name     | Description     |
| ------------- | ------------- |
| id | The unique identifier of the core |
| _alu_ops_per_cycle_ | The number of operations the alu can make (default 1)  |
| alu_count | The number of alu in the core |
| fpu_op_duration | The number of cycle to compute a single floating operation |
| fpu_count | The number of fpu (Float Processing Unit) in the core |
| _mem_bus_count_ | The number of bus accessing the memory, we consider it's the number of concurent memory access allowed (default 1) |
| l1_cache_access_duration | The number of cycle required to access the L1 cache |
| l1_cache_size | The size in bit of the L1 cache |
| l1_block_size | The size of the blocks in the L1 cache |
| l2_cache_access_duration | The number of cycle required to access the L2 cache |
| l2_cache_size | The size in bit of the L2 cache |
| l2_block_size | The size of the blocks in the L2 cache |

Parameters for the RAM : 

| Name  | Description |
|---  | --- |
| access_duration | The number of cycle required to perform a ram access |

Parameters for the L3 Cache : 

| Name  | Description |
|---  | --- |
| l3_cache_access_duration | The number of cycle required to access the L3 cache |
| l3_cache_size | The size in bit of the L3 cache |
| l3_block_size | The size of the blocks in the L3 cache |

## Task Description

(You can omit parameters in _italic_)

| Name  | Description |
|---  | --- |
| id | The unique identifier of the task |
| *start_time* | Sets a rigid timestamp before which a start cannot start (default 0) |
| *dependancies* | A list of task ids preceding the task. The present task will not start before the other ended (default no dependancies) |
| *mem_op_count* | The total amount of memory accesses made by the task (default 0) |
| *alu_op_count* | The total amount of alu operations made by the task (default 0) |
| *fpu_op_count* | The total amount of fpu operations made by the task (default 0) |
| cache_miss | A float between 0 and 1 indicating the amount of cache misses, determines the amount of operation going to the cache and the rest is going to the RAM |
| l1_cache_miss |  A float between 0 and 1 indicating the number of l1 cache accesses that will miss based on the number of cache accesses (and not total memory accesses) |
| l2_cache_miss | A float between 0 and 1 indicating the number of l2 accesses that will miss, based on the number of failed l1 cache accesses |



# The model

After the `cargo run` call, main.rs calls the parser (parse.rs) and then sends the tasks to the scheduler (sched.rs). The scheduler is managing most of the simulation. The scheduler checks which task is available to run, accounting for start times and dependancies, then ask the simulated platform (machine.rs) how much time does the task take. Then the scheduler comes back, jump in time if needed and schedules the remaining tasks. Each task run on a single core. The machine is the one computing the interferences on the bottlenecks induced by shared components. For example if two tasks need to use the RAM a lot, the machine will chose a task that will use the RAM first and then the other one will be delayed, creating lateness.

For the moment, the cache model is only a stochastic one, based only on the cache miss rates which force us to know this rate before simulating, an information that may not be known. Also this model is not able to adjust when changing parameters such as the cache size. In real life, we expect a changing cache size to impact the result but here it will not since the probability is coded in the task description.

We consider that inside a task, reordering is fully possible and allows perfect parallelism. this choise allows to simplify a lot of computations and if a dependancies is needed it can be simulated by splitting the task in two.

Consistency tests and basic stabilty tests can be found in test.rs and ran with `cargo test`.

This model will be evaluated and compared with [real life experiments](https://github.com/RPretet/ProjetLongRYRtempsReel) to ensure the validity and the usefulness of the simulator.