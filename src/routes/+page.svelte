<script lang="ts">
  import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
  import DollarSign from "lucide-svelte/icons/dollar-sign";
  import Users from "lucide-svelte/icons/users";
	
	import type { PageData } from './$types';
	
	export let data: PageData;

	const vendorCount = data?.countVendors ?? 0;
	const jobCount = data?.countJobs ?? 0;
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<div>
		<h1 class="container text-xl">Dashboard</h1>
	</div>
	<div class="container mt-6 flex justify-items-stretch gap-4">
		<Card.Root class="basis-1/3">
			<Card.Header
				class="flex flex-row items-center justify-between space-y-0 pb-2"
			>
				<Card.Title class="text-sm font-medium">Jobs</Card.Title>
				<Users class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-5xl font-bold">{ jobCount }</div>
			</Card.Content>
		</Card.Root>

		<Card.Root class="basis-1/3">
			<Card.Header
				class="flex flex-row items-center justify-between space-y-0 pb-2"
			>
				<Card.Title class="text-sm font-medium">Total Revenue</Card.Title>
				<DollarSign class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">$45,231.89</div>
				<p class="text-xs text-muted-foreground">+20.1% from last month</p>
			</Card.Content>
		</Card.Root>

		<Card.Root class="basis-1/3">
			<Card.Header
				class="flex flex-row items-center justify-between space-y-0 pb-2"
			>
				<Card.Title class="text-sm font-medium">Vendors</Card.Title>
				<Users class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-5xl font-bold">{ vendorCount }</div>
			</Card.Content>
		</Card.Root>
	</div>
</section>

