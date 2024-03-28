<script lang="ts">
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import CirclePlus from "lucide-svelte/icons/circle-plus";
  import { vendorSheetOpen } from "./store";

  import * as Form from "$lib/components/ui/form";
  import { formSchema, type FormSchema } from "./schema";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { invoke } from '@tauri-apps/api/tauri';

  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";

  export let createVendorForm: SuperValidated<Infer<FormSchema>>;

  const form = superForm(createVendorForm, {
    validators: zodClient(formSchema),
  });

  async function handleSubmit(event: { currentTarget: EventTarget & HTMLFormElement }) {
		const data = Object.fromEntries(new FormData(event.currentTarget).entries());

    console.log("Data:", JSON.stringify(data));
    const vendor = {
      name: data.name,
      contact: data.contact,
      email: data.email,
      phone: data.phone
    }
    const result = await invoke('create_vendor', { vendorArg: vendor });

    vendorSheetOpen.set(false)
    console.log("Result:", result);
    // goto('/vendors');
	}
 
  const { form: formData, enhance } = form;

</script>
<Sheet.Root bind:open={$vendorSheetOpen}>
  <Sheet.Trigger asChild let:builder>
    <Button builders={[builder]} variant="outline">
      <CirclePlus class="h-4 w-4 mr" />  
    </Button>
  </Sheet.Trigger>
  <Sheet.Content side="right">
    <Sheet.Header>
      <Sheet.Title>Create vendor</Sheet.Title>
      <Sheet.Description>
        Fill in the vendor details and click save when done.
      </Sheet.Description>
    </Sheet.Header>

    <form method="POST" use:enhance on:submit|preventDefault={handleSubmit}>
      <Form.Field {form} name="name">
        <Form.Control let:attrs>
          <Form.Label>Name</Form.Label>
          <Input {...attrs} bind:value={$formData.name} />
        </Form.Control>
        <Form.Description>This is your public display name.</Form.Description>
        <Form.FieldErrors />
      </Form.Field>
  
      <Form.Field {form} name="contact">
        <Form.Control let:attrs>
          <Form.Label>Contact</Form.Label>
          <Input {...attrs} bind:value={$formData.contact} />
        </Form.Control>
        <Form.Description>Person you typically communicate with</Form.Description>
        <Form.FieldErrors />
      </Form.Field>
  
      <Form.Field {form} name="email">
        <Form.Control let:attrs>
          <Form.Label>Email</Form.Label>
          <Input {...attrs} bind:value={$formData.email} />
        </Form.Control>
        <Form.Description></Form.Description>
        <Form.FieldErrors />
      </Form.Field>
    
      <Form.Field {form} name="phone">
        <Form.Control let:attrs>
          <Form.Label>Phone</Form.Label>
          <Input {...attrs} bind:value={$formData.phone} />
        </Form.Control>
        <Form.Description></Form.Description>
        <Form.FieldErrors />
      </Form.Field>
      <Form.Button class="mt-4">Save</Form.Button>
    </form>

  </Sheet.Content>
</Sheet.Root>