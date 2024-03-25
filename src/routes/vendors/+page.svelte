<script lang="ts">
  import DataTable from "./data-table.svelte";
  import VendorTable from "./vendor-table.svelte";
	import * as Sheet from "$lib/components/ui/sheet";
	import type { PageData } from './$types';
	
	export let data: PageData;

	const vendors = data?.vendors ?? [];
	// const payments = data?.payments ?? [];
</script>

<svelte:head>
	<title>Vendors</title>
</svelte:head>

<section class="px-10">
	<h1 class="text-xl px-10 my-4">Vendors</h1>

	<div class="container mx-auto">
		<VendorTable {vendors} />
	</div>

	<Sheet.Root>
		<Sheet.Trigger>Open</Sheet.Trigger>
		<Sheet.Content>
			<Sheet.Header>
				<Sheet.Title>Are you sure absolutely sure?</Sheet.Title>
				<Sheet.Description>
					This action cannot be undone. This will permanently delete your account
					and remove your data from our servers.
				</Sheet.Description>
			</Sheet.Header>
		</Sheet.Content>
	</Sheet.Root>	
</section>


