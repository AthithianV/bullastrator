export interface ReadWorkspaceModel {
  id: string;
  name: string;
  icon: string | null;
  color: string | null;
  role: string;
  isDefault?: boolean;
}

export interface CreateWorkspaceModel {
  name: string;
  icon?: string | null;
  color?: string | null;
}

export interface UpdateWorkspaceModel {
  name?: string;
  icon?: string | null;
  color?: string | null;
  isDefault?: boolean;
}
