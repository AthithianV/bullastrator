interface User {
  id: string;
  name: string;
  email: string;
  image: string;
  plan: "FREE" | "PRO" | "TEAMS" | "ENTERPRISE";
  maxMembers: number;
  maxWorkspaces: number;
}
