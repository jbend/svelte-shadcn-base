<script lang="ts">
  import Ellipsis from "lucide-svelte/icons/ellipsis";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";
  import { invoke } from '@tauri-apps/api/tauri';
 
  export let id: string;

  const handleViewVendor = () => {
    console.log(`View vendor ${id}`);
  };

  const handleDeleteVendor = async () => {
    console.log(`Delete vendor: ${id}`);
    await invoke('delete_vendor', { vendorId: id });
  };

  const handleCreateVendorPO = async () => {
    console.log(`Create new PO from this vendor ${id}`);
    // await invoke('delete_vendor', { vendorId: id });
  };

  
</script>
 
<DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
    <Button
      variant="ghost"
      builders={[builder]}
      size="icon"
      class="relative h-8 w-8 p-0"
    >
      <span class="sr-only">Open menu</span>
      <Ellipsis class="h-4 w-4" />
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Group>
      <DropdownMenu.Label>Actions</DropdownMenu.Label>
      <DropdownMenu.Item on:click={handleCreateVendorPO}>Create new PO for this vendor</DropdownMenu.Item>
      <DropdownMenu.Item on:click={() => navigator.clipboard.writeText(id)}>
        Copy payment ID
      </DropdownMenu.Item>
    </DropdownMenu.Group>
    <DropdownMenu.Separator />
    <DropdownMenu.Item on:click={handleViewVendor}>View vendor</DropdownMenu.Item>
    <DropdownMenu.Item on:click={handleDeleteVendor}>Delete vendor</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>