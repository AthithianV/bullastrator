export interface BillingSubscription {
  id: string;
  userId: string;
  plan: string;
  status: "active" | "canceled" | "past_due" | "unpaid" | "incomplete" | "incomplete_expired" | "trialing" | "paused";
  cancelledAt: string | null;
  cancelAtPeriodEnd: string;
  currentPeriodEnd: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface BillingData {
  subscriptions: BillingSubscription[];
}
