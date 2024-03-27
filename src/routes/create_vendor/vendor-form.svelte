<script lang="ts">
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";
  import { formSchema, type FormSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";

	import { invalidateAll, goto } from '$app/navigation';
	// import { applyAction, deserialize } from '$app/forms';
	// import type { ActionData } from './$types';
	// import type { ActionResult } from '@sveltejs/kit';
  import { invoke } from '@tauri-apps/api/tauri';

  export let data: SuperValidated<Infer<FormSchema>>;
 
  const form = superForm(data, {
    validators: zodClient(formSchema),
  });

  async function handleSubmit(event: { currentTarget: EventTarget & HTMLFormElement }) {
		const data = Object.fromEntries(new FormData(event.currentTarget).entries());
//    const jsonData = Object.fromEntries(data.entries());

    console.log("Data:", JSON.stringify(data));
    // console.log("jsonData:", jsonData);
    const vendor = {
      name: data.name,
      contact: data.contact,
      email: data.email,
      phone: data.phone
    }

    const result = await invoke('create_vendor', { vendorArg: vendor });

    console.log("Result:", result);

		// if (result.type === 'success') {
		// 	// rerun all `load` functions, following the successful update
		// 	await invalidateAll();
		// }

		// applyAction(result);


    goto('/vendors');
	
	}
 
  const { form: formData, enhance } = form;
</script>
 
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

  <Form.Button>Submit</Form.Button>
</form>