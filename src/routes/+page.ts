import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export const load: PageLoad = async ({ depends }) => {

  console.log('root page load');

  
  const vendorCount: number = await invoke('count_vendors');

  depends('app:root');

	return {
    vendorCount,
	};
};
