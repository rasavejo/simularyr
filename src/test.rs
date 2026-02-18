#[cfg(test)]
mod tests {
    use std::fs;

    use crate::simulate;

    #[test]
    fn all_examples() {
        let tasks = "examples/tasks";
        let machines = "examples/machines";
        for machine in fs::read_dir(machines).unwrap(){
            let machine = machine.unwrap().path();
            for task in fs::read_dir(tasks).unwrap() {
                let task = task.unwrap().path();
                simulate(machine.to_str().unwrap(), task.to_str().unwrap());
            }
        }
    }

    #[test]
    fn start_time_delay() {
        let machines = "examples/machines";
        for machine in fs::read_dir(machines).unwrap(){
            let machine = machine.unwrap().path();
            assert!(
                simulate(machine.to_str().unwrap(),"examples/tasks/one-task.json")
                <
                simulate(machine.to_str().unwrap(),"examples/tasks/one-task-delayed.json")
            );
        }
    }


    #[test]
    fn mono_core() {
        assert!(
            simulate("examples/machines/mono-core.json","examples/tasks/two-parallel-task.json")
            ==
            simulate("examples/machines/mono-core.json","examples/tasks/two-sequential-task.json")
        );
        assert!(
            simulate("examples/machines/mono-core.json","examples/tasks/one-task.json") * 2
            ==
            simulate("examples/machines/mono-core.json","examples/tasks/two-sequential-task.json")
        );
    }

    
    #[test]
    fn multiple_tasks() {
        assert!(
            simulate("examples/machines/dual-core.json","examples/tasks/two-parallel-task.json")
            <
            simulate("examples/machines/dual-core.json","examples/tasks/two-sequential-task.json")
        );
        assert!(
            simulate("examples/machines/mono-core.json","examples/tasks/two-sequential-task.json")
            ==
            simulate("examples/machines/dual-core.json","examples/tasks/two-sequential-task.json")
        );
    }

}