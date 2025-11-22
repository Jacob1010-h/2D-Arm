mod commands;

use std::{thread::sleep, time::Duration};

use commands::{
    command_scheduler::CommandScheduler,
    instant_command::InstantCommand,
    timed_command::TimedCommand,
    parallel_command_group::ParallelCommandGroup,
    sequential_command_group::SequentialCommandGroup,
    command_base::Boxable,
};

fn main() {
    println!("=============================");
    println!(" SIMPLE COMMAND TEST PROGRAM");
    println!("=============================\n");

    let mut scheduler = CommandScheduler::new();

    // --- Test 1: RunnableCommand ---
    scheduler.add_command(
        InstantCommand::new("SayHello", || {
            println!(">>> Hello from RunnableCommand!");
        })
    );

    // --- Test 2: TimedCommand ---
    scheduler.add_command(
        TimedCommand::new("Wait1Second", 1.0)
    );

    // --- Test 3: SequentialCommandGroup ---
    let seq = SequentialCommandGroup::new("Seq Test", vec![
        InstantCommand::new("Seq Step 1", || {
            println!(">>> Seq Step 1 Running!");
        }).boxed(),

        TimedCommand::new("Seq Wait 0.5s", 0.5).boxed(),

        InstantCommand::new("Seq Step 2", || {
            println!(">>> Seq Step 2 Running!");
        }).boxed(),
    ]);

    scheduler.add_command(seq);

    // --- Test 4: ParallelCommandGroup ---
    let par = ParallelCommandGroup::new("Parallel Test", vec![
        TimedCommand::new("Par Cmd 1 (0.8s)", 0.8).boxed(),
        TimedCommand::new("Par Cmd 2 (1.0s)", 1.0).boxed(),
    ]);

    scheduler.add_command(par);

    println!("\n--- Starting Scheduler Loop ---\n");

    // Simple "robot loop"
    loop {
        scheduler.run();
        sleep(Duration::from_millis(100));
    }
}
