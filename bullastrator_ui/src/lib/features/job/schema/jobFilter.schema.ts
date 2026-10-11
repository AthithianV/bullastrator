import { z } from "zod";

export const filterJobSchema = z.object({
  jobId: z.string().trim().optional(),
  keyword: z.string().trim().optional(),
  failedReason: z.string().trim().optional(),
});

// Helper type to use in your Svelte component
export type FilterJobValues = z.infer<typeof filterJobSchema>;
