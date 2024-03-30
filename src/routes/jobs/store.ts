import { writable } from 'svelte/store';
import type { Job } from './models';
import { invoke } from '@tauri-apps/api/tauri';

export const jobs = writable<Job[]>([]);

export const refreshJobs = async () => {
  jobs.set(await invoke('list_jobs'));  
}

export const addJob = (job: Job) => {
  jobs.update((j) => [...j, job]);
}

export const deleteJob = async (id: string) => {
  await invoke('delete_job', { jobId: id });  
}


export const createJobDialogOpen = writable(false);
export const editJobDialogOpen = writable(false);
