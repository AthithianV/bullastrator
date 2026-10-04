import { PUBLIC_API_URL } from "$env/static/public";
import { JOB_ROUTES } from "./jobs";
import { QUEUE_ROUTES } from "./queues";
import { TAB_ROUTES } from "./tabs";
import { WORKSPACE_ROUTES } from "./workspace";

const apiBaseUrl = (PUBLIC_API_URL || "http://127.0.0.1:3000").replace(
  /\/$/,
  "",
);

export type CommandArgs = Record<string, any>;
export type HttpMethod = "GET" | "POST" | "PATCH" | "DELETE";

export interface WebCommand {
  method: HttpMethod;
  path: (args: CommandArgs) => string;
  query?: (args: CommandArgs) => Record<string, unknown>;
  body?: (args: CommandArgs) => unknown;
}

export const id = (value: unknown, name: string): string => {
  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`Missing ${name}`);
  }
  return encodeURIComponent(value);
};

export const snakeCase = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(snakeCase);
  if (!value || typeof value !== "object") return value;

  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>).map(([key, item]) => [
      key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`),
      snakeCase(item),
    ]),
  );
};

/** Maps the existing desktop command names to the web API. */
export const webCommandMap: Record<string, WebCommand> = {
  ...JOB_ROUTES,
  ...WORKSPACE_ROUTES,
  ...QUEUE_ROUTES,
  ...TAB_ROUTES,
};

export async function callWebApi<T>(
  cmd: string,
  args: CommandArgs,
): Promise<T> {
  const route = webCommandMap[cmd];
  if (!route) throw new Error(`No web API mapping exists for command: ${cmd}`);

  const query = route.query?.(args);
  const queryString = query
    ? new URLSearchParams(
        Object.entries(query)
          .filter(([, value]) => value !== undefined && value !== null)
          .map(([key, value]) => [key, String(value)]),
      ).toString()
    : "";
  const path = `${apiBaseUrl}${route.path(args)}${queryString ? `?${queryString}` : ""}`;
  const body = route.body?.(args);

  const response = await fetch(path, {
    method: route.method,
    headers:
      body === undefined ? undefined : { "content-type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const contentType = response.headers.get("content-type") || "";
  const result = contentType.includes("application/json")
    ? await response.json()
    : await response.text();

  if (!response.ok) {
    const message =
      typeof result === "object" && result && "error" in result
        ? String((result as { error: unknown }).error)
        : String(result || response.statusText);
    throw new Error(message);
  }
  return result as T;
}
