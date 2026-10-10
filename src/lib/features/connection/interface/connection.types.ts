export interface ReadConnection {
  id: string;
  name: string;
  host: string;
  port: number;
  username: string | null;
  db: number;
  isDefault: boolean;
  createdAt: string;
  bullmqPrefix: string;
  isTlsEnabled?: boolean;

  color?: string;
  label?: string;
}

export interface CreateConnection {
  name: string;
  host: string;
  port: number;
  username?: string | null;
  password?: string | null;
  db?: number;
  isDefault?: boolean;
  isTlsEnabled: boolean | null;
  bullmqPrefix: string;

  color?: string | null;
  label?: string | null;
}

export interface UpdateConnection {
  name?: string;
  host?: string;
  port?: number;
  username?: string | null;
  password?: string | null;
  db?: number;
  isDefault?: boolean;
  isTlsEnabled?: boolean | null;
  bullmqPrefix: string;

  color?: string | null;
  label?: string | null;
}

export interface ConnectionHealth {
  id: string;
  isActive: boolean;
}
