import type { PageLoad } from "./$types";
// import type { PageLoad, Actions } from "./$types";
// import { fail } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { formSchema } from "./schema";
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageLoad = async () => {
  return {
    form: await superValidate(zod(formSchema)),
  };
};

// export const actions: Actions = {
//   default: async (event) => {
//     console.log("event", event);
//     const form = await superValidate(event, zod(formSchema));
//     if (!form.valid) {
//       return fail(400, {
//         form,
//       });
//     }
//     return {
//       form,
//     };
//   },
// };