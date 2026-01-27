# Design
The `machine` and all its components are *stateless*, they are only used as data holding structures.
E.G. : The `ALU` components describes the ALU speed and width (more to come)

Each component has a `run_tasks` method, which gets called recursively from the machine to the subcomponents.
this method returns the state of the machine (purely for tracing purposes, not used) until the next event, and the time until that event

schedules goes from event to event until there are no more tasks

# Problems
## Cache
- Instruction cache may not be that important for now, as most testing programs will not be able to overload it (so it is only relevant on context switches)
## Temperature
- Not important, cloud centers are *very* well cooled
## "Remaining" time on tasks
- when two tasks are launched and the first one finishes, how long remains on the first one
- could read the state on `run_tasks` and update the actual tasks (from main/schedule, aka persistent state)

# TODO

- explain every design choice
- Entrelacement entre differentes UF
- Frequence CPU/RAM (+ desyncs)
- Quantum scheduler
- Trace
- Cache
  - Ajouter taille des taches (pour cache instructions)
  - Modeliser cache (v1 : distance moyenne entre ldr -> ctx switch entre chaque tache)
- Launching multiple tasks at once
