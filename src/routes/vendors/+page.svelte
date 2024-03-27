<script lang="ts">
  import VendorTable from "./vendor-table.svelte";
	import * as Sheet from "$lib/components/ui/sheet";
	import { Label } from "$lib/components/ui/label";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
  import VendorSheet from "./vendor-sheet.svelte";


	import type { PageData } from './$types';
	
	export let data: PageData;
 
	const vendors = data?.vendors ?? [];
	const createVendorForm = data?.createVendorForm ?? {};
	
</script>

<svelte:head>
	<title>Vendors</title>
</svelte:head>

<section class="px-8">
	<div class="px-10 flex justify-between items-center">
		<h1 class="text-xl">Vendors</h1>
		<VendorSheet {createVendorForm} />
	</div>

	<div class="container mx-auto">
		<VendorTable {vendors} />
	</div>



</section>


