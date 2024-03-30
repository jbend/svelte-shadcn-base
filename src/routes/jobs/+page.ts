import type { PageLoad } from './$types';

import { superValidate } from "sveltekit-superforms";
import { createJobSchema, editJobSchema } from "./schema";
import { zod } from "sveltekit-superforms/adapters";

export const load: PageLoad = async () => {
  
  const createJobForm = await superValidate(zod(createJobSchema));
  const editJobForm = await superValidate(zod(editJobSchema));

	return {
    createJobForm,
    editJobForm
	};
};