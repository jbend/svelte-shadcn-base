import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

import { superValidate } from "sveltekit-superforms";
import { formSchema } from "./schema";
import { zod } from "sveltekit-superforms/adapters";


export type Vendor = {
  id: string;
  name: string;
  contact: string;
  email: string;
  phone: string;
  create_at: string;
};

export type Payment = {
  id: string;
  amount: number;
  status: "pending" | "processing" | "success" | "failed";
  email: string;
};

export const load: PageLoad = async ({ depends }) => {

  const createVendorForm = await superValidate(zod(formSchema));

  const vendors: Vendor[] = await invoke('list_vendors');

  depends('app:vendors');

	return {
    vendors,
    createVendorForm,
	};
};