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

  color?: string;
  label?: string;
}

export interface CreateConnection {
  name: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
  db?: number;
  isDefault?: boolean;
  bullmqPrefix: string;

  color?: string;
  label?: string;
}

export interface UpdateConnection {
  name?: string;
  host?: string;
  port?: number;
  username?: string;
  password?: string;
  db?: number;
  isDefault?: boolean;
  bullmqPrefix: string;

  color?: string;
  label?: string;
}

export interface ConnectionHealth {
  id: string;
  isActive: boolean;
}
