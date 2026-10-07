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
}

export interface AuthResponse {
  user: User;
  token: string;
  expiresAt: string;
}

export interface LogoutResponse {
  revoked: boolean;
}
