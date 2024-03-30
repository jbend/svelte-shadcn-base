import { writable } from 'svelte/store';
import type { Job, CreateJob, 
  UpdateJobName, UpdateJobActive, UpdateJobFavorite } from './models';
import { invoke } from '@tauri-apps/api/tauri';

export const jobs = writable<Job[]>([]);

const getJob = async (id: string): Promise<Job> => {
  const job: Job = await invoke('get_job', { jobId: id });
  return job;
}

export const refreshJobs = async () => {
  jobs.set(await invoke('list_jobs'));  
}

export const createJob = async(job: CreateJob) => {
  await invoke('create_job', { jobArg: job });
}

export const updateJobName = async(job: UpdateJobName) => {
  const fre = (await getJob(job.id)).name;

  await invoke('update_job_name', { id: job.id, name: job.name});
}

export const updateJobActive = async(job: UpdateJobActive) => {
  await invoke('update_job_active', { id: job.id, active: job.active});
}

export const updateJobFavorite = async(job: UpdateJobFavorite) => {
  await invoke('update_job_favorite', { id: job.id, favorite: job.favorite});
}

export const deleteJob = async (id: string) => {
  await invoke('delete_job', { jobId: id });  
}

export const createJobDialogOpen = writable(false);
export const editJobDialogOpen = writable(false);
