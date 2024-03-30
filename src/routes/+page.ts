import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export const load: PageLoad = async ({ depends }) => {

  const countVendors: number = await invoke('count_vendors');
  const countJobs: number = await invoke('count_jobs');

  depends('app:root');

	return {
    countVendors,
    countJobs,
	};
};
