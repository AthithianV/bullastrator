import { z } from "zod";

export const connectionSchema = z.object({
  name: z.string().min(1, "Name is required"),
  host: z.string().min(1, "Host is required"),
  port: z.number().int().min(1).max(65535),
  username: z.string().optional().nullable(),
  password: z.string().optional().nullable(),
  db: z.number().int().min(0),
  isDefault: z.boolean(),
  bullmqPrefix: z.string().min(1, "Prefix is required").default("bull"),
  isTlsEnabled: z.boolean().optional().nullable().default(false),
  color: z.string().optional().nullable().default("#00BCD4"),
  label: z.string().optional().nullable(),
});

export type ConnectionSchema = typeof connectionSchema;
