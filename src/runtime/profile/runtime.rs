use crate::runtime::{
    job::task::process::Process, job::task::Task, job::Job, profile::Profile, Runtime,
};
use log::debug;
use std::collections::HashMap;

pub fn runtime_from_job_names(
    profile: Profile,
    job_names: Vec<String>,
) -> Result<Runtime, std::io::Error> {
    debug!("Building runtime object from profile data");

    let Profile { jobs, processes } = profile;

    let process_lib: HashMap<String, Process> = processes
        .into_iter()
        .map(|(name, v)| {
            let process = Process::new(Process {
                name: name.clone(),
                command: v.command,
                cwd: v.cwd,
                onfail: match &v.onfail[..] {
                    "" => None,
                    _ => Some(v.onfail),
                },
                onsucceed: match &v.onsucceed[..] {
                    "" => None,
                    _ => Some(v.onsucceed),
                },
                silent: false,
            });

            (name, process)
        })
        .collect();

    let jobs = job_names
        .iter()
        .map(|job_name| {
            let job = match jobs.get(&job_name[..]) {
                Some(job) => job,
                None => panic!("runtime_from_job_names"),
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

                                    Process::new(Process {
                                        name: default_process.name.clone(),
                                        command: default_process.command.clone(),
                                        cwd: default_process.cwd.clone(),
                                        onfail: match &process.onfail {
                                            Some(onfail) => Some(onfail.into()),
                                            None => default_process.onfail.clone(),
                                        },
                                        onsucceed: match &process.onsucceed {
                                            Some(onsucceed) => Some(onsucceed.into()),
                                            None => default_process.onsucceed.clone(),
                                        },
                                        silent: process.silent,
                                    })
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    let runtime = Runtime::new().jobs(jobs).process_lib(process_lib);

    Ok(runtime)
}
