use std::collections::VecDeque;
use std::io;
/*
1. We get the number of processes from the user.
2. We get the details of each process from the user.
3. We calculate the average turnaround time and average waiting time of the processes using the round robin algorithm.
4. We print the average turnaround time and average waiting time of the processes.
5. We print the details of each process. 
*/

struct Process {
    name: String,
    execution_time: u32,
    arrival_time: u32,
}


// function to calculate the average turnaround time and average waiting time of the processes by using the round robin algorithm
fn round_robin(processes: &[Process], time_slice: u32) -> (f32, f32) {
    // we create a vector to store the waiting time of each process
    let mut waiting_time: Vec<u32> = vec![0; processes.len()];
    // we create a vector to store the remaining time of each process
    let mut remaining_time: Vec<_> = processes
        .iter()
        .map(|p: &Process| p.execution_time)
        .collect();
    // we create a variable to store the current time
    let mut current_time: u32 = 0;
    // we create a queue to store the processes
    let mut queue: VecDeque<usize> = VecDeque::new();
    // we create a variable to store the number of completed processes
    let mut completed: usize = 0;
// we loop until all the processes are completed
    while completed != processes.len() {
        // we loop through the processes and push the processes that have arrived into the queue
        for (i, &time) in remaining_time.iter().enumerate() {
            if time > 0 && processes[i].arrival_time <= current_time {
                queue.push_back(i);
            }
        }

        // we pop the first process from the queue and execute it for the time slice
        if let Some(process_index) = queue.pop_front() {
            // if the remaining time of the process is less than or equal to the time slice, we add the remaining time to the current time and calculate the waiting time of the process
            if remaining_time[process_index] <= time_slice {
                current_time += remaining_time[process_index];
                waiting_time[process_index] += current_time - processes[process_index].arrival_time;
                remaining_time[process_index] = 0;
                completed += 1;
            } else {
                // if the remaining time of the process is greater than the time slice, we add the time slice to the current time and subtract the time slice from the remaining time of the process and push the process into the queue
                current_time += time_slice;
                remaining_time[process_index] -= time_slice;
                queue.push_back(process_index);
            }
        } else {
            // if the queue is empty, we add 1 to the current time
            current_time += 1;
        }
    }

    // we calculate the turnaround time and waiting time of each process
    let turnaround_time: f32 = waiting_time
        .iter()
        .zip(processes.iter())
        .map(|(&wt, p)| (wt + p.execution_time - p.arrival_time) as f32)
        .sum();
    let waiting_time: f32 = waiting_time.iter().sum::<u32>() as f32;

    // we calculate the average turnaround time and average waiting time of the processes and return them
    let average_turnaround_time = turnaround_time / processes.len() as f32;
    let average_waiting_time = waiting_time / processes.len() as f32;

    (average_turnaround_time, average_waiting_time)
}


fn fcfs(processes: &[Process]) -> (f32, f32) {
    let num_processes = processes.len() as u32;
    let mut turnaround_times: Vec<u32> = Vec::new();
    let mut waiting_times: Vec<u32> = Vec::new();
    let mut current_time = 0;

    for process in processes {
        let waiting_time = if current_time >= process.arrival_time {
            current_time - process.arrival_time
        } else {
            0
        };
        let turnaround_time = waiting_time + process.execution_time;

        waiting_times.push(waiting_time);
        turnaround_times.push(turnaround_time);

        current_time += process.execution_time;
    }

    let total_turnaround_time: u32 = turnaround_times.iter().sum();
    let avg_turnaround_time = total_turnaround_time as f32 / num_processes as f32;

    let total_waiting_time: u32 = waiting_times.iter().sum();
    let avg_waiting_time = total_waiting_time as f32 / num_processes as f32;

    (avg_turnaround_time, avg_waiting_time)
}


fn main() {
    // we get the number of processes from the user
    println!("Enter the number of processes:");
    let mut num_processes = String::new();
    io::stdin()
        .read_line(&mut num_processes)
        .expect("Failed to read input.");
    let num_processes: usize = num_processes.trim().parse().expect("Invalid input.");

    let mut processes = Vec::new();

    for i in 0..num_processes {
        println!("Enter details for process {}:", i + 1);



        // we get the name, execution time and arrival time of the process from the user
        println!("Process Name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input.");
        let name = name.trim().to_string();

        println!("Execution Time:");
        let mut execution_time = String::new();
        io::stdin()
            .read_line(&mut execution_time)
            .expect("Failed to read input.");
        let execution_time: u32 = execution_time.trim().parse().expect("Invalid input.");

        println!("Arrival Time:");
        let mut arrival_time = String::new();
        io::stdin()
            .read_line(&mut arrival_time)
            .expect("Failed to read input.");
        let arrival_time: u32 = arrival_time.trim().parse().expect("Invalid input.");


        // we push the process into the vector
        processes.push(Process {
            name,
            execution_time,
            arrival_time,
        });

        println!();
    }


    let time_slice = 1;
    processes.sort_by_key(|p| p.arrival_time);

    // we call the round_robin function to calculate the average turnaround time and average waiting time of the processes
    let (average_turnaround_time, average_waiting_time) = round_robin(&processes, time_slice);
    let (average_turnaround_time_fcfs, average_waiting_time_fcfs) = fcfs(&processes);

    // we print the average turnaround time and average waiting time of the processes
    println!("Average Turnaround Time: {:.2}", average_turnaround_time);
    println!("Average Waiting Time: {:.2}", average_waiting_time);


    println!("Average Turnaround Time FCFS: {:.2}", average_turnaround_time_fcfs);
    println!("Average Waiting Time FCFS: {:.2}", average_waiting_time_fcfs);

    //print processes
    println!("Process Name\tExecution Time\tArrival Time");
    for i in 0..num_processes {
        println!(
            "{}\t\t{}\t\t{}",
            processes[i].name, processes[i].execution_time, processes[i].arrival_time
        );
    }
}