<script lang="ts">
	import { onMount } from 'svelte';
	import { goto, invalidate, invalidateAll } from '$app/navigation';
  import { vendorSheetOpen } from './store';

	import * as Sheet from "$lib/components/ui/sheet";
	import { Label } from "$lib/components/ui/label";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import { toast } from "svelte-sonner";
	
	// import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
	
  import VendorTable from "./vendor-table.svelte";
  import VendorSheet from "./vendor-sheet.svelte";


	import type { PageData } from './$types';
	
	export let data: PageData;
 
	const vendors = data?.vendors ?? [];
	const createVendorForm = data?.createVendorForm ?? {};

	onMount(async () => {

		await listen('vendor_deleted', async () => {
  	  console.log('Vendor deleted');
			toast("Vendor deleted")
			goto('/vendors');
			// invalidate('app:vendors');
			// invalidateAll();
	  });
		
		await listen('vendor_created', async () => {
			console.log('Vendor created');
			$vendorSheetOpen = false;
			toast("Vendor created")

			// goto('/vendors');
			// window.location.reload();
			// invalidate('app:vendors');
			// invalidateAll();
	  });
		
	});
	
	const reloadPage = () => {
		console.log('Reloading page');
		goto('/vendors');
		// window.location.reload();
		// invalidate('app:vendors');
	}


	
</script>

<svelte:head>
	<title>Vendors</title>
</svelte:head>

<section class="">
	<div class="container flex justify-between items-center">
		<h1 class="text-xl">Vendors</h1>
		<VendorSheet {createVendorForm} />
	</div>

	<div class="container">
		<VendorTable {vendors} />
	</div>

	<div class="container mt-6">
		<Button variant="outline" on:click={reloadPage}>Invalidate</Button>
	</div>

</section>


