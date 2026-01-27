# Design
The `machine` and all its components are *stateless*, they are only used as data holding structures.
E.G. : The `ALU` components describes the ALU speed and width (more to come)

Each component has a `run_tasks` method, which gets called from the machine to the subcomponents.
this method returns the state of the machine (purely for tracing purposes, not used) until the next event, and the time until that event

schedules goes from event to event until there are no more tasks

# Problems
## Cache

## Temperature

## "Remaining" time on tasks
- when two tasks are launched and the first one finishes, how long remains on the first one
- could read the state on `run_tasks` and update the actual tasks (from main/schedule, aka persistent state)
