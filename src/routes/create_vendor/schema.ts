import { z } from "zod";
 
export const formSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(2).max(50),
  contact: z.string().min(2).max(50),
  email: z.string().email(),
  phone: z.string().min(10).max(15),
});

export type FormSchema = typeof formSchema;
