import { z } from "zod";

export const workspaceSchema = z.object({
    name: z.string().min(1, "Name is required").max(50),
    icon: z.string().optional().nullable(),
    color: z
        .string()
        .regex(/^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/, "Invalid hex color")
        .default("#00CADB"),
    isDefault: z.boolean().default(false),
});

export type WorkspaceSchema = typeof workspaceSchema;
