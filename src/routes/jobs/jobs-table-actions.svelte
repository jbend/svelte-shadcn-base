<script lang="ts">
  import Ellipsis from "lucide-svelte/icons/ellipsis";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";
  import { deleteJob } from "./store";
 
  export let id: string;

  const handleDeleteJob = () => {
    console.log("Delete job", id);
    deleteJob(id);
  }

  const handleEditJob = () => {
    console.log("Edit job", id);
    deleteJob(id);
  }

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
      <DropdownMenu.Item on:click={() => navigator.clipboard.writeText(id)}>
        Copy payment ID
      </DropdownMenu.Item>
    </DropdownMenu.Group>
    <DropdownMenu.Separator />
    <DropdownMenu.Item on:click={handleEditJob}>Edit job</DropdownMenu.Item>
    <DropdownMenu.Item on:click={handleDeleteJob}>Delete job</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>