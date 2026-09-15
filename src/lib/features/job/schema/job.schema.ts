import { z } from "zod";

// Helper to validate JSON string
const jsonString = z.string().refine(
  (val) => {
    try {
      JSON.parse(val);
      return true;
    } catch (e) {
      return false;
    }
  },
  { message: "Invalid JSON format" }
);

export const addJobSchema = z.object({
  name: z.string().min(1, "Job name is required").default("New Job"),
  // Common BullMQ Options
  opts: z.object({
    delay: z.coerce.number().min(0).optional().default(0),
    attempts: z.coerce.number().min(1).optional().default(1),
    priority: z.coerce.number().min(1).optional(),
  }),
  // The data payload (entered as string in editor, sent as object)
  data: jsonString.default("{}\n\n\n\n\n\n\n\n"),
});

export type AddJobSchema = z.infer<typeof addJobSchema>;
