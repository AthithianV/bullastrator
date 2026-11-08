import { z } from "zod";

export const connectionSchema = z.object({
  name: z.string().min(1, "Name is required").max(50, "More 50 character is crazy bro"),
  host: z.string().min(1, "Host is required").max(100, "Are you sure your hostname has more than 100 characters?"),
  port: z.number().int().min(1).max(65535),
  username: z.string().optional().nullable(),
  password: z.string().optional().nullable(),
  db: z.number().int().min(0),
  isDefault: z.boolean(),
  bullmqPrefix: z.string().min(1, "Prefix is required").max(50, "More 50 character is crazy bro").default("bull"),
  isTlsEnabled: z.boolean().default(false),
  color: z.string().optional().nullable().default("#00BCD4"),
  label: z.string().optional().nullable(),
});

export type ConnectionSchema = typeof connectionSchema;
