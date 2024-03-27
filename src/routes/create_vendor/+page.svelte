<script lang="ts">
  import type { PageData } from "./$types.js";
  import VendorForm from "./vendor-form.svelte";
  export let data: PageData;
</script>
 
<VendorForm data={data.form} />