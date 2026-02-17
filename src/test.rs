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
}