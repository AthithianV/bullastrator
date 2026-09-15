export interface ReadWorkspaceModel {
  id: string;
  name: string;
  icon: string | null;
  color: string | null;
  isDefault: boolean;
  createdAt: string;
  isGuestMode: boolean;
  isPrimary: boolean;
  plan: string;
  role: string;
  maxConnections: number;
}

export interface CreateWorkspaceModel {
  name: string;
  icon?: string | null;
  color?: string | null;
  isDefault: boolean;
}

export interface UpdateWorkspaceModel {
  name?: string;
  icon?: string | null;
  color?: string | null;
  isDefault?: boolean;
}
