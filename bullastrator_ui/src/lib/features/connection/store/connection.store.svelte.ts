import type { ReadConnection } from "connection/interface/connection.types";

class ConnectionStore {
  connections = $state<ReadConnection[]>([]);
  connectionsHealth = $state<Record<string, boolean>>({});
}

export const connectionStore = new ConnectionStore();
