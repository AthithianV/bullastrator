export interface RegisterRequest {
  name: string;
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface User {
  id: string;
  name: string;
  email: string;
  image: string | null;
  role: "USER" | "GLOBAL_ADMIN";
}

export interface UserRole {
  id: string;
  userId: string;
  entityType: "WORKSPACE" | "CONNECTION" | "FOLDER" | "QUEUE";
  entityId: string;
  entityName: string;
  role: string;
}

export interface RoleEntityOption {
  id: string;
  name: string;
}

export interface AuthResponse {
  user: User;
  token: string;
  expiresAt: string;
}

export interface AuthenticatedSession {
  id: string;
}

export interface LogoutResponse {
  revoked: boolean;
}
