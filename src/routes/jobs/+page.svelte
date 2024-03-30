<script lang="ts">
	import { onMount } from 'svelte';
  // import { createJobDialogOpen } from './store';
  import { listen } from '@tauri-apps/api/event';
	// import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from "svelte-sonner";
  import { Button } from "$lib/components/ui/button";

  import JobsTable from "./jobs-table.svelte";
  import NewJobDialog from "./new-job-dialog.svelte";

	import type { PageData } from './$types';
	// import type { Job } from './models';
	import { refreshJobs } from './store';
	
	export let data: PageData;

	const createJobForm = data?.createJobForm ?? {};
	const editJobForm = data?.editJobForm ?? {};

	onMount(async () => {

    refreshJobs();

		await listen('job_created', async () => {
			toast("Job created");
      refreshJobs();
	  });

		await listen('job_deleted', async () => {
			toast("Job deleted");
      refreshJobs();
	  });

		await listen('job_updated', async () => {
			toast("Job updated");
      refreshJobs();
	  });

  });

  const onRefresh = async () => {
    refreshJobs();
  }

</script>

<svelte:head>
	<title>Jobs</title>
</svelte:head>

<section class="">
	<div class="container flex justify-between items-center">
		<h1 class="text-xl">Jobs</h1>
    <NewJobDialog {createJobForm} />
	</div>

	<div class="container">
    <JobsTable />
	</div>

  <div class="mt-4">
    <Button on:click={onRefresh} variant="outline">Refresh</Button>
  </div>

</section>
