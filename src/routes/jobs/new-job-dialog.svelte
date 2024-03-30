<script lang="ts">
  import {
    Button,
    buttonVariants
  } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import CirclePlus from "lucide-svelte/icons/circle-plus";

  import * as Form from "$lib/components/ui/form";
  import { createJobSchema, type CreateJobSchema } from "./schema";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { createJobDialogOpen, createJob } from './store';
  import type { CreateJob } from './models';

  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";

  export let createJobForm: SuperValidated<Infer<CreateJobSchema>>;

  const form = superForm(createJobForm, {
    validators: zodClient(createJobSchema),
  });

  async function handleSubmit(event: { currentTarget: EventTarget & HTMLFormElement }) {
		const data = Object.fromEntries(new FormData(event.currentTarget).entries());

    console.log("Data:", JSON.stringify(data));
    const job: CreateJob = {
      name: data.name.toString(),
    }
    createJob(job);
    // const result = await invoke('create_job', { jobArg: job });
    createJobDialogOpen.set(false);
	}

  const { form: formData, enhance } = form;

</script>
 
<Dialog.Root bind:open={$createJobDialogOpen}>
  <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
    <CirclePlus class="w-6 h-6 mr-2" />
  </Dialog.Trigger>  
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Create job</Dialog.Title>
      <Dialog.Description>
        Create a new job by giving it a name.
      </Dialog.Description>
    </Dialog.Header>

    <form method="POST" use:enhance on:submit|preventDefault={handleSubmit}>
      <Form.Field {form} name="name">
        <Form.Control let:attrs>
          <Form.Label>Name</Form.Label>
          <Input {...attrs} bind:value={$formData.name} />
        </Form.Control>
        <Form.Description>Job name</Form.Description>
        <Form.FieldErrors />
      </Form.Field>
      <Form.Button class="mt-4">Create job</Form.Button>
    </form>
    
  </Dialog.Content>
</Dialog.Root>