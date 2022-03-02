use crate::runtime::{
    job::{
        task::{action::BUILTIN_ACTIONS, log_monitor::LogMonitor, process::Process, Task},
        Job,
    },
    profile::{deserialize, Profile},
    Runtime,
};
use anyhow::{ensure, Context, Error, Result};
use log::debug;
use std::{collections::HashMap, env::var, path::Path};

pub struct RuntimeBuilder;

impl RuntimeBuilder {
    pub fn from_profile_and_job_names(profile: Profile, job_names: &[String]) -> Result<Runtime> {
        debug!("Building runtime object from profile data");

        debug!("Building log_monitor_lib");

        let log_monitor_lib = Self::build_log_monitor_lib(profile.log_monitors);

        ensure!(
            log_monitor_lib.len()
                <= var("ARPX_LOG_MONITORS_MAX")
                    .unwrap_or_else(|_| "200".to_owned())
                    .parse::<usize>()
                    .unwrap_or(200),
            "Too many log_monitors defined in profile"
        );

        debug!("Building process_lib");

        let process_lib = Self::build_process_lib(profile.processes)?;

        ensure!(
            process_lib.len()
                <= var("ARPX_PROCESSES_MAX")
                    .unwrap_or_else(|_| "200".to_owned())
                    .parse::<usize>()
                    .unwrap_or(200),
            "Too many processes defined in profile"
        );

        ensure!(
            !process_lib.is_empty(),
            "No valid processes exist in profile"
        );

        debug!("Building jobs object");

        ensure!(!job_names.is_empty(), "No jobs requested for runtime");

        let jobs = job_names
            .iter()
            .map(|job_name| {
                let job = profile.jobs
                    .get(&job_name[..])
                    .context(format!("Requested job \"{}\" not defined in jobs", job_name))?;

                return Ok(Job::new(
                    job_name.into(),
                    job.tasks
                        .iter()
                        .enumerate()
                        .map(|(i, task)| {
                            let task_index = i + 1;

                            ensure!(
                                task.processes.len()
                                    <= var("ARPX_CONCURRENT_PROCESSES_MAX")
                                        .unwrap_or_else(|_| "500".to_owned())
                                        .parse::<usize>()
                                        .unwrap_or(500),
                                "Job \"{}\", task {}: too many processes",
                                job_name,
                                task_index
                            );

                            return Ok(Task::new(
                                task.processes
                                    .iter()
                                    .map(|process| {
                                        let default_process =
                                            process_lib.get(&process.name[..]).context(format!(
                                                "Job \"{}\", task {}: process \"{}\" not defined in processes",
                                                job_name,
                                                task_index,
                                                process.name
                                            ))?;

                                        ensure!(
                                            task.processes.len() + process.log_monitors.len()
                                                <= var("ARPX_THREAD_MAX")
                                                    .unwrap_or_else(|_| "500".to_owned())
                                                    .parse::<usize>()
                                                    .unwrap_or(500),
                                            "Job \"{}\", task {}: too many threads (reduce processes or log_monitors on task)",
                                            job_name,
                                            task_index
                                        );

                                        for log_monitor in &process.log_monitors {
                                            ensure!(
                                                log_monitor_lib.contains_key(log_monitor),
                                                "Job \"{}\", task {}: log monitor \"{}\" not defined in log_monitors",
                                                job_name,
                                                task_index,
                                                log_monitor
                                            );
                                        }

                                        return Ok(Process::new(default_process.name.clone())
                                            .command(default_process.command.clone())
                                            .cwd(default_process.cwd.clone())
                                            .log_monitors(process.log_monitors.clone())
                                            .onfail(match &process.onfail {
                                                Some(onfail) => {
                                                    ensure!(
                                                        process_lib.contains_key(onfail) || BUILTIN_ACTIONS.contains(&&onfail[..]),
                                                        "Job \"{}\", task {}: invalid onfail \"{}\" provided",
                                                        job_name,
                                                        task_index,
                                                        onfail
                                                    );

                                                    Some(onfail.into())
                                                }
                                                None => default_process.onfail.clone(),
                                            })
                                            .onsucceed(match &process.onsucceed {
                                                Some(onsucceed) => {
                                                    ensure!(
                                                        process_lib.contains_key(onsucceed) || BUILTIN_ACTIONS.contains(&&onsucceed[..]),
                                                        "Job \"{}\", task {}: invalid onsucceed \"{}\" provided",
                                                        job_name,
                                                        task_index,
                                                        onsucceed
                                                    );

                                                    Some(onsucceed.into())
                                                }
                                                None => default_process.onsucceed.clone(),
                                            }))
                                    })
                                    .collect::<Result<Vec<Process>, Error>>()?,
                            ))
                        })
                        .collect::<Result<Vec<Task>, Error>>()?,
                ))
            })
            .collect::<Result<Vec<Job>, Error>>()?;

        debug!("Building runtime object");

        let runtime = Runtime::new()
            .jobs(jobs)
            .log_monitor_lib(log_monitor_lib)
            .process_lib(process_lib);

        debug!("Runtime object built");

        Ok(runtime)
    }

    pub fn build_log_monitor_lib(
        log_monitors: HashMap<String, deserialize::log_monitors::LogMonitor>,
    ) -> HashMap<String, LogMonitor> {
        log_monitors
            .into_iter()
            .map(|(name, v)| {
                let log_monitor = LogMonitor::new(name.clone())
                    .buffer_size(v.buffer_size)
                    .ontrigger(v.ontrigger)
                    .test(v.test)
                    .variable_pattern(v.variable_pattern);

                (name, log_monitor)
            })
            .collect()
    }

    pub fn build_process_lib(
        processes: HashMap<String, deserialize::processes::Process>,
    ) -> Result<HashMap<String, Process>> {
        processes
            .into_iter()
            .map(|(name, v)| {
                ensure!(
                    Path::new(&v.cwd).is_dir(),
                    "Configured current working directory \"{}\" on process \"{}\" is not a valid directory",
                    v.cwd,
                    name
                );

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

                Ok((name, process))
            })
            .collect::<Result<HashMap<String, Process>, Error>>()
    }
}
