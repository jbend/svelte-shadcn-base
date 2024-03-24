<script lang="ts">
  import DataTable from "./data-table.svelte";
	import type { PageData } from './$types';
	
	export let data: PageData;

	const vendors = data?.vendors ?? [];
	const payments = data?.payments ?? [];
</script>

<svelte:head>
	<title>Vendors</title>
</svelte:head>

<section class="px-10">
	<h1 class="text-xl px-10 my-4">Vendors</h1>

	<div class="container mx-auto">
		<DataTable {payments} />
	</div>
</section>


