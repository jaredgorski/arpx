use crate::runtime::{
    job::{
        task::{log_monitor::LogMonitor, process::Process, Task},
        Job,
    },
    profile::Profile,
    Runtime,
};
use log::debug;
use std::collections::HashMap;

pub fn runtime_from_profile(
    profile: Profile,
    job_names: Vec<String>,
) -> Result<Runtime, std::io::Error> {
    debug!("Building runtime object from profile data");

    let Profile {
        jobs,
        processes,
        log_monitors,
    } = profile;

    let log_monitor_lib: HashMap<String, LogMonitor> = log_monitors
        .into_iter()
        .map(|(name, v)| {
            let log_monitor = LogMonitor::new(name.clone())
                .buffer_size(v.buffer_size)
                .ontrigger(v.ontrigger)
                .test(v.test)
                .variable_pattern(v.variable_pattern);

            (name, log_monitor)
        })
        .collect();

    let process_lib: HashMap<String, Process> = processes
        .into_iter()
        .map(|(name, v)| {
            let process = Process::new(name.clone())
                .command(v.command)
                .cwd(v.cwd)
                .log_monitors(v.log_monitors)
                .onfail(match &v.onfail[..] {
                    "" => None,
                    _ => Some(v.onfail),
                })
                .onsucceed(match &v.onsucceed[..] {
                    "" => None,
                    _ => Some(v.onsucceed),
                });

            (name, process)
        })
        .collect();

    let jobs = job_names
        .iter()
        .map(|job_name| {
            let job = match jobs.get(&job_name[..]) {
                Some(job) => job,
                None => panic!("runtime_from_profile"),
            };

            Job::new(
                job_name.into(),
                job.tasks
                    .iter()
                    .map(|task| {
                        Task::new(
                            task.processes
                                .iter()
                                .map(|process| {
                                    let default_process = &process_lib[&process.name[..]];

                                    Process::new(default_process.name.clone())
                                        .command(default_process.command.clone())
                                        .cwd(default_process.cwd.clone())
                                        .log_monitors(process.log_monitors.clone())
                                        .onfail(match &process.onfail {
                                            Some(onfail) => Some(onfail.into()),
                                            None => default_process.onfail.clone(),
                                        })
                                        .onsucceed(match &process.onsucceed {
                                            Some(onsucceed) => Some(onsucceed.into()),
                                            None => default_process.onsucceed.clone(),
                                        })
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    let runtime = Runtime::new()
        .jobs(jobs)
        .log_monitor_lib(log_monitor_lib)
        .process_lib(process_lib);

    Ok(runtime)
}
