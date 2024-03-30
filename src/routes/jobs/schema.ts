import { z } from "zod";
 
export const createJobSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(2).max(50),
});

export const editJobSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(2).max(50),
});

export type CreateJobSchema = typeof createJobSchema;
export type EditJobSchema = typeof editJobSchema;
