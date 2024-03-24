<script>
	import "../app.pcss";
  // import './styles.css';

	import { ModeWatcher } from "mode-watcher";

	import { Separator } from "$lib/components/ui/separator";

	import Header from "$lib/components//Header.svelte";
	import Footer from "$lib/components//Footer.svelte";

</script>

<ModeWatcher />

<Header />
<Separator class="my-2" />
<div class="pt-4 flex flex-grow">
	<aside class="p-4 w-48">
		<ol class="text-sm space-y-2">
			<li><a class="hover:underline" href="/">Dashboard</a></li>
			<li><a class="hover:underline" href="/purchase_orders">Purchase Orders</a></li>
		</ol>
		<Separator class="my-4" />
		<ol class="text-sm space-y-2">
			<li><a class="hover:underline" href="/vendors">Vendors</a></li>
		</ol>
	</aside>
	<main>
		<slot></slot>
	</main>
</div>
<Separator class="my-4" />
<Footer />
